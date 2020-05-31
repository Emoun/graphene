//! An implementation of Tarjan's Strongly Connected Components (SCC) algorithm.
//!
//! The algorithm:
//!
//! Starting from some vertex, do a Dfs.
//! For each visited vertex, push it on top of a stack that you maintain.
//! With the vertex you maintain the following information:
//! "Index": A unique, increasing value for each vertex.
//! 	A vertex higher on the stack (I.e. was pushed last) will have a higher
//! index. "Lowlink value": A reference to another vertex on the stack that is
//! reachable from this vertex. The lowlink value should be seen as the lowest
//! index reachable.
//!
//! When a vertex is pushed in the stack, it is assigned an index. It is also
//! given a lowlink value equal to its index (since we know it can at least
//! reach itself). When you are finished visiting the children of a vertex,
//! check all vertices that are reachable from the current one. If they are on
//! the stack, and their lowlink is smaller than the current lowlink, update the
//! lowlink value to be the smaller one. This propagates the lowest index
//! reachable from this vertex. Then, lastly, if the lowlink value is smaller
//! than the vertex's index, keep it on the stack, and finish the visit. This is
//! because this vertex is clearly not the root of an SCC. (the root vertex of
//! an SCC is simply the first one to be put on the stack) If the lowlink value
//! is unchanged (I.e. still the same as the index), we know this vertex is a
//! root of an SCC. We also know that all other vertices in the SCC have been
//! kept on the stack. Therefore, we can now pop all vertices on the stack until
//! the current vertex (and including it), and they then make up an SCC.
//!
//! When the Dfs can no longer reach any vertices, the algorithm starts again on
//! a new unvisited vertex.
//! When all vertices have been visited, all SCCs have been found and the
//! algorithm is done.
//!
//! Implementation:
//!
//! The Stack:
//! We use a Vec as stack. A vertex's index is simply its position on the stack.
//! We can do this, since indices aren't used after a vertex is popped from the
//! stack, so the next time a vertex is "assigned" the index (by being pushed on
//! the stack), it's still unique. Besides pushing the vertex on the stack, we
//! also add the lowlink value.
//!
//! Most of the action happens in the Dfs's on_exit function. However, instead
//! of allowing the Dfs to call the on_exit on its own, we start every call to
//! `next()` by prompting Dfs to call any on_exit it can. This is because any
//! on_exit call can result in an SCC being found. However, we can't return it
//! from the on_exit to the next(). Therefore, we instead put it in a temporary.
//! By calling the on_exit from next() manually, we can check if an SCC is
//! produced, and if so return it. If not, we can continue the algorithm and if
//! the Dfs is done, check for any unvisited vertices.
use crate::{
	algo::Dfs,
	core::{
		property::{ConnectedGraph, HasVertex},
		proxy::SubgraphProxy,
		Directed, EnsureUnloaded, Graph,
	},
};
use std::cmp::min;

/// Implements Tarjan's [Strongly Connected Components](https://mathworld.wolfram.com/StronglyConnectedComponent.html) Algorithm.
///
/// It implements [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html). [`next`]
/// is therefore the primary way to use this struct.
/// Each call to [`next`] will traverse the graph enough to identify a strongly
/// connected component (SCC) and return it.
///
///
/// A unique property of this algorithm is that it returns the SCCs in reverse
/// topological order. If we treat every SCC as a vertex in a graph, and the
/// edges between the SCCs as edges in this new graph, then it is guaranteed that it is a [DAG](https://mathworld.wolfram.com/AcyclicDigraph.html).
/// Any DAG can be put in a [topological order](https://mathworld.wolfram.com/TopologicalSort.html),
/// where a vertex earlier in the order may have an edge to a vertex later in
/// the order, but a vertex later in the order can't have an edge to a vertex
/// earlier in the order. Tarjan's algorithm will therefore return SCCs such
/// that for any two returned SCCs, the later SCC may have an edge to the
/// earlier, but not the other way around.
///
/// ### Usage
/// ```
/// # use graphene::{
/// # 	algo::TarjanScc,
/// # 	common::AdjListGraph,
/// # 	core::{
/// # 		EnsureUnloaded,
/// # 		property::{
/// # 			NewVertex, AddEdge, HasVertexGraph, Subgraph
/// # 		}
/// # 	},
/// # };
/// # use graphene::core::Graph;
/// let mut graph = AdjListGraph::<(),()>::new();
///
/// let v0 = graph.new_vertex().unwrap();
/// let v1 = graph.new_vertex().unwrap();
/// let v2 = graph.new_vertex().unwrap();
/// let v3 = graph.new_vertex().unwrap();
///
/// // First SCC has v0 and v1
/// graph.add_edge(&v0,&v1).unwrap();
/// graph.add_edge(&v1,&v0).unwrap();
/// // Second SCC has v2 and v3
/// graph.add_edge(&v2,&v3).unwrap();
/// graph.add_edge(&v3,&v2).unwrap();
/// // Connect first SCC to second
/// graph.add_edge(&v0,&v2).unwrap();
///
/// // We use `HasVertexGraph` because we don't care where we start
/// let graph = HasVertexGraph::ensure(graph).unwrap();
///
/// // Initialize algorithm
/// let mut tarj = TarjanScc::new(&graph);
///
/// let tarj_scc1 = tarj.next().unwrap();
/// let tarj_scc2 = tarj.next().unwrap();
/// assert!(tarj.next().is_none());
/// assert!(tarj_scc1.contains_vertex(v2) &&
/// 		tarj_scc1.contains_vertex(v3));
/// assert!(tarj_scc2.contains_vertex(v0) &&
/// 		tarj_scc2.contains_vertex(v1));
/// assert!(tarj_scc2.reaches(&tarj_scc1).is_some());
/// ```
///
/// We initialize `TarjanScc` with the [`new`](#method.new) function.
/// We then use [`next`] to receive each SCC. The SCCs are [`connected`]
/// [`subgraphs`] of the original graph (and therefore also borrow it.) When all
/// SCCs have been returned [`next`] will return [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
///
/// In the above example we create a graph with 2 SCCs, each with 2 vertices and
/// one connected to the other. We can see from the assertions that the SCCs are
/// returned in reverse topological order, where the first SCC is returned
/// second because it points to the second SCC. We use [`Subgraph::reaches`] to
/// check this.
///
/// ### Related
/// - [`Connected`]: All SCCs are strongly connected by definition.
/// - [`ConnectedGraph`](../core/property/struct.ConnectedGraph.html):
/// Returned by [`next`] and implements [`Connected`].
/// - [`Subgraph`]: All SCCs are subgraphs of the original graph.
/// - [`SubgraphProxy`](../core/proxy/struct.SubgraphProxy.html):
/// Returned by [`next`] and implements [`Subgraph`].
/// TODO: other SCC algorithms
///
/// [`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
/// [`connected`]: ../core/property/trait.Connected.html
/// [`Connected`]: ../core/property/trait.Connected.html
/// [`subgraphs`]: ../core/property/trait.Subgraph.html
/// [`Subgraph`]: ../core/property/trait.Subgraph.html
/// [`Subgraph::reaches`]: ../core/property/trait.Subgraph.html#method.reaches
pub struct TarjanScc<'a, G>
where
	G: 'a + Graph<Directedness = Directed>,
{
	dfs: Dfs<'a, G, Vec<(G::Vertex, usize)>>,

	/// We use this to keep track of which vertices we have check for
	/// whether they have been visited.
	unchecked: Box<dyn 'a + Iterator<Item = G::Vertex>>,
}

