//! Tests the `Graph` and `AutoGraph` implementations of `AdjListGraph`
//!

use crate::{
	common::adjacency_list::adj_list_from_mock,
	mock_graph::{
		arbitrary::{ArbEdgeIn, ArbTwoVerticesIn, ArbVertexIn},
		utilities::unordered_equivalent_lists_equal,
		MockGraph,
	},
};
use duplicate::duplicate;
use graphene::core::{
	property::{EdgeCount, HasVertex, RemoveEdge, RemoveVertex, VertexCount},
	Directed, Edge, Graph, GraphMut, ReleaseUnloaded, Undirected,
};

#[duplicate(
	directedness; [Directed]; [Undirected];
)]
mod __
{
	use super::*;

	/// Tests that adding vertices to the graph results in the same vertices
	/// being output by `all_vertices_weighted`
	#[quickcheck]
	fn same_vertices(mock: MockGraph<directedness>) -> bool
	{
		let (g, v_map) = adj_list_from_mock(&mock);

		unordered_equivalent_lists_equal(
			&mock.all_vertices().map(|v| v_map[&v]).collect(),
			&g.all_vertices().collect(),
		)
	}

	/// Tests that adding vertices to the graph results in the correct weights
	/// for each.
	#[quickcheck]
	fn same_vertex_weight(mock: MockGraph<directedness>) -> bool
	{
		let (g, v_map) = adj_list_from_mock(&mock);

		unordered_equivalent_lists_equal(
			&mock
				.all_vertices()
				.map(|v| (v_map[&v], mock.vertex_weight(v)))
				.collect(),
			&g.all_vertices().map(|v| (v, g.vertex_weight(v))).collect(),
		)
	}

	/// Tests that the reference to vertex weights is the same regardless of
	/// mutability
	#[quickcheck]
	fn same_vertex_weight_mut(mock: ArbVertexIn<MockGraph<directedness>>) -> bool
	{
		let v = mock.get_vertex().clone();
		let (mut g, v_map) = adj_list_from_mock(&mock.release_all());

		g.vertex_weight(v_map[&v]).map(|w| w as *const _)
			== g.vertex_weight_mut(v_map[&v]).map(|w| w as *const _)
	}

	/// Tests that when we create an AdjListGraph from a MockGraph,
	/// any edge in the mock is in the AdjListGraph
	#[quickcheck]
	fn edges_between(mock: ArbTwoVerticesIn<MockGraph<directedness>>) -> bool
	{
		let (v1, v2) = mock.get_both();
		let mock = mock.0.release_all();
		let (g, v_map) = adj_list_from_mock(&mock);

		unordered_equivalent_lists_equal(
			&mock.edges_between(&v1, &v2).collect(),
			&g.edges_between(&v_map[&v1], &v_map[&v2]).collect(),
		)
	}

	/// Tests that `edges_between_mut` returns the same edges as its immutable
	/// version
	#[quickcheck]
	fn edges_between_mut(mock: ArbTwoVerticesIn<MockGraph<directedness>>) -> bool
	{
		let (v1, v2) = mock.get_both();
		let mock = mock.0.release_all();
		let (mut g, v_map) = adj_list_from_mock(&mock);

		// we check that we can mutate
		if let Some(e_weight) = g.edges_between_mut(&v_map[&v1], &v_map[&v2]).next()
		{
			*e_weight = e_weight.clone();
		}

		unordered_equivalent_lists_equal(
			&g.edges_between(&v_map[&v1], &v_map[&v2])
				.map(|w| w.clone())
				.collect(),
			&g.edges_between_mut(&v_map[&v1], &v_map[&v2])
				.map(|w| w.clone())
				.collect(),
		)
	}

	/// Tests that removing a vertex works as expected
	#[quickcheck]
	fn remove_vertex(mock: ArbVertexIn<MockGraph<directedness>>) -> bool
	{
		let v_remove = mock.get_vertex().clone();
		let mock = mock.release_all();
		let (mut g, v_map) = adj_list_from_mock(&mock);
		let v_removed = v_map[&v_remove];

		if g.remove_vertex(v_removed).is_err()
		{
			false
		}
		else
		{
			let removed_weight = mock.vertex_weight(v_remove).unwrap();

			let expected_vertex_count = mock.vertex_count() - 1;
			let expected_edge_count = mock.edge_count() - mock.edges_incident_on(v_remove).count();

			let expected_vertices_with_removed_weight = mock
				.all_vertex_weights()
				.filter(|w| *w == removed_weight)
				.count() - 1;
			let actual_vertices_with_removed_weight = g
				.all_vertex_weights()
				.filter(|w| *w == removed_weight)
				.count();

			g.vertex_count() == expected_vertex_count
				&& g.edge_count() == expected_edge_count
				&& actual_vertices_with_removed_weight == expected_vertices_with_removed_weight

			// TODO: Test that the right edges were removed?
		}
	}

	/// Tests removing an edge
	#[quickcheck]
	fn remove_edge(ArbEdgeIn(mock, edge): ArbEdgeIn<MockGraph<directedness>>) -> bool
	{
		let (mut g, v_map) = adj_list_from_mock(&mock);

		let mapped_source = v_map[&edge.source()];
		let mapped_sink = v_map[&edge.sink()];

		if g.remove_edge_where_weight((mapped_source, mapped_sink), |w| *w == edge.2)
			.is_ok()
		{
			// Ensure that one less edge matches our edge
			g.edges_between(&mapped_source, &mapped_sink)
				.filter(|&w| *w == edge.2)
				.count() == (mock
				.edges_between(&edge.source(), &edge.sink())
				.filter(|&w| *w == edge.2)
				.count() - 1)
		}
		else
		{
			false
		}
	}
}
