use crate::core::{Graph, Edge, EdgeWeighted, trait_aliases::*, AutoGraph, ManualGraph, Constrainer, BaseGraph};
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
{
	fn remove_vertex_looped(&mut self, v: Self::Vertex) -> Result<(Self::VertexWeight, Self::EdgeWeight), ()>;
}

pub struct ReflexiveGraph<G>(G)
	where G: Graph, G::EdgeWeight: Default;

impl<G> Graph for ReflexiveGraph<G>
	where G: Graph, G::EdgeWeight: Default
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	
	delegate! {
		target self.0 {
			fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I;
			
			fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>;
			
			fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
			
			fn all_edges<'a, I>(&'a self) -> I
				where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn all_edges_mut<'a, I>(&'a mut self) -> I
				where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.remove_vertex_looped(v).map(|(v_weight,_)| v_weight)
	}
	
	fn remove_vertex_forced(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>
	{
		// Remove v's loop that is guaranteed to be there.
		// If removing it failed, something else must have gone wrong
		self.0.remove_edge((v,v))?;
		
		// Then we can remove the rest of the edges using the usual method
		self.0.remove_vertex_forced(v)
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.0.remove_edge_where(|e| f(e) && !e.is_loop())
	}
}

impl<G> AutoGraph for ReflexiveGraph<G>
	where G: AutoGraph, G::EdgeWeight: Default
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>
	{
		let v = self.0.new_vertex_weighted(w)?;
		self.0.add_edge((v,v))?;
		Ok(v)
	}
}

impl<G> ManualGraph for ReflexiveGraph<G>
	where G: ManualGraph, G::EdgeWeight: Default
{
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(), ()>
	{
		self.0.add_vertex_weighted(v, w)?;
		self.0.add_edge((v,v))
	}
}

impl<G> Reflexive for ReflexiveGraph<G>
	where G: Graph, G::EdgeWeight: Default
{
	fn remove_vertex_looped(&mut self, v: Self::Vertex)
		-> Result<(Self::VertexWeight, Self::EdgeWeight), ()>
	{
		let edge_weight = self.0.remove_edge((v, v))?;
		self.0.remove_vertex(v).map(|vertex_weight| (vertex_weight,edge_weight))
	}
}

impl_constraints!{
	ReflexiveGraph<G>: Reflexive
	where G::EdgeWeight: Default
}

impl<B, C> Constrainer for ReflexiveGraph<C>
	where B: BaseGraph, C: Graph + Constrainer<BaseGraph=B>, C::EdgeWeight: Default
{
	type BaseGraph = B;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{

		if g.all_vertices::<Vec<_>>().into_iter()
			.all(|v| {
				let mut between = g.edges_between::<Vec<_>>(v,v).into_iter();
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