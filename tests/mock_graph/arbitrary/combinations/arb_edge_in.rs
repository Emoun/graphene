use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, GuidedArbGraph, Limit, NonUnique},
	MockEdgeWeight, MockVertex, TestGraph,
};
use graphene::{
	core::{
		property::{AddEdge, HasVertexGraph, RemoveEdge},
		Edge, EnsureUnloaded, Graph, GraphDerefMut, GraphMut, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{collections::HashSet, ops::RangeBounds};

/// An arbitrary graph with an edge that is guaranteed to be in the graph (the
/// weight is a clone)
#[derive(Clone, Debug)]
pub struct ArbEdgeIn<G>(
	pub HasVertexGraph<G>,
	pub (MockVertex, MockVertex, MockEdgeWeight),
)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph;

impl<Gr> GuidedArbGraph for ArbEdgeIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + GraphMut + AddEdge + RemoveEdge,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		let (v_min, v_max, e_min, e_max) =
			Self::validate_ranges(g, v_min..v_max, std::cmp::max(1, e_min)..e_max);

		let arb_graph = Gr::arbitrary_guided(g, v_min..v_max, e_min..e_max);

		let graph = arb_graph.graph();
		let edge = graph
			.all_edges()
			.nth(g.gen_range(0, graph.all_edges().count()))
			.unwrap();

		let edge_clone = (edge.source(), edge.sink(), edge.2.clone());

		Self(HasVertexGraph::ensure_unvalidated(arb_graph), edge_clone)
	}

	fn shrink_guided(&self, _: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		// 	First, we can simply shrink the weight
		result.extend((self.1).2.shrink().map(|shrunk| {
			let mut clone = self.0.clone();
			let edge = clone
				.graph_mut()
				.edges_between_mut(self.1.source(), self.1.sink())
				.find(|w| **w == (self.1).2)
				.unwrap();
			*edge = shrunk.clone();
			Self(clone, ((self.1).0, (self.1).1, shrunk))
		}));

		// We shrink each vertex in the edge
		let mut without_edge = self.0.clone().release();
		without_edge
			.graph_mut()
			.remove_edge_where_weight(&self.1.source(), &self.1.sink(), |w| *w == (self.1).2)
			.unwrap();
		result.extend(
			ArbTwoVerticesIn::<_, NonUnique>::new(without_edge, (self.1).0, (self.1).1)
				.shrink()
				.map(|mut g| {
					let (v1, v2) = g.get_both();
					g.graph_mut()
						.add_edge_weighted(&v1, &v2, (self.1).2.clone())
						.unwrap();
					Self(
						HasVertexGraph::ensure(g.release().release().release()).unwrap(),
						(v1, v2, (self.1).2.clone()),
					)
				}),
		);

		Box::new(result.into_iter())
	}
}
impl<Gr> Arbitrary for ArbEdgeIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + GraphMut + AddEdge + RemoveEdge,
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self::arbitrary_guided(g, .., 1..)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl<G> graphene::core::Ensure for ArbEdgeIn<G>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		let edge = c.all_edges().next().unwrap();
		let edge_copy = (edge.0, edge.1, edge.2.clone());
		Self(c, edge_copy)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		c.all_edges().count() >= 1
	}
}

impl_ensurer! {
	use<G> ArbEdgeIn<G>: Ensure
	as (self.0): HasVertexGraph<G>
	where
	G: GuidedArbGraph,
	G::Graph:  TestGraph
}
