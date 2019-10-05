//!
//! Tests the `core::Connected` trait and its constrainer `core::ConnectedGraph`.
//!

duplicate_for_directedness! {
	$directedness
	use graphene::core::{ Constrainer, constraint::ConnectedGraph};
	use crate::mock_graph::arbitrary::{ArbConnectedGraph, ArbUnconnectedGraph};
	
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
}