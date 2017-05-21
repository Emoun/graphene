extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
mod arbitraries;

use graphene::implementations::*;
use graphene::graph::*;
use arbitraries::{ArbitraryUsizeGraph};


fn can_add_vertex(mut g: ArbitraryUsizeGraph) -> bool{
	let init_v = g.graph.number_of_vertices();
	g.graph.new_vertex();
	init_v + 1 == g.graph.number_of_vertices()
}

quickcheck!{
	fn prop_can_add_vertex(g: ArbitraryUsizeGraph) -> bool{
		can_add_vertex(g)
	}
}

#[test]
fn test_runner() {
	match main_test() {
		Err(_) => assert!(false),
		_ => assert!(true),
	}
}

fn main_test() -> Result<(), ()> {
	let mut g = UsizeGraph::new(2);
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 0);
	assert!(g.outgoing_edges(&0)?.len() == 0);
	assert!(g.outgoing_edges(&1)?.len() == 0);
	assert!(g.incoming_edges(&0)?.len() == 0);
	assert!(g.incoming_edges(&1)?.len() == 0);
	
	g = g.add_edge(UsizeEdge{source: 0,sink: 1}).0;
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 1);
	assert!(g.outgoing_edges(&0)?.len() == 1);
	assert!(g.outgoing_edges(&1)?.len() == 0);
	assert!(g.incoming_edges(&0)?.len() == 0);
	assert!(g.incoming_edges(&1)?.len() == 1);
	
	g = g.add_edge(UsizeEdge{source: 1,sink: 0}).0;
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 2);
	assert!(g.outgoing_edges(&0)?.len() == 1);
	assert!(g.outgoing_edges(&1)?.len() == 1);
	assert!(g.incoming_edges(&0)?.len() == 1);
	assert!(g.incoming_edges(&1)?.len() == 1);
	
	g = g.remove_edge(UsizeEdge{source: 1,sink: 0}).0;
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 1);
	assert!(g.outgoing_edges(&0)?.len() == 1);
	assert!(g.outgoing_edges(&1)?.len() == 0);
	assert!(g.incoming_edges(&0)?.len() == 0);
	assert!(g.incoming_edges(&1)?.len() == 1);
	
	Ok(())
}

