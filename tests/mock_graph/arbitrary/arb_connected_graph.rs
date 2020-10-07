use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph, MockVertex,
};
use graphene::{
	core::{
		property::{AddEdge, ConnectedGraph, WeakGraph},
		Directed, Directedness, Graph, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::{
	collections::{hash_map::RandomState, HashSet},
	ops::RangeBounds,
};

/// This is very inefficient (but never-the-less correct)
fn is_connected<D: Directedness>(graph: &MockGraph<D>) -> bool
{
	let v_all = graph.all_vertices().collect::<Vec<_>>();
	let v_count = v_all.len();

	if v_count <= 1
	{
		// Trivial cases.
		return true;
	}

	if D::directed()
	{
		v_all
			.iter()
			.flat_map(|&v| v_all.iter().map(move |&v_other| (v, v_other)))
			.all(|(v, v_other)| {
				let mut visited = Vec::new();
				graph.dfs_rec(v, Some(v_other), &mut visited)
			})
	}
	else
	{
		let mut visited = Vec::new();
		graph.dfs_rec(v_all[0], None, &mut visited);
		visited.len() == v_count
	}
}

fn is_weak(graph: &MockGraph<Directed>) -> bool
{
	let v_all = graph.all_vertices().collect::<Vec<_>>();
	let v_count = v_all.len();

	if v_count <= 1
	{
		// Trivial cases.
		return true;
	}

	let mut visited = Vec::new();

	fn dfs(graph: &MockGraph<Directed>, v: MockVertex, visited: &mut Vec<MockVertex>)
	{
		if !visited.contains(&v)
		{
			visited.push(v);

			for (e, _) in graph.edges_incident_on(v)
			{
				dfs(graph, e, visited);
			}
		}
	}

	dfs(graph, v_all[0], &mut visited);
	visited.len() == v_count
}

/// An arbitrary graph that is connected
#[derive(Clone, Debug)]
pub struct ArbConnectedGraph<D: Directedness>(pub ConnectedGraph<MockGraph<D>>);

impl<D: Directedness> GuidedArbGraph for ArbConnectedGraph<D>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		// If we are asked to make the singleton or empty graph, we just do
		if v_max <= 2
		{
			return Self(ConnectedGraph::new(MockGraph::arbitrary_guided(
				g,
				v_min..v_max,
				e_min..e_max,
			)));
		}

		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max
		{
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max);
		}

		let (v_count_1, v_count_2) = ((v_min / 2) + (v_min % 2), v_min / 2);
		let e_min_2 = e_min / 2;
		let e_min_1 = e_min_2 + (e_min % 2);
		let e_max_2 = e_max / 2;
		let e_max_1 = e_max_2 + (e_max % 2);

		// We create two smaller connected graphs
		let mut graph = Self::arbitrary_guided(g, v_count_1..v_count_1 + 1, e_min_1..e_max_1)
			.0
			.release();
		let g2 = Self::arbitrary_guided(g, v_count_2..v_count_2 + 1, e_min_2..e_max_2).0;

		// We find random vertices in each graph for later use
		let v11 = graph
			.all_vertices()
			.nth(g.gen_range(0, graph.all_vertices().count()))
			.unwrap();
		let v12 = graph
			.all_vertices()
			.nth(g.gen_range(0, graph.all_vertices().count()))
			.unwrap();
		let v21 = g2
			.all_vertices()
			.nth(g.gen_range(0, g2.all_vertices().count()))
			.unwrap();
		let v22 = g2
			.all_vertices()
			.nth(g.gen_range(0, g2.all_vertices().count()))
			.unwrap();

		// Join the second into the first making an unconnected graph with the 2
		// components
		let v_map = graph.join(&g2);

		// Add edges connecting the two components
		graph
			.add_edge_weighted(&v11, &v_map[&v21], MockEdgeWeight::arbitrary(g))
			.unwrap();
		if D::directed()
		{
			graph
				.add_edge_weighted(&v_map[&v22], &v12, MockEdgeWeight::arbitrary(g))
				.unwrap();
		}

		Self(ConnectedGraph::new(graph))
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.0.clone().release();

		// Shrink by removing any edge that isn't critical for connectedness
		graph.shrink_by_removing_edge(&limits, &mut result, is_connected);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self(ConnectedGraph::new(g))))
	}
}

