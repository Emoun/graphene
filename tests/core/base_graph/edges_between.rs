//!
//! Tests the `edges_between` optional method for `BaseGraph`.
//!

use mock_graphs::{
	MockBaseGraph, MockVertex,
	utilities::*
};
use graphene::core::{
	BaseGraph, Edge
};

///
/// Ensures that all the returned edges are incident on the given vertices.
///
fn all_edges_incident_on_the_vertices(
	g: MockBaseGraph,
	v1_cand: MockVertex, v2_cand: MockVertex)
	-> bool
{
	if g.values.len() == 0 {
		// If the graph has no vertices,
		// then there can be no edges between the two given vertices,
		// since they are not part of the graph.
		return g.edges_between(v1_cand,v2_cand).len() == 0;
	}
	
	let v1 = appropriate_vertex_value_from_index(&g, v1_cand.value as usize);
	let v2 = appropriate_vertex_value_from_index(&g, v2_cand.value as usize);
	
	let edges_between = g.edges_between(v1,v2);
	let edges_between_len = edges_between.len();
	
	let valid_edges = edges_between.into_iter().filter(|e| {
		(*e.source() == v1 && *e.sink() == v2) ||
			(*e.source() == v2 && *e.sink() == v1)
	}).collect::<Vec<_>>();
	
	edges_between_len == valid_edges.len()
}

///
/// Ensures that all the edges between the two vertices are returned
///
fn all_edges_returned(
	g: MockBaseGraph,
	v1_cand: MockVertex, v2_cand: MockVertex)
	-> bool
{
	if g.values.len() == 0 {
		// If the graph has no vertices,
		// then there can be no edges between the two given vertices,
		// since they are not part of the graph.
		return g.edges_between(v1_cand,v2_cand).len() == 0;
	}
	
	let v1 = appropriate_vertex_value_from_index(&g, v1_cand.value as usize);
	let v2 = appropriate_vertex_value_from_index(&g, v2_cand.value as usize);
	
	let edges_between = g.edges_between(v1,v2);
	let expected = g.all_edges().into_iter().filter(
		|&(so,si,_)| (so == v1 && si == v2) || (so == v2 && si == v1)
	).collect();
	
	unordered_sublist_equal(&edges_between, &expected) &&
		unordered_sublist_equal(&expected, &edges_between)
}

quickcheck!{
	fn PROP_all_edges_incident_on_the_vertices(
		g: MockBaseGraph,
		v1_cand: MockVertex, v2_cand: MockVertex)
		-> bool
	{
		all_edges_incident_on_the_vertices(g,v1_cand, v2_cand)
	}
	
	fn PROP_all_edges_returned(
		g: MockBaseGraph,
		v1_cand: MockVertex, v2_cand: MockVertex)
		-> bool
	{
		all_edges_returned(g,v1_cand, v2_cand)
	}
}














