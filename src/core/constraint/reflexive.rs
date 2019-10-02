use crate::core::{Graph, Edge, EdgeWeighted, AddVertex, Constrainer, AddEdge, GraphMut};
use delegate::delegate;

///
/// A marker trait for a reflexive graph.
///
/// Every vertex in a reflexive graph has exactly one loop. This means that
/// it is impossible to add or remove a vertex without doing the same for its loop edge.
/// Because of this, the edge weight must implement Default, such that Graph's methods
/// can add edge weights automatically.
///
///
pub trait Reflexive: Graph
	where Self::EdgeWeight: Default
{}

pub struct ReflexiveGraph<G>(G)
	where G: Graph, G::EdgeWeight: Default;

impl<G: Graph> Graph for ReflexiveGraph<G>
	where G::EdgeWeight: Default
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

impl<G: GraphMut> GraphMut for ReflexiveGraph<G>
	where G::EdgeWeight: Default
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

impl<G: AddVertex + AddEdge> AddVertex for ReflexiveGraph<G>
	where G::EdgeWeight: Default
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
						   -> Result<Self::Vertex, ()>
	{
		let v = self.0.new_vertex_weighted(w)?;
		self.0.add_edge((v,v))?;
		Ok(v)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.0.remove_edge((v, v))?;
		self.0.remove_vertex(v)
	}
}

impl<G: AddEdge> AddEdge for ReflexiveGraph<G>
	where G::EdgeWeight: Default
{
	delegate! {
		target self.0 {
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.0.remove_edge_where(|e| f(e) && !e.is_loop())
	}
}

impl<G> Reflexive for ReflexiveGraph<G>
	where G: Graph, G::EdgeWeight: Default
{}

impl_constraints!{
	ReflexiveGraph<G>: Reflexive
	where G::EdgeWeight: Default
}

impl<C> Constrainer for ReflexiveGraph<C>
	where
		C: Constrainer,
		C::EdgeWeight: Default
{
	type BaseGraph = C::BaseGraph;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{

		if g.all_vertices().all(|v| {
				let mut between = g.edges_between(v,v);
				if let Some(_) = between.next() {
					between.next().is_none()
				} else {
					false
				}
			})
		{
			Ok(ReflexiveGraph(g))
		} else {
			Err(())
		}
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}