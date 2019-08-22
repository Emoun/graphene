use crate::core::Graph;

///
/// Defines whether something is directed or not.
///
/// The marker types [`Directed`](struct.Directed.html) and [`Undirected`](struct.Undirected.html) should be used
/// when bounding types. However, other types can also implement this trait, e.g. any [`Graph`](trait.Graph.html)
/// implements it according to its edge's [directedness](trait.Graph.html#associatedtype.Directedness).
///
pub trait Directedness {
	
	///
	/// Returns whether this instance is directed or not.
	///
	fn directed() -> bool;
}

///
/// Marker type for something that is directed.
///
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Directed();
impl Directedness for Directed{
	fn directed() -> bool {
		true
	}
}
///
/// Marker type for something that is not directed.
///
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Undirected();
impl Directedness for Undirected{
	fn directed() -> bool {
		false
	}
}

impl<T,D:Directedness> Directedness for T where T: Graph<Directedness=D> {
	fn directed() -> bool {
		D::directed()
	}
}