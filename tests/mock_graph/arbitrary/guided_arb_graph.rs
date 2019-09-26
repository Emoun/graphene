use quickcheck::{Gen, Arbitrary};
use std::ops::RangeBounds;
use std::collections::HashSet;

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Limit {
	/// Shrinkages shouldn't remove vertices
	VertexRemove,
	
	/// Shrinkages shouldn't remove edges
	EdgeRemove,
	
	/// Shrinkages shouldn't remove vertices if it results in their count
	/// becoming lower than the given.
	VertexMin(usize),

	/// Shrinkages shouldn't remove vertices if it results in their count
	/// becoming lower than the given.
	EdgeMin(usize),
}

///
/// A vertion of `quickcheck::Arbitrary` for Graphs that can be guided how to make the graph
///
pub trait GuidedArbGraph: Arbitrary
{
	
	///
	/// Generates an arbitrary graph, where the number of vertices and edges is within the given
	/// ranges.
	///
	/// If the minimum number of vertices is 0, a graph may be generated that has no vertices
	/// and no edges, regardless of the range of edges.
	///
	/// The ranges are only guides, and adherence to them depends on implementation.
	///
	fn arbitrary_guided<G: Gen, R: RangeBounds<usize>>(g: &mut G, _v_range: R, _e_range: R)
		-> Self
	{
		Self::arbitrary(g)
	}
	
	fn shrink_guided(&self, _limits: HashSet<Limit>) -> Box<dyn Iterator<Item=Self>>
	{
		self.shrink()
	}
	
}