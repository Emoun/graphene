//!
//! Tests the `core::Connected` trait and its constrainer `core::ConnectedGraph`.
//!


duplicate_for_directedness! {
	$directedness
	use graphene::{
		core::{ Constrainer, AddEdge, Edge, constraint::ConnectedGraph, Directedness, },
	};
	use crate::mock_graph::{ MockEdgeWeight,
		arbitrary::{ArbConnectedGraph, ArbUnconnectedGraph, ArbTwoVerticesIn}
	};

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
}
