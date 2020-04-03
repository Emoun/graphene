//! Tests the `core::Connected` trait and its ensurer `core::ConnectedGraph`.

use crate::mock_graph::{
	arbitrary::{
		ArbConnectedGraph, ArbTwoVerticesIn, ArbUnconnectedGraph, ArbVertexIn, ArbVerticesIn,
	},
	MockEdgeWeight, MockVertexWeight,
};
use duplicate::duplicate;
use graphene::core::{
	property::{AddEdge, ConnectedGraph, NewVertex, NonNull, RemoveEdge, RemoveVertex},
	Directed, Directedness, Edge, Ensure, Release, Undirected,
};

#[duplicate(
	module			[ directed ] [ undirected ]
	directedness 	[ Directed ] [ Undirected ]
)]
mod module
{
	use super::*;

	/// Tests that Connected Graph correctly identifies connected graphs.
	#[quickcheck]
	fn accept_connected(g: ArbConnectedGraph<directedness>) -> bool
	{
		ConnectedGraph::validate(&g.release_all())
	}

	/// Tests that Connected Graph correctly rejects unconnected graphs.
	#[quickcheck]
	fn reject_unconnected(g: ArbUnconnectedGraph<directedness>) -> bool
	{
		!ConnectedGraph::validate(&g.0)
	}

	/// Tests that a ConnectedGraph always accepts adding an edge.
	#[quickcheck]
	fn accept_add_edge_weighted(
		mut g: ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = g.get_both();
		g.add_edge_weighted((v1, v2, e_weight.clone())).is_ok()
	}

	/// Tests that a ConnectedGraph accepts removing an edge that isn't critical
	/// for connectedness
	#[quickcheck]
	fn accept_remove_edge_where(
		mut g: ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = g.get_both();
		// To ensure we can remove an edge, we first create an edge to remove
		g.add_edge_weighted((v1, v2, e_weight.clone())).unwrap();

		g.remove_edge_where(|e| (e.source() == v1 && e.sink() == v2))
			.is_ok()
	}

	/// Tests that a ConnectedGraph rejects removing an edge that is critical
	/// for connectedness
	#[quickcheck]
	fn reject_remove_edge_where(
		g1: ArbVertexIn<ArbConnectedGraph<directedness>>,
		g2: ArbVertexIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let v1 = g1.get_vertex();
		let v2 = g2.get_vertex();
		let mut graph = g1.release_all();
		// We start by joining 2 connected graphs into a unconnected graph with the 2
		// components
		let v_map = graph.join(&g2);

		// We then connect the two components
		graph
			.add_edge_weighted((v1, v_map[&v2], e_weight.clone()))
			.unwrap();
		if directedness::directed()
		{
			graph
				.add_edge_weighted((v_map[&v2], v1, e_weight.clone()))
				.unwrap();
		}
		let mut connected = ConnectedGraph::ensure(graph).unwrap();

		// We now try to remove the the added edge
		connected
			.remove_edge_where(|e| (e.source() == v1 && e.sink() == v_map[&v2]))
			.is_err()
	}

	/// Tests that a ConnectedGraph accepts removing a vertex if the remaining
	/// graph is still connected.
	#[quickcheck]
	fn accept_remove_vertex(
		ArbVerticesIn(graph, verts): ArbVerticesIn<
			ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = graph.get_both();
		let mut graph = ((graph).0).0.release_all();
		// It is only acceptable to remove a vertex (and any edge incident on it)
		// if after doing so, the rest of the vertices are still connected.

		// We take a connected graph and add new vertex to it.
		let v_new = graph.new_vertex_weighted(v_weight).unwrap();

		// We then connect it to the other vertices, making the whole graph connected
		// again
		graph
			.add_edge_weighted((v_new, v1, e_weight.clone()))
			.unwrap();
		if directedness::directed()
		{
			graph
				.add_edge_weighted((v2, v_new, e_weight.clone()))
				.unwrap();
		}

		// We add auxiliary edges from the new vertex to the others
		for (idx, v_other) in verts.into_iter().enumerate()
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
		ConnectedGraph::new(graph).remove_vertex(v_new).is_ok()
	}

	/// Tests that a ConnectedGraph rejects removing a vertex if it renders the
	/// graph unconnected
	#[quickcheck]
	fn reject_remove_vertex(
		g1: ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		g2: ArbTwoVerticesIn<ArbConnectedGraph<directedness>>,
		e_weight: MockEdgeWeight,
		v_weight: MockVertexWeight,
	) -> bool
	{
		let (v11, v12) = g1.get_both();
		let (v21, v22) = g2.get_both();
		let mut graph = g1.0.release_all();
		// We start by joining 2 connected graphs into a unconnected graph with the 2
		// components
		let v_map = graph.join(&g2.0);

		// We then connect the two components through a vertex
		let new_v = graph.new_vertex_weighted(v_weight.clone()).unwrap();
		graph
			.add_edge_weighted((v11, new_v, e_weight.clone()))
			.unwrap();
		graph
			.add_edge_weighted((new_v, v_map[&v21], e_weight.clone()))
			.unwrap();
		if directedness::directed()
		{
			let new_v = graph.new_vertex_weighted(v_weight.clone()).unwrap();
			graph
				.add_edge_weighted((v_map[&v22], new_v, e_weight.clone()))
				.unwrap();
			graph
				.add_edge_weighted((new_v, v12, e_weight.clone()))
				.unwrap();
		}
		let mut connected = ConnectedGraph::ensure(graph).unwrap();

		// We now try to remove the the added vertex
		connected.remove_vertex(new_v).is_err()
	}
}
