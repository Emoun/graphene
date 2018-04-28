
use mock_graphs::{
	MockEdgeId, MockVertex
};
use graphene::{
	core::{
		BaseGraph, Edge,
		trait_aliases::Id
	},
	common::AdjListGraph
};

#[derive(Clone, Debug)]
pub struct MockBaseGraph
{
	pub edges: Vec<Vec<(usize,MockEdgeId)>>,
	pub values:Vec<MockVertex>,
}

impl BaseGraph for MockBaseGraph
{
	type Vertex = MockVertex;
	type EdgeId = MockEdgeId;
	type VertexIter = Vec<Self::Vertex>;
	type EdgeIter = Vec<(Self::Vertex,Self::Vertex,Self::EdgeId)>;
	
	fn empty_graph() -> Self{
		Self{values: Vec::new(), edges: Vec::new()}
	}
	
	fn all_vertices(&self) -> Self::VertexIter {
		let mut result = Vec::new();
		
		//For each value, output a copy
		for i in 0..self.values.len() {
			result.push(self.values[i]);
		}
		result
	}
	
	fn all_edges(&self) -> Self::EdgeIter {
		let mut result = Vec::new();
		
		//For each vertex (source)
		for (source_idx, ref out) in self.edges.iter().enumerate() {
			let source_value = self.values[source_idx];
			//For each outgoing edge (sink)
			for &(sink_idx, id) in out.iter() {
				let sink_value = self.values[sink_idx];
				//Return the edge
				result.push((source_value, sink_value, id));
			}
		}
		result
	}
	
	fn add_vertex(&mut self, v: Self::Vertex) -> Result<(),()>{
		
		if self.values.contains(&v){
			Err(())
		}else{
			self.values.push(v);
			self.edges.push(Vec::new());
			Ok(())
		}
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<(),()>{
		//Get index of vertex
		if let Some(v_idx) = self.values.iter().position(|value| *value == v){
			/* Remove all incoming edges to v
			 */
			//Go through all vertices
			for temp_v2_idx in 0..self.values.len(){
				let mut to_remove = Vec::new();
				//Go through all outgoing edges
				let ref mut temp_v2_out = self.edges[temp_v2_idx];
				for i in 0..temp_v2_out.len() {
					//If an edge points to v, collect its index
					if temp_v2_out[i].0 == v_idx {
						to_remove.push(i);
					}
				}
				//Delete all collected edges
				for i in (0..to_remove.len()).rev() {
					//Delete the last indices first so
					//that the other indices aren't invalidated
					temp_v2_out.remove(to_remove[i]);
				}
			}
			
			// For efficiency, instead of just removing v and shifting all
			// other vertices' indeces, we swap the vertex with the highest
			// index into the index of v
			
			// Start by re-point all edges pointing to last vertex (called 'last' from now on)
			// to point to the index of v
			let last_idx = self.values.len() - 1;
			//For each vertex
			for temp_v_idx in 0..self.edges.len() {
				//any edge pointing to the last value
				//should now point to v
				let ref mut temp_v_out = self.edges[temp_v_idx];
				for edge_idx in 0..temp_v_out.len(){
					if temp_v_out[edge_idx].0 == last_idx {
						temp_v_out[edge_idx].0 = v_idx
					}
				}
			}
			
			// Remove v, swapping in the value of last
			self.values.swap_remove(v_idx);
			self.edges.swap_remove(v_idx);
			return Ok(());
		}
		//Vertex not part of the core
		Err(())
	}
	
	fn add_edge_copy<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>
	{
		// Find the indeces of the vertices
		if let (Some(v1_idx), Some(v2_idx)) =
			(	self.values.iter().position(|value| *value == *e.source()),
				 self.values.iter().position(|value| *value == *e.sink())
			)
		{
			// Add the edge
			self.edges[v1_idx].push((v2_idx, *e.id()));
			Ok(())
		}else{
			Err(())
		}
	}
	
	fn remove_edge<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>
	{
		// Find the indeces of the vertices
		if let (Some(v1_idx), Some(v2_idx)) =
			(	self.values.iter().position(|value| *value == *e.source()),
				self.values.iter().position(|value| *value == *e.sink())
			)
		{
			// Find the index of the edge
			if let Some(idx) = self.edges[v1_idx].iter().position(
				|&(sink_idx, id)| sink_idx == v2_idx && id == *e.id())
			{
				//remove edge
				self.edges[v1_idx].remove(idx);
				return Ok(());
			}
		}
		Err(())
	}
}



