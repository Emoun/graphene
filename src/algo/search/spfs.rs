use crate::{
	algo::{retain::UnretainedIterator, DijkstraShortestPaths},
	core::{property::VertexIn, Graph},
};
use num_traits::{PrimInt, Unsigned};

/// Shortest-Path-First search
///
/// next() doesn't return the starting vertex.
pub struct Spfs<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	dijk: DijkstraShortestPaths<G>,
}

impl<G> Spfs<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	pub fn new(graph: &G) -> Self
	where
		G: VertexIn<1>,
	{
		Self {
			dijk: DijkstraShortestPaths::new(graph),
		}
	}
}

impl<G> UnretainedIterator<G> for Spfs<G>
where
	G: Graph,
	G::EdgeWeight: PrimInt + Unsigned,
{
	type Item = G::Vertex;

	fn next(&mut self, g: &G) -> Option<Self::Item>
	{
		Some(self.dijk.next(g)?.1)
	}
}
