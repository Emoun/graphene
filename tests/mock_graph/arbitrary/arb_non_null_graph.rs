use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockGraph,
};
use graphene::{
	core::{
		property::NonNullGraph, BaseGraph, Directedness, GraphDeref, GraphDerefMut, Insure, Release,
	},
	impl_insurer,
};
use quickcheck::{Arbitrary, Gen};
use static_assertions::_core::ops::RangeBounds;
use std::collections::HashSet;

/// An arbitrary graph with at least 1 vertex
#[derive(Clone, Debug)]
pub struct ArbNonNullGraph<D: Directedness>(pub NonNullGraph<MockGraph<D>>);

impl<D: Directedness> GuidedArbGraph for ArbNonNullGraph<D>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);

		// Create a graph with at least 1 vertex
		let v_min_max = if 1 < v_min { v_min } else { 1 };
		let graph = MockGraph::arbitrary_guided(g, v_min_max..v_max, e_min..e_max);

		Self(graph.insure_all().expect("Graph is null."))
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		// Don't let it shrink to less than 1 vertex
		limits.insert(Limit::VertexMin(1));

		Box::new(
			self.0
				.clone()
				.release_all()
				.shrink_guided(limits)
				.map(|g| Self(NonNullGraph::insure(g).unwrap())),
		)
	}
}

impl<D: Directedness> Arbitrary for ArbNonNullGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self::arbitrary_guided(g, .., ..)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl_insurer! {
	ArbNonNullGraph<D>
	for NonNullGraph<MockGraph<D>> as (self.0)
	where D: Directedness
}
