use crate::mock_graph::{MockType, MockVertex, TestGraph};
use graphene::core::{Ensure, Graph};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{
	collections::HashSet,
	fmt::Debug,
	ops::{Bound, RangeBounds},
};

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Limit
{
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

	/// Shrinkages shouldn't remove any edge with the given source and sink
	EdgeKeep(MockVertex, MockVertex),
}

impl Limit
{
	pub fn min_vertices(limits: &HashSet<Limit>) -> usize
	{
		let mut min_vert = std::usize::MAX;

		if !limits.contains(&Limit::VertexRemove)
		{
			// in case no min is give, MAX shouldn't be used.
			let mut any_min = false;
			for l in limits.iter()
			{
				if let Limit::VertexMin(min) = l
				{
					min_vert = std::cmp::min(min_vert, *min);
					any_min = true;
				}
			}
			if !any_min
			{
				0
			}
			else
			{
				min_vert
			}
		}
		else
		{
			min_vert
		}
	}
}

/// A version of `quickcheck::Arbitrary` for Graphs that can be guided how to
/// make the graph
pub trait GuidedArbGraph: Ensure + 'static + Send + Clone
where
	Self::Graph: TestGraph,
	<Self::Graph as Graph>::EdgeWeight: MockType,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		// Validate ranges
		let e_min = match e_range.start_bound()
		{
			Bound::Included(&x) => x,
			Bound::Excluded(&x) => x + 1,
			Bound::Unbounded => 0,
		};
		let v_min = match v_range.start_bound()
		{
			Bound::Included(&x) => std::cmp::max(0 + ((e_min > 0) as usize), x),
			Bound::Excluded(&x) => x + 1,
			Bound::Unbounded =>
			{
				if e_min > 0
				{
					1
				}
				else
				{
					0
				}
			},
		};
		let v_max = match v_range.end_bound()
		{
			Bound::Included(&x) => x + 1,
			Bound::Excluded(&x) => x,
			Bound::Unbounded => std::cmp::max(g.size(), v_min + 1),
		};
		assert!(
			!(v_max <= 1 && e_min > 0),
			"Cannot generate a graph with 0 vertices but minimum {} edges.",
			e_min
		);
		assert!(v_min < v_max, "{} >= {}", v_min, v_max);

		let e_max = match e_range.end_bound()
		{
			Bound::Included(&x) => x + 1,
			Bound::Excluded(&x) => x,
			Bound::Unbounded => std::cmp::max(g.size(), e_min + 1),
		};

		assert!(e_min < e_max);

		let (v_count, e_count) = Self::choose_size(g, v_min, v_max, e_min, e_max);
		Self::arbitrary_fixed(g, v_count, e_count)
	}

	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		(g.gen_range(v_min, v_max), g.gen_range(e_min, e_max))
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self;

	fn shrink_guided(&self, _limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>;
}

#[derive(Clone, Debug)]
pub struct Arb<G: GuidedArbGraph>(pub G)
where
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType;

impl<Gr: GuidedArbGraph> Arbitrary for Arb<Gr>
where
	Gr::Graph: TestGraph,
	<Gr::Graph as Graph>::EdgeWeight: MockType,
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Arb(Gr::arbitrary_guided(g, .., ..))
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(self.0.shrink_guided(HashSet::new()).map(|g| Arb(g)))
	}
}
