//!
//! Tests the `core::Connected` trait and its constrainer `core::ConnectedGraph`.
//!

use graphene::{
	core::{Constrainer, AddEdge, Edge, constraint::ConnectedGraph, Directedness },
};
use crate::mock_graph::{ MockEdgeWeight,
						 arbitrary::{ArbConnectedGraph, ArbUnconnectedGraph, ArbTwoVerticesIn, ArbVertexIn}
};

duplicate_for_directedness! {
	$directedness
	
	///
	/// Tests that Connected Graph correctly identifies connected graphs.
	///
	#[quickcheck]
	fn accept_connected(g: ArbConnectedGraph<directedness>) -> bool
	{
		ConnectedGraph::constrain_single(g.0.unconstrain()).is_ok()
	}

	///
	/// Tests that Connected Graph correctly rejects unconnected graphs.
	///
	#[quickcheck]
	fn reject_unconnected(g: ArbUnconnectedGraph<directedness>) -> bool
	{
		ConnectedGraph::constrain_single(g.0).is_err()
	}
	

	///
	/// Tests that a ConnectedGraph accepts removing an edge that isn't critical for connectedness
	///
	#[quickcheck]
	fn accept_remove_edge_where(ArbTwoVerticesIn(mut g,v1,v2):
		ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight)
		-> bool
	{
		// To ensure we can remove an edge, we first create an edge to remove
		g.0.add_edge_weighted((v1,v2, e_weight.clone())).unwrap();
		
		g.0.remove_edge_where(|e| (e.source() == v1 && e.sink() == v2)).is_ok()
	}
	
	///
	/// Tests that a ConnectedGraph rejects removing an edge that is critical for connectedness
	///
	#[quickcheck]
	fn reject_remove_edge_where(ArbVertexIn(g1,v1):
		ArbVertexIn<ArbConnectedGraph<directedness>>, ArbVertexIn(g2,v2):
		ArbVertexIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight)
		-> bool
	{
		let mut graph = g1.0.unconstrain();
		// We start by joining 2 connected graphs into a unconnected graph with the 2 components
		let v_map = graph.join(&g2.0);

		// We then connect the two components
		graph.add_edge_weighted((v1,v_map[&v2], e_weight.clone())).unwrap();
		if directedness::directed() {
			graph.add_edge_weighted((v_map[&v2],v1, e_weight.clone())).unwrap();
		}
		let mut connected = ConnectedGraph::constrain_single(graph).unwrap();

		// We now try to remove the the added edge
		connected.remove_edge_where(|e| (e.source() == v1 && e.sink() == v_map[&v2])).is_err()
	}
}
