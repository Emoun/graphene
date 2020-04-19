//! Tests the `core::Unique` trait and its ensurer `core::UniqueGraph`.
//!
use crate::mock_graph::{
	arbitrary::{ArbEdgeIn, ArbNonUniqueGraph, ArbUniqueGraph, ArbVertexIn},
	MockEdgeWeight, MockVertexWeight,
};
use duplicate::duplicate;
use graphene::core::{
	property::{AddEdge, HasVertex, NewVertex, UniqueGraph},
	Directed, Edge, EnsureUnloaded, Graph, ReleaseUnloaded, Undirected,
};

#[duplicate(
	module			[ directed ] [ undirected ]
	directedness 	[ Directed ] [ Undirected ]
)]
mod module
{
	use super::*;

	/// Tests that UniqueGraph correctly identifies unique graphs.
	#[quickcheck]
	fn accept_unique(g: ArbUniqueGraph<directedness>) -> bool
	{
		UniqueGraph::validate(&g.0.release_all())
	}

	/// Tests that UniqueGraph correctly rejects non-unique graphs.
	#[quickcheck]
	fn reject_non_unique(g: ArbNonUniqueGraph<directedness>) -> bool
	{
		!UniqueGraph::validate(&g.0)
	}

	/// Tests that a UniqueGraph accepts adding a non-duplicate edge
	#[quickcheck]
	fn accept_add_edge(
		mut g: ArbVertexIn<ArbUniqueGraph<directedness>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let v = g.get_vertex();
		// To ensure we add a non-duplicate edge,
		// we create a new vertex and add an edge to it from an existing one.
		let v2 = g.new_vertex_weighted(v_weight).unwrap();
		let accepted = g.add_edge_weighted((v, v2, e_weight)).is_ok();
		accepted && g.edges_between(v, v2).count() == 1
	}

	/// Tests that a UniqueGraph rejects adding a duplicate edge
	#[quickcheck]
	fn reject_add_edge(
		ArbEdgeIn(mut g, e): ArbEdgeIn<ArbUniqueGraph<directedness>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		g.add_edge_weighted((e.source(), e.sink(), weight)).is_err()
	}
}