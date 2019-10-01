use crate::core::{Graph, Edge, EdgeWeighted, AutoGraph, Constrainer, BaseGraph};

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

delegate_graph!{
	ReflexiveGraph<G> where G:: EdgeWeight: Default
	{
		
		fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
		{
			self.remove_vertex_looped(v).map(|(v_weight,_)| v_weight)
		}
	
		fn remove_edge_where<F>(&mut self, f: F)
								-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
			where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
		{
			self.0.remove_edge_where(|e| f(e) && !e.is_loop())
		}
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