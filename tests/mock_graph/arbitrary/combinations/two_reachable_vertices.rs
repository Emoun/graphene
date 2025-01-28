use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, TestGraph,
};
use graphene::{
	algo::{Bfs, Dfs},
	core::{
		property::{AddEdge, RemoveEdge, VertexIn, VertexInGraph},
		Ensure, Graph, GraphDerefMut, Release,
	},
	impl_ensurer,
};
use quickcheck::Gen;
use rand::Rng;
use std::{collections::HashSet, fmt::Debug};

/// An arbitrary graph and two vertices in it.
///
/// Guarantees that the second vertex is reachable from the first.
///
/// Depending on `U`, the two vertices are either allowed to be the same
/// (`NonUnique`, default), or they must be unique (`Unique`).
///
/// Note: All graphs will have at least 1 vertex for non-unique and 2 vertices
/// for unique, meaning this type never includes the empty graph.
#[derive(Clone, Debug)]
pub struct TwoReachableVerticesIn<G, const UNIQUE: bool = false>(pub VertexInGraph<G, 2, UNIQUE>)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType;

impl_ensurer! {
	use<G; const U: bool> TwoReachableVerticesIn<G,U>
	as (self.0): VertexInGraph<G, 2, U>
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
}

impl<Gr, const U: bool> GuidedArbGraph for TwoReachableVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + RemoveEdge + AddEdge,
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
		assert!(e_max > 1);

		// we need at least 1 edge. We'll delegate to TwoVerticesIn to ensure we get
		// at least 1 or 2 vertices (depending on U).
		VertexInGraph::<Gr, 2, U>::choose_size(g, v_min, v_max, std::cmp::max(e_min, 1), e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		// Create a graph with at least 1 or 2 vertices (1 for non-unique, 2 for Unique)
		let graph = VertexInGraph::<Gr, 2, U>::arbitrary_fixed(g, v_count, e_count).release();

		let mut vert_reachables: Vec<_> = graph
			.graph()
			.all_vertices()
			.map(|v| (v, Vec::new()))
			.collect();

		// Find all vertices with outgoing paths
		for (v, reachable) in vert_reachables.iter_mut()
		{
			reachable.extend(Dfs::new_simple(&VertexInGraph::ensure_unchecked(
				graph.graph(),
				[v.clone()],
			)));
			if !U && graph.graph().edges_between(v.clone(), v.clone()).count() > 0
			{
				reachable.push(v.clone());
			}
		}

		let verts_with_reachables: Vec<_> = vert_reachables
			.into_iter()
			.filter(|(_, reachables)| !reachables.is_empty())
			.collect();

		// Choose a vertex that starts the path
		let (v1, reachable) = verts_with_reachables
			.get(g.gen_range(0, verts_with_reachables.len()))
			.unwrap();

		// Choose a vertex that ends the path
		let v2 = reachable[g.gen_range(0, reachable.len())];

		Self(VertexInGraph::ensure_unchecked(graph, [v1.clone(), v2]))
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result: Vec<Self> = Vec::new();
		let v1 = self.0.vertex_at::<0>();
		let v2 = self.0.vertex_at::<1>();

		// First find a path between the vertices
		let g = VertexInGraph::ensure_unchecked(self, [v1]);
		let mut bfs = Bfs::new(&g);

		let _ = bfs.find(|&v| v == v2);

		// Ensure the path doesn't get removed
		let mut sink = v2;
		while sink != v1
		{
			limits.insert(Limit::VertexKeep(sink));
			let source = bfs.predecessor(sink).unwrap();
			limits.insert(Limit::EdgeKeep(source, sink));
			sink = source;
		}

		if v1 == v2
		{
			limits.insert(Limit::EdgeKeep(v1, v2));
		}

		// Shrink everything else, then ensure
		// the order of the vertices is the same (ArbTwoVerticesIn doesn't
		// guarantee their order is maintained through shrink)
		let clone = self.0.clone();
		result.extend({
			clone
				.0
				.shrink_guided(limits)
				.map(|g| Self(VertexInGraph::ensure_unchecked(g, [v1, v2])))
		});

		// Shrink by either removing superfluous edges from last link
		// in the path, or removing v2
		if v1 != v2 || self.edges_between(v1, v1).count() > 1
		{
			let source = if v1 == v2
			{
				v1
			}
			else
			{
				bfs.predecessor(v2).unwrap()
			};
			let sink = v2;
			let mut g = clone.clone();

			let v2 = if self.edges_between(source, sink).count() > 1
			{
				v2
			}
			else
			{
				source
			};
			let w = g.remove_edge(source, sink).unwrap();
			if v2 == v1
			{
				g.add_edge_weighted(v1, v1, w).unwrap();
			}
			let g = g.release();
			result.push(Self(VertexInGraph::ensure_unchecked(g, [v1, v2])));
		}
		Box::new(result.into_iter())
	}
}
