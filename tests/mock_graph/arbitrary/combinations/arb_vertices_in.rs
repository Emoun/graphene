use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockVertex, MockVertexWeight,
};
use graphene::core::{Graph, ImplGraph, ImplGraphMut};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{
	collections::{HashSet},
	ops::RangeBounds,
};

/// An arbitrary graph and an arbitrary set of vertices in it.
#[derive(Clone, Debug)]
pub struct ArbVerticesIn<G>(pub G, pub HashSet<MockVertex>)
where
	G: Arbitrary + ImplGraph,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVerticesIn<Gr>
where
	Gr: GuidedArbGraph + ImplGraphMut,
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
impl<Gr> GuidedArbGraph for ArbVerticesIn<Gr>
where
	Gr: GuidedArbGraph + ImplGraphMut,
	Gr::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let graph = Gr::arbitrary_guided(g, v_range, e_range);
		let v_count = graph.graph().all_vertices().count();

		let mut set = HashSet::new();

		if v_count > 0
		{
			let v_expected = g.gen_range(0, v_count + 1);
			let v_saturation = v_expected as f64 / v_count as f64;
			for v in graph.graph().all_vertices()
			{
				if g.gen_bool(v_saturation)
				{
					set.insert(v);
				}
			}
		}
		Self(graph, set)
	}

	fn shrink_guided(
		&self,
		mut limits: HashSet<Limit>,
	) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let arb_graph = &self.0;

		// First we shrink the graph without touching the designated vertices
		for v in self.1.iter()
		{
			limits.insert(Limit::VertexKeep(*v));
		}
		result.extend(
			arb_graph
				.shrink_guided(limits.clone())
				.map(|g| Self(g, self.1.clone())),
		);

		// The we simply remove one of the vertices and keep the rest
		if Limit::min_vertices(&limits) < self.0.graph().all_vertices().count() {
			for v in self.1.iter()
			{
				let mut new_set = self.1.clone();
				new_set.remove(v);
				result.push(Self(self.0.clone(), new_set));
			}
		}
		Box::new(result.into_iter())
	}
}
