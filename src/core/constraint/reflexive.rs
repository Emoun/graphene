use crate::core::{Graph, Edge, EdgeWeighted, AddVertex, Constrainer, AddEdge, GraphMut, BaseGraph, ConstrainerMut};

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

pub struct ReflexiveGraph<C: Constrainer>(C)
	where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default;

impl<C: Constrainer> Constrainer for ReflexiveGraph<C>
	where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>{
		let g = c.base();
		if g.all_vertices().all(|v| {
			let mut between = g.edges_between(v,v);
			if let Some(_) = between.next() {
				between.next().is_none()
			} else {
				false
			}
		})
		{
			Ok(ReflexiveGraph(c))
		} else {
			Err(())
		}
	}
	
	fn constrained(&self) -> &Self::Constrained {
		&self.0
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
impl<C: ConstrainerMut> ConstrainerMut for ReflexiveGraph<C>
	where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default
{
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut {
		&mut self.0
	}
}

impl<C: Constrainer> Graph for ReflexiveGraph<C>
	where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default
{
	type Vertex = <<C::Base as BaseGraph>::Graph as Graph>::Vertex;
	type VertexWeight = <<C::Base as BaseGraph>::Graph as Graph>::VertexWeight;
	type EdgeWeight = <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight;
	type Directedness = <<C::Base as BaseGraph>::Graph as Graph>::Directedness;
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.base().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.base().all_edges()
	}
}

impl<C: ConstrainerMut>  GraphMut for ReflexiveGraph<C>
	where
		<<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default,
		<C::Base as BaseGraph>::Graph: GraphMut
{
	fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.base_mut().all_vertices_weighted_mut()
	}
	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.base_mut().all_edges_mut()
	}
}

impl<C: ConstrainerMut> AddVertex for ReflexiveGraph<C>
	where
		<C::Base as BaseGraph>::Graph: AddVertex + AddEdge,
		<<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default,
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
						   -> Result<Self::Vertex, ()>
	{
		let v = self.base_mut().new_vertex_weighted(w)?;
		self.base_mut().add_edge((v,v))?;
		Ok(v)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.base_mut().remove_edge((v, v))?;
		self.base_mut().remove_vertex(v)
	}
}

impl<C: ConstrainerMut> AddEdge for ReflexiveGraph<C>
	where
		<C::Base as BaseGraph>::Graph: AddEdge,
		<<C::BaseMut as BaseGraph>::Graph as Graph>::EdgeWeight: Default
{
	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.base_mut().remove_edge_where(|e| f(e) && !e.is_loop())
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.base_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer> Reflexive for ReflexiveGraph<C>
	where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default
{}

impl_constraints!{
	ReflexiveGraph<C>: Reflexive
		where <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight: Default,
}
