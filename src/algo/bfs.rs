use crate::core::{Directedness, Edge, Graph};
use std::collections::VecDeque;

pub struct Bfs<'a, G>
where
	G: 'a + Graph,
{
	graph: &'a G,
	queue: VecDeque<G::Vertex>,
	visited: Vec<G::Vertex>,
}

impl<'a, G> Bfs<'a, G>
where
	G: 'a + Graph,
{
	pub fn new(graph: &'a G, v: G::Vertex) -> Self
	{
		let mut queue = VecDeque::new();
		queue.push_back(v);
		let visited = vec![v];
		Self {
			graph,
			queue,
			visited,
		}
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
			// Queue up the children
			let visited = &mut self.visited;
			let queue = &mut self.queue;
			self.graph
				.edges_incident_on(v)
				.filter_map(|e| {
					let child = if v == e.source()
					{
						e.sink()
					}
					// In a directed graph, we have to skip incoming edges
					else if G::Directedness::directed()
					{
						return None;
					}
					else
					{
						e.source()
					};

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
				.for_each(|child| queue.push_back(child));

			Some(v)
		}
		else
		{
			None
		}
	}
}
