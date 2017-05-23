extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
mod arbitraries;

use graphene::implementations::*;
use graphene::graph::*;
use arbitraries::{ArbitraryUsizeGraph};


fn can_add_vertex(mut g: ArbitraryUsizeGraph, val: usize) -> bool{
	let init_v = g.graph.vertex_count();
	let new_graph = g.graph.add_vertex(val).unwrap();
	init_v + 1 == new_graph.vertex_count()
}

quickcheck!{
	fn prop_can_add_vertex(g: ArbitraryUsizeGraph, val: usize) -> bool{
		can_add_vertex(g,val)
	}
}

#[test]
fn main_test() -> () {
	let mut g = UsizeGraph::new(vec![1,2]);
	assert!(g.vertex_count() == 2);
	assert!(g.edge_count() == 0);
	assert!(g.outgoing_edges(&0).unwrap().len() == 0);
	assert!(g.outgoing_edges(&1).unwrap().len() == 0);
	assert!(g.incoming_edges(&0).unwrap().len() == 0);
	assert!(g.incoming_edges(&1).unwrap().len() == 0);
	
}

