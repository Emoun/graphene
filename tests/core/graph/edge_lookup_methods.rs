//! Tests the edge lookup methods of `graphene::core::Graph`
//!
use crate::mock_graph::{
	arbitrary::{ArbEdgeOutside, ArbTwoVerticesIn, ArbVertexIn, ArbVertexOutside},
	utilities::*,
	MockGraph,
};
use duplicate::duplicate;
use graphene::core::{property::HasVertex, Directed, Edge, Graph, ReleaseUnloaded, Undirected};

#[duplicate(
	#[
		module_nested				method_nested		closure_nested		directedness_nested;
		[edges_sourced_in_directed]	[edges_sourced_in]	[e.source() == v]	[ Directed ];
		[edges_sinked_in_directed]	[edges_sinked_in]	[e.sink() == v]		[ Directed ];
		[edges_incident_directed]	[edges_incident_on]	[e.source() == v
															|| e.sink() == v]	[ Directed ];
		#[
			module							method;
			[edges_sourced_in_undirected]	[edges_sourced_in];
			[edges_sinked_in_undirected]	[edges_sinked_in];
			[edges_incident_undirected]		[edges_incident_on];
		][
			[module]	[method]	[e.source() == v || e.sink() == v]	[ Undirected ];
		]
	][
		[
			module					[ module_nested ]
			method 					[ method_nested ]
			vertices				[ &v ]
			vertices_init			[ let v = g.get_vertex() ]
			vertices_init_invalid 	[ let v = g.1 ]
			closure 				[ |e| if closure_nested {Some((e.other(v),e.2))} else {None} ]
			arb_graph 				[ ArbVertexIn ]
			arb_invalid_graph 		[ ArbVertexOutside ]
			base_graph 				[ MockGraph<directedness_nested> ]
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
			vertices				[ &v, &_v2 ]
			vertices_init			[let (v, _v2) = g.get_both()]
			vertices_init_invalid 	[let v = g.1;let _v2 = g.2]
			closure 				[
				|e| if (e.source() == v && e.sink() == _v2)
						|| closure_additional {
						Some(e.2)} else {None}
			]
			arb_graph 				[ ArbTwoVerticesIn ]
			arb_invalid_graph 		[ ArbEdgeOutside ]
			base_graph 				[ MockGraph<directedness> ]
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
		let g = g.release_all();
		let edges = g.method(vertices).collect();
		let expected = g.edges().filter_map(closure).collect();

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
}
