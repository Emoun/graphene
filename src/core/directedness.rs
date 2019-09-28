
///
/// Defines whether something is directed or not.
///
/// The marker types [`Directed`](struct.Directed.html) and [`Undirected`](struct.Undirected.html) should be used
/// when bounding types.
///
pub trait Directedness: Copy + Clone + Send + Sync + 'static + Eq + Ord {
	
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
