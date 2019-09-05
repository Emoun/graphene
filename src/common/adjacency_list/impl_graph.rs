
use crate::core::{Graph, EdgeWeighted, trait_aliases::{
	IntoFromIter, EdgeIntoFromIter, EdgeIntoFromIterMut
}, Directedness, BaseGraph, AutoGraph, Edge};
use crate::common::AdjListGraph;


impl<Vw,Ew,D> Graph for AdjListGraph<Vw,Ew,D>
	where D: Directedness
{
	type Vertex = usize;
	type VertexWeight = Vw;
	type EdgeWeight = Ew;
	type Directedness = D;
	
	fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I
	{
		(0..self.vertices.len()).collect()
	}
	
	fn all_edges<'a, I>(&'a self) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>
	{
		self.vertices.iter().enumerate().flat_map(
			|(source_id,( _, out))| {
				out.iter().map( move|(sink_idx, e_weight)| {
					(source_id, *sink_idx, e_weight)
				})
			}
		).collect()
	}
	fn all_edges_mut<'a, I>(&'a mut self) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>
	{
		self.vertices.iter_mut().enumerate().flat_map(
			|(source_id,( _, out))| {
				out.iter_mut().map( move|(sink_idx, e_weight)| {
					(source_id, *sink_idx, e_weight)
				})
			}
		).collect()
	}
	
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>
	{
		if v < self.vertices.len() {
			Some(&self.vertices[v].0)
		} else {
			None
		}
	}
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		if v < self.vertices.len() {
			Some(&mut self.vertices[v].0)
		} else {
			None
		}
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>
	{
		if v < self.vertices.len() {
			while let Ok(_)  = self.remove_edge_where(|e| e.sink() == v || e.source() == v) {
				// Drop edge
			}
			Ok(self.vertices.remove(v).0)
		} else {
			Err(())
		}
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where
			E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		let len = self.vertices.len();
		if e.source() < len && e.sink() < len {
			self.vertices[e.source()].1.push((e.sink(), e.get_weight()));
			Ok(())
		} else {
			Err(())
		}
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		let found = self.vertices.iter().enumerate()
			.flat_map(|(so_i,(_,edges))|
				edges.iter().enumerate().map(move|(si_i,(si, w))| ((so_i, si_i, si, w)))
			).find(|(so_i, _, si, w)| f((*so_i, **si, w)));
		
		if let Some((so,si_i,_,_)) = found {
			let (si,w) = self.vertices[so].1.remove(si_i);
			Ok((so,si,w))
		}else{
			Err(())
		}
	}
}

impl<Vw,Ew,D> AutoGraph for AdjListGraph<Vw,Ew,D>
	where D: Directedness,
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex,()>
	{
		let new_v = self.vertices.len();
		self.vertices.push((w,Vec::new()));
		Ok(new_v)
	}
}

impl<Vw,Ew,D> BaseGraph for AdjListGraph<Vw,Ew,D>
	where D: Directedness,
{}






