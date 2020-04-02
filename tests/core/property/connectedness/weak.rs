//! Tests the `core::Weak` trait and its ensurer `core::WeakGraph`.
//!

use crate::mock_graph::{
	arbitrary::{ArbTwoVerticesIn, ArbUnconnectedGraph, ArbVertexIn, ArbVerticesIn, ArbWeakGraph},
	MockEdgeWeight, MockVertexWeight,
};
use graphene::core::{
	property::{AddEdge, NewVertex, NonNull, RemoveEdge, RemoveVertex, WeakGraph},
	Directed, Edge, Ensure, Release,
};

/// Tests that WeakGraph correctly identifies weak graphs.
#[quickcheck]
fn accept_weak(g: ArbWeakGraph) -> bool
{
	WeakGraph::validate(&g.0.release_all())
}

/// Tests that WeakGraph correctly rejects non-weak graphs.
#[quickcheck]
fn reject_weak(g: ArbUnconnectedGraph<Directed>) -> bool
{
	!WeakGraph::validate(&g.0)
}

/// Tests that WeakGraph always accepts adding an edge
#[quickcheck]
fn accept_add_edge_weighted(mut g: ArbTwoVerticesIn<ArbWeakGraph>, e_weight: MockEdgeWeight)
	-> bool
{
	let (v1, v2) = g.get_both();
	g.add_edge_weighted((v1, v2, e_weight)).is_ok()
}

/// Tests that a WeakGraph accepts removing an edge that isn't critical for weak
/// connectivity.
#[quickcheck]
fn accept_remove_edge_where(mut g: ArbTwoVerticesIn<ArbWeakGraph>, e_weight: MockEdgeWeight)
	-> bool
{
	let (v1, v2) = g.get_both();
	// To ensure we can remove an edge, we first create an edge to remove
	g.add_edge_weighted((v1, v2, e_weight.clone())).unwrap();

	g.remove_edge_where(|e| (e.source() == v1 && e.sink() == v2))
		.is_ok()
}

/// Tests that a WeakGraph rejects removing an edge that is critical for weak
/// connectivity.
#[quickcheck]
fn reject_remove_edge_where(
	g1: ArbVertexIn<ArbWeakGraph>,
	g2: ArbVertexIn<ArbWeakGraph>,
	e_weight: MockEdgeWeight,
) -> bool
{
	let v1 = g1.get_vertex();
	let v2 = g2.get_vertex();
	let mut graph = g1.release_all();
	// We start by joining 2 connected graphs into a unconnected graph with the 2
	// components
	let v_map = graph.join(&g2);

	// We then connect the two components with 1 edge, making in unilateral.
	graph
		.add_edge_weighted((v1, v_map[&v2], e_weight.clone()))
		.unwrap();

	let mut weak = WeakGraph::ensure(graph).unwrap();

	// We now try to remove the the added edge
	weak.remove_edge_where(|e| (e.source() == v1 && e.sink() == v_map[&v2]))
		.is_err()
}

/// Tests that a UnilateralGraph accepts removing a vertex if the remaining
/// graph is still unilateral.
#[quickcheck]
fn accept_remove_vertex(
	mock: ArbVerticesIn<ArbTwoVerticesIn<ArbWeakGraph>>,
	v_weight: MockVertexWeight,
	e_weight: MockEdgeWeight,
	e_direction: bool,
) -> bool
{
	let v_set = mock.1;
	let v1 = mock.0.get_vertex();
	let mut graph = ((mock.0).0).0.release_all();
	// It is only acceptable to remove a vertex (and any edge incident on it)
	// if after doing so, the rest of the vertices are still weakly connected.

	// We take a weak graph and add new vertex to it.
	let v_new = graph.new_vertex_weighted(v_weight).unwrap();

	// We then connect it to the other vertices, making the whole graph weak again
	if e_direction
	{
		graph
			.add_edge_weighted((v_new, v1, e_weight.clone()))
			.unwrap();
	}
	else
	{
		graph
			.add_edge_weighted((v1, v_new, e_weight.clone()))
			.unwrap();
	}

	// We add auxiliary edges from the new vertex to the others
	for (idx, v_other) in v_set.into_iter().enumerate()
	{
		// just to add some variance
		if idx % 2 == 0
		{
			graph
				.add_edge_weighted((v_other, v_new, e_weight.clone()))
				.unwrap();
		}
		else
		{
			graph
				.add_edge_weighted((v_new, v_other, e_weight.clone()))
				.unwrap();
		}
	}

	// We then try to remove the vertex again
	WeakGraph::new(graph).remove_vertex(v_new).is_ok()
}

/// Tests that a WeakGraph rejects removing a vertex if it renders the graph
/// unconnected
#[quickcheck]
fn reject_remove_vertex(
	g1: ArbVertexIn<ArbWeakGraph>,
	g2: ArbVertexIn<ArbWeakGraph>,
	e_weight: MockEdgeWeight,
	v_weight: MockVertexWeight,
) -> bool
{
	let v1 = g1.get_vertex();
	let v2 = g2.get_vertex();
	let mut graph = g1.release_all();
	// We start by joining 2 connected graphs into a unconnected graph with the 2
	// components
	let v_map = graph.join(&g2);

	// We then connect the two components through a vertex, making it weakly
	// connected
	let new_v = graph.new_vertex_weighted(v_weight.clone()).unwrap();
	graph
		.add_edge_weighted((v1, new_v, e_weight.clone()))
		.unwrap();
	graph
		.add_edge_weighted((new_v, v_map[&v2], e_weight.clone()))
		.unwrap();

	let mut weak = WeakGraph::ensure(graph).unwrap();

	// We now try to remove the the added vertex
	weak.remove_vertex(new_v).is_err()
}
