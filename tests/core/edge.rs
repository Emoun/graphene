//!
//! Tests the edge traits' API
//!
#![allow(unused_must_use)]

use crate::mock_graph::{MockVertex, MockEdgeWeight};
use graphene::core::{Edge, EdgeWeighted, EdgeDeref, EdgeDerefMut};


#[test]
fn edge_test() {
	let weight =  MockEdgeWeight{value: 2};
	let mut weight_mut =  MockEdgeWeight{value: 2};

	let no_weight_edge = (MockVertex{value: 0}, MockVertex{value: 1});
	let _:MockVertex = no_weight_edge.source();
	let _:MockVertex = no_weight_edge.sink();
	no_weight_edge.weight_ref() == &();
	no_weight_edge.weight_owned() == ();
	no_weight_edge.weight_owned() == (); // Twice to ensure 'no_weight_edge' wasn't moved

	let mut weight_edge = (MockVertex{value: 0}, MockVertex{value: 1}, MockEdgeWeight{value: 2});
	let _:MockVertex = weight_edge.source();
	let _:MockVertex = weight_edge.sink();
	weight_edge.weight_ref() == &weight;
	*weight_edge.weight_ref_mut() = weight.clone();
	weight_edge.weight_owned() == weight;
	

	let weight_ref_edge = (MockVertex{value: 0}, MockVertex{value: 1}, &weight);
	let _:MockVertex = weight_ref_edge.source();
	let _:MockVertex = weight_ref_edge.sink();
	weight_ref_edge.weight() == &weight;
	weight_ref_edge.weight() == &weight; // Twice to ensure 'weight_ref_edge' wasn't consumed
	(&weight_ref_edge).weight() == &weight;
	(&weight_ref_edge).weight() == &weight; // Twice to ensure 'weight_ref_edge' wasn't consumed
	

	let mut weight_ref_mut_edge = (MockVertex{value: 0}, MockVertex{value: 1}, &mut weight_mut);
	let _:MockVertex = weight_ref_mut_edge.source();
	let _:MockVertex = weight_ref_mut_edge.sink();
	weight_ref_mut_edge.weight() == &weight;
	*weight_ref_mut_edge.weight_mut() = weight.clone();
	*(&mut weight_ref_mut_edge).weight_mut() = weight.clone();
	// Using 'weight_owned()' on mutable reference weight consumes the edge
	weight_ref_mut_edge.weight_owned() == &weight;
	
	let weight_ref_mut_edge = (MockVertex{value: 0}, MockVertex{value: 1}, &mut weight_mut);
	let (edge, lone_weight) = weight_ref_mut_edge.split();
	let _:MockVertex = edge.source();
	let _:MockVertex = edge.sink();
	lone_weight == &weight;
	lone_weight == &weight; // 'split' allows for mutable weight reuse
	*lone_weight = weight.clone();
	*lone_weight = weight.clone(); // 'split' allows for mutable weight reuse
}

