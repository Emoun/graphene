use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit, VertexOutside},
	MockType, MockVertex, TestGraph,
};
use graphene::{
	core::{Ensure, Graph},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use std::{collections::HashSet, fmt::Debug};

/// An arbitrary graph and two vertices where at least one is not in the graph.
#[derive(Clone, Debug)]
pub struct EdgeOutside<G>(pub G, pub MockVertex, pub MockVertex)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType;

impl<G> Ensure for EdgeOutside<G>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
{
	fn ensure_unchecked(_c: Self::Ensured, _: ()) -> Self
	{
		unimplemented!()
	}

	fn can_ensure(_c: &Self::Ensured, _: &()) -> bool
	{
		unimplemented!()
	}
}

impl_ensurer! {
	use<G> EdgeOutside<G>: Ensure
	as (self.0): G
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType
}

impl<Gr> GuidedArbGraph for EdgeOutside<Gr>
where
	Gr: GuidedArbGraph,
	Gr::Graph: TestGraph,
	<Gr::Graph as Graph>::EdgeWeight: MockType,
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		Gr::choose_size(g, v_min, v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let single_invalid = VertexOutside::arbitrary_fixed(g, v_count, e_count);
		Self(single_invalid.0, single_invalid.1, MockVertex::arbitrary(g))
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		// 	Shrink the graph, keeping only the shrunk graphs where the edge is still
		// invalid.
		result.extend(
			self.0
				.shrink_guided(limits.clone())
				.filter(|g| {
					!g.graph().contains_vertex(self.1) || !g.graph().contains_vertex(self.2)
				})
				.map(|g| Self(g, self.1, self.2)),
		);

		// 	We then shrink the vertices, ensuring that at least one of them stays
		// invalid
		result.extend(
			self.1
				.shrink()
				.filter(|v| {
					!self.0.graph().contains_vertex(v) || !self.0.graph().contains_vertex(self.2)
				})
				.map(|v| Self(self.0.clone(), v, self.2)),
		);
		result.extend(
			self.2
				.shrink()
				.filter(|v| {
					!self.0.graph().contains_vertex(self.1) || !self.0.graph().contains_vertex(v)
				})
				.map(|v| Self(self.0.clone(), self.1, v)),
		);
		Box::new(result.into_iter())
	}
}
