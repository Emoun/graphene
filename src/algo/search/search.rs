use crate::core::{Ensure, Graph, GraphDeref};
use std::borrow::Borrow;

/// A search algorithm that does not retain the searched graph.
pub trait Search<G>
where
	G: Ensure + GraphDeref,
	Self: Sized,
{
	/// Returns the next vertex in teh search of the given graph
	///
	/// Assumes the same graph is given for every call.
	/// Changing the graph's vertices or edges between calls may cause an error
	/// in the search.
	fn next(&mut self, g: impl Borrow<G>) -> Option<<G::Graph as Graph>::Vertex>;

	/// Turns the search into a retained search, continuing the search using the
	/// given graph.
	fn retain(self, graph: G) -> Retained<G, Self>
	{
		Retained {
			search: self,
			graph,
		}
	}
}

/// This struct turns an unretained graph search into a retained search.
///
/// This struct implements the [`Iterator`] trait, allowing each call to
/// [`next`](Retained::next) to return the next vertex in the search.
///
/// The graph may be released at any time by destructuring, in case the graph
/// needs to be mutated (carefully) or the search is done.
pub struct Retained<G, S>
where
	G: Ensure + GraphDeref,
	S: Search<G>,
{
	/// The underlying search state
	pub search: S,

	/// The retained graph being searched
	pub graph: G,
}

impl<G, S> Iterator for Retained<G, S>
where
	G: Ensure + GraphDeref,
	S: Search<G>,
{
	type Item = <G::Graph as Graph>::Vertex;

	fn next(&mut self) -> Option<Self::Item>
	{
		self.search.next(&self.graph)
	}
}
