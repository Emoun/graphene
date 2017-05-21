extern crate graphene;
extern crate quickcheck;

use graphene::implementations::UsizeGraph;
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
			graph = graph.set_edge(source, sink);
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
						new_graph = new_graph.delete_edge(v, e.sink());
						result.push(ArbitraryUsizeGraph {graph : new_graph.clone()});
					},
				_ => (),
			}
		}
		let mut source;
		let mut sink;
		
		//Shrink by removing a vertex
		for v in 0..self.graph.number_of_vertices() {
			new_graph = UsizeGraph::new(self.graph.number_of_vertices()-1);
			
			//For all edges in the original graph
			for e in self.graph.all_edges(){
				//Ignore edges to or from the deleted vertex
				if !(e.source == v || e.sink == v) {
					source =e.source;
					sink =e.sink;
					
					//If either source or sink are larger
					//than the removed vertex, reduce their index by 1
					if e.source > v {
						source -= 1;
					}
					if e.sink > v {
						sink -= 1;
					}
					new_graph = new_graph.set_edge(source, sink);
				}
			}
			result.push(ArbitraryUsizeGraph {graph : new_graph.clone()});
		}
		
		Box::new(result.into_iter())
	}
}