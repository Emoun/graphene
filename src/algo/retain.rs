use crate::core::{Graph, GraphDeref};

/// This struct turns an unretained graph algorithm into a retained algorithm.
///
/// It retains the borrowing/ownership of the graph the algorithm being run on,
/// allowing for easier execution of the agorithm For example, retained
/// [`Search`](crate::algo::search::Search)es implement [`Iterator`]
/// automatically.
///
/// To retain an algorithm, use [`Retainable::retain`] and provide the graph the
/// algorithm is being run on. The graph may be released at any time by
/// destructuring, in case the graph needs to be mutated (carefully) or the
/// algorithm is done.
pub struct Retained<G, A>
where
	G: GraphDeref,
{
	/// The underlying algorithm state
	pub algo: A,

	/// The retained graph being searched
	pub graph: G,
}

/// Allows all algorithms to be retained.
pub trait Retainable: Sized
{
	/// Retains the given graph for the algorithm
	fn retain<GD: GraphDeref>(self, graph: GD) -> Retained<GD, Self>
	{
		Retained { algo: self, graph }
	}
}

impl<T> Retainable for T {}

/// Algorithms that iterate over a graph unretained.
///
/// [`Iterator`] is implemented for the retained version of an algorithm
/// implementing this trait.
pub trait UnretainedIterator<G: Graph>
{
	type Item;

	fn next(&mut self, g: &G) -> Option<Self::Item>;
}

impl<G: GraphDeref, A: UnretainedIterator<G::Graph>> Iterator for Retained<G, A>
{
	type Item = A::Item;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.algo.next(self.graph.graph())
	}
}
