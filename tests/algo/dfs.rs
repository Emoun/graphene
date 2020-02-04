//! Tests `DFS`
//!

use crate::mock_graph::{
	arbitrary::{ArbConnectedGraph, ArbVertexIn, ArbVerticesIn},
	MockEdgeWeight, MockGraph, MockVertex,
};
use graphene::{
	algo::DFS,
	core::{AddEdge, Constrainer, Directed, Graph, ImplGraph},
};
use std::{cell::Cell, collections::HashSet};

duplicate_for_directedness! {
	$directedness

	///
	/// Tests that all vertices in a connected component are produced exactly once.
	///
	#[quickcheck]
	fn visits_component_once(ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>)
			 -> bool
	{
		// Use a set to ensure we only count each vertex once
		let mut visited = HashSet::new();
		let mut visited_once = true;
		DFS::new_simple(mock.graph(), v).for_each(|v|{ visited_once &= visited.insert(v); });

		// We ensure all vertices were visited, but only once
		visited.len() == mock.0.all_vertices().count() && visited_once
	}

	///
	/// Tests that no vertices outside a connected component are produced
	///
	#[quickcheck]
	fn visits_none_outside_component(ArbVertexIn(g1, v): ArbVertexIn<ArbConnectedGraph<directedness>>,
		g2: MockGraph<directedness>)
		 -> bool
	{
		// Our starting connected component
		let mut graph = g1.0.unconstrain();

		// First join the two graphs
		let v_map = graph.join(&g2);

		// Ensure that no visited vertex comes from outside the start component
		DFS::new_simple(&graph, v).all(|visit| v_map.values().all(|&new_v| visit != new_v))
	}

	///
	/// Tests that the 'on_exit' closure is called in stack order compared to the
	/// produced vertices.
	///
	#[quickcheck]
	fn on_exit_stack_call_order(
		ArbVertexIn(mock, v): ArbVertexIn<ArbConnectedGraph<directedness>>)
		-> bool
	{
		let stack: Cell<Vec<MockVertex>> = Cell::new(Vec::new());
		let mut success = true;

		fn on_exit(v: MockVertex, (stack, success): &mut (&Cell<Vec<MockVertex>>, &mut bool)){
			// On exit, check that the same vertex is on top of the stack
			let mut s = stack.take();
			if let Some(&v2) = s.last() {
				if v == v2 {
					s.pop();
				} else {
					**success = false;
				}
			} else {
				**success = false;
			}
			stack.replace(s);
		}

		DFS::new(mock.graph(), v, on_exit, (&stack, &mut success))
			.for_each(|v| {
				// When a vertex is produced by the DFS, put it on the stack.
				let mut s = stack.take();
				s.push(v);
				stack.replace(s);
			});
		success
	}
}

/// Tests for directed graphs, even if an edge targets a connected component
/// from a different component, only the first component's vertices are
/// produced.
///
/// This is different from `visits_none_outside_component` because in that case
/// the components are completely unconnected with no edges between them
/// (incoming or outgoing). This test therefore ensures edges aren't taken the
/// wrong directed.
#[quickcheck]
fn directed_doesnt_visit_incomming_component(
	component: ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
	rest: ArbVerticesIn<MockGraph<Directed>>,
	weight: MockEdgeWeight,
) -> bool
{
	let mut graph = ((component.0).0).0.unconstrain();
	let comp_set = component.1;
	let v = (component.0).1;
	let g2 = rest.0;
	let g2_set = rest.1;

	// First join the two graphs
	let v_map = graph.join(&g2);

	// Add edges from the other component to the start component
	for (v1, v2) in comp_set.iter().zip(g2_set.iter())
	{
		graph
			.add_edge_weighted((v_map[v2], *v1, weight.clone()))
			.unwrap();
	}

	// Ensure that no visited vertex comes from outside the start component
	DFS::new_simple(&graph, v).all(|visit| v_map.values().all(|&new_v| visit != new_v))
}

/// Tests for directed graphs that any component with an edge to it from the
/// start component is also visited in full.
#[quickcheck]
fn directed_visits_outgoing_component(
	comp1: ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
	comp2: ArbVerticesIn<ArbVertexIn<ArbConnectedGraph<Directed>>>,
	weight: MockEdgeWeight,
) -> bool
{
	let mut graph = ((comp1.0).0).0.unconstrain();
	let comp1_set = comp1.1;
	let v = (comp1.0).1;
	let g2 = ((comp2.0).0).0;
	let comp2_set = comp2.1;
	let v2 = (comp2.0).1;

	// First join the two graphs
	let v_map = graph.join(&g2);

	// Add edges from start component to the other component
	graph
		.add_edge_weighted((v, v_map[&v2], weight.clone()))
		.unwrap();
	for (v1, v2) in comp1_set.iter().zip(comp2_set.iter())
	{
		graph
			.add_edge_weighted((*v1, v_map[v2], weight.clone()))
			.unwrap();
	}

	// Ensure that all vertices are visited
	DFS::new_simple(&graph, v).count() == graph.all_vertices().count()
}
