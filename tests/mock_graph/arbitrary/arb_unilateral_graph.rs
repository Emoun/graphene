use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph,
};
use graphene::core::{
	property::{AddEdge, UnilateralGraph},
	Directed, EnsureUnloaded, Graph, ReleaseUnloaded,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::collections::HashSet;

fn is_unilateral(graph: &MockGraph<Directed>) -> bool
{
	let v_all = graph.all_vertices().collect::<Vec<_>>();
	let v_count = v_all.len();

	if v_count <= 1
	{
		// Trivial cases.
		return true;
	}

	let mut iter = v_all.iter();
	while let Some(v1) = iter.next()
	{
		let rest = iter.clone();
		for v2 in rest
		{
			if !graph.dfs_rec(*v1, Some(*v2), &mut Vec::new())
				&& !graph.dfs_rec(*v2, Some(*v1), &mut Vec::new())
			{
				return false;
			}
		}
	}
	true
}

impl GuidedArbGraph for UnilateralGraph<MockGraph<Directed>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(e_max > (v_max - 1));

		let v_count = g.gen_range(v_min, v_max);

		(
			v_count,
			g.gen_range(std::cmp::max(v_count.saturating_sub(1), e_min), e_max),
		)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let mut graph = MockGraph::arbitrary_fixed(g, v_count, 0);

		let verts: Vec<_> = graph.all_vertices().collect();
		let mut v_iter = verts.iter();

		// First, make a path through all the vertices.
		// For 1 or 0, they are trivially unilateral
		if verts.len() > 1
		{
			let mut v1 = v_iter.next().unwrap();
			for v2 in v_iter
			{
				graph
					.add_edge_weighted(v1, v2, MockEdgeWeight::arbitrary(g))
					.unwrap();
				v1 = v2;
			}
		}

		// Add random edges as needed
		if verts.len() >= 1
		{
			for _ in v_count..e_count
			{
				let v1 = verts[g.gen_range(0, verts.len())];
				let v2 = verts[g.gen_range(0, verts.len())];
				graph
					.add_edge_weighted(v1, v2, MockEdgeWeight::arbitrary(g))
					.unwrap();
			}
		}

		Self::ensure_unvalidated(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.clone().release();

		// Shrink by removing any edge that isn't critical for unilateralism
		graph.shrink_by_removing_edge(&limits, &mut result, is_unilateral);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self::ensure_unvalidated(g)))
	}
}
