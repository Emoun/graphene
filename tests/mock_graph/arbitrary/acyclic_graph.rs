use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph,
};
use graphene::{
	algo::Dfs,
	core::{
		property::{AcyclicGraph, AddEdge, VertexIn, VertexInGraph},
		Directedness, Graph, Guard, Release,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::collections::HashSet;

impl<D: Directedness> GuidedArbGraph for AcyclicGraph<MockGraph<D>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(v_max > (e_min + 1));

		let v_count = g.gen_range(std::cmp::max(v_min, e_min + 1), v_max);
		let e_count = g.gen_range(e_min, std::cmp::min(v_count, e_max));

		(v_count, e_count)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		// first get a graph with no edges, which is trivially acyclic
		let mut graph = MockGraph::<D>::arbitrary_fixed(g, v_count, 0);
		let verts: Vec<_> = graph.all_vertices().collect();
		let mut edges_added = 0;

		while edges_added < e_count
		{
			// Randomly choose two vertices to connect
			let v1 = verts[g.gen_range(0, verts.len())];
			let v2 = verts[g.gen_range(0, verts.len())];

			// Ensure there isn't already a path between the two
			let v1_in_g: VertexInGraph<_> = graphene::core::Ensure::ensure_unchecked(&graph, [v1]);
			if v1 != v2 && Dfs::new_simple(&v1_in_g).find(|&v| v == v2).is_none()
			{
				graph
					.add_edge_weighted(v2, v1, MockEdgeWeight::arbitrary(g))
					.unwrap();
				edges_added += 1;
			}
		}

		Self::guard_unchecked(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.clone()
				.release()
				.shrink_guided(limits)
				.map(|g| Self::guard_unchecked(g)),
		)
	}
}

/// An arbitrary graph that is cyclic
#[derive(Clone, Debug)]
pub struct CyclicGraph<D: Directedness>(pub MockGraph<D>);

impl_ensurer! {
	use<D> CyclicGraph<D>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0) : MockGraph<D>
	where D: Directedness
}

impl<D: Directedness> GuidedArbGraph for CyclicGraph<D>
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
		MockGraph::<D>::choose_size(g, std::cmp::max(v_min, 1), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let mut graph = VertexInGraph::<MockGraph<_>>::arbitrary_fixed(g, v_count, e_count);

		let mut reachable: Vec<_> = Dfs::new_simple(&graph).collect();
		reachable.push(graph.vertex_at::<0>()); // not added by DFS

		// Add random edge back to the beginning
		graph
			.add_edge_weighted(
				reachable[g.gen_range(0, reachable.len())],
				graph.vertex_at::<0>(),
				MockEdgeWeight::arbitrary(g),
			)
			.unwrap();

		Self(graph.release_all())
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(
			self.0
				.clone()
				.release_all()
				.shrink_guided(limits)
				.filter(|g| !AcyclicGraph::can_guard(&g))
				.map(|g| Self(g)),
		)
	}
}
