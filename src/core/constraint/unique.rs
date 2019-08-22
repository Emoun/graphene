use crate::core::{Graph, EdgeWeighted, trait_aliases::*, Directedness, Edge};
use delegate::delegate;
///
/// A marker trait for graphs containing only unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting two vertices.
/// If the graph is directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2,_) and (v2,v1,_).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2,_) or (v1,v2,_).
/// Regardless of directedness, only one loop is allowed for each vertex,
/// i.e. only one (v,v,_).
///
///
///
pub trait Unique: Graph
{}

pub struct UniqueGraph<G: Graph>(pub G);

impl<G: Graph> Graph for UniqueGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	
	delegate! {
		target self.0 {
	
			fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I;
			
			fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight> ;
			
			fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
			
			fn all_edges<'a, I>(&'a self) -> I
				where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn all_edges_mut<'a, I>(&'a mut self) -> I
				where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool ;
		}
	}
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if G::directed() {
			if self.edges_between::<Vec<_>>(e.source(), e.sink()).into_iter()
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink()){
				return Err(());
			}
		} else {
			if !self.edges_between::<Vec<_>>(e.source(), e.sink()).is_empty() {
				return Err(());
			}
		}
		self.0.add_edge_weighted(e)
	}
}
