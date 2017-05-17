extern crate graphene;

use graphene::implementations::UsizeGraph;
use graphene::graph::Graph;

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
	
	g.set_edge(0, 1);
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 1);
	assert!(g.outgoing_edges(&0)?.len() == 1);
	assert!(g.outgoing_edges(&1)?.len() == 0);
	assert!(g.incoming_edges(&0)?.len() == 0);
	assert!(g.incoming_edges(&1)?.len() == 1);
	
	g.set_edge(1, 0);
	assert!(g.number_of_vertices() == 2);
	assert!(g.number_of_edges() == 2);
	assert!(g.outgoing_edges(&0)?.len() == 1);
	assert!(g.outgoing_edges(&1)?.len() == 1);
	assert!(g.incoming_edges(&0)?.len() == 1);
	assert!(g.incoming_edges(&1)?.len() == 1);
	
	Ok(())
}
