extern crate graphene;
extern crate quickcheck;

use graphene::implementations::*;
use graphene::graph::*;
use quickcheck::{Arbitrary, Gen};

#[derive(Clone,Debug)]
pub struct ArbitraryUsizeGraph {
	pub graph : UsizeGraph,
}
/*
impl Arbitrary for ArbitraryUsizeGraph {
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let v_count = g.gen_range(1, 100) as usize;
		let mut graph = UsizeGraph::new(vec![]);
		for i in 0..v_count{
			graph = graph.add_vertex(g.gen_range(1,100)).unwrap();
		}
		let vertices = graph.all_vertices();
		
		//How many edges
		let edges = g.gen_range(0, 100);
		for _ in 0..edges {
			
			let source_i: usize = g.gen_range(0, v_count);
			let sink_i: usize  = g.gen_range(0, v_count);
			graph = graph.add_edge(UsizeEdge{	source: vertices[source_i],
												sink: vertices[sink_i]}).unwrap().0;
		}
		ArbitraryUsizeGraph {graph}
	}
	
	fn shrink(&self)  -> Box<Iterator<Item=ArbitraryUsizeGraph>> {
		let mut result = Vec::new();
		let mut new_graph;
		
		//Shrink by removing an edge
		for v_i in 0..self.graph.vertex_count(){
			new_graph = self.graph.clone();
			let vertices = new_graph.all_vertices();
			match new_graph.outgoing_edges(vertices[v_i]){
				Ok(o) =>
					for e in o{
						new_graph = new_graph.remove_edge(e).unwrap();
						result.push(ArbitraryUsizeGraph {graph : new_graph.clone()});
					},
				_ => panic!("Impossible"),
			}
		}
		
		//Shrink by removing a vertex
		for v_i in 0..self.graph.vertex_count(){
			new_graph = self.graph.clone();
			let vertices = new_graph.all_vertices();
			new_graph = new_graph.remove_vertex(vertices[v_i]).unwrap().0;
			result.push(ArbitraryUsizeGraph {graph : new_graph});
		}
		
		Box::new(result.into_iter())
	}
}
*/