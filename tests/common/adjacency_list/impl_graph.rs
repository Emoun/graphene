//! Tests the `Graph` and `AutoGraph` implementations of `AdjListGraph`
//!

use crate::{
	common::adjacency_list::adj_list_from_mock,
	mock_graph::{
		arbitrary::{ArbEdgeIn, ArbVertexIn},
		utilities::unordered_equivalent_lists_equal,
		MockDirectedness, MockGraph,
	},
};
use duplicate::duplicate;
use graphene::core::{
	property::{HasVertex, RemoveEdge, RemoveVertex},
	Directed, Edge, EdgeWeighted, Graph, GraphMut, ReleaseUnloaded, Undirected,
};

/// Tests that adding vertices to the graph results in the same vertices being
/// output by `all_vertices_weighted`
#[quickcheck]
fn same_vertices(mock: MockGraph<MockDirectedness>) -> bool
{
	let (g, v_map) = adj_list_from_mock(&mock);

	unordered_equivalent_lists_equal(
		&mock.all_vertices().map(|v| v_map[&v]).collect(),
		&g.all_vertices().collect(),
	)
}

/// Tests that adding edges to the graph results in the same edges being output
/// by `all_edges`
#[quickcheck]
fn same_edges(mock: MockGraph<MockDirectedness>) -> bool
{
	let (g, v_map) = adj_list_from_mock(&mock);

	unordered_equivalent_lists_equal(
		&mock
			.all_edges()
			.map(|(so, si, w)| (v_map[&so], v_map[&si], w))
			.collect(),
		&g.all_edges().collect(),
	)
}

/// Tests that adding vertices to the graph results in the correct weights for
/// each.
#[quickcheck]
fn same_vertex_weight(mock: MockGraph<MockDirectedness>) -> bool
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
fn same_vertex_weight_mut(mock: ArbVertexIn<MockGraph<MockDirectedness>>) -> bool
{
	let v = mock.get_vertex();
	let (mut g, v_map) = adj_list_from_mock(&mock.release_all());

	g.vertex_weight(v_map[&v]).map(|w| w as *const _)
		== g.vertex_weight_mut(v_map[&v]).map(|w| w as *const _)
}

/// Tests that the reference to edge weights is the same regardless of
/// mutability
#[quickcheck]
fn same_edge_weight_mut(mut mock: MockGraph<MockDirectedness>) -> bool
{
	let (mut g, v_map) = adj_list_from_mock(&mock);

	unordered_equivalent_lists_equal(
		&mock
			.all_edges_mut()
			.map(|(so, si, w)| (v_map[&so], v_map[&si], w))
			.collect(),
		&g.all_edges_mut().collect(),
	)
}

#[duplicate(
	module 			directedness;
	[directed]		[Directed];
	[undirected]	[Undirected];
)]
mod module
{
	use super::*;

	/// Tests that removing a vertex works as expected
	#[quickcheck]
	fn remove_vertex(mock: ArbVertexIn<MockGraph<directedness>>) -> bool
	{
		let v_remove = mock.get_vertex();
		let mock = mock.release_all();
		let (mut g, v_map) = adj_list_from_mock(&mock);
		let v_removed = v_map[&v_remove];

		if g.remove_vertex(v_removed).is_err()
		{
			false
		}
		else
		{
			// Check that the number of vertices decreased by 1
			( g.all_vertices_weighted().count() ==
				(mock.all_vertices().count() - 1)
			) &&

			// Check that the number of edges decreased by same as the number that was incident
			// on the vertex
			( g.all_edges().count() ==
				(mock.all_edges().count() -
					mock.edges_incident_on(&v_remove).count())
			) &&

			// Check that one less vertex has the same weight as the one removed
			( g.all_vertex_weights()
				.filter(|&w| w == mock.vertex_weight(v_remove).unwrap()).count() ==
				(mock.all_vertex_weights()
					.filter(|&w| w == mock.vertex_weight(v_remove).unwrap()).count() - 1)
			)

			// TODO: Test that the right edges were removed?
		}
	}

	/// Tests removing an edge
	#[quickcheck]
	fn remove_edge(ArbEdgeIn(mock, edge): ArbEdgeIn<MockGraph<directedness>>) -> bool
	{
		let (mut g, v_map) = adj_list_from_mock(&mock);

		let mapped_edge = (
			v_map[&edge.source()],
			v_map[&edge.sink()],
			edge.weight_ref(),
		);

		if g.remove_edge_where(|e| e == mapped_edge).is_ok()
		{
			// Ensure that one less edge matches our edge
			g.edges_between(&mapped_edge.source(), &mapped_edge.sink())
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
