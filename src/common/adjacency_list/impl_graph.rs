
use crate::core::{Graph, EdgeWeighted, ManualGraph, trait_aliases::{
	Id, IntoFromIter, EdgeIntoFromIter, EdgeIntoFromIterMut
}, Directedness};
use crate::common::AdjListGraph;


impl<V,Vw,Ew,D> Graph for AdjListGraph<V,Vw,Ew,D>
	where
		V: Id,
		D: Directedness
{
	type Vertex = V;
	type VertexWeight = Vw;
	type EdgeWeight = Ew;
	type Directedness = D;
	
	fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I
	{
		self.vertices.iter().map(|(id,_,_)| *id).collect()
	}
	
	fn all_edges<'a, I>(&'a self) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>
	{
		self.vertices.iter().flat_map(
			|(source_id, _, out)| {
				out.iter().map( move|(sink_idx, e_weight)| {
					(*source_id, self.vertices[*sink_idx].0, e_weight)
				})
			}
		).collect()
	}
	fn all_edges_mut<'a, I>(&'a mut self) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>
	{
		let map: Vec<Self::Vertex> = self.vertices.iter().map(|(id,_,_)| id).cloned().collect();
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
		self.vertices.iter().find(|(id,_,_)| *id == v).map(|(_,w,_)| w)
	}
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		self.vertices.iter_mut().find(|(id,_,_)| *id == v).map(|(_,w,_)| w)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>
	{
		//Get index of vertex
		if let Some(v_idx) = self.vertices.iter().position(|(id,_,_)| *id == v){
			if self.vertices[v_idx].2.len() != 0 {
				return Err(());
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
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where
			E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		// Find the indices of the vertices
		if let (Some(v1_idx), Some(v2_idx)) =
		(	self.vertices.iter().position(|(id,_,_)| *id == e.source()),
			 self.vertices.iter().position(|(id,_,_)| *id == e.sink())
		)
			{
				// Add the edge
				self.vertices[v1_idx].2.push((v2_idx, e.get_weight()));
				Ok(())
			}else{
			Err(())
		}
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		let mut to_delete: Option<(usize, usize, Self::Vertex, Self::Vertex)> = None;
		'l:
			for (so_idx, (so_v, _, out)) in self.vertices.iter().enumerate() {
			for(e_idx, (si_idx, e_weight)) in out.iter().enumerate() {
				let si_v = self.vertices[*si_idx].0;
				if f((*so_v, si_v, e_weight)) {
					to_delete = Some((so_idx, e_idx, *so_v, si_v));
					break 'l;
				}
			}
		}
		if let Some((so_idx, e_idx, so_v, si_v)) = to_delete {
			let (_, weight) = self.vertices[so_idx].2.remove(e_idx);
			Ok((so_v, si_v, weight))
		}else{
			Err(())
		}
	}
}

impl<V,Vw,Ew,D> ManualGraph for AdjListGraph<V,Vw,Ew,D>
	where
		V: Id, D: Directedness,
{
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(),()>
	{
		if self.vertices.iter().any(|(id,_,_)| *id == v ){
			Err(())
		}else{
			self.vertices.push((v,w,Vec::new()));
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









