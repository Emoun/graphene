use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, GuidedArbGraph, Limit, NonUnique},
	MockVertex, TestGraph,
};
use graphene::{
	core::{
		property::{HasVertex, VertexInGraph},
		Graph, GraphDerefMut, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{
	collections::{hash_map::RandomState, HashSet},
	ops::RangeBounds,
};

/// An arbitrary graph and a vertex in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never
/// includes the empty graph.
#[derive(Clone, Debug)]
pub struct ArbVertexIn<G>(pub VertexInGraph<G, MockVertex>)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph;
impl<Gr> Arbitrary for ArbVertexIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph,
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
	Gr::Graph: TestGraph,
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
		let v = verts[g.gen_range(0, verts.len())];

		Self(VertexInGraph::new_unvalidated(graph, v))
	}

	fn shrink_guided(&self, limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			ArbTwoVerticesIn::<_, NonUnique>::new(
				self.0.clone().release(),
				self.get_vertex(),
				self.get_vertex(),
			)
			.shrink_guided(limits)
			.map(|g| g.0),
		)
	}
}

impl_ensurer! {
	use<G> ArbVertexIn<G>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as ( self.0) : VertexInGraph<G, MockVertex>
	where
	G: GuidedArbGraph,
	G::Graph:  TestGraph
}
