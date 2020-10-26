//! Tests the `core::Connected` trait and its ensurer `core::ConnectedGraph`.

use crate::mock_graph::{
	arbitrary::{Arb, TwoVerticesIn, UnconnectedGraph},
	MockDirectedness, MockEdgeWeight, MockGraph, MockVertexWeight,
};
use duplicate::duplicate;
use graphene::core::{
	property::{
		AddEdge, Connected, ConnectedGraph, HasVertex, NewVertex, RemoveEdge, RemoveVertex,
		Unilateral, UnilateralGraph, VertexInGraph, Weak, WeakGraph,
	},
	Directed, EnsureUnloaded, ReleaseUnloaded, Undirected,
};
use static_assertions::assert_impl_all;

#[duplicate(
	#[
		module_connected		directedness;
		[connected_directed]	[Directed];
		[connected_undirected]	[Undirected]
	][
		[
			module				[ module_connected ]
			connected_graph		[ ConnectedGraph ]
			arb_connected		[ ConnectedGraph<MockGraph<directedness>> ]
			arb_unconnected		[ UnconnectedGraph<directedness> ]
			arb_reject_remove	[ ConnectedGraph<MockGraph<directedness>> ]
		]
	]
	[
		module				[ unilateral ]
		connected_graph		[ UnilateralGraph ]
		arb_connected		[ UnilateralGraph<MockGraph<Directed>> ]
		arb_unconnected		[ UnconnectedGraph<Directed> ]
		arb_reject_remove	[ ConnectedGraph<MockGraph<Directed>> ]
	]
	[
		module				[ weak ]
		connected_graph		[ WeakGraph ]
		arb_connected		[ WeakGraph<MockGraph<Directed>> ]
		arb_unconnected		[ UnconnectedGraph<Directed> ]
		arb_reject_remove	[ WeakGraph<MockGraph<Directed>> ]
	]
)]
mod module
{
	use super::*;

	/// Tests that the graph correctly identifies graphs with its connectedness.
	#[quickcheck]
	fn accept_connected(Arb(g): Arb<arb_connected>) -> bool
	{
		connected_graph::validate(&g.release_all())
	}

	/// Tests that the graph correctly rejects graphs without its connectedness.
	#[quickcheck]
	fn reject_unconnected(Arb(g): Arb<arb_unconnected>) -> bool
	{
		!connected_graph::validate(&g.release_all())
	}

	/// Tests that a graph always accepts adding an edge.
	#[quickcheck]
	fn accept_add_edge_weighted(
		Arb(g): Arb<TwoVerticesIn<arb_connected>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = g.get_both();
		let mut g = connected_graph::ensure_unvalidated(g.release_all());

		g.add_edge_weighted(&v1, &v2, e_weight).is_ok()
	}

	/// Tests that a graph accepts removing an edge that isn't critical
	/// for connectedness
	#[quickcheck]
	fn accept_remove_edge_where_weight(
		Arb(g): Arb<TwoVerticesIn<arb_connected>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = g.get_both();
		let mut g = connected_graph::ensure_unvalidated(g.release_all());
		// To ensure we can remove an edge, we first create an edge to remove
		g.add_edge_weighted(&v1, &v2, e_weight.clone()).unwrap();

		g.remove_edge_where_weight(&v1, &v2, |w| *w == e_weight)
			.is_ok()
	}

	/// Tests that a graph rejects removing an edge that is critical
	/// for connectedness
	///
	/// TODO: Needs better graph generation for unilateral.
	/// TODO: Right now uses 2 connected graphs, should find a way to use
	/// TODO: unilateral graphs while still being able to find a critical edge.
	#[quickcheck]
	fn reject_remove_edge_where_weight(
		Arb(g1): Arb<VertexInGraph<arb_reject_remove>>,
		Arb(g2): Arb<VertexInGraph<arb_reject_remove>>,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let v1 = g1.get_vertex().clone();
		let v2 = g2.get_vertex().clone();
		let mut graph = g1.release_all();
		// We start by joining 2 connected graphs into a unconnected graph with the 2
		// components
		let v_map = graph.join(&g2);

		// We then connect the two components
		graph
			.add_edge_weighted(&v1, &v_map[&v2], e_weight.clone())
			.unwrap();
		if !connected_graph::validate(&graph)
		{
			graph
				.add_edge_weighted(&v_map[&v2], &v1, e_weight.clone())
				.unwrap();
		}
		let mut connected = connected_graph::ensure_unvalidated(graph);

		// We now try to remove the the added edge
		connected
			.remove_edge_where_weight(&v1, &v_map[&v2], |_| true)
			.is_err()
	}

