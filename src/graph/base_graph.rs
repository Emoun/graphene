use super::*;

pub trait BaseGraph<'a,>
{
	type Vertex: 		Copy + Eq;
	type Weight: 		Copy + Eq;
	type VertexIter: 	IntoIterator<Item=Self::Vertex>;
	type EdgeIter: 		IntoIterator<Item=BaseEdge<Self::Vertex,Self::Weight>>;
	
	
	fn all_vertices(&'a self) -> Self::VertexIter;
	
	fn all_edges(&'a self) -> Self::EdgeIter;
	
	fn add_vertex(&'a mut self, v: Self::Vertex) -> Result<(),()>;
	
	fn remove_vertex(&'a mut self, v: Self::Vertex) -> Result<(),()>;
	
	fn add_edge(&'a mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
	fn remove_edge(&'a mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
}

