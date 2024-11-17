//! Tests the `core::Unique` trait and its ensurer `core::UniqueGraph`.
use crate::mock_graph::{
	arbitrary::{Arb, EdgeIn, NonUniqueGraph},
	MockEdgeWeight, MockGraph, MockVertexWeight,
};
use duplicate::duplicate_item;
use graphene::core::{
	property::{
		AddEdge, HasVertex, NewVertex, Simple, SimpleGraph, Unique, UniqueGraph, VertexInGraph,
	},
	Directed, Graph, Guard, Release, Undirected,
};
use static_assertions::assert_impl_all;

#[duplicate_item(
	test_graph directedness_to_test edge_type;
	[ UniqueGraph ] [[Directed ]; [ Undirected ]] [MockEdgeWeight];
	[ SimpleGraph ] [[ Undirected ]] [()]
)]
mod __
{
	use super::*;
	#[duplicate_item(
		directedness; directedness_to_test
	)]
	mod __
	{
		use super::*;

		/// Tests that test_graph correctly identifies its own graphs.
		#[quickcheck]
		fn accept_property(g: Arb<test_graph<MockGraph<directedness, edge_type>>>) -> bool
		{
			test_graph::can_guard(&g.0.release_all())
		}

		/// Tests that test_graph correctly rejects non-unique graphs.
		#[quickcheck]
		fn reject_non_unique(g: Arb<NonUniqueGraph<directedness, edge_type>>) -> bool
		{
			!test_graph::can_guard(&g.0)
		}

		/// Tests that a test_graph accepts adding a non-duplicate edge
		#[quickcheck]
		fn accept_add_edge(
			Arb(mut g): Arb<VertexInGraph<test_graph<MockGraph<directedness, edge_type>>>>,
			v_weight: MockVertexWeight,
			e_weight: edge_type,
		) -> bool
		{
			let v = g.get_vertex().clone();
			// To ensure we add a non-duplicate edge,
			// we create a new vertex and add an edge to it from an existing one.
			let v2 = g.new_vertex_weighted(v_weight).unwrap();
			let accepted = g.add_edge_weighted(&v, &v2, e_weight).is_ok();
			accepted && g.edges_between(v, &v2).count() == 1
		}

		/// Tests that a test_graph rejects adding a duplicate edge
		#[quickcheck]
		fn reject_add_edge(
			Arb(g): Arb<EdgeIn<test_graph<MockGraph<directedness, edge_type>>>>,
			weight: edge_type,
		) -> bool
		{
			let source = g.get_vertex();
			let EdgeIn(mut g, sink, _) = g;
			g.add_edge_weighted(source, sink, weight).is_err()
		}
	}
}

// Test that all simple graphs are also unique.
assert_impl_all!(SimpleGraph<MockGraph<Undirected, ()>>: Simple, Unique);
