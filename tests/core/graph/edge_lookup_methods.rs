//! Tests the edge lookup methods of `graphene::core::Graph`
use crate::mock_graph::{
	arbitrary::{EdgeOutside, VertexOutside},
	utilities::*,
	MockGraph,
};
use duplicate::duplicate_item;
use graphene::core::{
	property::{VertexIn, VertexInGraph},
	Directed, Edge, Graph, Release, Undirected,
};

use crate::mock_graph::arbitrary::Arb;

#[duplicate_item(
	duplicate!{
		[
			module_nested				method_nested		closure_nested		directedness_nested;
			[edges_sourced_in_directed]	[edges_sourced_in]	[e.source() == v]	[ Directed ];
			[edges_sinked_in_directed]	[edges_sinked_in]	[e.sink() == v]		[ Directed ];
			[edges_incident_directed]	[edges_incident_on]	[e.source() == v
																|| e.sink() == v]	[ Directed ];
			duplicate!{
				[
					module							method;
					[edges_sourced_in_undirected]	[edges_sourced_in];
					[edges_sinked_in_undirected]	[edges_sinked_in];
					[edges_incident_undirected]		[edges_incident_on];
				]
				[module]	[method]	[e.source() == v || e.sink() == v]	[ Undirected ];
			}
		]
		[
			module					[ module_nested ]
			method 					[ method_nested ]
			vertices				[ &v ]
			vertices_init			[ let v = g.vertex_at::<0>().clone() ]
			vertices_init_invalid 	[ let v = g.1 ]
			closure 				[ |e| if closure_nested {Some((e.other(v),e.2))} else {None} ]
			arb_graph(G) 			[ VertexInGraph<G, 1, true> ]
			arb_invalid_graph 		[ VertexOutside ]
			base_graph 				[ MockGraph<directedness_nested> ]
		]
	}
	duplicate!{
		[
			module_nested				directedness 	closure_additional;
			[edges_between_directed] 	[Directed]		[ false ];
			[edges_between_undirected] 	[Undirected]	[ (e.source() == _v2 && e.sink() == v) ];
		]
		[
			module					[module_nested]
			method 					[edges_between]
			vertices				[ &v, &_v2 ]
			vertices_init			[
				let v = g.vertex_at::<0>();
				let _v2 = g.vertex_at::<1>()
			]
			vertices_init_invalid 	[let v = g.1;let _v2 = g.2]
			closure 				[
				|e| {
					if (e.source() == v && e.sink() == _v2)
						|| closure_additional
					{
						Some(e.2)
					} else {None}
				}
			]
			arb_graph(G) 			[ VertexInGraph<G, 2, false> ]
			arb_invalid_graph 		[ EdgeOutside ]
			base_graph 				[ MockGraph<directedness> ]
		]
	}
)]
mod module
{
	use super::*;

	/// Ensures that all edges are returned.
	#[quickcheck]
	fn returns_all_edges(Arb(g): Arb<arb_graph([base_graph])>) -> bool
	{
		vertices_init;
		let g = g.release_all();
		let edges = g.method(vertices).collect();
		let expected = g.all_edges().filter_map(closure).collect();

		unordered_equivalent_lists_equal(&edges, &expected)
	}

	/// Ensures that when the vertex is not in the graph, no edges are
	/// returned.
	#[quickcheck]
	fn invalid_vertex(Arb(g): Arb<arb_invalid_graph<base_graph>>) -> bool
	{
		vertices_init_invalid;
		let result = g.0.method(vertices).next().is_none();
		result
	}
}
