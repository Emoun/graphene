use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, GuidedArbGraph, Limit},
	MockEdgeWeight, MockVertex, MockVertexWeight,
};
use graphene::core::{Graph, GraphDeref, GraphDerefMut};
use quickcheck::{Arbitrary, Gen};
use std::{
	collections::{hash_map::RandomState, HashSet},
	ops::RangeBounds,
};

/// An arbitrary graph and a vertex in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never
/// includes the empty graph.
#[derive(Clone, Debug)]
pub struct ArbVertexIn<G>(pub G, pub MockVertex)
where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVertexIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
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
impl<Gr> GuidedArbGraph for ArbVertexIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let arb = ArbTwoVerticesIn::arbitrary_guided(g, v_range, e_range);
		ArbVertexIn(arb.0, arb.1)
	}

	fn shrink_guided(&self, limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			ArbTwoVerticesIn(self.0.clone(), self.1, self.1)
				.shrink_guided(limits)
				.map(|ArbTwoVerticesIn(g, v, _)| ArbVertexIn(g, v)),
		)
	}
}

impl<G> GraphDeref for ArbVertexIn<G>
where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
{
	type Graph = G::Graph;

	fn graph(&self) -> &Self::Graph
	{
		self.0.graph()
	}
}
impl<G> GraphDerefMut for ArbVertexIn<G>
where
	G: Arbitrary + GraphDerefMut,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self.0.graph_mut()
	}
}
