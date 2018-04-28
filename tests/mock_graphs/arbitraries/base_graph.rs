//!
//! Implements `Arbitrary` for MockBaseGraph
//!

use mock_graphs::{
	ArbId, MockBaseGraph, MockVertex, MockEdgeId
};
use quickcheck::{
	Arbitrary, Gen
};
use graphene::{
	core::BaseGraph
};

impl Arbitrary for MockBaseGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let COUNT = 10;
		let mut vertex_values = Vec::new();
		let mut edges = Vec::new();
		
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
				while vertex_values.contains(&next_value) {
					// Create a new value
					next_value = MockVertex::arbitrary(g);
				}
				vertex_values.push(next_value);
				edges.push(Vec::new());
			}
			
			/* Create edges
			 */
			//For each edge
			for _ in 0..edge_count {
				/* Create a valid edge
				 */
				let t_source = usize::arbitrary(g) % vertex_count;
				let t_sink = usize::arbitrary(g) % vertex_count;
				let t_id = MockEdgeId::arbitrary(g);
				edges[t_source].push((t_sink, t_id));
			}
		}
		Self { values: vertex_values, edges}
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		
		/* Base case
		 */
		if self.values.len() == 0 {
			return Box::new(Vec::new().into_iter());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertices
		 */
		let mut new_values;
		//For each vertex
		for (i,&val) in self.values.iter().enumerate(){
			//Get all possible shrinkages
			let shrunk_values = val.shrink();
			//For each shrunk value
			for s in shrunk_values{
				//If no other vertex has that value
				if !self.values.contains(&s) {
					/* Add to the result a desc copy where that vertex
					 * has been shrunk to the value.
					 */
					new_values = self.values.clone();
					new_values[i] = s;
					result.push(Self {
						values: new_values, edges: self.edges.clone()});
				}
			}
		}
		
		/* Shrink by shrinking edge ids
		 */
		//For each edge
		for &(so,si, id) in self.all_edges().iter() {
			//Get all possible shrinkages
			let shrunk_ids = id.shrink();
			//For each shrunk id
			for s_id in shrunk_ids {
				/* Add to the result a desc copy where that
				 * edge id has been shrunk to the value.
				 */
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_edge((so, si, id)).unwrap();
				shrunk_graph.add_edge_copy((so, si, s_id)).unwrap();
				result.push(shrunk_graph);
			}
		}
		
		/* Shrink by removing an edge
		 */
		//For each edge
		for e in self.all_edges(){
			/* Add to the result a copy of the graph
			 * without the edge
			 */
			let mut shrunk_graph = self.clone();
			shrunk_graph.remove_edge(e).unwrap();
			result.push(shrunk_graph);
		}
		
		/* Shrink by removing a vertex
		 */
		for v in self.all_vertices(){
			let mut shrunk_graph = self.clone();
			shrunk_graph.remove_vertex(v).unwrap();
			result.push(shrunk_graph);
		}
		
		Box::new(result.into_iter())
	}
}