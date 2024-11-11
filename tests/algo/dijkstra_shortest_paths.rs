use crate::mock_graph::arbitrary::{Arb, VerticesIn};
/// tests `DijkstraShortestPath`
use crate::mock_graph::{MockEdgeWeight, MockGraph};
use duplicate::duplicate_item;
use graphene::{
	algo::DijkstraShortestPaths,
	core::{
		property::{AddEdge, ConnectedGraph, HasVertex, VertexInGraph},
		proxy::EdgeWeightMap,
		Directed, Ensure, Graph, GraphDeref, Release, Undirected,
	},
};
use std::collections::{HashMap, HashSet};

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;

	/// Tests that all vertices in a connected component are produced
	/// exactly once.
	#[quickcheck]
	fn visits_component_once(
		Arb(mock): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
	) -> bool
	{
		// Use a set to ensure we only count each vertex once
		let mut visited = HashSet::new();
		visited.insert(mock.get_vertex().clone());
		let mut visited_once = true;
		let e_map = EdgeWeightMap::new(mock.graph(), |_, _, w| w.value);
		DijkstraShortestPaths::new(&e_map).for_each(|(_, v, _)| {
			visited_once &= visited.insert(v);
		});

		// We ensure all vertices were visited, but only once
		visited.len() == mock.all_vertices().count() && visited_once
	}

	/// Tests that no vertices outside a connected component are produced
	#[quickcheck]
	fn visits_none_outside_component(
		Arb(g1): Arb<VertexInGraph<ConnectedGraph<MockGraph<directedness>>>>,
		Arb(g2): Arb<MockGraph<directedness>>,
	) -> bool
	{
		// Our starting connected component
		let (mut graph, (v, _)) = g1.release_all();

		// First join the two graphs
		let v_map = graph.join(&g2);

		// Ensure that no visited vertex comes from outside the start component
		let e_map = EdgeWeightMap::new(VertexInGraph::ensure_unvalidated(graph, v), |_, _, w| {
			w.value
		});
		DijkstraShortestPaths::new(&e_map).all(|(_, v, _)| v_map.values().all(|&new_v| v != new_v))
	}

	/// Tests that the paths returned are always increasing.
	#[quickcheck]
	fn increasing_path_lengths(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
	{
		let mut path_weights = HashMap::new();
		path_weights.insert(g.get_vertex().clone(), 0);
		let mut len = 0;

		let e_map = EdgeWeightMap::new(g, |_, _, w| w.value);
		for (source, sink, w) in DijkstraShortestPaths::new(&e_map)
		{
			let len_to_source = path_weights[&source];
			let len_to_sink = len_to_source + w;
			if len_to_sink < len
			{
				return false;
			}
			path_weights.insert(sink, len_to_sink);
			len = len_to_sink;
		}
		true
	}

	/// Next path must be sourced in a previously produced vertex
	#[quickcheck]
	fn path_source_already_seen(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
	{
		let mut seen = HashSet::new();
		seen.insert(g.get_vertex().clone());

		let e_map = EdgeWeightMap::new(g, |_, _, w| w.value);
		for (source, sink, _) in DijkstraShortestPaths::new(&e_map)
		{
			if !seen.contains(&source)
			{
				return false;
			}
			seen.insert(sink);
		}
		true
	}
}

/// Tests for directed graphs, even if an edge targets a connected component
/// from a different component, only the first component's vertices are
/// produced.
///
/// This is different from `visits_none_outside_component` because in that
/// case the components are completely unconnected with no edges between
/// them (incoming or outgoing). This test therefore ensures edges aren't
/// taken the wrong directed.
#[quickcheck]
fn directed_doesnt_visit_incoming_component(
	Arb(VerticesIn(comp, verts)): Arb<
		VerticesIn<VertexInGraph<ConnectedGraph<MockGraph<Directed>>>>,
	>,
	Arb(VerticesIn(g2, g2_verts)): Arb<VerticesIn<MockGraph<Directed>>>,
	weight: MockEdgeWeight,
) -> bool
{
	let (mut graph, (v, _)) = comp.release_all();

	// First join the two graphs
	let v_map = graph.join(&g2);

	// Add edges from the other component to the start component
	for (v1, v2) in verts.iter().zip(g2_verts.iter())
	{
		graph
			.add_edge_weighted(&v_map[v2], v1, weight.clone())
			.unwrap();
	}

	let e_map = EdgeWeightMap::new(VertexInGraph::ensure_unvalidated(graph, v), |_, _, w| {
		w.value
	});
	// Ensure that no visited vertex comes from outside the start component
	DijkstraShortestPaths::new(&e_map).all(|(_, v, _)| v_map.values().all(|&new_v| v != new_v))
}

/// Tests for directed graphs that any component with an edge to it from the
/// start component is also visited in full.
#[quickcheck]
fn directed_visits_outgoing_component(
	Arb(VerticesIn(comp1, verts1)): Arb<
		VerticesIn<VertexInGraph<ConnectedGraph<MockGraph<Directed>>>>,
	>,
	Arb(VerticesIn(comp2, verts2)): Arb<
		VerticesIn<VertexInGraph<ConnectedGraph<MockGraph<Directed>>>>,
	>,
	weight: MockEdgeWeight,
) -> bool
{
	let (mut graph, (v, _)) = comp1.release_all();

	let (g2, (v2, _)) = comp2.release_all();

	// First join the two graphs
	let v_map = graph.join(&g2);

	// Add edges from start component to the other component
	graph
		.add_edge_weighted(&v, &v_map[&v2], weight.clone())
		.unwrap();
	for (v1, v2) in verts1.iter().zip(verts2.iter())
	{
		graph
			.add_edge_weighted(v1, &v_map[v2], weight.clone())
			.unwrap();
	}

	// Ensure that all vertices are visited
	let count = graph.all_vertices().count() - 1;
	let e_map = EdgeWeightMap::new(VertexInGraph::ensure_unvalidated(graph, v), |_, _, w| {
		w.value
	});
	DijkstraShortestPaths::new(&e_map).count() == count
}
