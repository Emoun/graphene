//!
//! Tests the edge traits' API
//!

use crate::mock_graph::{MockVertex, MockEdgeWeight};
use graphene::core::{Edge, WeightRef, EdgeWeighted, WeightRefMut};

#[test]
fn edge_test() {
	let weight =  MockEdgeWeight{value: 2};
	let mut weight_mut =  MockEdgeWeight{value: 2};
	
	let no_weight_edge = (MockVertex{value: 0}, MockVertex{value: 1});
	let _:MockVertex = no_weight_edge.source();
	let _:MockVertex = no_weight_edge.sink();
	no_weight_edge.weight() == &();
	no_weight_edge.get_weight() == ();
	
	let mut weight_edge = (MockVertex{value: 0}, MockVertex{value: 1}, MockEdgeWeight{value: 2});
	let _:MockVertex = weight_edge.source();
	let _:MockVertex = weight_edge.sink();
	weight_edge.weight() == &weight;
	weight_edge.weight_mut() == &mut weight_mut;
	(weight_edge.clone()).get_weight() == weight;
	(&weight_edge).weight() == &weight;
	
	let weight_ref_edge = (MockVertex{value: 0}, MockVertex{value: 1}, &weight);
	let _:MockVertex = weight_ref_edge.source();
	let _:MockVertex = weight_ref_edge.sink();
	weight_ref_edge.weight() == &weight;
	(&weight_ref_edge).weight() == &weight;
	
	let mut weight_ref_mut_edge = (MockVertex{value: 0}, MockVertex{value: 1}, &mut weight_mut);
	let _:MockVertex = weight_ref_mut_edge.source();
	let _:MockVertex = weight_ref_mut_edge.sink();
	weight_ref_mut_edge.weight() == &weight;
	weight_ref_mut_edge.weight_mut() == &weight;
	(&weight_ref_mut_edge).weight() == &weight;
	(&weight_ref_mut_edge).weight()== &weight;
	(&mut weight_ref_mut_edge).weight_mut()== &weight;

}

