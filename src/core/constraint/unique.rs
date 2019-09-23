use crate::core::{Graph, EdgeWeighted, Directedness, Edge, AutoGraph, Constrainer, BaseGraph};
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

pub struct UniqueGraph<G: Graph>(G);

impl<G: Graph> Graph for UniqueGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	
	delegate! {
		target self.0 {
	
			fn all_vertices_weighted<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>;
				
			fn all_vertices_weighted_mut<'a>(&'a mut self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
			
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
			
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
		}
	}
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if G::directed() {
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

impl<G: AutoGraph> AutoGraph for UniqueGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
		}
	}
}

impl<G: Graph> Unique for UniqueGraph<G>{}

impl_constraints!{
	UniqueGraph<G>: Unique
}

impl<B, C> Constrainer for UniqueGraph<C>
	where B: BaseGraph, C: Graph + Constrainer<BaseGraph=B>
{
	type BaseGraph = B;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		for e in g.all_edges() {
			for e2 in g.all_edges() {
				if (e.source() == e2.source() && e.sink() == e2.sink()) ||
					(e.source() == e2.sink() && e.sink() == e2.source() && !C::directed())
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