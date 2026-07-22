use crate::{
	algo::retain::{Retainable, UnretainedIterator},
	core::{property::VertexIn, Edge, Graph},
};
use num_traits::{PrimInt, Unsigned, Zero};
use std::borrow::Borrow;
/// [Dijkstra's shortest paths algorithm](https://mathworld.wolfram.com/DijkstrasAlgorithm.html)
pub struct DijkstraShortestPaths<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	visited: Vec<G::Vertex>,
	// We keep it sorted with the lowest weight at the end for efficiency.
	queue: Vec<(G::EdgeWeight, (G::Vertex, G::Vertex))>,
}

impl<G> DijkstraShortestPaths<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	pub fn new(graph: &G) -> Self
	where
		G: VertexIn<1>,
	{
		let mut dijk = Self {
			visited: Vec::new(),
			queue: Vec::new(),
		};
		dijk.visit(graph, graph.vertex_at::<0>(), G::EdgeWeight::zero());
		dijk
	}

	fn visit(&mut self, graph: impl Borrow<G>, v: G::Vertex, w: G::EdgeWeight)
	{
		self.visited.push(v);
		let visited = &self.visited;
		let edges = graph.borrow().edges_sourced_in(v)
			// Remove any edge to a visited vertex
			.filter(|(edge, _)| !visited.contains(&edge));

		for (sink, weight) in edges
		{
			let new_weight = w + *weight;
			if let Some((old_weight, old_edge)) =
				self.queue.iter_mut().find(|(_, (_, vert))| *vert == sink)
			{
				if *old_weight > new_weight
				{
					*old_weight = new_weight;
					*old_edge = (v, sink);
				}
			}
			else
			{
				self.queue.push((new_weight, (v, sink)));
			}
		}
		self.queue.sort_by(|(w1, _), (w2, _)| w2.cmp(w1));
	}

	/// Returns the vertices reachable from the designated vertex and the
	/// weighted distance to them
	pub fn distances(graph: &G) -> impl Iterator<Item = (G::Vertex, G::EdgeWeight)> + use<'_, G>
	where
		G: VertexIn<1>,
	{
		let mut distances = vec![(graph.vertex_at::<0>(), G::EdgeWeight::zero())];

		Self::new(graph).retain(graph).map(move |(so, si, w)| {
			let dist = distances.iter().find(|(v, _)| so == *v).unwrap().1;
			let new_dist = dist + w;
			distances.push((si, new_dist));
			(si, new_dist)
		})
	}

	pub fn shortest_edge_between(graph: &G, so: G::Vertex, si: G::Vertex) -> G::EdgeWeightRef<'_>
	{
		let mut edges = graph.edges_between(so, si);
		let first = edges.next().unwrap();
		let weight = edges.fold(first, |acc, w| {
			if *acc < *w
			{
				acc
			}
			else
			{
				w
			}
		});
		weight
	}
}

impl<G> UnretainedIterator<G> for DijkstraShortestPaths<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	type Item = (G::Vertex, G::Vertex, G::EdgeWeight);

	fn next(&mut self, graph: &G) -> Option<Self::Item>
	{
		let (weight, edge) = self.queue.pop()?;

		self.visit(graph.borrow(), edge.sink(), weight);

		Some((
			edge.source(),
			edge.sink(),
			*Self::shortest_edge_between(graph, edge.source(), edge.sink()),
		))
	}
}
