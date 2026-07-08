use crate::{
	algo::search::{Retained, Search},
	core::{property::VertexIn, Ensure, Graph, GraphDeref},
};
use std::borrow::Borrow;

/// Performs [depth-first search](https://mathworld.wolfram.com/Depth-FirstTraversal.html)
/// of a graph's vertices.
///
/// ### Simple Usage
///
/// ```
/// # use graphene::{
/// # 	algo::{search::{Dfs, Search}},
/// # 	common::AdjListGraph,
/// # 	core::{
/// # 		Ensure,
/// # 		property::{
/// # 			NewVertex, AddEdge, VertexInGraph
/// # 		}
/// # 	},
/// # };
/// # use graphene::core::Graph;
/// let mut graph = AdjListGraph::<usize,()>::new();
///
/// let v0 = graph.new_vertex_weighted(0).unwrap();
/// let v1 = graph.new_vertex_weighted(1).unwrap();
/// let v2 = graph.new_vertex_weighted(2).unwrap();
///
/// graph.add_edge(&v0,&v1).unwrap();
/// graph.add_edge(&v1,&v2).unwrap();
///
/// // We use `VertexInGraph` to ensure the search starts at v0.
/// let graph = VertexInGraph::ensure(graph, [v0]).unwrap();
///
/// // Initialize the search
/// let mut dfs = Dfs::new_simple(&graph).retain(&graph);
///
/// // We search for the first vertex with weight == 1.
/// let found_vertex = dfs.find(|&v| graph.vertex_weight(&v).unwrap() == &1).unwrap();
/// assert_eq!(v1, found_vertex)
/// ```
///
/// The most basic use of this struct is through the
/// [`new_simple`](#method.new_simple) function which creates a search over
/// the given graph. In our example above, we use this to implement an actual
/// search, by using [`find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find), looking for the
/// first vertex visited that has a given weight.
/// Since searching is lazy, `v2` was never visited, since `v1` was found before
/// `v2` was explored. Therefore, we could theoretically continue the search on
/// the same dfs.
///
/// ### Notes
///
/// _Why isn't `on_exit` a closure?_
///
/// There are three possibilities for an API using closures:
///
/// 1. Direct closure : This requires Dfs to be generic on the closures type.
/// this is possible but means Dfs cannot be used in places where its explicit
/// type is needed. I.e. it cannot be a field in a different struct nor be
/// returned by a function.
///
/// 2. Referenced closure: If Dfs takes a reference to a closure, it no longer
/// needs to be generic on the closures type. However, it limits where Dfs can
/// be used, since it's now bound by the lifetime of the reference. It also
/// doesn't solve the issue with other structs using Dfs, because you can't have
/// the closure anywhere when not using the Dfs.
///
/// 3. Boxed closure: Technically possible, but requires `std` and imposes
/// allocations.
///
/// This solution is as flexible as nr. 1, but solves the issue with naming the
/// closures type. In essence, we are simulating a closure by have `on_exit` be
/// a function and taking `on_exit_args`, that's basically what a closure is.
///
/// ### Related
/// - [Bfs](struct.Bfs.html): Another graph search but using breadth-first.
pub struct Dfs<G, F>
where
	G: Ensure + GraphDeref,
{
	/// A custom payload, available to the function called upon a vertex exit.
	/// See [`new`](#method.new).
	pub payload: F,
	visited: Vec<<G::Graph as Graph>::Vertex>,

	/// The vertex on the stack, and whether on_exit should be called upon
	/// popping.
	stack: Vec<(<G::Graph as Graph>::Vertex, bool)>,

	/// Function to call when visiting a vertex
	on_visit: fn(&G::Graph, <G::Graph as Graph>::Vertex, &mut F),

	/// Function to call when exiting a vertex.
	///
	/// Provides a reference to the graph, the vertex that is exiting,
	/// and a mutable reference to the payload given to the Dfs.
	on_exit: fn(&G::Graph, <G::Graph as Graph>::Vertex, &mut F),

	/// Function to call when exploring an edge.
	///
	/// When a vertex is being visited, this function is called for
	/// every outgoing edge, regardless of whether the sinked vertex
	/// (second vertex argument) has already been visited.
	on_explore: fn(
		&G::Graph,
		<G::Graph as Graph>::Vertex,
		<G::Graph as Graph>::Vertex,
		&<G::Graph as Graph>::EdgeWeight,
		&mut F,
	),
}

