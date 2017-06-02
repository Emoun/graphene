
///
/// Represents an edge between two vertices in a graph.
///
///	The edge is implicitly directed from the `source` to the `sink` with some weight `weight`.
///
/// Parameters:
///
/// - `V`: The type of the vertices.
/// - `W`: The type of the weight.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct BaseEdge<V,W>
where
	V: Copy+Eq,
	W: Copy+Eq,
{
	/// The vertex the edge starts in.
	pub source: V,
	/// The vertex the edge ends in.
	pub sink:V,
	/// The weight of the edge.
	pub weight: W,
}

impl<V:Copy+Eq, W:Copy+Eq> BaseEdge<V,W>{
	
	/// Creates a new edge
	pub fn new(source: V, sink: V, weight: W)-> BaseEdge<V,W>{
		BaseEdge{source, sink, weight}
	}
	/// Returns a copy of the vertex the edge starts in.
	pub fn source(&self) -> V {
		self.source
	}
	/// Returns a copy of the vertex the edge ends in.
	pub fn sink(&self) -> V {
		self.sink
	}
	/// Returns a copy of the weight of the edge.
	pub fn weight(&self) -> W{
		self.weight
	}
}

