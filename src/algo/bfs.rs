use crate::core::{property::NonNull, Directedness, Edge, Graph};
use std::collections::VecDeque;

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
	pub fn new(graph: &'a G) -> Self
	where
		G: NonNull,
	{
		let mut queue = VecDeque::new();
		let v = graph.get_vertex();
		queue.push_back(v);
		let visited = vec![v];
		let predecessor = vec![(v, None)];
		Self {
			graph,
			queue,
			visited,
			predecessor,
		}
	}

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
			let pred = &mut self.predecessor;
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
				.for_each(|child| {
					queue.push_back(child);
					pred.push((child, Some(v)))
				});

			Some(v)
		}
		else
		{
			None
		}
	}
}
