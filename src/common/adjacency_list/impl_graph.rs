
use core::{
	Graph, Edge, EdgeWeighted, ManualGraph,
	trait_aliases::{
		Id,
	}
};
use common::AdjListGraph;


impl<'a,V,Vw,Ew> Graph<'a> for AdjListGraph<V,Vw,Ew>
	where
		V: Id, Ew:'a,
{
	type Vertex = V;
	type VertexWeight = Vw;
	type EdgeWeight = Ew;
	type VertexIter = Vec<Self::Vertex>;
	type EdgeIter = Vec<(Self::Vertex,Self::Vertex,&'a Self::EdgeWeight)>;
	type EdgeMutIter = Vec<(Self::Vertex,Self::Vertex,&'a mut Self::EdgeWeight)>;
	
	fn all_vertices(&self) -> Self::VertexIter
	{
		unimplemented!()
	}
	
	fn all_edges(&'a self) -> Self::EdgeIter
	{
		unimplemented!()
	}
	fn all_edges_mut(&'a mut self) -> Self::EdgeMutIter
	{
		unimplemented!()
	}
	
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>
	{
		unimplemented!()
	}
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		unimplemented!()
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>
	{
		unimplemented!()
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where
			E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		unimplemented!()
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		unimplemented!()
	}
}

impl<'a,V,Vw,Ew> ManualGraph<'a> for AdjListGraph<V,Vw,Ew>
	where
		V: Id, Ew:'a,
{
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(),()>
	{
		unimplemented!()
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









