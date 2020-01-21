use crate::core::{Graph, Directed, Constrainer};
use crate::algo::DFS;
use crate::core::proxy::SubGraph;
use std::cmp::min;
use crate::core::constraint::ConnectedGraph;
use std::mem::replace;

///
/// Implements Tarjan's Strongly Connected Components Algorithm
///
///
///
pub struct TarjanSCC<'a, G>
	where
		G:'a + Graph<Directedness=Directed>,
{
	graph: &'a G,
	dfs: DFS<'a, G, (&'a G, Vec<(G::Vertex, usize)>, Option<SubGraph<&'a G>>)>,
}

impl<'a,G> TarjanSCC<'a, G>
	where
		G:'a + Graph<Directedness=Directed>,
{
	pub fn new(g: &'a G, start: G::Vertex) -> Self
	{
		// We track vertices on the stack with their low-link value.
		// Each vertex's ID is its index on the stack,
		// which means low-link values refer to stack indices.
		fn on_exit<'a, G>(v: G::Vertex, (g,stack, next_scc): &mut
			(&'a G, Vec<(G::Vertex, usize)>, Option<SubGraph<&'a G>>))
			where G: Graph<Directedness=Directed>
		{
			let index = stack.iter().position(|(v2, _)| *v2 == v).unwrap();
			
			// Check which vertices can be reached, and update lowlink if necessary
			for e in g.edges_sourced_in(v) {
				if let Some(&lowlink) = stack.iter().find_map(|(v2, lowlink)|
					if e.1 == *v2 { Some(lowlink) } else {None} ) {
					stack[index].1 = min(stack[index].1, lowlink);
				}
			}
			
			// Then check whether it needs popping
			if stack[index].1 == index {
				// Vertex is root of SCC, pop stack for all before it
				let mut scc = SubGraph::new(*g);
				
				while stack.len() > index {
					scc.expand(stack.pop().unwrap().0).unwrap();
				}
				
				*next_scc = Some(scc)
			} else {
				// Vertex is part of SCC but not root, keep it on stack.
			}
		}
		
		let dfs = DFS::new(g, start, on_exit, (g, Vec::new(), None));
		Self{graph: g, dfs}
	}
}

impl<'a, G> Iterator for TarjanSCC<'a,G>
	where
		G:'a + Graph<Directedness=Directed>,
{
	type Item = ConnectedGraph<SubGraph<&'a G>>;
	
	fn next(&mut self) -> Option<Self::Item> {
		'l:
		loop {
			// If we have already found an scc, return it.
			while self.dfs.advance_next_exit() {
				if let Some(scc) = replace(&mut self.dfs.args_mut().2, None) {
					return Some(ConnectedGraph::constrain_single(scc)
						.expect("Tarjans algorithm produced non-strongly-connected subgraph"));
//					return Some(ConnectedGraph::new(scc));
				}
			}
			
			// Otherwise, let the DFS run once
			if let Some(v) = self.dfs.next() {
				// First push vertex onto stack, with lowlink value equal to its index
				let stack = &mut self.dfs.args_mut().1;
				stack.push((v.clone(), stack.len()));
			} else {
				for v in self.graph.all_vertices() {
					if self.dfs.continue_from(v) {
						continue 'l;
					}
				}
				return None
			}
		}
	}
}

















