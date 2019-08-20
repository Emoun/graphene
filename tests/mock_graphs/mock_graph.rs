
use crate::mock_graphs::{
	MockVertex, MockEdgeWeight, MockVertexWeight
};
use graphene::{
	core::{
		Graph, Edge, ManualGraph, EdgeWeighted,
		trait_aliases::{
			Id, IntoFromIter
		}
	},
};
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};

#[derive(Clone)]
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
	)>
}

impl Debug for MockGraph {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		f.write_str("MockGraph { vertices: [ ")?;
		for (v,w,_) in &self.vertices {
			f.write_fmt(format_args!("({:?}, {:?}), ", v.value, w.value))?;
		}
		f.write_str("], edges: [ ")?;
		for (v,_,edges) in &self.vertices {
			for (idx, w) in edges {
				f.write_fmt(format_args!("({:?}, {:?}, {:?}), ",
										 v.value, self.vertices[*idx].0.value, w.value))?;
			}
		}
		f.write_str("] }")?;
		Ok(())
	}
}

impl MockGraph {
	
	pub fn new() -> Self
	{
		Self{vertices: Vec::new()}
	}
	
	///
	/// Replaces an existing vertex with another, maintaining any edges, weight or other.
	/// Effectively changes the vertex's ID.
	///
	/// Panics if the replacement value already exists in the graph.
	///
	pub fn replace_vertex(&mut self, to_replace: MockVertex, replacement: MockVertex)
	{
		assert!( !self.vertices.iter().any(|(v,_,_)| v.value == replacement.value) );
		
		let pos = self.vertices.iter_mut().find(|(v,_,_)| v.value == to_replace.value).unwrap();
		pos.0 = replacement;
	}
}

impl Graph for MockGraph
{
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	
	fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I
	{
		I::from_iter(self.vertices.iter().map(|(v,_,_)| *v))
	}
	
	fn all_edges<'a, I>(&'a self) -> I
		where I: IntoFromIter<(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>
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
		where I: IntoFromIter<(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>
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
		if let Some((_,w,_)) = self.vertices.iter().find(|(id,_,_)| id.value == v.value){
			Some(w)
		}else{
			None
		}
	}
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		if let Some((_,w,_)) = self.vertices.iter_mut().find(|(id,_,_)| id.value == v.value){
			Some(w)
		}else{
			None
		}
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>{
		//Get index of vertex
		if let Some(v_idx) = self.vertices.iter().position(|(id,_,_)| id.value == v.value){
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

impl ManualGraph for MockGraph
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
	use crate::mock_graphs::{MockGraph, MockVertex, ArbGraphAndTwoVertices};
	use graphene::core::{ManualGraph, Graph, Edge};
	use quickcheck::Arbitrary;
	
	#[test]
	fn func(){
		use crate::mock_graphs::{MockGraph,MockVertex};
		use graphene::core::{Graph,ManualGraph};
		let mut g = MockGraph::new();
		let m0 = MockVertex{value: 0};
		let m1 = MockVertex{value: 1};
		let m2 = MockVertex{value: 2};
		g.add_vertex(m0).unwrap();
		g.add_vertex(m1).unwrap();
		g.add_vertex(m2).unwrap();
		assert_eq!(g.all_edges::<Vec<_>>().len(), 0);
		g.add_edge((m0, m1)).unwrap();
		assert_eq!(g.all_edges::<Vec<_>>().len(), 1);
		g.add_edge((m1, m2)).unwrap();
		g.add_edge((m2, m0)).unwrap();
		assert_eq!(g.all_edges::<Vec<_>>().len(), 3);
		let mut g2 = g.clone();
		
		assert!(g.all_edges::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m0.value) && (v2.value == m1.value)));
		assert!(g.all_edges::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g.all_edges::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		assert!(!g.all_edges::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m1.value)));
		
		assert!(g.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m0.value) && (v2.value == m1.value)));
		assert!(g.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		assert!(!g.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m1.value)));
		
		assert!(g.remove_vertex(m0).is_err());
		assert_eq!(g.all_vertices::<Vec<_>>().len(), 3);
		assert_eq!(g.all_edges::<Vec<_>>().len(), 3);
		assert_eq!(g.edges_incident_on::<Vec<_>>(m0).len(), 2);
		assert_eq!(g.edges_incident_on::<Vec<_>>(m1).len(), 2);
		assert_eq!(g.edges_incident_on::<Vec<_>>(m2).len(), 2);
		
		g.remove_vertex_forced(m0).unwrap();
		assert_eq!(g.all_vertices::<Vec<_>>().len(), 2);
		assert_eq!(g.all_edges::<Vec<_>>().len(), 1);
		assert!(g.all_edges::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		
		g.remove_vertex_forced(m1).unwrap();
		assert_eq!(g.all_vertices::<Vec<_>>().len(), 1);
		assert_eq!(g.all_edges::<Vec<_>>().len(), 0);
		
		g2.remove_edge((m0,m1)).unwrap();
		assert_eq!(g2.all_vertices::<Vec<_>>().len(), 3);
		assert_eq!(g2.all_edges::<Vec<_>>().len(), 2);
		assert!(g2.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m1.value) && (v2.value == m2.value)));
		assert!(g2.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		
		g2.remove_edge((m1,m2)).unwrap();
		assert_eq!(g2.all_vertices::<Vec<_>>().len(), 3);
		assert_eq!(g2.all_edges::<Vec<_>>().len(), 1);
		assert!(g2.all_edges_mut::<Vec<_>>().into_iter().any(|(v1,v2,_)|
			(v1.value == m2.value) && (v2.value == m0.value)));
		
		g2.remove_edge((m2,m0)).unwrap();
		assert_eq!(g2.all_vertices::<Vec<_>>().len(), 3);
		assert_eq!(g2.all_edges::<Vec<_>>().len(), 0);
		
	}
}


