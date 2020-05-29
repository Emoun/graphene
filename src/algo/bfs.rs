use crate::core::{property::HasVertex, Graph};
use std::collections::VecDeque;

/// Performs [breadth-first traversal](https://mathworld.wolfram.com/Breadth-FirstTraversal.html)
/// of a graph's vertices.
///
/// Even though the 's' in its name implies a search, this struct only performs
/// traversal, delegating the searching to the user.
///
/// It implements [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html). [`next`]
/// is therefore the primary way to use this struct.
/// Each call will traverse the graph just enough to visit the next vertex and
/// return it. The initial vertex visited is the one returned by calling the
/// graph's [`get_vertex`] method. That vertex is never returned
/// by the a call to [`next`].
///
/// When the traversal is finished, either because all vertices in the graph
/// have been visited or because no more vertices can be reached,
/// [`next`] will return [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
///
/// ### Usage
///
/// ```
/// # use graphene::{
/// # 	algo::Bfs,
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
/// let v3 = graph.new_vertex_weighted(2).unwrap();
///
/// graph.add_edge((v0,v1)).unwrap();
/// graph.add_edge((v0,v2)).unwrap();
/// graph.add_edge((v1,v3)).unwrap();
///
/// // We use `VertexInGraph` to ensure traversal starts at v0.
/// let graph = VertexInGraph::ensure(graph, v0).unwrap();
///
/// // Initialize the traversal
/// let mut dfs = Bfs::new(&graph);
///
/// // We search for the first vertex with weight == 2.
/// let found_vertex = dfs.find(|&v| graph.vertex_weight(v).unwrap() == &2).unwrap();
/// assert_eq!(v2, found_vertex)
/// ```
///
/// To begin a new traversal use the [`new`](#method.new) function which creates
/// a traversal over the given graph. In our example above, we use this to
/// implement an actual search, by using [`find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find),
/// looking for the first vertex visited that has a given weight.
/// Since traversal is lazy, `v3` was never visited, since `v2` was found before
/// `v3` was explored. Therefore, we could theoretically continue the traversal
/// on the same bfs.
///
/// ### Related
/// - [Dfs](struct.Dfs.html): Another graph traversal but using depth-first.
///
/// [`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
/// [`get_vertex`]: ../core/property/trait.HasVertex.html#method.get_vertex
pub struct Bfs<'a, G>
where
	G: 'a + Graph,
{
	graph: &'a G,
	queue: VecDeque<G::Vertex>,
	visited: Vec<G::Vertex>,
	predecessor: Vec<(G::Vertex, Option<G::Vertex>)>,
}

impl<'a, G> Bfs<'a, G>
where
	G: 'a + Graph,
{
	/// Constructs a new `Bfs` to traverse the specified graph.
	///
	/// It calls [`get_vertex`] on the graph, making the traversal start from
	/// the returned vertex. The first call to [`next`] on the constructed `Bfs`
	/// is guaranteed to return the aforementioned vertex.
	///
	/// ### Hint
	///
	/// [`VertexInGraph`](../core/property/struct.VertexInGraph.html) can be
	/// used to select which specific vertex is returned by [`get_vertex`] and
	/// thereby the starting vertex for the traversal.
	///
	/// [`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
	/// [`get_vertex`]: ../core/property/trait.HasVertex.html#method.get_vertex
	pub fn new(graph: &'a G) -> Self
	where
		G: HasVertex,
	{
		let v = graph.get_vertex();

		let mut result = Self {
			graph,
			queue: VecDeque::new(),
			visited: vec![v],
			predecessor: vec![(v, None)],
		};
		result.explore(v);
		result
	}

	/// Returns the traversal depth of the given vertex.
	///
	/// A traversal can be seen as a [spanning tree](https://mathworld.wolfram.com/SpanningTree.html)
	/// of the graph. This method will therefore return the depth of the given
	/// node in the spanning tree, where the root of the tree is the starting
	/// vertex.
	///
	/// Calling this method with a vertex that isn't in the graph or hasn't been
	/// visited yet is undefined behaviour.
	///
	/// TODO: Eliminate undefined behaviour.
	pub fn depth(&self, v: G::Vertex) -> usize
	{
		let mut count = 0;
		let mut current = v;
		while let Some(p) = self.predecessor(current)
		{
			current = p;
			count += 1;
		}
		count
	}

	/// Returns the unique predecessor in the traversal of the given vertex.
	///
	/// A traversal can be seen as a [spanning tree](https://mathworld.wolfram.com/SpanningTree.html)
	/// of the graph. This method will therefore return the vertex in this tree
	/// with a 1 less depth but is connected to the given graph.
	///
	/// If the given vertex hasn't been visited by the traversal yet, or isn't
	/// in the graph outright, [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) is returned.
	pub fn predecessor(&self, v: G::Vertex) -> Option<G::Vertex>
	{
		if let Some((_, p)) = self.predecessor.iter().find(|(v1, _)| *v1 == v)
		{
			*p
		}
		else
		{
			None
		}
	}

	/// Explores the outgoing edges from the given vertex,
	/// queueing up any previously unvisited vertices.
	fn explore(&mut self, v: G::Vertex)
	{
		let visited = &mut self.visited;
		let queue = &mut self.queue;
		let pred = &mut self.predecessor;

		self.graph
			.edges_sourced_in(&v)
			.filter_map(|(child, _)| {
				if !visited.contains(&child)
				{
					visited.push(child);
					Some(child)
				}
				else
				{
					None
				}
			})
			.for_each(|child| {
				queue.push_back(child);
				pred.push((child, Some(v)))
			});
	}
}

impl<'a, G> Iterator for Bfs<'a, G>
where
	G: 'a + Graph,
{
	type Item = G::Vertex;

	fn next(&mut self) -> Option<Self::Item>
	{
		if let Some(v) = self.queue.pop_front()
		{
			self.explore(v);
			Some(v)
		}
		else
		{
			None
		}
	}
}
