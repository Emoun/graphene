use crate::mock_graph::{
	arbitrary::{ArbVertexIn, GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph, MockVertexWeight,
};
use graphene::{
	core::{
		property::{AddEdge, NewVertex, NonNull, RemoveEdge, UnilateralGraph},
		Directed, Edge, Graph, GraphDeref, GraphDerefMut, Insure, Release,
	},
	impl_insurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{collections::HashSet, ops::RangeBounds};

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

/// An arbitrary graph that is unilaterally connected
#[derive(Clone, Debug)]
pub struct ArbUnilatralGraph(pub UnilateralGraph<MockGraph<Directed>>);

impl GuidedArbGraph for ArbUnilatralGraph
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		// If we are asked to make the empty graph, we just do
		if v_max <= 1
		{
			return Self(UnilateralGraph::insure_unvalidated(
				MockGraph::arbitrary_guided(g, v_min..v_max, v_min..v_max),
			));
		}

		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max
		{
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max);
		}

		let mut graph;
		let v;
		// If the target is exactly 1 vertex, just create it
		if v_min == 1
		{
			graph = MockGraph::empty();
			v = graph
				.new_vertex_weighted(MockVertexWeight::arbitrary(g))
				.unwrap();
		}
		else
		{
			// For larger graphs, we start by making a unilateral graph with 1 less vertex
			// and get an edge
			let arb = ArbVertexIn::<Self>::arbitrary_guided(g, v_min - 1..v_min, e_min..e_max);
			let v_original = arb.get_vertex();
			graph = (arb.0).release().0.release();

			// Add a new vertex to the graph
			v = graph
				.new_vertex_weighted(MockVertexWeight::arbitrary(g))
				.unwrap();

			// Add an edge to/from the new and old vertices
			if g.gen_bool(0.5)
			{
				// To insure unilateralism, take all outgoing edges from the original vertex
				// and move them to the new one.
				let outgoing_sinks = graph
					.edges_sourced_in(v_original)
					.map(|e| e.sink())
					.collect::<Vec<_>>();
				for sink in outgoing_sinks
				{
					let weight = graph.remove_edge((v_original, sink)).unwrap();
					graph.add_edge_weighted((v, sink, weight)).unwrap();
				}

				graph
					.add_edge_weighted((v_original, v, MockEdgeWeight::arbitrary(g)))
					.unwrap();
			}
			else
			{
				// To insure unilateralism, take all the incoming edges from the original vertex
				// and move them to the new one.
				let sources = graph
					.edges_sinked_in(v_original)
					.map(|e| e.source())
					.collect::<Vec<_>>();
				for source in sources
				{
					let weight = graph.remove_edge((source, v_original)).unwrap();
					graph.add_edge_weighted((source, v, weight)).unwrap();
				}

				graph
					.add_edge_weighted((v, v_original, MockEdgeWeight::arbitrary(g)))
					.unwrap();
			}
		}

		// We now randomly create additional edges from/to the new vertex
		let p = 0.25 / (v_min as f64);
		for v_other in graph.all_vertices().collect::<Vec<_>>()
		{
			if g.gen_bool(p)
			{
				graph
					.add_edge_weighted((v, v_other, MockEdgeWeight::arbitrary(g)))
					.unwrap();
			}
			if g.gen_bool(p)
			{
				graph
					.add_edge_weighted((v_other, v, MockEdgeWeight::arbitrary(g)))
					.unwrap();
			}
		}

		Self(UnilateralGraph::insure_unvalidated(graph))
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.0.clone().release();

		// Shrink by removing any edge that isn't critical for unilateralism
		graph.shrink_by_removing_edge(&limits, &mut result, is_unilateral);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(
			result
				.into_iter()
				.map(|g| Self(UnilateralGraph::insure_unvalidated(g))),
		)
	}
}

