use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockEdgeWeight, MockGraph, MockVertex, MockVertexWeight,
};
use graphene::{
	core::{
		property::{AddEdge, ConnectedGraph, NewVertex, UnilateralGraph, WeakGraph},
		Directed, Directedness, Graph, Guard, Release, Undirected,
	},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use std::collections::{HashMap, HashSet};

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

impl GuidedArbGraph for ConnectedGraph<MockGraph<Directed>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		assert!(e_max >= v_max);

		let v_count = g.gen_range(v_min, v_max);

		(v_count, g.gen_range(std::cmp::max(v_count, e_min), e_max))
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		let mut graph = MockGraph::arbitrary_fixed(g, v_count, 0);

		let verts: Vec<_> = graph.all_vertices().collect();
		let mut v_iter = verts.iter();

		// First, make a cycle if there are more than 1 vertex.
		// For 1 or 0, they are trivially connected
		if verts.len() > 1
		{
			let mut v1 = v_iter.next().unwrap();
			for v2 in v_iter.chain(Some(v1))
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

		Self::guard_unchecked(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.clone().release();

		// Shrink by removing any edge that isn't critical for connectedness
		graph.shrink_by_removing_edge(&limits, &mut result, is_connected);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self::guard_unchecked(g)))
	}
}

impl GuidedArbGraph for ConnectedGraph<MockGraph<Undirected>>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		WeakGraph::choose_size(g, v_min, v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		// We simply copy a WeakGraph
		let weak_graph = WeakGraph::arbitrary_fixed(g, v_count, e_count);

		let mut graph = MockGraph::<Undirected>::empty();
		let mut map = HashMap::new();

		for (v, w) in weak_graph.all_vertices_weighted()
		{
			let new_v = graph.new_vertex_weighted(w.clone()).unwrap();
			map.insert(v, new_v);
		}

		for (v1, v2, w) in weak_graph.all_edges()
		{
			graph
				.add_edge_weighted(map[&v1], map[&v2], w.clone())
				.unwrap()
		}

		Self::guard_unchecked(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.clone().release();

		// Shrink by removing any edge that isn't critical for connectedness
		graph.shrink_by_removing_edge(&limits, &mut result, is_connected);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self::guard_unchecked(g)))
	}
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

		Self::guard_unchecked(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.clone().release();

		// Shrink by removing any edge that isn't critical for unilateralism
		graph.shrink_by_removing_edge(&limits, &mut result, is_unilateral);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self::guard_unchecked(g)))
	}
}

impl GuidedArbGraph for WeakGraph<MockGraph<Directed>>
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
		assert!(e_count >= (v_count.saturating_sub(1)));

		Self::guard_unchecked(
			if v_count < 2
			{
				MockGraph::arbitrary_fixed(g, v_count, 0)
			}
			else
			{
				// We make a graph with 1 less vertex and all weakly connected
				let mut graph = Self::arbitrary_fixed(g, v_count - 1, v_count - 2).release();

				// We add the last vertex, and connect it randomly
				let mut existing_verts: Vec<_> = graph.all_vertices().collect();
				let v = graph
					.new_vertex_weighted(MockVertexWeight::arbitrary(g))
					.unwrap();
				let v2 = existing_verts[g.gen_range(0, existing_verts.len())];
				let weight = MockEdgeWeight::arbitrary(g);

				if g.gen_bool(0.5)
				{
					graph.add_edge_weighted(v, v2, weight).unwrap();
				}
				else
				{
					graph.add_edge_weighted(v2, v, weight).unwrap();
				}

				// Add new vertex to list
				existing_verts.push(v);

				// Add edges as needed
				for _ in (v_count - 1)..e_count
				{
					let v1 = existing_verts[g.gen_range(0, existing_verts.len())];
					let v2 = existing_verts[g.gen_range(0, existing_verts.len())];
					graph
						.add_edge_weighted(v1, v2, MockEdgeWeight::arbitrary(g))
						.unwrap();
				}

				graph
			},
		)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let graph = self.clone().release();

		// Shrink by removing any edge that isn't critical for connectedness
		graph.shrink_by_removing_edge(&limits, &mut result, is_weak);

		graph.shrink_by_replacing_vertex_with_edges(&limits, &mut result);

		graph.shrink_values(&limits, &mut result);

		Box::new(result.into_iter().map(|g| Self::guard_unchecked(g)))
	}
}

/// An arbitrary graph that is unconnected
#[derive(Clone, Debug)]
pub struct UnconnectedGraph<D: Directedness>(pub MockGraph<D>);

impl_ensurer! {
	use<D> UnconnectedGraph<D>
	as (self.0): MockGraph<D>
	where D: Directedness
}

impl<D: Directedness> GuidedArbGraph for UnconnectedGraph<D>
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		// Ensure we have at least 2 vertices, otherwise, its trivially connected
		assert!(v_max > 2);

		MockGraph::<D>::choose_size(g, std::cmp::max(2, v_min), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count > 1);

		// We merge 2 graphs into 1. This will ensure they are not connected.
		let v_count_1 = g.gen_range(1, v_count);
		let v_count_2 = v_count - v_count_1;

		let e_count_1 = if e_count > 0
		{
			g.gen_range(0, e_count)
		}
		else
		{
			0
		};
		let e_count_2 = e_count - e_count_1;

		let mut graph = MockGraph::arbitrary_fixed(g, v_count_1, e_count_1);
		let g2 = MockGraph::arbitrary_fixed(g, v_count_2, e_count_2);

		// Join the two graph into one graph with no edges between the two parts
		graph.join(&g2);

		assert!(!is_connected(&graph));
		Self(graph)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();

		// We shrink the MockGraph, keeping only the shrunk graphs that are still
		// unconnected
		result.extend(
			self.0
				.shrink_guided(limits)
				.filter(|g| !is_connected(&g))
				.map(|g| Self(g)),
		);

		Box::new(result.into_iter())
	}
}