impl<'a, G> TarjanScc<'a, G>
where
	G: 'a + Graph<Directedness = Directed> + HasVertex,
{
	/// Constructs a new `TarjanScc` to find the [strongly connected components](https://mathworld.wolfram.com/StronglyConnectedComponent.html)
	/// of the specified graph.
	///
	/// It calls [`get_vertex`] on the graph, making the algorithm start from
	/// the returned vertex. TODO: what does this imply?
	///
	/// ### Hint
	///
	/// [`HasVertexGraph`](../core/property/struct.HasVertexGraph.html) can be
	/// used when you don't care which vertex the algorithm starts at. When you
	/// do, [`VertexInGraph`](../core/property/struct.VertexInGraph.html) can be
	/// used.
	///
	/// [`next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
	/// [`get_vertex`]:
	/// ../core/property/trait.HasVertex.html#tymethod.get_vertex
	pub fn new(graph: &'a G) -> Self
	{
		/// Implements part of Tarjan's algorithm, namely what happens when we
		/// are finished visiting a vertex.
		fn on_exit<G>(g: &G, v: G::Vertex, stack: &mut Vec<(G::Vertex, usize)>)
		where
			G: Graph<Directedness = Directed>,
		{
			// Find the index of the vertex
			let index = stack.iter().position(|(v2, _)| *v2 == v).unwrap();

			// Check which vertices can be reached, and update lowlink if necessary
			for e in g.edges_sourced_in(&v)
			{
				if let Some(&lowlink) = stack.iter().find_map(|(v2, lowlink)| {
					if e.0 == *v2
					{
						Some(lowlink)
					}
					else
					{
						None
					}
				})
				{
					stack[index].1 = min(stack[index].1, lowlink);
				}
			}
		}

		// Push the start vertex on the stack with low-link = 0
		let dfs = Dfs::new(graph, on_exit, vec![(graph.get_vertex(), 0)]);
		Self {
			dfs,
			unchecked: graph.all_vertices(),
		}
	}
}

impl<'a, G> Iterator for TarjanScc<'a, G>
where
	G: 'a + Graph<Directedness = Directed>,
{
	type Item = ConnectedGraph<SubgraphProxy<&'a G>>;

	fn next(&mut self) -> Option<Self::Item>
	{
		// Repeat until either an SCC is found or all vertices have been visited.
		loop
		{
			// For each vertex we are finished visiting, check if its the root of a SCC.
			while let Some(v) = self.dfs.advance_next_exit()
			{
				let stack = &mut self.dfs.payload;

				// Find the index of the vertex
				let index = stack.iter().position(|(v2, _)| *v2 == v).unwrap();

				if stack[index].1 == index
				{
					// Vertex is root of SCC, pop stack for all before it

					let mut scc = SubgraphProxy::new(self.dfs.graph);
					while stack.len() > index
					{
						scc.expand(stack.pop().unwrap().0).unwrap();
					}

					return Some(
						ConnectedGraph::ensure(scc)
							.expect("Tarjans algorithm produced non-strongly-connected subgraph"),
					);
					// return Some(ConnectedGraph::new(scc));
				}
				// Vertex is part of SCC but not root, keep it on stack.
			}

			// No SCCs found, let the Dfs run once
			if let Some(v) = self.dfs.next()
			{
				// First push vertex onto stack, with lowlink value equal to its index
				let stack = &mut self.dfs.payload;
				stack.push((v.clone(), stack.len()));
			}
			else
			{
				let dfs = &mut self.dfs;
				if !self.unchecked.any(|v| dfs.continue_from(v))
				{
					return None;
				}
			}
		}
	}
}
