//!
//! Tests the `edges_sourced_in` and `edges_sinked_in` optional methods for `BaseGraph`.
//!
use crate::mock_graphs::{
	MockGraph, MockVertex,
	utilities::*
};
use graphene::core::{
	Graph, Edge
};

///
/// Tests the `edges_sourced_in` optional method for `BaseGraph`.
///
/// Ensures that all the returned edges are sourced in the given vertex.
///
#[quickcheck]
fn all_edges_sourced_in_the_vertice(
	g: MockGraph,
	v_cand: MockVertex)
	-> bool
{
	if g.vertices.len() == 0 {
		// If the graph has no vertices,
		// then there can be no edges sourced in the given vertex,
		// as it is not part of the graph.
		g.edges_sourced_in::<Vec<_>>(v_cand).len() == 0
	} else {
		let v = appropriate_vertex_value_from_index(&g, v_cand.value as usize);
		
		let sourced_edges = g.edges_sourced_in::<Vec<_>>(v);
		let sourced_edges_len = sourced_edges.len();
		
		let valid_edges = sourced_edges.into_iter().filter(|e|{
			e.source() == v
		}).collect::<Vec<_>>();
		
		sourced_edges_len == valid_edges.len()
	}
}

///
/// Tests that the `edges_sinked_in` optional method for `BaseGraph`.
///
/// Ensured that all the returned edges are sinked in the given vertex.
///
#[quickcheck]
fn all_edges_sinked_in_the_vertice(
	g: MockGraph,
	v_cand: MockVertex)
	-> bool
{
	if g.all_vertices::<Vec<_>>().len() == 0 {
		// If the graph has no vertices,
		// then there can be no edges sourced in the given vertex,
		// as it is not part of the graph.
		g.edges_sinked_in::<Vec<_>>(v_cand).len() == 0
	} else {
		let v = appropriate_vertex_value_from_index(&g, v_cand.value as usize);
		
		let sinked_edges = g.edges_sinked_in::<Vec<_>>(v);
		let sinked_edges_len = sinked_edges.len();
		
		let valid_edges = sinked_edges.into_iter().filter(|e|{
			e.sink() == v
		}).collect::<Vec<_>>();
		
		sinked_edges_len == valid_edges.len()
	}
}
