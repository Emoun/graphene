use crate::mock_graph::arbitrary::Arb;
/// Common tests for all 'searching' algorithms.
use crate::mock_graph::{MockEdgeWeight, MockGraph};
use duplicate::duplicate_item;
use graphene::{
	algo::{Bfs, Dfs, Spfs},
	core::{
		property::{AddEdge, ConnectedGraph, HasVertex, VertexInGraph},
		proxy::EdgeWeightMap,
		Directed, Ensure, Graph, GraphDeref, ReleasePayload, Undirected,
	},
};
use std::collections::HashSet;

#[duplicate_item(
	module		search_algo_new(graph);
	[ dfs ]		[ Dfs::new_simple(graph) ];
	[ bfs ]		[ Bfs::new(graph) ];
	[ spfs ]	[
		let ref_graph = graph;
		let map_graph = EdgeWeightMap::new(ref_graph, |_,_,w| w.value);
		Spfs::new(&map_graph)
	]
)]
mod module
{
	use super::*;
	use crate::mock_graph::arbitrary::VerticesIn;

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

			// Add the starting vertex to ensure it is not produced.
			visited.insert(mock.get_vertex().clone());

			let mut visited_once = true;
			search_algo_new([mock.graph()]).for_each(|v| {
				// Track whether we have seen the vertex before
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
			search_algo_new([&VertexInGraph::ensure_unchecked(graph, v)])
				.all(|visit| v_map.values().all(|&new_v| visit != new_v))
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

		// Ensure that no visited vertex comes from outside the start component
		search_algo_new([&VertexInGraph::ensure_unchecked(graph, v)])
			.all(|visit| v_map.values().all(|&new_v| visit != new_v))
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
			.add_edge_weighted(&v[0], &v_map[&v2[0]], weight.clone())
			.unwrap();
		for (v1, v2) in verts1.iter().zip(verts2.iter())
		{
			graph
				.add_edge_weighted(v1, v_map[v2], weight.clone())
				.unwrap();
		}

		// Ensure that all vertices are visited except the start
		let count = graph.all_vertices().count() - 1;
		search_algo_new([&VertexInGraph::ensure_unchecked(graph, v)]).count() == count
	}
}
