//! Test the `core::property::Acyclic` trait and its ensurer

use crate::mock_graph::{
	arbitrary::{ArbAcyclicGraph, ArbCyclicGraph, ArbEdgeIn, ArbTwoReachableVerticesIn},
	MockEdgeWeight, MockGraph,
};
use duplicate::duplicate;
use graphene::core::{
	property::{Acyclic, AcyclicGraph, AddEdge, NoLoops, RemoveEdge},
	Directed, EnsureUnloaded, Graph, ReleaseUnloaded, Undirected,
};
use static_assertions::assert_impl_all;

#[duplicate(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;

	/// Tests that AcyclicGraph correctly identifies acyclic graphs.
	#[quickcheck]
	fn accept_acyclic(g: ArbAcyclicGraph<directedness>) -> bool
	{
		AcyclicGraph::validate(&g.0.release_all())
	}

	/// Tests that AcyclicGraph correctly rejects cyclic graphs.
	#[quickcheck]
	fn reject_cyclic(g: ArbCyclicGraph<directedness>) -> bool
	{
		!AcyclicGraph::validate(&g.0)
	}

	/// Tests that a AcyclicGraph accepts adding an edge that doesn't
	/// result in a cycle
	#[quickcheck]
	fn accept_add_edge(
		ArbEdgeIn(g, (source, sink, _)): ArbEdgeIn<ArbAcyclicGraph<directedness>>,
	) -> bool
	{
		// We start by removing the edge, so that we can re-add it later
		let mut g = g.release_all();
		let edge_count = g.edges_between(source, sink).count();
		let weight = g.remove_edge(source, sink).unwrap();

		let mut g = AcyclicGraph::ensure_unvalidated(g);

		g.add_edge_weighted(source, sink, weight).is_ok()
			&& g.edges_between(source, sink).count() == edge_count
	}

	/// Tests that a AcyclicGraph rejects adding an edge that results in a cycle
	#[quickcheck]
	fn reject_add_edge(
		graph: ArbTwoReachableVerticesIn<ArbAcyclicGraph<directedness>>,
		weight: MockEdgeWeight,
	) -> bool
	{
		let (v1, v2) = graph.0.get_both();
		let edge_count = graph.all_edges().count();

		let mut g = AcyclicGraph::ensure_unvalidated(graph.release_all());

		g.add_edge_weighted(v2, v1, weight).is_err() && g.all_edges().count() == edge_count
	}

	assert_impl_all!(AcyclicGraph<MockGraph<directedness>>: Acyclic, NoLoops);
}