	/// Tests that a graph accepts removing a vertex if the remaining
	/// graph is still connected.
	// #[quickcheck]
	// fn accept_remove_vertex(
	// 	ArbVerticesIn(graph, verts): ArbVerticesIn<ArbTwoVerticesIn<arb_connected>>,
	// 	v_weight: MockVertexWeight,
	// 	e_weight: MockEdgeWeight,
	// ) -> bool
	// {
	// 	let (v1, v2) = graph.get_both();
	// 	let mut graph = ((graph).0).0.release_all();
	//
	// 	// It is only acceptable to remove a vertex (and any edge incident on it)
	// 	// if after doing so, the rest of the vertices maintain connectedness
	//
	// 	// We take a connectedness graph and add new vertex to it.
	// 	let v_new = graph.new_vertex_weighted(v_weight).unwrap();
	//
	// 	// We then connect it to the other vertices,
	// 	// returning connectedness to the whole graph
	// 	graph
	// 		.add_edge_weighted(&v_new, &v1, e_weight.clone())
	// 		.unwrap();
	//
	// 	if <<arb_connected as Graph>::Directedness as Directedness>::directed()
	// 	{
	// 		graph
	// 			.add_edge_weighted(&v2, &v_new, e_weight.clone())
	// 			.unwrap();
	// 	}
	//
	// 	// We add auxiliary edges from the new vertex to the others
	// 	for (idx, v_other) in verts.into_iter().enumerate()
	// 	{
	// 		// just to add some variance
	// 		if idx % 2 == 0
	// 		{
	// 			graph
	// 				.add_edge_weighted(&v_other, &v_new, e_weight.clone())
	// 				.unwrap();
	// 		}
	// 		else
	// 		{
	// 			graph
	// 				.add_edge_weighted(&v_new, &v_other, e_weight.clone())
	// 				.unwrap();
	// 		}
	// 	}
	//
	// 	// We then try to remove the vertex again
	// 	connected_graph::ensure_unvalidated(graph)
	// 		.remove_vertex(&v_new)
	// 		.is_ok()
	// }

	/// Tests that a graph rejects removing a vertex if it renders the
	/// graph unconnected
	#[quickcheck]
	fn reject_remove_vertex(
		Arb(g1): Arb<TwoVerticesIn<arb_reject_remove>>,
		Arb(g2): Arb<TwoVerticesIn<arb_reject_remove>>,
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
			.add_edge_weighted(&v11, &new_v, e_weight.clone())
			.unwrap();
		graph
			.add_edge_weighted(&new_v, &v_map[&v21], e_weight.clone())
			.unwrap();
		if !connected_graph::validate(&graph)
		{
			let new_v = graph.new_vertex_weighted(v_weight.clone()).unwrap();
			graph
				.add_edge_weighted(&v_map[&v22], &new_v, e_weight.clone())
				.unwrap();
			graph
				.add_edge_weighted(&new_v, &v12, e_weight.clone())
				.unwrap();
		}
		let mut connected = connected_graph::ensure_unvalidated(graph);

		// We now try to remove the the added vertex
		connected.remove_vertex(&new_v).is_err()
	}
}

// Test that all Connected graphs are also unilateral and weak.
assert_impl_all!(ConnectedGraph<MockGraph<MockDirectedness>>: Connected, Unilateral, Weak);

// Test that all Unilateral graphs are also weak.
assert_impl_all!(UnilateralGraph<MockGraph<Directed>>: Unilateral, Weak);
