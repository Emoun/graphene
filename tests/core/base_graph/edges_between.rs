//!
//! Tests the `edges_between` optional method for 'Graph`.
//!

use crate::mock_graphs::{MockGraph, MockVertex, utilities::*, ArbGraphAndTwoVertices};
use graphene::core::{
	Graph, Edge
};

///
/// Ensures that all the returned edges are incident on the given vertices.
///
#[quickcheck]
fn returns_valid_edges(ArbGraphAndTwoVertices(g, v1, v2): ArbGraphAndTwoVertices) -> bool
{
	let edges_between = g.edges_between::<Vec<_>>(v1,v2);
	let edges_between_len = edges_between.len();
	
	let valid_edges = edges_between.into_iter().filter(|e| {
		(e.source() == v1 && e.sink() == v2) ||
			(e.source() == v2 && e.sink() == v1)
	}).collect::<Vec<_>>();
	
	edges_between_len == valid_edges.len()
}

///
/// Ensures that all the edges between the two vertices are returned
///
#[quickcheck]
fn all_edges_returned(ArbGraphAndTwoVertices(g, v1, v2): ArbGraphAndTwoVertices)
	-> bool
{
	let edges_between = g.edges_between::<Vec<_>>(v1,v2);
	let expected = g.all_edges::<Vec<_>>().into_iter().filter(
		|&(so,si,_)| (so == v1 && si == v2) || (so == v2 && si == v1)
	).collect();
	
	unordered_sublist_equal(&edges_between, &expected) &&
		unordered_sublist_equal(&expected, &edges_between)
}















