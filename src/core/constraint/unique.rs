use crate::core::{Graph, EdgeWeighted, Directedness, Edge, AddVertex, Constrainer, GraphMut, AddEdge};
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
{
	fn edge_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Option<&Self::EdgeWeight>
	{
		self.edges_between(v1,v2).next().map(|(_,_,w)| w)
	}
}

#[derive(Clone, Debug)]
pub struct UniqueGraph<G: Graph>(G);

impl<G: Graph> UniqueGraph<G>
{
	///
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	///
	pub fn unchecked(g: G) -> Self
	{
		Self(g)
	}
}

impl<G: Graph> Graph for UniqueGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	delegate!{
		target self.0 {
			fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a Self::VertexWeight)>> ;
			
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>> ;
		}
	}
}

impl<G: GraphMut> GraphMut for UniqueGraph<G>
{
	delegate!{
		target self.0 {
			fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a mut Self::VertexWeight)>> ;
	
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>> ;
	
		}
	}
}

impl<G: AddVertex> AddVertex for UniqueGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
		}
	}
}

impl<G: AddEdge> AddEdge for UniqueGraph<G>
{
	delegate! {
		target self.0 {
			fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;

		}
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if Self::Directedness::directed() {
			if self.edges_between(e.source(), e.sink())
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink()){
				return Err(());
			}
		} else {
			if self.edges_between(e.source(), e.sink()).next().is_some() {
				return Err(());
			}
		}
		self.0.add_edge_weighted(e)
	}
}

impl<G: Graph> Unique for UniqueGraph<G>{}

impl_constraints!{
	UniqueGraph<G>: Unique
}

impl<C: Constrainer> Constrainer for UniqueGraph<C>
{
	type BaseGraph = C::BaseGraph;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		let edges: Vec<_> = g.all_edges().collect();
		let mut iter = edges.iter();
		while let  Some(e) = iter.next() {
			for e2 in iter.clone() {
				if (e.source() == e2.source() && e.sink() == e2.sink()) ||
					(e.source() == e2.sink() && e.sink() == e2.source() && !C::Directedness::directed())
				{
					return Err(())
				}
			}
		}

		Ok(UniqueGraph(g))
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}