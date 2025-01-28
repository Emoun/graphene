//! Test the `core::property::Acyclic` trait and its ensurer

use crate::mock_graph::{
	arbitrary::{Arb, EdgeIn, TwoReachableVerticesIn},
	MockEdgeWeight, MockGraph,
};
use duplicate::duplicate_item;
use graphene::core::{
	property::{Acyclic, AcyclicGraph, AddEdge, NoLoops, RemoveEdge, VertexIn},
	Directed, Graph, Guard, Release, Undirected,
};
use static_assertions::assert_impl_all;

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;
	use crate::mock_graph::arbitrary::CyclicGraph;

	/// Tests that AcyclicGraph correctly identifies acyclic graphs.
	#[quickcheck]
	fn accept_acyclic(g: Arb<AcyclicGraph<MockGraph<directedness>>>) -> bool
	{
		AcyclicGraph::can_guard(&g.0.release_all())
	}

	/// Tests that AcyclicGraph correctly rejects cyclic graphs.
	#[quickcheck]
	fn reject_cyclic(g: Arb<CyclicGraph<directedness>>) -> bool
	{
		!AcyclicGraph::can_guard(&g.0)
	}

	/// Tests that a AcyclicGraph accepts adding an edge that doesn't
	/// result in a cycle
	#[quickcheck]
	fn accept_add_edge(Arb(g): Arb<EdgeIn<AcyclicGraph<MockGraph<directedness>>>>) -> bool
	{
		let source = g.vertex_at::<0>();
		let sink = g.1;
		let mut g = g.release_all();

		// We start by removing the edge, so that we can re-add it later
		let edge_count = g.edges_between(source, sink).count();
		let weight = g.remove_edge(source, sink).unwrap();

		let mut g = AcyclicGraph::guard_unchecked(g);

		g.add_edge_weighted(source, sink, weight).is_ok()
			&& g.edges_between(source, sink).count() == edge_count
	}

	/// Tests that a AcyclicGraph rejects adding an edge that results in a cycle
	#[quickcheck]
	fn reject_add_edge(
		Arb(graph): Arb<TwoReachableVerticesIn<AcyclicGraph<MockGraph<directedness>>>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		let v1 = graph.0.vertex_at::<0>();
		let v2 = graph.0.vertex_at::<1>();
		let edge_count = graph.all_edges().count();

		let mut g = AcyclicGraph::guard_unchecked(graph.release_all());

		g.add_edge_weighted(v2, v1, weight).is_err() && g.all_edges().count() == edge_count
	}

	assert_impl_all!(AcyclicGraph<MockGraph<directedness>>: Acyclic, NoLoops);
}