impl Arbitrary for ArbUnilatralGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Self::arbitrary_guided(g, .., ..).0.release();
		// 		assert!(is_unilateral(&graph));
		Self(UnilateralGraph::insure_unvalidated(graph))
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl_insurer! {
	ArbUnilatralGraph:
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex,
	// Can never impl the following
	Subgraph, NonNull
	for UnilateralGraph<MockGraph<Directed>> as (self.0)
}

/// An arbitrary graph that is not unilaterally connected
#[derive(Clone, Debug)]
pub struct ArbNonUnilatralGraph(pub MockGraph<Directed>);

impl GraphDeref for ArbNonUnilatralGraph
{
	type Graph = MockGraph<Directed>;

	fn graph(&self) -> &Self::Graph
	{
		&self.0
	}
}
impl GraphDerefMut for ArbNonUnilatralGraph
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		&mut self.0
	}
}

impl GuidedArbGraph for ArbNonUnilatralGraph
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);

		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max
		{
			// Ensure we only try to create graphs with 2 <= vertices, as any smaller is
			// trivially unilateral.
			let (v_min, v_max) = if v_min >= 2
			{
				(v_min, v_max)
			}
			else
			{
				(2, if v_max > 2 { v_max } else { 3 })
			};

			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max);
		}

		// If we are asked to make the empty or singleton graph, we cant do that (as its
		// trivially unilateral)
		if v_max <= 2
		{
			panic!(
				"Cannot make a non-unilateral graph with at most '{}' vertices (its trivially \
				 unilateral).",
				v_max - 1
			);
		}

		let mut graph;
		// If we are asked to make a graph with 2 vertices, we do so directly
		if v_min == 2
		{
			graph = MockGraph::<Directed>::arbitrary_guided(g, 1..2, e_min / 2..e_max / 2);
			let g2 = MockGraph::<Directed>::arbitrary_guided(g, 1..2, e_min / 2..e_max / 2);
			graph.join(&g2);
		}
		else
		{
			// Create two unilateral graphs
			let g1_count = g.gen_range(1, v_min - 1);
			let g2_count = (v_min - 1) - g1_count;

			let g1 = ArbVertexIn::<ArbUnilatralGraph>::arbitrary_guided(
				g,
				g1_count..g1_count + 1,
				e_min / 2..e_max / 2,
			);
			let g2 = ArbVertexIn::<ArbUnilatralGraph>::arbitrary_guided(
				g,
				g2_count..g2_count + 1,
				e_min / 2..e_max / 2,
			);

			let v1 = g1.get_vertex();
			let v2 = g2.get_vertex();
			graph = g1.0.release().0.release();
			let g2 = g2.0.release().0.release();

			// Join them and add a vertex that is reachable from or can reach both,
			// but doesn't have a path through it, ensuring the two components can't reach
			// each other.
			let map = graph.join(&g2);

			let v = graph
				.new_vertex_weighted(MockVertexWeight::arbitrary(g))
				.unwrap();

			if g.gen_bool(0.5)
			{
				graph
					.add_edge_weighted((v, v1, MockEdgeWeight::arbitrary(g)))
					.unwrap();
				if g.gen_bool(0.8)
				{
					graph
						.add_edge_weighted((v, map[&v2], MockEdgeWeight::arbitrary(g)))
						.unwrap();
				}
			}
			else
			{
				graph
					.add_edge_weighted((v1, v, MockEdgeWeight::arbitrary(g)))
					.unwrap();
				if g.gen_bool(0.8)
				{
					graph
						.add_edge_weighted((map[&v2], v, MockEdgeWeight::arbitrary(g)))
						.unwrap();
				}
			}
		}
		Self(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();

		// We shrink the MockGraph, keeping only the shrunk graphs that are still
		// non-unilateral
		result.extend(
			self.0
				.shrink_guided(limits)
				.filter(|g| !is_unilateral(&g))
				.map(|g| Self(g)),
		);

		Box::new(result.into_iter())
	}
}

impl Arbitrary for ArbNonUnilatralGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Self::arbitrary_guided(g, .., ..).0.release();
		assert!(!is_unilateral(&graph));
		Self(graph)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}
