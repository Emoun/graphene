/// Common tests for all 'searching' algorithms.
use crate::mock_graph::{
	arbitrary::{ArbConnectedGraph, ArbVertexIn, ArbVerticesIn},
	MockEdgeWeight, MockGraph,
};
use duplicate::duplicate;
use graphene::{
	algo::{Bfs, DFS},
	core::{
		property::{AddEdge, NonNull},
		Directed, Graph, GraphDeref, Release, Undirected,
	},
};
use std::collections::HashSet;

#[duplicate(
	module			[ dfs ]				[ bfs ]
	search_algo_new	[ DFS::new_simple ]	[ Bfs::new ]
)]
mod module
{
	use super::*;
	use graphene::core::property::VertexInGraph;

	#[duplicate(
		module2			[ directed ] [ undirected ]
		directedness 	[ Directed ] [ Undirected ]
	)]
	mod module2
	{
		use super::*;

		/// Tests that all vertices in a connected component are produced
		/// exactly once.
		#[quickcheck]
		fn visits_component_once(mock: ArbVertexIn<ArbConnectedGraph<directedness>>) -> bool
		{
			// Use a set to ensure we only count each vertex once
			let mut visited = HashSet::new();
			let mut visited_once = true;
			search_algo_new(mock.graph()).for_each(|v| {
				visited_once &= visited.insert(v);
			});

			// We ensure all vertices were visited, but only once
			visited.len() == mock.all_vertices().count() && visited_once
		}

		/// Tests that no vertices outside a connected component are produced
		#[quickcheck]
		fn visits_none_outside_component(
			g1: ArbVertexIn<ArbConnectedGraph<directedness>>,
			g2: MockGraph<directedness>,
		) -> bool
		{
			// Our starting connected component
			let v = g1.get_vertex();
			let mut graph = g1.release_all();

			// First join the two graphs
			let v_map = graph.join(&g2);

			// Ensure that no visited vertex comes from outside the start component
			search_algo_new(&VertexInGraph::new_unvalidated(graph, v))
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
		ArbVerticesIn(comp, verts): ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
		ArbVerticesIn(g2, g2_verts): ArbVerticesIn<MockGraph<Directed>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		let v = comp.get_vertex();
		let mut graph = comp.release_all();

		// First join the two graphs
		let v_map = graph.join(&g2);

		// Add edges from the other component to the start component
		for (v1, v2) in verts.iter().zip(g2_verts.iter())
		{
			graph
				.add_edge_weighted((v_map[v2], *v1, weight.clone()))
				.unwrap();
		}

		// Ensure that no visited vertex comes from outside the start component
		search_algo_new(&VertexInGraph::new_unvalidated(graph, v))
			.all(|visit| v_map.values().all(|&new_v| visit != new_v))
	}

	/// Tests for directed graphs that any component with an edge to it from the
	/// start component is also visited in full.
	#[quickcheck]
	fn directed_visits_outgoing_component(
		ArbVerticesIn(comp1, verts1): ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
		ArbVerticesIn(comp2, verts2): ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		let v = comp1.get_vertex();
		let mut graph = comp1.release_all();

		let v2 = comp2.get_vertex();
		let g2 = comp2.release_all();

		// First join the two graphs
		let v_map = graph.join(&g2);

		// Add edges from start component to the other component
		graph
			.add_edge_weighted((v, v_map[&v2], weight.clone()))
			.unwrap();
		for (v1, v2) in verts1.iter().zip(verts2.iter())
		{
			graph
				.add_edge_weighted((*v1, v_map[v2], weight.clone()))
				.unwrap();
		}

		// Ensure that all vertices are visited
		let count = graph.all_vertices().count();
		search_algo_new(&VertexInGraph::new_unvalidated(graph, v)).count() == count
	}
}
