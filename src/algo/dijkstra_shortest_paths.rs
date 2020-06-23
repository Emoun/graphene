use crate::core::{property::HasVertex, Edge, Graph};
use num_traits::{PrimInt, Unsigned};

/// [Dijkstra's shortest paths algorithm](https://mathworld.wolfram.com/DijkstrasAlgorithm.html)
pub struct DijkstraShortestPaths<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	graph: &'a G,
	visited: Vec<G::Vertex>,
	// We keep is sorted with the lowest weight at the end for efficiency.
	queue: Vec<(W, (G::Vertex, G::Vertex, &'a G::EdgeWeight))>,
	get_weight: fn(&G::EdgeWeight) -> W,
}

impl<'a, G, W> DijkstraShortestPaths<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	pub fn new(graph: &'a G, get_weight: fn(&G::EdgeWeight) -> W) -> Self
	where
		G: HasVertex,
	{
		let mut dijk = Self {
			graph,
			visited: Vec::new(),
			queue: Vec::new(),
			get_weight,
		};
		dijk.visit(graph.get_vertex().clone(), W::zero());
		dijk
	}

	fn visit(&mut self, v: G::Vertex, w: W)
	{
		self.visited.push(v);
		let visited = &self.visited;
		{
			let edges = self.graph.edges_sourced_in(&v)
				// Remove any edge to a visited vertex
				.filter(|(sink, _)| !visited.contains(&sink));

			for (sink, weight) in edges
			{
				let new_weight = w + (self.get_weight)(weight);
				if let Some((old_weight, old_edge)) = self
					.queue
					.iter_mut()
					.find(|(_, (_, vert, _))| *vert == sink)
				{
					if *old_weight > new_weight
					{
						*old_weight = new_weight;
						*old_edge = (v, sink, weight);
					}
				}
				else
				{
					self.queue.push((new_weight, (v, sink, weight)));
				}
			}
		}
		self.queue.sort_by(|(w1, _), (w2, _)| w2.cmp(w1));
	}
}

impl<'a, G> DijkstraShortestPaths<'a, G, G::EdgeWeight>
where
	G: 'a + Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	pub fn new_simple(graph: &'a G) -> Self
	where
		G: HasVertex,
	{
		Self::new(graph, Clone::clone)
	}
}

impl<'a, G, W> Iterator for DijkstraShortestPaths<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	type Item = (G::Vertex, G::Vertex, &'a G::EdgeWeight);

	fn next(&mut self) -> Option<Self::Item>
	{
		let (weight, result) = self.queue.pop()?;

		self.visit(result.sink(), weight);

		Some(result)
	}
}

/// Shortest-Path-First search
///
/// next() doesn't return the starting vertex.
pub struct Spfs<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	dijk: DijkstraShortestPaths<'a, G, W>,
}

impl<'a, G, W> Spfs<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	pub fn new(graph: &'a G, get_weight: fn(&G::EdgeWeight) -> W) -> Self
	where
		G: HasVertex,
	{
		Self {
			dijk: DijkstraShortestPaths::new(graph, get_weight),
		}
	}
}

impl<'a, G> Spfs<'a, G, G::EdgeWeight>
where
	G: 'a + Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	pub fn new_simple(graph: &'a G) -> Self
	where
		G: HasVertex,
	{
		Self::new(graph, Clone::clone)
	}
}

impl<'a, G, W> Iterator for Spfs<'a, G, W>
where
	G: 'a + Graph,
	W: PrimInt + Unsigned,
{
	type Item = G::Vertex;

	fn next(&mut self) -> Option<Self::Item>
	{
		Some(self.dijk.next()?.1)
	}
}
