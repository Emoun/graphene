extern crate graphene;
extern crate quickcheck;

use graphene::implementations::*;
use graphene::graph::*;
use quickcheck::{Arbitrary, Gen};

#[derive(Clone,Debug)]
pub struct ArbitraryUsizeGraph {
	pub graph : UsizeGraph,
}

impl Arbitrary for ArbitraryUsizeGraph {
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let vertices = g.gen_range(1, 100) as usize;
		let mut graph = UsizeGraph::new(vertices);
		
		//How many edges
		let edges = g.gen_range(0, 100);
		for _ in 0..edges {
			let source: usize = g.gen_range(0, vertices);
			let sink: usize  = g.gen_range(0, vertices);
			assert!(source < vertices);
			assert!(sink < vertices);
			graph = graph.add_edge(UsizeEdge{source, sink}).0;
		}
		ArbitraryUsizeGraph {graph}
	}
	
	fn shrink(&self)  -> Box<Iterator<Item=ArbitraryUsizeGraph>> {
		let mut result = Vec::new();
		let mut new_graph;
		
		//Shrink by removing an edge
		for v in 0..self.graph.number_of_vertices(){
			new_graph = self.graph.clone();
			match new_graph.outgoing_edges(&v){
				Ok(o) =>
					for e in o{
						new_graph = new_graph.remove_edge(e).0;
						result.push(ArbitraryUsizeGraph {graph : new_graph.clone()});
					},
				_ => (),
			}
		}
		
		//Shrink by removing a vertex
		for v in 0..self.graph.number_of_vertices() {
			new_graph = self.graph.clone();
			new_graph = new_graph.remove_vertex(v).0;
			result.push(ArbitraryUsizeGraph {graph : new_graph});
		}
		
		Box::new(result.into_iter())
	}
}