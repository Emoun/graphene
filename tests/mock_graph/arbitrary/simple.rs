use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockGraph,
};
use graphene::core::{
	property::{AddEdge, SimpleGraph},
	EnsureUnloaded, Graph, ReleaseUnloaded, Undirected,
};
use quickcheck::Gen;
use rand::Rng;
use std::collections::HashSet;

impl GuidedArbGraph for SimpleGraph<MockGraph<Undirected, ()>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		mut v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(
			!(v_max <= 1 && e_min >= 1),
			"A simple graph of one or less vertices cannot have any edges"
		);

		// Used to calculate the maximum possible number of edges
		// in a simple graph with a number of vertices
		let max_allowed = |v: usize| (v * (v.saturating_sub(1))) / 2;

		if v_min <= 1 && v_max > 1 && e_min >= 1
		{
			// A simple graph of 1 vertex cannot have any edges, so increase the minimum
			// number of vertices
			v_min = 2;
		}

		assert!(
			e_min <= max_allowed(v_max),
			"Minimum number of edges higher than theoretically possible: e_min: {}, Max possible: \
			 {}",
			e_min,
			max_allowed(v_max)
		);

		let v_count = g.gen_range(v_min, v_max);
		let t_max = max_allowed(v_count) + 1;
		let e_count = g.gen_range(e_min, std::cmp::min(e_max, t_max));

		(v_count, e_count)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let mut graph = MockGraph::arbitrary_fixed(g, v_count, 0);
		let verts: Vec<_> = graph.all_vertices().collect();
		let mut edges_added = 0;

		while edges_added < e_count
		{
			assert!(v_count > 0);
			let v1 = verts[g.gen_range(0, v_count)];
			let v2 = verts[g.gen_range(0, v_count)];

			if v1 != v2 && graph.edges_between(v1, v2).count() == 0
			{
				graph.add_edge_weighted(v1, v2, ()).unwrap();
				edges_added += 1;
			}
		}

		Self::ensure(graph).unwrap()
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.clone()
				.release_all()
				.shrink_guided(limits)
				.map(|g| Self::unchecked(g)),
		)
	}
}
