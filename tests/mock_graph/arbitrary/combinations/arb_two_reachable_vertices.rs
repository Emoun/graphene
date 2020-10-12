use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, ArbVertexIn, GuidedArbGraph, Limit, NonUnique, Uniqueness},
	TestGraph,
};
use graphene::{
	algo::{Bfs, Dfs},
	core::{
		property::{AddEdge, RemoveEdge, VertexInGraph},
		Ensure, Graph, GraphDerefMut, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{collections::HashSet, ops::RangeBounds};

/// An arbitrary graph and two vertices in it.
///
/// Depending on `U`, the two vertices are either allowed to be the same
/// (`NonUnique`, default), or they must be unique (`Unique`).
///
/// Note: All graphs will have at least 1 vertex for non-unique and 2 vertices
/// for unique, meaning this type never includes the empty graph.
#[derive(Clone, Debug)]
pub struct ArbTwoReachableVerticesIn<G, U = NonUnique>(pub ArbTwoVerticesIn<G, U>)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	U: Uniqueness;

impl<Gr, U> Arbitrary for ArbTwoReachableVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + RemoveEdge + AddEdge,
	U: 'static + Uniqueness,
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
impl<Gr, U> GuidedArbGraph for ArbTwoReachableVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph: TestGraph + RemoveEdge + AddEdge,
	U: 'static + Uniqueness,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(
			g,
			v_min..v_max,
			std::cmp::max(e_min, 1)..std::cmp::max(e_max, 2),
		);

		// Create a graph with at least 1 or 2 vertices (1 for non-unique, 2 for Unique)
		let v_min_min = 1 + (U::unique() as usize);
		let v_min_max = if v_min_min < v_min { v_min } else { v_min_min };
		let graph = Gr::arbitrary_guided(g, v_min_max..v_max, e_min..e_max);

		let mut vert_reachables: Vec<_> = graph
			.graph()
			.all_vertices()
			.map(|v| (v, Vec::new()))
			.collect();

		// Find all vertices with outgoing paths
		for (v, reachable) in vert_reachables.iter_mut()
		{
			reachable.extend(Dfs::new_simple(&VertexInGraph::ensure_unvalidated(
				graph.graph(),
				v.clone(),
			)));
			if !U::unique() && graph.graph().edges_between(v.clone(), v.clone()).count() > 0
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

		Self(ArbTwoVerticesIn::new(graph, v1.clone(), v2))
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result: Vec<Self> = Vec::new();
		let (v1, v2) = self.0.get_both();

		// First find a path between the vertices
		let g = VertexInGraph::ensure_unvalidated(self, v1);
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
		result.extend(clone.shrink_guided(limits.clone()).map(|mut g| {
			g.0 = ArbVertexIn(VertexInGraph::ensure_unvalidated((g.0).0.release(), v1));
			g.1 = v2;
			Self(g)
		}));

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
			g.1 = v2;
			result.push(Self(g));
		}
		Box::new(result.into_iter())
	}
}

impl_ensurer! {
	use<G,U> ArbTwoReachableVerticesIn<G,U>
	as (self.0): ArbTwoVerticesIn<G,U>
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	U: Uniqueness
}
