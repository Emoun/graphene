//! Tests the `Graph` and `AutoGraph` implementations of `AdjListGraph`

use crate::{
	common::{adj_list_from_mock, vertex_map_from_mock},
	mock_graph::{
		arbitrary::{Arb, EdgeIn},
		utilities::unordered_equivalent_lists_equal,
		MockGraph,
	},
};
use duplicate::duplicate_item;
use graphene::{
	common::AdjListGraph,
	core::{
		property::{
			EdgeCount, NewVertex, RemoveEdge, RemoveVertex, VertexCount, VertexIn, VertexInGraph,
		},
		Directed, Graph, GraphMut, Release, Undirected,
	},
};
use quickcheck::TestResult;

#[duplicate_item(
	directedness; [Directed]; [Undirected];
)]
mod __
{
	use super::*;

	#[duplicate_item(
		graph_impl 			from_mock;
		[AdjListGraph] 		[adj_list_from_mock];
		[VertexMapGraph] 	[vertex_map_from_mock];
	)]
	mod __
	{
		use super::*;

		/// Tests that adding vertices to the graph results in the same vertices
		/// being output by `all_vertices_weighted`
		#[quickcheck]
		fn same_vertices(Arb(mock): Arb<MockGraph<directedness>>) -> bool
		{
			let (g, v_map) = from_mock(&mock);

			unordered_equivalent_lists_equal(
				&mock.all_vertices().map(|v| v_map[&v]).collect(),
				&g.all_vertices().collect(),
			)
		}

		/// Tests that adding vertices to the graph results in the correct
		/// weights for each.
		#[quickcheck]
		fn same_vertex_weight(Arb(mock): Arb<MockGraph<directedness>>) -> bool
		{
			let (g, v_map) = from_mock(&mock);

			unordered_equivalent_lists_equal(
				&mock
					.all_vertices()
					.map(|v| (v_map[&v], mock.vertex_weight(&v)))
					.collect(),
				&g.all_vertices().map(|v| (v, g.vertex_weight(&v))).collect(),
			)
		}

		/// Tests that the reference to vertex weights is the same regardless of
		/// mutability
		#[quickcheck]
		fn same_vertex_weight_mut(Arb(mock): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			let v = mock.vertex_at::<0>().clone();
			let (mut g, v_map) = from_mock(&mock.release_all());

			g.vertex_weight(&v_map[&v]).map(|w| w as *const _)
				== g.vertex_weight_mut(v_map[&v]).map(|w| w as *const _)
		}

		/// Tests that when we create an AdjListGraph from a MockGraph,
		/// any edge in the mock is in the AdjListGraph
		#[quickcheck]
		fn edges_between(Arb(mock): Arb<VertexInGraph<MockGraph<directedness>, 2, false>>) -> bool
		{
			let v1 = mock.vertex_at::<0>();
			let v2 = mock.vertex_at::<1>();
			let mock = mock.0.release_all();
			let (g, v_map) = from_mock(&mock);

			unordered_equivalent_lists_equal(
				&mock.edges_between(&v1, &v2).collect(),
				&g.edges_between(&v_map[&v1], &v_map[&v2]).collect(),
			)
		}

		/// Tests that `edges_between_mut` returns the same edges as its
		/// immutable version
		#[quickcheck]
		fn edges_between_mut(
			Arb(mock): Arb<VertexInGraph<MockGraph<directedness>, 2, false>>,
		) -> bool
		{
			let v1 = mock.vertex_at::<0>();
			let v2 = mock.vertex_at::<1>();
			let mock = mock.0.release_all();
			let (mut g, v_map) = from_mock(&mock);

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

		/// Tests removing an edge
		#[quickcheck]
		fn remove_edge(Arb(mock): Arb<EdgeIn<MockGraph<directedness>>>) -> bool
		{
			let (mut g, v_map) = from_mock(&mock);
			let source = mock.vertex_at::<0>();
			let EdgeIn(mock, sink, weight) = mock;

			let mapped_source = v_map[&source];
			let mapped_sink = v_map[&sink];

			if g.remove_edge_where_weight(&mapped_source, &mapped_sink, |w| *w == weight)
				.is_ok()
			{
				// Ensure that one less edge matches our edge
				g.edges_between(&mapped_source, &mapped_sink)
					.filter(|&w| *w == weight)
					.count() == (mock
					.edges_between(source, sink)
					.filter(|&w| *w == weight)
					.count() - 1)
			}
			else
			{
				false
			}
		}
	}

	/// Tests that removing a vertex works as expected for AdjListGraph
	#[quickcheck]
	fn adj_list_remove_vertex(Arb(mock): Arb<VertexInGraph<MockGraph<directedness>>>)
		-> TestResult
	{
		let mv_remove = mock.vertex_at::<0>().clone();
		let mock = mock.release_all();

		// To be able to identify if add edges are correct after removing a vertex,
		// we need to be able to uniquely identify all vertices even after removal.
		// Therefore, we use the mock graph's vertices in the weight for identification
		let mut g = AdjListGraph::new();
		let v_map = crate::mock_graph::utilities::auto_copy_from(&mut g, &mock, |g, mv, w| {
			g.new_vertex_weighted((mv, w)).unwrap()
		});
		let v_removed = v_map[&mv_remove];

		if g.remove_vertex(&v_removed).is_err()
		{
			TestResult::failed()
		}
		else
		{
			let expected_vertex_count = mock.vertex_count() - 1;
			let expected_edge_count = mock.edge_count() - mock.edges_incident_on(mv_remove).count();

			let remaining_vertices_untouched = mock
				.all_vertices_weighted()
				.filter(|(v, _)| *v != mv_remove)
				.all(|(mv, w)| {
					g.all_vertices_weighted()
						.find(|(_, (mv2, w2))| *mv2 == mv && w == w2)
						.is_some()
				});

			// Check if all remaining vertices have the expected edges
			let correct_edges = g.all_vertices_weighted().all(|(v, (mv, _))| {
				// All edges in the graph are in the mock
				let in_graph_so = g.edges_sourced_in(v).all(|(v2, ew)| {
					let mv2 = g.vertex_weight(v2).unwrap().0;
					mock.edges_between(mv, mv2).find(|ew2| ew == *ew2).is_some()
				});
				let in_graph_si = g.edges_sinked_in(v).all(|(v2, ew)| {
					let mv2 = g.vertex_weight(v2).unwrap().0;
					mock.edges_between(mv2, mv).find(|ew2| ew == *ew2).is_some()
				});

				// All edges in the mock (except those removed) are in the graph
				let in_mock_so = mock
					.edges_sourced_in(mv)
					.filter(|(mv2, _)| *mv2 != mv_remove)
					.all(|(mv2, ew)| {
						let v2 = g
							.all_vertices_weighted()
							.find(|(_, (mv3, _))| *mv3 == mv2)
							.unwrap()
							.0;
						g.edges_between(v, v2).find(|ew2| ew == *ew2).is_some()
					});
				let in_mock_si = mock
					.edges_sinked_in(mv)
					.filter(|(mv2, _)| *mv2 != mv_remove)
					.all(|(mv2, ew)| {
						let v2 = g
							.all_vertices_weighted()
							.find(|(_, (mv3, _))| *mv3 == mv2)
							.unwrap()
							.0;
						g.edges_between(v2, v).find(|ew2| ew == *ew2).is_some()
					});

				in_graph_so && in_graph_si && in_mock_so && in_mock_si
			});

			TestResult::from_bool(
				g.vertex_count() == expected_vertex_count
					&& g.edge_count() == expected_edge_count
					&& remaining_vertices_untouched
					&& correct_edges,
			)
		}
	}
}