impl<G, F> Dfs<G, F>
where
	G: Ensure + GraphDeref,
{
	pub fn new(
		g: impl Borrow<G>,
		on_visit: fn(&G::Graph, <G::Graph as Graph>::Vertex, &mut F),
		on_exit: fn(&G::Graph, <G::Graph as Graph>::Vertex, &mut F),
		on_explore: fn(
			&G::Graph,
			<G::Graph as Graph>::Vertex,
			<G::Graph as Graph>::Vertex,
			&<G::Graph as Graph>::EdgeWeight,
			&mut F,
		),
		payload: F,
	) -> Self
	where
		G::Graph: VertexIn<1>,
	{
		let v = g.borrow().graph().vertex_at::<0>();
		let mut result = Self {
			visited: Vec::new(),
			stack: vec![(v, true)],
			on_visit,
			on_exit,
			on_explore,
			payload,
		};
		// We never result the starting vertex, so throw it away
		result.visit(g, v);
		result
	}

	fn visit(&mut self, graph: impl Borrow<G>, to_return: <G::Graph as Graph>::Vertex)
	{
		(self.on_visit)(graph.borrow().graph(), to_return, &mut self.payload);
		// Mark visited
		self.visited.push(to_return);

		// Explore children
		for (child, weight) in graph.borrow().graph().edges_sourced_in(to_return.clone())
		{
			(self.on_explore)(
				graph.borrow().graph(),
				to_return,
				child,
				weight.borrow(),
				&mut self.payload,
			);
			if !self.visited(child.clone())
			{
				// Push to stack without exit mark
				self.stack.push((child, false));
			}
		}
	}

	pub fn visited(&self, v: <G::Graph as Graph>::Vertex) -> bool
	{
		self.visited.contains(&v)
	}

	/// Pops the next vertex that it is finished visiting off the stack, calling
	/// `on_exit` on it.
	///
	///  If there was nothing to pop and call `on_exit` on, return false,
	/// otherwise returns true.
	pub fn advance_next_exit(
		&mut self,
		graph: impl Borrow<G>,
	) -> Option<<G::Graph as Graph>::Vertex>
	{
		while let Some(last) = self.stack.last()
		{
			if self.visited(last.0.clone())
			{
				let last = self.stack.pop().unwrap();

				// If its exit marked, call the closure on it.
				if last.1
				{
					(self.on_exit)(graph.borrow().graph(), last.0, &mut self.payload);
					return Some(last.0);
				}
			}
			else
			{
				return None;
			}
		}
		None
	}

	pub fn continue_from(&mut self, v: <G::Graph as Graph>::Vertex) -> bool
	{
		if !self.visited(v.clone())
		{
			self.stack.push((v, true));
			true
		}
		else
		{
			false
		}
	}

	pub fn do_nothing_on_visit(_: &G::Graph, _: <G::Graph as Graph>::Vertex, _: &mut F) {}

	pub fn do_nothing_on_exit(_: &G::Graph, _: <G::Graph as Graph>::Vertex, _: &mut F) {}

	pub fn do_nothing_on_explore(
		_: &G::Graph,
		_: <G::Graph as Graph>::Vertex,
		_: <G::Graph as Graph>::Vertex,
		_: &<G::Graph as Graph>::EdgeWeight,
		_: &mut F,
	)
	{
	}
}

impl<G> Dfs<G, ()>
where
	G: Ensure + GraphDeref,
	G::Graph: VertexIn<1>,
{
	/// Constructs a new `Dfs` to search the specified graph.
	///
	/// It calls [`get_vertex`] on the graph, making the traversal start from
	/// the returned vertex. The first call to [`next`]
	/// on the constructed `Dfs` is guaranteed to return the aforementioned
	/// vertex.
	///
	/// ### Hint
	///
	/// [`VertexInGraph`](../core/property/struct.VertexInGraph.html) can be
	/// used to select which specific vertex is returned by [`get_vertex`] and
	/// thereby the starting vertex for the traversal.
	///
	/// [`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
	/// [`get_vertex`]: ../core/property/trait.HasVertex.html#method.get_vertex
	pub fn new_simple(g: impl Borrow<G>) -> Self
	{
		Self::new(
			g,
			Self::do_nothing_on_visit,
			Self::do_nothing_on_exit,
			Self::do_nothing_on_explore,
			(),
		)
	}
}

impl<G, F> Retained<G, Dfs<G, F>>
where
	G: Ensure + GraphDeref,
{
	pub fn visited(&self, v: <G::Graph as Graph>::Vertex) -> bool
	{
		self.search.visited.contains(&v)
	}

	pub fn advance_next_exit(&mut self) -> Option<<G::Graph as Graph>::Vertex>
	{
		self.search.advance_next_exit(&self.graph)
	}

	pub fn continue_from(&mut self, v: <G::Graph as Graph>::Vertex) -> bool
	{
		self.search.continue_from(v)
	}
}

impl<G, F> Search<G> for Dfs<G, F>
where
	G: Ensure + GraphDeref,
{
	fn next(&mut self, g: impl Borrow<G>) -> Option<<G::Graph as Graph>::Vertex>
	{
		// The meaning of markers:
		//
		// If it's on the stack it means we are still visiting it or its children.
		//
		// If its exit marked, it means when we are finished visiting it and its
		// children, we will call the 'on_exit' closure on it, and then pop it.
		// If it's not exit marked, it means this instance of it on the stack was
		// never used for visiting this vertex's children and we just pop it, without
		// calling the closure.
		//
		// If it is marked visited it means we are either visiting its children, or
		// we are finished doing so. Either way, it shouldn't go on the stack again
		// at any point.

		// Pop any vertices that we are done visiting (and since it's on the top of the
		// stack, we must be done visiting its children).
		while self.advance_next_exit(g.borrow()).is_some()
		{}

		// Get the top of the stack. This is necessarily a non-visited vertex.
		// If the stack is empty, then this will return none
		let to_return = self.stack.last()?.0.clone();
		// Exit mark, since we will use it for exploring its children
		self.stack.last_mut()?.1 = true;

		self.visit(g, to_return);

		Some(to_return)
	}
}
