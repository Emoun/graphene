//! Tests the `core::Unique` trait and its ensurer `core::UniqueGraph`.
use crate::mock_graph::{
	arbitrary::{Arb, EdgeIn, NonUniqueGraph},
	MockEdgeWeight, MockGraph, MockVertexWeight,
};
use duplicate::duplicate;
use graphene::core::{
	property::{AddEdge, HasVertex, NewVertex, UniqueGraph, VertexInGraph},
	Directed, EnsureUnloaded, Graph, ReleaseUnloaded, Undirected,
};

#[duplicate(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;

	/// Tests that UniqueGraph correctly identifies unique graphs.
	#[quickcheck]
	fn accept_unique(g: Arb<UniqueGraph<MockGraph<directedness>>>) -> bool
	{
		UniqueGraph::validate(&g.0.release_all())
	}

	/// Tests that UniqueGraph correctly rejects non-unique graphs.
	#[quickcheck]
	fn reject_non_unique(g: Arb<NonUniqueGraph<directedness>>) -> bool
	{
		!UniqueGraph::validate(&g.0)
	}

	/// Tests that a UniqueGraph accepts adding a non-duplicate edge
	#[quickcheck]
	fn accept_add_edge(
		Arb(mut g): Arb<VertexInGraph<UniqueGraph<MockGraph<directedness>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let v = g.get_vertex().clone();
		// To ensure we add a non-duplicate edge,
		// we create a new vertex and add an edge to it from an existing one.
		let v2 = g.new_vertex_weighted(v_weight).unwrap();
		let accepted = g.add_edge_weighted(&v, &v2, e_weight).is_ok();
		accepted && g.edges_between(v, &v2).count() == 1
	}

	/// Tests that a UniqueGraph rejects adding a duplicate edge
	#[quickcheck]
	fn reject_add_edge(
		Arb(g): Arb<EdgeIn<UniqueGraph<MockGraph<directedness>>>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		let source = g.get_vertex();
		let EdgeIn(mut g, sink, _) = g;
		g.add_edge_weighted(source, sink, weight).is_err()
	}
}
