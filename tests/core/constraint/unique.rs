//!
//! Tests the `core::Unique` trait and its constrainer `core::UniqueGraph`.
//!

macro_rules! test_directedness {

	{
		$dir:ident
	} => {
		#[allow(non_snake_case)]
		mod $dir {
			use crate::mock_graph::arbitrary::{
				ArbUniqueGraph, ArbNonUniqueGraph, ArbEdgeIn, ArbVertexIn
			};
			#[allow(unused_imports)]
			use graphene::core::{Directed, Constrainer, Undirected, Graph, Edge, AutoGraph};
			use graphene::core::constraint::UniqueGraph;
			use crate::mock_graph::{MockEdgeWeight, MockVertexWeight};
			
			///
			/// Tests that UniqueGraph correctly identifies unique graphs.
			///
			#[quickcheck]
			fn accept_unique(g: ArbUniqueGraph<$dir>) -> bool
			{
				UniqueGraph::constrain_single(g.0.unconstrain()).is_ok()
			}

			///
			/// Tests that UniqueGraph correctly rejects non-unique graphs.
			///
			#[quickcheck]
			fn reject_non_unique(g: ArbNonUniqueGraph<$dir>) -> bool
			{
				UniqueGraph::constrain_single(g.0).is_err()
			}

			///
			/// Tests that a UniqueGraph rejects adding a duplicate edge
			///
			#[quickcheck]
			fn reject_add_edge(ArbEdgeIn(mut g,e): ArbEdgeIn<ArbUniqueGraph<$dir>>,
										weight: MockEdgeWeight) -> bool
			{
				g.add_edge_weighted((e.source(), e.sink(), weight)).is_err()
			}

			///
			/// Tests that a UniqueGraph accepts adding a non-duplicate edge
			///
			#[quickcheck]
			fn accept_add_edge(ArbVertexIn(mut g,v): ArbVertexIn<ArbUniqueGraph<$dir>>,
				v_weight: MockVertexWeight, e_weight: MockEdgeWeight)
				-> bool
			{
				// To ensure we add a non-duplicate edge,
				// we create a new vertex and add an edge to it from an existing one.
				let v2 = g.new_vertex_weighted(v_weight).unwrap();
				let accepted = g.add_edge_weighted((v, v2, e_weight)).is_ok();
					accepted && g.edges_between(v,v2).count() == 1
			}
		}
	}
}

test_directedness!{Directed}
test_directedness!{Undirected}
