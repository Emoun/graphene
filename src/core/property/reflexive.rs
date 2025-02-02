use crate::core::{
	property::{AddEdge, NewVertex, RemoveEdge, RemoveVertex},
	Ensure, Graph, GraphDerefMut,
};
use std::borrow::Borrow;

/// A marker trait for a reflexive graph.
///
/// Every vertex in a reflexive graph has exactly one loop. This means that
/// it is impossible to add or remove a vertex without doing the same for its
/// loop edge. Because of this, the edge weight must implement Default, such
/// that Graph's methods can add edge weights automatically.
///
/// TODO: Should be possible for the `Default` requirement to be omitted and two
/// different ensurers be implemented one taking `Default` and another taking a
/// closure.
pub trait Reflexive: Graph
where
	Self::EdgeWeight: Default,
{
}

pub struct ReflexiveGraph<C: Ensure>(C)
where
	<C::Graph as Graph>::EdgeWeight: Default;

impl<C: Ensure> Ensure for ReflexiveGraph<C>
where
	<C::Graph as Graph>::EdgeWeight: Default,
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		let g = c.graph();
		g.all_vertices().all(|v| {
			let mut between = g.edges_between(v, v);
			if let Some(_) = between.next()
			{
				between.next().is_none()
			}
			else
			{
				false
			}
		})
	}
}

impl<C: Ensure + GraphDerefMut> NewVertex for ReflexiveGraph<C>
where
	C::Graph: NewVertex + AddEdge,
	<C::Graph as Graph>::EdgeWeight: Default,
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		let v = self.0.graph_mut().new_vertex_weighted(w)?;
		self.0.graph_mut().add_edge(v.clone(), v.clone())?;
		Ok(v)
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for ReflexiveGraph<C>
where
	C::Graph: RemoveVertex + RemoveEdge,
	<C::Graph as Graph>::EdgeWeight: Default,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		self.0.graph_mut().remove_edge(v.borrow(), v.borrow())?;
		self.0.graph_mut().remove_vertex(v)
	}
}

impl<C: Ensure> Reflexive for ReflexiveGraph<C> where <C::Graph as Graph>::EdgeWeight: Default {}

impl_ensurer! {
	use<C> ReflexiveGraph<C>: Ensure, NewVertex, RemoveVertex, Reflexive
	as (self.0) : C
	where <C::Graph as Graph>::EdgeWeight: Default,
}
