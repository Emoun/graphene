//!
//! Tests `DFS`
//!

duplicate_for_directedness!{
	$directedness
	
	use crate::mock_graph::{
		MockGraph, MockVertex,
		arbitrary::{ ArbVertexIn, ArbConnectedGraph}
	};
	use graphene::core::{ImplGraph, Graph, Constrainer, AddVertex, AddEdge};
	use graphene::algo::DFS;
	use std::collections::{ HashSet, HashMap };
	
	///
	/// Tests that all vertices in a connected component are produced exactly once.
	///
	#[quickcheck]
	fn visits_component_once(ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>)
			 -> bool
	{
		// Use a set to ensure we only count each vertex once
		let mut visited = HashSet::new();
		let mut visited_once = true;
		DFS::new(mock.graph(), v).for_each(|v|{ visited_once &= visited.insert(v); });
		
		// We ensure all vertices were visited, but only once
		visited.len() == mock.0.all_vertices().count() && visited_once
	}
	
	///
	/// Tests that no vertices outside a connected component are produced
	///
	#[quickcheck]
	fn visits_none_outside_component(ArbVertexIn(g1, v): ArbVertexIn<ArbConnectedGraph<directedness>>,
		g2: MockGraph<directedness>)
		 -> bool
	{
		// Our starting connected component
		let mut graph = g1.0.unconstrain();
		
		// First join the two graphs
		let mut v_map: HashMap<MockVertex,MockVertex> = HashMap::new();
		for (v,w) in g2.all_vertices_weighted() {
			let new_v = graph.new_vertex_weighted(w.clone()).unwrap();
			v_map.insert(v, new_v);
		}
		for (so,si, w) in g2.all_edges() {
			graph.add_edge_weighted((v_map[&so], v_map[&si], w.clone())).unwrap();
		}
		
		// Ensure that no visited vertex comes from outside the start component
		DFS::new(&graph, v).all(|visit| v_map.values().all(|&new_v| visit != new_v))
	}
}


