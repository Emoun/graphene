use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, TestGraph,
};
use graphene::core::{
	property::{HasVertex, VertexInGraph},
	Ensure, Graph, Release,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl<Gr: GuidedArbGraph> GuidedArbGraph for VertexInGraph<Gr>
where
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
		assert!(v_max > 1);
		Gr::choose_size(g, std::cmp::max(v_min, 1), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= 1);
		let graph = Gr::arbitrary_fixed(g, v_count, e_count);
		let v = graph
			.graph()
			.all_vertices()
			.nth(g.gen_range(0, v_count))
			.unwrap();

		Self::ensure_unchecked(graph, [v])
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let v = self.get_vertex();
		limits.insert(Limit::VertexKeep(v));
		Box::new(
			self.clone()
				.release()
				.shrink_guided(limits)
				.map(move |g| Self::ensure_unchecked(g, [v])),
		)
	}
}
