use crate::core::{Graph, Edge, Directedness};

///
///
///
pub struct DFS<'a, G: 'a + Graph>
{
	graph: &'a G,
	visited: Vec<G::Vertex>,
	stack: Vec<G::Vertex>,
}

impl<'a, G:'a + Graph> DFS<'a,G>
{
	pub fn new(g: &'a G, v: G::Vertex) -> Self
	{
		Self{graph: g, visited: Vec::new(), stack: vec![v]}
	}
}

impl<'a, G:'a + Graph> Iterator for DFS<'a,G>
{
	type Item = G::Vertex;
	
	fn next(&mut self) -> Option<Self::Item> {
		let to_return = self.stack.last()?.clone();
		self.visited.push(to_return);

		'l: loop {
			if let Some(&next) = self.stack.last() {
				for e in self.graph.edges_incident_on(next)
				{
					let other_v =
						if next == e.source() { e.sink() }
						// In a directed graph, we have to skip incomming edges
						else if G::Directedness::directed() { continue }
						else { e.source() };

					if !self.visited.contains(&other_v) {
						self.stack.push(other_v);
						break 'l;
					}
				}
				self.stack.pop();
			} else {
				break 'l;
			}
		}
		Some(to_return)
	}
}
