use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, MockVertex, TestGraph,
};
use graphene::{
	core::{
		property::{AddEdge, RemoveEdge, VertexIn, VertexInGraph},
		Directedness, Graph, GraphDerefMut, GraphMut,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{borrow::Borrow, collections::HashSet, fmt::Debug};

/// An arbitrary graph with an edge that is guaranteed to be in the graph (the
/// weight is a clone).
/// The source of the edge can be accessed through `.get_vertex`, the sink `.1`,
/// and the weight `.2`
#[derive(Clone, Debug)]
pub struct EdgeIn<G: GuidedArbGraph>(
	pub VertexInGraph<G>,
	pub MockVertex,
	pub <G::Graph as Graph>::EdgeWeight,
)
where
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType;

impl<G> graphene::core::Ensure for EdgeIn<G>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		let (sink, weight) = {
			let edge = c.edges_sourced_in(c.vertex_at::<0>()).next().unwrap();
			(edge.0, edge.1.borrow().clone())
		};
		Self(c, sink, weight)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		c.all_edges().count() >= 1
	}
}

impl_ensurer! {
	use<G> EdgeIn<G>: Ensure
	as ( self.0) : VertexInGraph<G>
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType
}

impl<Gr> GuidedArbGraph for EdgeIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + GraphMut + AddEdge + RemoveEdge,
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
		assert!(v_max > 1);
		assert!(e_max > 1);
		Gr::choose_size(
			g,
			std::cmp::max(v_min, 1),
			v_max,
			std::cmp::max(e_min, 1),
			e_max,
		)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= 1);
		assert!(e_count >= 1);

		let graph = Gr::arbitrary_fixed(g, v_count, e_count);

		let (source, sink, weight) = graph
			.graph()
			.all_edges()
			.map(|(so, si, w)| (so, si, w.borrow().clone()))
			.nth(g.gen_range(0, e_count))
			.unwrap();
		Self(
			graphene::core::Ensure::ensure_unchecked(graph, [source]),
			sink,
			weight,
		)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let v1 = self.vertex_at::<0>();
		let v2 = self.1;
		let mut result = Vec::new();

		// First shrink anything except this edge (or any others
		// with the same source sink)
		let mut lims = limits.clone();
		lims.insert(Limit::EdgeKeep(v1, v2));
		result.extend(self.0.shrink_guided(lims).map(|g| {
			let weight = g.edges_between(v1, v2).next().unwrap().borrow().clone();
			Self(g, v2, weight.clone())
		}));

		// Now shrink by removing any extra edges
		// and shrinking them (not at the same time)
		if !limits.contains(&Limit::EdgeKeep(v1, v2))
			&& (<Self as Graph>::Directedness::directed()
				|| !limits.contains(&Limit::EdgeKeep(v2, v1)))
		{
			let mut saw_reference_edge_before = false;
			let mut shrunk_reference_weight_before = false;
			for w in self.edges_between(v1, v2)
			{
				let w = w.borrow();
				// Remove edge
				if !saw_reference_edge_before && *w == self.2
				{
					// Cannot remove the reference edge, if its the only one
					saw_reference_edge_before = true
				}
				else
				{
					let mut g = self.clone();
					g.remove_edge_where_weight(v1, v2, |weight| w == weight)
						.unwrap();
					result.push(g);
				}

				// Shrink weight
				self.2.shrink().for_each(|s_w| {
					let mut shrunk_graph = self.clone();
					*shrunk_graph
						.edges_between_mut(v1, v2)
						.find(|w| **w == self.2)
						.unwrap() = s_w.clone();
					if !shrunk_reference_weight_before && self.2 == *w
					{
						shrunk_graph.2 = s_w;
						// We only need to update the reference weight for one edge
						shrunk_reference_weight_before = true;
					}
					result.push(shrunk_graph);
				});
			}
		}
		Box::new(result.into_iter())
	}
}
