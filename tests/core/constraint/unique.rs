//!
//! Tests the `core::Unique` trait and its constrainer `core::UniqueGraph`.
//!

use graphene::core::{Constrainer, Graph, Edge, NewVertex, AddEdge, constraint::UniqueGraph};
use crate::mock_graph::{
	MockEdgeWeight, MockVertexWeight,
	arbitrary::{
		ArbUniqueGraph, ArbNonUniqueGraph, ArbEdgeIn, ArbVertexIn
	}
};

duplicate_for_directedness!{
	$directedness
	
	///
	/// Tests that UniqueGraph correctly identifies unique graphs.
	///
	#[quickcheck]
	fn accept_unique(g: ArbUniqueGraph<directedness>) -> bool
	{
		UniqueGraph::constrain_single(g.0.unconstrain()).is_ok()
	}
	
	///
	/// Tests that UniqueGraph correctly rejects non-unique graphs.
	///
	#[quickcheck]
	fn reject_non_unique(g: ArbNonUniqueGraph<directedness>) -> bool
	{
		UniqueGraph::constrain_single(g.0).is_err()
	}
	
	///
	/// Tests that a UniqueGraph accepts adding a non-duplicate edge
	///
	#[quickcheck]
	fn accept_add_edge(ArbVertexIn(mut g,v): ArbVertexIn<ArbUniqueGraph<directedness>>,
		v_weight: MockVertexWeight, e_weight: MockEdgeWeight)
		-> bool
	{
		// To ensure we add a non-duplicate edge,
		// we create a new vertex and add an edge to it from an existing one.
		let v2 = g.0.new_vertex_weighted(v_weight).unwrap();
		let accepted = g.0.add_edge_weighted((v, v2, e_weight)).is_ok();
			accepted && g.0.edges_between(v,v2).count() == 1
	}
	
	///
	/// Tests that a UniqueGraph rejects adding a duplicate edge
	///
	#[quickcheck]
	fn reject_add_edge(ArbEdgeIn(mut g,e): ArbEdgeIn<ArbUniqueGraph<directedness>>,
								weight: MockEdgeWeight) -> bool
	{
		g.0.add_edge_weighted((e.source(), e.sink(), weight)).is_err()
	}
}
