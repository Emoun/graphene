use crate::{algo::retain::UnretainedIterator, core::Graph};

/// A search algorithm that does not retain the searched graph.
pub trait Search<G>: UnretainedIterator<G, Item = G::Vertex>
where
	G: Graph,
	Self: Sized,
{
}

impl<G, T> Search<G> for T
where
	G: Graph,
	T: UnretainedIterator<G, Item = G::Vertex>,
{
}
