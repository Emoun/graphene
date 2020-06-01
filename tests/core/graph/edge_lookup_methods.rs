//! Tests the edge lookup methods of `graphene::core::Graph`
//!
use crate::mock_graph::{
	arbitrary::{ArbEdgeOutside, ArbTwoVerticesIn, ArbVertexIn, ArbVertexOutside},
	utilities::*,
	MockGraph, MockVertex,
};
use duplicate::duplicate;
use graphene::core::{property::HasVertex, Directed, Edge, Graph, GraphMut, Undirected};

#[duplicate(
	#[
		module_nested				method_nested		method_mut_nested		closure_nested		directedness_nested;
		[edges_sourced_in_directed]	[edges_sourced_in]	[edges_sourced_in_mut]	[e.source() == v]	[ Directed ];
		[edges_sinked_in_directed]	[edges_sinked_in]	[edges_sinked_in_mut]	[e.sink() == v]		[ Directed ];
		[edges_incident_directed]	[edges_incident_on]	[edges_incident_on_mut]	[e.source() == v
																				|| e.sink() == v]	[ Directed ];
		#[
			module							method				method_mut;
			[edges_sourced_in_undirected]	[edges_sourced_in]	[edges_sourced_in_mut];
			[edges_sinked_in_undirected]	[edges_sinked_in]	[edges_sinked_in_mut];
			[edges_incident_undirected]		[edges_incident_on]	[edges_incident_on_mut];
		][
			[module]	[method]	[method_mut]	[e.source() == v || e.sink() == v]	[ Undirected ];
		]
	][
		[
			module					[ module_nested ]
			method 					[ method_nested ]
			method_mut 				[ method_mut_nested ]
			vertices				[ &v ]
			vertices_init			[ let v = g.get_vertex(); ]
			vertices_init_invalid 	[ let v = g.1; ]
			closure 				[ |e| if closure_nested {Some((e.other(v),e.2))} else {None} ]
			arb_graph 				[ ArbVertexIn ]
			arb_invalid_graph 		[ ArbVertexOutside ]
			base_graph 				[ MockGraph<directedness_nested> ]
			get_weight				[ edges_mut[0].1 ]
			mut_equality			[ _2_tuple_equality(), _2_tuple_equality() ]
		]
	]
	#[
		module_nested				directedness 	closure_additional;
		[edges_between_directed] 	[Directed]		[ false ];
		[edges_between_undirected] 	[Undirected]	[ (e.source() == _v2 && e.sink() == v) ];
	][
		[
			module					[module_nested]
			method 					[edges_between]
			method_mut 				[edges_between_mut]
			vertices				[ &v, &_v2 ]
			vertices_init			[let (v, _v2) = g.get_both();]
			vertices_init_invalid 	[let v = g.1;let _v2 = g.2;]
			closure 				[
				|e| if (e.source() == v && e.sink() == _v2)
						|| closure_additional {
						Some(e.2)} else {None}
			]
			arb_graph 				[ ArbTwoVerticesIn ]
			arb_invalid_graph 		[ ArbEdgeOutside ]
			base_graph 				[ MockGraph<directedness> ]
			get_weight				[ edges_mut[0] ]
			mut_equality			[ std::cmp::PartialEq::eq, std::cmp::PartialEq::eq]
		]
	]
)]
mod module
{
	use super::*;

	/// Ensures that all edges are returned.
	#[quickcheck]
	fn returns_all_edges(g: arb_graph<base_graph>) -> bool
	{
		vertices_init;
		let edges = g.0.method(vertices).collect();
		let expected = g.0.all_edges().filter_map(closure).collect();

		unordered_equivalent_lists_equal(&edges, &expected)
	}

	/// Ensures that when the vertex is not in the graph, no edges are returned.
	#[quickcheck]
	fn invalid_vertex(g: arb_invalid_graph<base_graph>) -> bool
	{
		vertices_init_invalid;
		let result = g.0.method(vertices).next().is_none();
		result
	}

	/// Ensures that the mutable version returns the same edges as the original.
	#[quickcheck]
	fn mut_equivalent(g: base_graph, v: MockVertex, _v2: MockVertex) -> bool
	{
		#[allow(unused_mut)]
		let mut clone = g.clone();
		let mut edges_mut: Vec<_> = clone.method_mut(vertices).collect();

		if edges_mut.len() > 0
		{
			// Ensure its mutable by updating a weight
			let old_weight = (get_weight).clone();
			*(get_weight) = old_weight;
		}

		let edges = g.method(vertices).collect();

		unordered_equivalent_lists(&edges, &edges_mut, mut_equality)
	}
}
