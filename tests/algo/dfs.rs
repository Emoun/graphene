//!
//! Tests `DFS`
//!

use crate::mock_graph::{MockGraph, MockVertex, arbitrary::{ArbVertexIn, ArbConnectedGraph}, MockEdgeWeight};
use graphene::core::{ImplGraph, Graph, Constrainer, AddVertex, AddEdge, Directed};
use graphene::algo::DFS;
use std::collections::{ HashSet, HashMap };
use crate::mock_graph::arbitrary::ArbVerticesIn;

duplicate_for_directedness!{
	$directedness
	
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
		let mut v_map = graph.join(&g2);
		
		// Ensure that no visited vertex comes from outside the start component
		DFS::new(&graph, v).all(|visit| v_map.values().all(|&new_v| visit != new_v))
	}
}

///
/// Tests for directed graphs, even if an edge targets a connected component from a different
/// component, only the first component's vertices are produced.
///
/// This is different from `visits_none_outside_component` because in that case the components
/// are completely unconnected with no edges between them (incomming or outgoing).
/// This test therefore ensures edges aren't taken the wrong directed.
///
#[quickcheck]
fn directed_doesnt_visit_incomming_component(
	component: ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
	rest: ArbVerticesIn<MockGraph<Directed>>, weight: MockEdgeWeight)
	-> bool
{
	let mut graph = ((component.0).0).0.unconstrain();
	let comp_set = component.1;
	let v = (component.0).1;
	let g2 = rest.0;
	let g2_set = rest.1;

	// First join the two graphs
	let mut v_map = graph.join(&g2);

	// Add edges from other components to the start component
	for (v1,v2) in comp_set.iter().zip(g2_set.iter()) {
		graph.add_edge_weighted((v_map[v2], *v1, weight.clone())).unwrap();
	}

	// Ensure that no visited vertex comes from outside the start component
	DFS::new(&graph, v).all(|visit| v_map.values().all(|&new_v| visit != new_v))
}
