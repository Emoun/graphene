use quickcheck::{Gen, Arbitrary};
use std::ops::{RangeBounds, Bound};
use std::collections::HashSet;
use crate::mock_graph::MockVertex;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash)]
pub enum Limit {
	/// Shrinkages shouldn't remove any vertices
	VertexRemove,
	
	/// Shrinkages shouldn't remove any edges
	EdgeRemove,
	
	/// Shrinkages shouldn't alter the given vertex
	/// (neither change its value nor remove it from the graph)
	VertexKeep(MockVertex),
	
	/// Shrinkages shouldn't remove vertices if it results in their count
	/// becoming lower than the given.
	VertexMin(usize),

	/// Shrinkages shouldn't remove edges if it results in their count
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
	fn arbitrary_guided<G: Gen>(g: &mut G, _v_range: impl RangeBounds<usize>,
								_e_range: impl RangeBounds<usize>)
		-> Self
	{
		Self::arbitrary(g)
	}
	
	fn validate_ranges<G: Gen>(g: &mut G, v_range: impl RangeBounds<usize>,
							   e_range: impl RangeBounds<usize>)
		-> (usize, usize, usize, usize)
	{
		let e_min = match e_range.start_bound() {
			Bound::Included(&x) =>  x,
			Bound::Excluded(&x) => x + 1,
			Bound::Unbounded => 0,
		};
		let v_min = match v_range.start_bound() {
			Bound::Included(&x) =>  if e_min > 0 && x == 0 {
				panic!("Cannot generate a graph with 0 vertices but minimum {} edges.", e_min)
			} else { x },
			Bound::Excluded(&x) => x + 1,
			Bound::Unbounded => if e_min > 0 { 1 } else {0},
		};
		let v_max = match v_range.end_bound() {
			Bound::Included(&x) =>  x + 1 ,
			Bound::Excluded(&x) => x,
			Bound::Unbounded => g.size(),
		};
		let e_max = match e_range.end_bound() {
			Bound::Included(&x) =>  x + 1 ,
			Bound::Excluded(&x) => x,
			Bound::Unbounded => v_max,
		};
		assert!(v_min < v_max);
		assert!(e_min < e_max);
		(v_min, v_max, e_min, e_max)
	}
	
	fn shrink_guided(&self, _limits: HashSet<Limit>) -> Box<dyn Iterator<Item=Self>>
	{
		self.shrink()
	}
	
}