use crate::mock_graph::{
	arbitrary::{ArbVerticesIn, GuidedArbGraph, Limit},
	MockEdgeWeight, MockVertex, MockVertexWeight,
};
use graphene::core::{Graph, GraphDeref, GraphDerefMut};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{collections::HashSet, iter::FromIterator, ops::RangeBounds};

/// An arbitrary graph and two vertices in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never
/// includes the empty graph.
#[derive(Clone, Debug)]
pub struct ArbTwoVerticesIn<G>(pub G, pub MockVertex, pub MockVertex)
where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>;

impl<Gr> Arbitrary for ArbTwoVerticesIn<Gr>
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
impl<Gr> GuidedArbGraph for ArbTwoVerticesIn<Gr>
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
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);

		// Create a graph with at least 1 vertex
		let v_min_max = if 1 < v_min { v_min } else { 1 };
		let graph = Gr::arbitrary_guided(g, v_min_max..v_max, e_min..e_max);
		let verts: Vec<_> = graph.graph().all_vertices().collect();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = verts[g.gen_range(0, verts.len())];

		ArbTwoVerticesIn(graph, v1, v2)
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		// Don't let it shrink to less than 1 vertex, can happen if self.1 and self.2
		// are equal
		limits.insert(Limit::VertexMin(1));
		Box::new(
			ArbVerticesIn(
				self.0.clone(),
				HashSet::from_iter([self.1, self.2].iter().cloned()),
			)
			.shrink_guided(limits)
			.map(|g| {
				// we cycle, such that when the set only contains 1 vertex, we can use the same
				// one for both positions.
				let mut set = g.1.iter().cycle();
				Self(g.0, *set.next().unwrap(), *set.next().unwrap())
			}),
		)
	}
}

impl<G> GraphDeref for ArbTwoVerticesIn<G>
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
impl<G> GraphDerefMut for ArbTwoVerticesIn<G>
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
