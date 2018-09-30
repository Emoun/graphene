
use mock_graphs::{
	MockVertex, MockEdgeWeight, MockVertexWeight
};
use graphene::{
	core::{
		Graph, Edge, ManualGraph,
		trait_aliases::Id
	},
};

#[derive(Clone, Debug)]
pub struct MockGraph
{
	///
	/// The vertices in the graph.
	/// Each entry in the vector is a vertex.
	/// The first element for each vertex is its ID.
	/// The second element is its weight.
	///
	pub vertices: Vec<(
		MockVertex,
		MockVertexWeight,
		Vec<(usize,MockEdgeWeight)>
	)>,
}

impl<'a> Graph<'a> for MockGraph
{
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	type VertexIter = Vec<Self::Vertex>;
	type EdgeIter = Vec<(Self::Vertex,Self::Vertex,&'a Self::EdgeWeight)>;
	type EdgeMutIter = Vec<(Self::Vertex,Self::Vertex,&'a mut Self::EdgeWeight)>;
	
	fn empty_graph() -> Self{
		Self{vertices: Vec::new()}
	}
	
	fn all_vertices(&self) -> Self::VertexIter {
		let mut result = Vec::new();
		
		//For each value, output a copy
		for i in 0..self.vertices.len() {
			result.push(self.vertices[i].0);
		}
		result
	}
	
	fn all_edges(&'a self) -> Self::EdgeIter
	{
		self.vertices.iter().flat_map(
			|(source_id, _, out)| {
				out.iter().map( move|(sink_idx, e_weight)| {
					(*source_id, self.vertices[*sink_idx].0, e_weight)
				})
			}
		).collect()
	}
	fn all_edges_mut(&'a mut self) -> Self::EdgeMutIter
	{
		let map: Vec<MockVertex> = self.vertices.iter().map(|(id,_,_)| id).cloned().collect();
		self.vertices.iter_mut().flat_map(
			|(source_id, _, out)| {
				let map = &map;
				out.iter_mut().map( move|(sink_idx, e_weight)| {
					(*source_id, map[*sink_idx], e_weight)
				})
			}
		).collect()
	}
	
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>
	{
		if let Some((_,w,_)) = self.vertices.iter().find(|(id,weight,_)| id.value == v.value){
			Some(w)
		}else{
			None
		}
	}
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		if let Some((_,w,_)) = self.vertices.iter_mut().find(|(id,weight,_)| id.value == v.value){
			Some(w)
		}else{
			None
		}
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>{
		//Get index of vertex
		if let Some(v_idx) = self.vertices.iter().position(|(id,_,_)| id.value == v.value){
			/* Remove all incoming edges to v
			 */
			//Go through all vertices
			for temp_v2_idx in 0..self.vertices.len(){
				let mut to_remove = Vec::new();
				//Go through all outgoing edges
				//If an edge points to v, collect its index
				let ref mut temp_v2_out = self.vertices[temp_v2_idx].2;
				temp_v2_out.iter().filter(|(sink,_)| *sink == v_idx).enumerate().for_each(
					|(i, _)| to_remove.push(i));
				//Delete all collected edges
				to_remove.into_iter().rev().for_each(
					|i| {temp_v2_out.remove(i);}
				);
			}
			
			// For efficiency, instead of just removing v and shifting all
			// other vertices' indices, we swap the vertex with the highest
			// index into the index of v
			
			// Start by re-point all edges pointing to last vertex (called 'last' from now on)
			// to point to the index of v
			let last_idx = self.vertices.len() - 1;
			//For each vertex
			//any edge pointing to the last value
			//should now point to v
			self.vertices.iter_mut().flat_map(|(_,_,out)| out.iter_mut())
				.filter(|(sink_idx, _)| *sink_idx == last_idx)
				.for_each(|(sink_idx, _)| *sink_idx = v_idx);
			
			// Remove v, swapping in the value of last
			return Ok(self.vertices.swap_remove(v_idx).1);
		}
		//Vertex not part of the core
		Err(())
	}
	
	fn add_edge_weighted<E>(&mut self, e: E, w: Self::EdgeWeight) -> Result<(),()>
		where
			E: Edge<Self::Vertex,()>,
	{
		// Find the indices of the vertices
		if let (Some(v1_idx), Some(v2_idx)) =
			(	self.vertices.iter().position(|(id,_,_)| *id == e.source()),
				 self.vertices.iter().position(|(id,_,_)| *id == e.sink())
			)
		{
			// Add the edge
			self.vertices[v1_idx].2.push((v2_idx, w));
			Ok(())
		}else{
			Err(())
		}
	}
	
	fn remove_edge<E>(&mut self, e: E) -> Result<Self::EdgeWeight,()>
		where E: Edge<Self::Vertex, ()>
	{
		// Find the indices of the vertices
		if let (Some(v1_idx), Some(v2_idx)) =
			(	self.vertices.iter().position(|(id,_,_)| id.value == e.source().value),
				self.vertices.iter().position(|(id,_,_)| id.value == e.sink().value)
			)
		{
			// Find the index of the edge
			if let Some(idx) = self.vertices[v1_idx].2.iter().position(
				|&(sink_idx, _)| sink_idx == v2_idx)
			{
				//remove edge
				return Ok(self.vertices[v1_idx].2.remove(idx).1);
			}
		}
		Err(())
	}
}

impl<'a> ManualGraph<'a> for MockGraph
{
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(),()>
	{
		if self.vertices.iter().any(|(id,_,_)| id.value == v.value ){
			Err(())
		}else{
			self.vertices.push((v,w,Vec::new()));
			Ok(())
		}
	}
	
}


mod test{
	#[test]
	fn func(){
		use mock_graphs::{MockGraph,MockVertex};
		use graphene::core::{Graph,ManualGraph};
		let mut g = MockGraph::empty_graph();
		let m0 = MockVertex{value: 0};
		let m1 = MockVertex{value: 1};
		let m2 = MockVertex{value: 2};
		g.add_vertex(m0).unwrap();
		g.add_vertex(m1).unwrap();
		g.add_vertex(m2).unwrap();
		assert_eq!(g.all_edges().len(), 0);
		g.add_edge((m0, m1)).unwrap();
		assert_eq!(g.all_edges().len(), 1);
		g.add_edge((m1, m2)).unwrap();
		g.add_edge((m2, m0)).unwrap();
		assert_eq!(g.all_edges().len(), 3);
		let mut g2 = g.clone();
		
		assert!(g.all_edges().into_iter().any(|(v1,v2,_)|
			(v1.value == m0.value) && (v2.value == m1.value)));
		assert!(g.all_edges().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g.all_edges().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		assert!(!g.all_edges().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m1.value)));
		
		assert!(g.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m0.value) && (v2.value == m1.value)));
		assert!(g.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		assert!(!g.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m1.value)));
		
		g.remove_vertex(m0).unwrap();
		assert_eq!(g.all_vertices().len(), 2);
		assert_eq!(g.all_edges().len(), 1);
		assert!(g.all_edges().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		
		g.remove_vertex(m1).unwrap();
		assert_eq!(g.all_vertices().len(), 1);
		assert_eq!(g.all_edges().len(), 0);
		
		g2.remove_edge((m0,m1)).unwrap();
		assert_eq!(g2.all_vertices().len(), 3);
		assert_eq!(g2.all_edges().len(), 2);
		assert!(g2.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g2.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		
		g2.remove_edge((m1,m2)).unwrap();
		assert_eq!(g2.all_vertices().len(), 3);
		assert_eq!(g2.all_edges().len(), 1);
		assert!(g2.all_edges_mut().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		
		g2.remove_edge((m2,m0)).unwrap();
		assert_eq!(g2.all_vertices().len(), 3);
		assert_eq!(g2.all_edges().len(), 0);
	}
}


