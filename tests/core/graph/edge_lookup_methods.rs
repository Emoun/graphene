//!
//! Tests the edge lookup methods of `graphene::core::Graph`
//!
use crate::mock_graph::{
	MockGraph, MockVertex, MockDirectedness,
	arbitrary::{
		ArbVertexIn, ArbVertexOutside,
		ArbTwoVerticesIn, ArbEdgeOutside
	},
	utilities::*,
};
use graphene::core::{
	Graph, Edge, Directed
};

///
/// Creates quickcheck tests for an edge lookup method and its `_mut` version.
///
/// Creates 4 tests:
/// 1. That any edge returned is correct.
/// 2. That all correct edges are returned.
/// 3. That no edges are returned in invalid vertices are given.
/// 4. That the mutable version of the method always returns the same edge set.
///
/// Arguments:
/// First it takes the name of the method to test and the name of its mutable version.
/// Then it takes a set of vertex names that are used as input to the methods.
/// Then it takes a closure that given an edge, checks whether that edge is a correct output
/// of the method (the names of the vertices given before can be used here).
///
///
macro_rules! edge_lookup_method_tests {
	{
		$func:ident // The name of the method to test
		$func_mut:ident	// The name of the mutable version of the method to test
		($($vertex_ids:ident),*) // The vertex identifiers used in the next closure
		{$($correct:tt)*} // checks whether an edge returned from the method is correct
		$arb_graph:ident // The arbitraty graph and vertex struct the test function take
		$arb_invalid_graph:ident // The arbitraty graph with invalid vertices to use in test
		$base_graph:ty
	} => {
		mod $func {
			use super::*;
			///
			/// Ensures that all the returned edges are correct.
			///
			#[quickcheck]
			fn returns_valid_edges($arb_graph(g, $($vertex_ids),*): $arb_graph<$base_graph>) -> bool
			{
				let edges_len = g.$func($($vertex_ids),*).count();
				
				let valid_edges_len = g.$func($($vertex_ids),*).filter($($correct)*).count();
				
				edges_len == valid_edges_len
			}
			
			///
			/// Ensures that all edges are returned.
			///
			#[quickcheck]
			fn returns_all_edges($arb_graph(g, $($vertex_ids),*): $arb_graph<$base_graph>) -> bool
			{
				let edges = g.$func($($vertex_ids),*).collect();
				let expected = g.all_edges().filter($($correct)*).collect();
				
				unordered_equivalent_lists_equal(&edges, &expected)
			}
			
			///
			/// Ensures that when the vertex is not in the graph, no edges are returned.
			///
			#[quickcheck]
			fn invalid_vertex($arb_invalid_graph(g, $($vertex_ids),*): $arb_invalid_graph<$base_graph>) -> bool
			{
				g.$func($($vertex_ids),*).next().is_none()
			}
			
			///
			/// Ensures that the mutable version returns the same edges as the original.
			///
			#[quickcheck]
			fn mut_equivalent(g: $base_graph, $($vertex_ids: MockVertex),*) -> bool
			{
				let mut clone = g.clone();
				let edges_mut = clone.$func_mut($($vertex_ids),*).collect();
				let edges= g.$func($($vertex_ids),*).collect();
				
				unordered_equivalent_lists(&edges, &edges_mut,
					_3_tuple_equality(), _3_tuple_equality())
			}
		}
	}
}

edge_lookup_method_tests!(
	edges_sourced_in
	edges_sourced_in_mut
	(v)
	{|e| e.source() == v }
	ArbVertexIn
	ArbVertexOutside
	MockGraph<Directed>
);
edge_lookup_method_tests!(
	edges_sinked_in
	edges_sinked_in_mut
	(v)
	{|e| e.sink() == v }
	ArbVertexIn
	ArbVertexOutside
	MockGraph<Directed>
);
edge_lookup_method_tests!(
	edges_incident_on
	edges_incident_on_mut
	(v)
	{|e| e.source() == v || e.sink() == v }
	ArbVertexIn
	ArbVertexOutside
	MockGraph<MockDirectedness>
);
edge_lookup_method_tests!(
	edges_between
	edges_between_mut
	(v1, v2)
	{|e| (e.source() == v1 && e.sink() == v2) || (e.source() == v2 && e.sink() == v1)}
	ArbTwoVerticesIn
	ArbEdgeOutside
	MockGraph<MockDirectedness>
);