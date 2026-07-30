use crate::{
	algo::UnretainedIterator,
	core::{
		property::{Acyclic, VertexIn},
		Directed,
	},
};

/// Performs search in [topological order](https://mathworld.wolfram.com/TopologicalSort.html)
///
/// This requires the graph to be directed and acyclic.
/// The order the vertices are produced ensures no edge (v1, v2) where v1 is
/// produced later than v2.
///
/// As with other searches, never produces the initial vertex.
pub struct Topo<G: Acyclic<Directedness = Directed>>
{
	visited: Vec<G::Vertex>,
	queue: Vec<G::Vertex>,
}

impl<G: Acyclic<Directedness = Directed>> Topo<G>
{
	pub fn new(graph: &G) -> Self
	where
		G: VertexIn<1>,
	{
		let v = graph.vertex_at::<0>();

		let mut result = Self {
			visited: vec![v],
			queue: Vec::new(),
		};

		result.explore(graph, v);

		result
	}

	/// Explores the outgoing edges from the given vertex,
	/// queueing up any previously unvisited vertices.
	fn explore(&mut self, graph: &G, v: G::Vertex)
	{
		graph
			.edges_sourced_in(v)
			.filter_map(|(child, _)| {
				if !self.visited.contains(&child)
				{
					self.visited.push(child);
					Some(child)
				}
				else
				{
					None
				}
			})
			.for_each(|child| {
				self.queue.push(child);
			});
	}
}

impl<G> UnretainedIterator<G> for Topo<G>
where
	G: Acyclic<Directedness = Directed>,
{
	type Item = G::Vertex;

	fn next(&mut self, graph: &G) -> Option<Self::Item>
	{
		// We use Kahn's algorithm: https://en.wikipedia.org/wiki/Topological_sorting
		// However, because it's a search, we only want to cover the vertices reachable
		// from the start

		// Get a list of vertices with no incoming edges from non-visited vertices
		let mut vs = self.queue.iter().enumerate().filter(|(_, v)| {
			graph
				.edges_sinked_in(*v)
				.all(|(v2, _)| self.visited.contains(&v2) && !self.queue.contains(&v2))
		});

		// Take out any vertex
		vs.next().map(|(i, _)| i).and_then(|i| {
			let v = self.queue.remove(i);
			self.explore(graph, v);
			Some(v)
		})
	}
}
