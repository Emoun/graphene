//! An implementation of Tarjan's Strongly Connected Components (SCC) algorithm.
//!
//! The algorithm:
//!
//! Starting from some vertex, do a DFS.
//! For each visited vertex, push it on top of a stack that you maintain.
//! With the vertex you maintain the following information:
//! "Index": A unique, increasing value for each vertex.
//! 	A vertex higher on the stack (I.e. was pushed last) will have a higher
//! index. "Lowlink value": A reference to another vertex on the stack that is
//! reachable from this vertex. 	The lowlink value should be seen as the lowest
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
//! When the DFS can no longer reach any vertices, the algorithm starts again on
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
//! Most of the action happens in the DFS's on_exit function. However, instead
//! of allowing the DFS to call the on_exit on its own, we start every call to
//! `next()` by prompting DFS to call any on_exit it can. This is because any
//! on_exit call can result in an SCC being found. However, we can't return it
//! from the on_exit to the next(). Therefore, we instead put it in a temporary.
//! By calling the on_exit from next() manually, we can check if an SCC is
//! produced, and if so return it. If not, we can continue the algorithm and if
//! the DFS is done, check for any unvisited vertices.
use crate::{
	algo::DFS,
	core::{constraint::ConnectedGraph, proxy::SubgraphProxy, Constrainer, Directed, Graph},
};
use std::{cmp::min};

/// Implements Tarjan's Strongly Connected Components Algorithm.
pub struct TarjanSCC<'a, G>
where
	G: 'a + Graph<Directedness = Directed>,
{
	dfs: DFS<'a, G, Vec<(G::Vertex, usize)>>,

	/// We use this to keep track of which vertices we have check for
	/// whether they have been visited.
	unchecked: Box<dyn 'a + Iterator<Item = G::Vertex>>,
}

impl<'a, G> TarjanSCC<'a, G>
where
	G: 'a + Graph<Directedness = Directed>,
{
	pub fn new(graph: &'a G, start: G::Vertex) -> Self
	{
		/// Implements part of Tarjan's algorithm, namely what happens when we
		/// are finished visiting a vertex.
		fn on_exit<G>(
			g: &G,
			v: G::Vertex,
			stack: &mut Vec<(G::Vertex, usize)>,
		) where
			G: Graph<Directedness = Directed>,
		{
			// Find the index of the vertex
			let index = stack.iter().position(|(v2, _)| *v2 == v).unwrap();

			// Check which vertices can be reached, and update lowlink if necessary
			for e in g.edges_sourced_in(v)
			{
				if let Some(&lowlink) = stack.iter().find_map(|(v2, lowlink)| {
					if e.1 == *v2
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

		let dfs = DFS::new(graph, start, on_exit, Vec::new());
		Self {
			dfs,
			unchecked: graph.all_vertices(),
		}
	}
}

impl<'a, G> Iterator for TarjanSCC<'a, G>
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
						ConnectedGraph::constrain_single(scc)
							.expect("Tarjans algorithm produced non-strongly-connected subgraph"),
					);
					// return Some(ConnectedGraph::new(scc));
				}
				// Vertex is part of SCC but not root, keep it on stack.
			}

			// No SCCs found, let the DFS run once
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
