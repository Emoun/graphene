//! Tests the edge lookup methods of `graphene::core::Graph`
//!
use crate::mock_graph::{
	arbitrary::{ArbEdgeOutside, ArbTwoVerticesIn, ArbVertexIn, ArbVertexOutside},
	utilities::*,
	MockDirectedness, MockGraph, MockVertex,
};
use duplicate::duplicate;
use graphene::core::{property::NonNull, Directed, Edge, Graph, GraphMut};

#[duplicate(
	#[
		method_nested		[edges_sourced_in]		[edges_sinked_in]		[edges_incident_on]
		method_mut_nested	[edges_sourced_in_mut]	[edges_sinked_in_mut]	[edges_incident_on_mut]
		closure_nested		[|e| e.source() == v]	[|e| e.sink() == v]		[|e| e.source() == v
																				|| e.sink() == v]
		directedness_nested	[ Directed ]			[ Directed ]			[ MockDirectedness ]
	][
		[
			method 					[ method_nested ]
			method_mut 				[ method_mut_nested ]
			vertices				[ v ]
			vertices_init			[ let v = g.get_vertex(); ]
			vertices_init_invalid 	[ let v = g.1; ]
			closure 				[ closure_nested ]
			arb_graph 				[ ArbVertexIn ]
			arb_invalid_graph 		[ ArbVertexOutside ]
			base_graph 				[ MockGraph<directedness_nested> ]
		]
	]
	[
		method 					[edges_between]
		method_mut 				[edges_between_mut]
		vertices				[ v, _v2 ]
		vertices_init			[let (v, _v2) = g.get_both();]
		vertices_init_invalid 	[let v = g.1;let _v2 = g.2;]
		closure 				[
			|e| (e.source() == v && e.sink() == _v2) || (e.source() == _v2 && e.sink() == v)
		]
		arb_graph 				[ ArbTwoVerticesIn ]
		arb_invalid_graph 		[ ArbEdgeOutside ]
		base_graph 				[ MockGraph<MockDirectedness> ]
	]
)]
mod method
{
	use super::*;
	
	/// Ensures that all the returned edges are correct.
	#[quickcheck]
	fn returns_valid_edges(g: arb_graph<base_graph>) -> bool
	{
		vertices_init;
		let edges_len = g.0.method(vertices).count();

		let valid_edges_len = g.0.method(vertices).filter(closure).count();

		edges_len == valid_edges_len
	}

	/// Ensures that all edges are returned.
	#[quickcheck]
	fn returns_all_edges(g: arb_graph<base_graph>) -> bool
	{
		vertices_init;
		let edges = g.0.method(vertices).collect();
		let expected = g.0.all_edges().filter(closure).collect();

		unordered_equivalent_lists_equal(&edges, &expected)
	}

	/// Ensures that when the vertex is not in the graph, no edges are returned.
	#[quickcheck]
	fn invalid_vertex(g: arb_invalid_graph<base_graph>) -> bool
	{
		vertices_init_invalid;
		g.0.method(vertices).next().is_none()
	}

	/// Ensures that the mutable version returns the same edges as the original.
	#[quickcheck]
	fn mut_equivalent(g: base_graph, v: MockVertex, _v2: MockVertex) -> bool
	{
		#[allow(unused_mut)]
		let mut clone = g.clone();
		let mut edges_mut: Vec<_> = clone.method_mut(vertices).collect();
		
		if edges_mut.len() > 0 {
			// Ensure its mutable by updating a weight
			let old_weight = (edges_mut[0].2).clone();
			*(edges_mut[0].2) = old_weight;
			
		}
		
		let edges = g.method(vertices).collect();

		unordered_equivalent_lists(&edges, &edges_mut, _3_tuple_equality(), _3_tuple_equality())
	}
}
