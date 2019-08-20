//!
//! Implements `Arbitrary` for MockGraph
//!

use crate::mock_graphs::{
	ArbVertex, ArbT, MockGraph, MockVertex, MockEdgeWeight, MockVertexWeight
};
use quickcheck::{
	Arbitrary, Gen
};
use graphene::{
	core::{
		Graph, ManualGraph
	},
};
use rand::Rng;

impl Arbitrary for MockGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let COUNT = 10;
		let mut vertices:  Vec<(MockVertex, MockVertexWeight, _)> = Vec::new();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0,COUNT);
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			//Decide the amount of edges
			let edge_count = g.gen_range(0, COUNT);
			
			/* Create vertex values ensuring that
			 * each vertex has a unique value
			 */
			let mut next_value = MockVertex::arbitrary(g);
			//For each vertex
			for _ in 0..vertex_count {
				// As long as the created value is already used by another vertex
				while vertices.iter().any( |&(id,_,_)| id.value == next_value.value) {
					// Create a new value
					next_value = MockVertex::arbitrary(g);
				}
				vertices.push((next_value, MockVertexWeight::arbitrary(g), Vec::new()));
			}
			
			/* Create edges
			 */
			//For each edge
			for _ in 0..edge_count {
				/* Create a valid edge
				 */
				let t_source = usize::arbitrary(g) % vertex_count;
				let t_sink = usize::arbitrary(g) % vertex_count;
				let t_weight = MockEdgeWeight::arbitrary(g);
				vertices[t_source].2.push((t_sink, t_weight));
			}
		}
		Self { vertices }
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		
		/* Base case
		 */
		if self.vertices.len() == 0 {
			return Box::new(std::iter::empty());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertices
		 */
		//For each vertex
		self.vertices.iter().enumerate()
			//Get all possible shrinkages
			.flat_map(|(idx,(id,_,_))| id.shrink().map(move|s| (idx,s)))
			//Remove any that are equal to existing vertices
			.filter(|(_,shrunk_id)|
				self.vertices.iter().all(|(id,_,_)| shrunk_id.value != id.value))
			//copy the graph, and change the id to the shrunk id
			.for_each(|(idx, shrunk_id)| {
				let mut new_id = self.vertices.clone();
				new_id[idx].0 = shrunk_id;
				result.push(Self{vertices: new_id});
			});
		
		/* Shrink by shrinking vertex weight
		 */
		self.vertices.iter().enumerate()
			//Get all possible shrinkages
			.flat_map(|(idx, (_,weight,_))| weight.shrink().map(move|s| (idx,s)))
			//For each shrunk weight,
			//create a new graph where the vertex has that weight
			.for_each(|(idx, shrunk_weight)|{
				let mut new_graph = self.clone();
				new_graph.vertices[idx].1 = shrunk_weight;
				result.push(new_graph);
			});
		
		/* Shrink by shrinking edge weight
		 */
		//For each edge
		self.all_edges::<Vec<_>>().into_iter().for_each(|(source,sink,ref weight)|{
			let shrunk_weights = weight.shrink();
			
			shrunk_weights.for_each( |s_w| {
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_edge_where_weight((source, sink),
					|ref w| w.value == weight.value
				).unwrap();
				shrunk_graph.add_edge_weighted((source, sink, s_w)).unwrap();
				result.push(shrunk_graph);
			});
		});
		
		/* Shrink by removing an edge
		 */
		//For each edge
		for e in self.all_edges::<Vec<_>>(){
			/* Add to the result a copy of the graph
			 * without the edge
			 */
			let mut shrunk_graph = self.clone();
			shrunk_graph.remove_edge(e).unwrap();
			result.push(shrunk_graph);
		}
		
		/* Shrink by removing a vertex that has no edges.
		 * We don't remove any edges in this step (to be able to remove a vertex)
		 * because we are already shrinking by removing edges, which means, there
		 * should be a set of edge shrinkages that result in a removable vertex.
		 */
		for v in self.all_vertices::<Vec<_>>(){
			let sourced_in: Vec<_> = self.edges_sourced_in(v);
			let sinked_in: Vec<_> = self.edges_sinked_in(v);
			let number_of_edges = sourced_in.len() + sinked_in.len();
			if number_of_edges == 0 {
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_vertex(v).unwrap();
				result.push(shrunk_graph);
			}
		}
		
		Box::new(result.into_iter())
	}
}