impl<D: Directedness> Arbitrary for ArbConnectedGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Self::arbitrary_guided(g, .., ..).0.release();
		// 		assert!(is_connected(&graph));
		Self(ConnectedGraph::new(graph))
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl_ensurer! {
	use<D> ArbConnectedGraph<D>:
	// Can never impl the following because MockGraph doesn't
	Reflexive
	as (self.0): ConnectedGraph<MockGraph<D>>
	where D: Directedness
}

/// An arbitrary graph that is unconnected
#[derive(Clone, Debug)]
pub struct ArbUnconnectedGraph<D: Directedness>(pub MockGraph<D>);

impl<D: Directedness> Arbitrary for ArbUnconnectedGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Self::arbitrary_guided(g, .., ..).0;
		// 		assert!(is_connected(&graph));
		Self(graph)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl<D: Directedness> GuidedArbGraph for ArbUnconnectedGraph<D>
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		_v_range: impl RangeBounds<usize>,
		_e_range: impl RangeBounds<usize>,
	) -> Self
	{
		// We merge 2 graphs into 1. This will ensure they are not connected.
		// They must each have at least 1 vertex, otherwise the result
		// might be trivially connected (<2 vertices)
		let mut graph = MockGraph::arbitrary_guided(g, 1..(g.size() / 2), ..);
		let g2 = <MockGraph<D>>::arbitrary_guided(g, 1..(g.size() / 2), ..);

		// Join the two graph into one graph with no edges between the two parts
		graph.join(&g2);

		assert!(!is_connected(&graph));
		Self(graph)
	}

	fn shrink_guided(&self, _limits: HashSet<Limit, RandomState>)
		-> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();

		// We shrink the MockGraph, keeping only the shrunk graphs that are still
		// unconnected
		result.extend(
			self.0
				.shrink()
				.filter(|g| !is_connected(&g))
				.map(|g| Self(g)),
		);

		Box::new(result.into_iter())
	}
}

impl_ensurer! {
	use<D> ArbUnconnectedGraph<D>
	as (self.0): MockGraph<D>
	where D: Directedness
}

/// An arbitrary graph that is weakly connected
#[derive(Clone, Debug)]
pub struct ArbWeakGraph(pub WeakGraph<MockGraph<Directed>>);

impl GuidedArbGraph for ArbWeakGraph
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		// If we are asked to make the singleton or empty graph, we just do
		if v_max <= 2
		{
			return Self(WeakGraph::new(MockGraph::arbitrary_guided(
				g,
				v_min..v_max,
				e_min..e_max,
			)));
		}

		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max
		{
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max);
		}

		let (v_count_1, v_count_2) = ((v_min / 2) + (v_min % 2), v_min / 2);
		let e_min_2 = e_min / 2;
		let e_min_1 = e_min_2 + (e_min % 2);
		let e_max_2 = e_max / 2;
		let e_max_1 = e_max_2 + (e_max % 2);

		// We create two smaller weak graphs
		let mut graph = Self::arbitrary_guided(g, v_count_1..v_count_1 + 1, e_min_1..e_max_1)
			.0
			.release();
		let g2 = Self::arbitrary_guided(g, v_count_2..v_count_2 + 1, e_min_2..e_max_2).0;

		// We find random vertices in each graph for later use
		let v1 = graph
			.all_vertices()
			.nth(g.gen_range(0, graph.all_vertices().count()))
			.unwrap();
		let v2 = g2
			.all_vertices()
			.nth(g.gen_range(0, g2.all_vertices().count()))
			.unwrap();

		// Join the second into the first making an unconnected graph with the 2
		// components
		let v_map = graph.join(&g2);

		// Add edges connecting the two components
		if g.gen_bool(0.5)
		{
			graph
				.add_edge_weighted(&v1, &v_map[&v2], MockEdgeWeight::arbitrary(g))
				.unwrap();
		}
		else
		{
			graph
				.add_edge_weighted(&v_map[&v2], &v1, MockEdgeWeight::arbitrary(g))
				.unwrap();
		}

		Self(WeakGraph::new(graph))
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.0.clone().release();

		// Shrink by removing any edge that isn't critical for connectedness
		graph.shrink_by_removing_edge(&limits, &mut result, is_weak);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self(WeakGraph::new(g))))
	}
}

impl Arbitrary for ArbWeakGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Self::arbitrary_guided(g, .., ..).0.release();
		assert!(is_weak(&graph));
		Self(WeakGraph::new(graph))
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}

impl_ensurer! {
	ArbWeakGraph:
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex,
	// Can never impl the following
	Unique, NoLoops, Reflexive, Unilateral, Connected, Subgraph, HasVertex
	as (self.0) : WeakGraph<MockGraph<Directed>>
}
