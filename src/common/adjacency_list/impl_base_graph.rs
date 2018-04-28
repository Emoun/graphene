
use core::{
	BaseGraph, Edge, AutoEdgeGraph,
	trait_aliases::{
		Id,
	}
};
use common::AdjListGraph;


impl<V,W> BaseGraph for AdjListGraph<V,W>
	where
		V: Id,
{
	type Vertex = V;
	type EdgeId = usize;
	type VertexIter = Vec<V>;
	type EdgeIter = Vec<(V,V,Self::EdgeId)>;
	
	fn empty_graph() -> AdjListGraph<V,W>{
		AdjListGraph{values: Vec::new(), edges: Vec::new(), edge_weights: Vec::new()}
	}
	
	fn all_vertices(&self) -> Vec<V> {
		let mut result = Vec::new();
		
		//For each value, output a copy
		for i in 0..self.values.len() {
			result.push(self.values[i]);
		}
		result
	}
	
	fn all_edges(& self) -> Vec<(V,V,Self::EdgeId)> {
		let mut result = Vec::new();
		
		//For each vertex (source)
		for (source_i, ref out) in self.edges.iter().enumerate() {
			let source_value = self.values[source_i];
			//For each outgoing edge (sink)
			for &(sink_i, weight ) in out.iter() {
				let sink_value = self.values[sink_i];
				//Return the edge
				result.push((source_value, sink_value, weight));
			}
		}
		result
	}
	
	fn add_vertex(&mut self, v: V) -> Result<(),()>{
		
		if self.values.contains(&v){
			Err(())
		}else{
			self.values.push(v);
			self.edges.push(Vec::new());
			Ok(())
		}
	}
	
	fn remove_vertex(&mut self, v: V) -> Result<(),()>{
		//Get index of vertex
		if let Some(v_i) = self.get_index(v){
			/* Remove all incoming edges to v
			 */
			//Go through all vertices
			for t_v_i in 0..self.values.len(){
				let mut to_remove = Vec::new();
				//Go through all outgoing edges
				let ref mut t_v_out = self.edges[t_v_i];
				for i in 0..t_v_out.len() {
					//If an edge points to v, collect its index
					if t_v_out[i].0 == v_i {
						to_remove.push(i);
					}
				}
				//Delete all collected edges
				for i in (0..to_remove.len()).rev() {
					//Delete the last indices first so
					//that the other indices aren't invalidated
					t_v_out.remove(to_remove[i]);
				}
			}
			
			// For efficiency, instead of just removing v and shifting all
			// other vertices' indeces, we swap the vertex with the highest
			// index into the index of v
			
			// Start by re-point all edges pointing to last vertex (called 'last' from now on)
			// to point to the index of v
			let last_i = self.values.len() - 1;
			//For each vertex
			for t_v_i in 0..self.edges.len() {
				//any edge pointing to the old last value
				//should now point to v
				let ref mut t_v_out = self.edges[t_v_i];
				for edge_i in 0..t_v_out.len(){
					if t_v_out[edge_i].0 == last_i {
						t_v_out[edge_i].0 = v_i
					}
				}
			}
			
			// Remove v, swapping in the value of last
			self.values.swap_remove(v_i);
			self.edges.swap_remove(v_i);
			return Ok(());
		}
		//Vertex not part of the core
		Err(())
	}
	
	fn add_edge_copy<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>
	{
		self.if_valid_edge( e, |s, source_i, sink_i, id|{
			s.edges[source_i].push((sink_i,id));
			Ok(())
		})
	}
	
	fn remove_edge<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>
	{
		self.if_valid_edge(e, |s, source_i, sink_i, weight|{
			if let Some(i) = s.edges[source_i].iter().position(|&(sink_cand, w )| {
				sink_cand == sink_i && w == weight
			}) {
				s.edges[source_i].remove(i);
				return Ok(());
			}
			Err(())
		})
	}
}

impl<V,W> AutoEdgeGraph for AdjListGraph<V,W>
	where
		V: Id,
		W: Default
{
	fn add_edge<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, ()>
	{
		let id = self.edge_weights.len();
		self.edge_weights.push(W::default());
		
		//check whether that results in a valid edge
		if let Err(()) = self.if_valid_edge((*e.source(), *e.sink(),id),
			|s, _, _, id|{
				s.add_edge_copy((*e.source(),*e.sink(),id))
			}
		){
			self.edge_weights.pop();
			Err(())
		} else {
			Ok(())
		}
	}
}

/*
impl<V,W> ConstrainedGraph for AdjListGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{
	impl_base_constraint!{}
}


impl<V,W> ExactGraph for AdjListGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{}

#[test]
fn empty_has_no_vertices(){
	assert_eq!(0, AdjListGraph::<u32,u32>::empty_graph().all_vertices().len());
}

#[test]
fn empty_has_no_edges(){
	assert_eq!(0, AdjListGraph::<u32,u32>::empty_graph().all_edges().len());
}


*/









