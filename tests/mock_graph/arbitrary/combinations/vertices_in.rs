use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockVertex, TestGraph,
};
use graphene::{
	core::{Ensure, Graph, GraphDerefMut},
	impl_ensurer,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

/// An arbitrary graph and an arbitrary set of vertices in it.
#[derive(Clone, Debug)]
pub struct VerticesIn<G>(pub G, pub HashSet<MockVertex>)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph;

impl<G> Ensure for VerticesIn<G>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c, HashSet::new())
	}

	fn validate(_: &Self::Ensured, _: &()) -> bool
	{
		true
	}
}

impl_ensurer! {
	use<G> VerticesIn<G>: Ensure
	as (self.0): G
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph
}

impl<Gr> GuidedArbGraph for VerticesIn<Gr>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph,
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
		let graph = Gr::arbitrary_fixed(g, v_count, e_count);
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

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let arb_graph = &self.0.clone();

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
		if Limit::min_vertices(&limits) < self.0.graph().all_vertices().count()
		{
			for v in self
				.1
				.iter()
				.filter(|v| !limits.contains(&Limit::VertexKeep(**v)))
			{
				let mut new_set = self.1.clone();
				new_set.remove(v);
				result.push(Self(self.0.clone(), new_set));
			}
		}
		Box::new(result.into_iter())
	}
}
