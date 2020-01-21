///
/// Tests `TarjanSCC`
///

use crate::mock_graph::arbitrary::{ArbVertexIn, ArbConnectedGraph};
use graphene::core::{Directed, Graph};
use graphene::algo::TarjanSCC;
use crate::mock_graph::utilities::unordered_equivalent_lists_equal;

///
/// Tests that if the graph is made up of a single component, it is found.
///
#[quickcheck]
fn finds_single_component(ArbVertexIn(graph, v): ArbVertexIn<ArbConnectedGraph<Directed>>)
	-> bool
{
	let sccs = TarjanSCC::new(&graph.0, v).collect::<Vec<_>>();
	
	sccs.len() == 1 &&
		unordered_equivalent_lists_equal( &sccs[0].all_vertices().collect(),
										  &graph.0.all_vertices().collect())
}



