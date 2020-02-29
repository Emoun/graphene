use crate::core::{constraint::RemoveVertex, Constrainer, Graph, GraphDerefMut};
use delegate::delegate;

/// A marker trait for graphs with at least 1 vertex.
pub trait NonNull: Graph
{
	/// Returns a vertex in the graph.
	///
	/// Successive calls do not have to return the same vertex,
	/// even though the graph hasn't changed.
	fn get_vertex(&self) -> Self::Vertex
	{
		self.all_vertices()
			.next()
			.expect("NonNull graph is null (has no vertices).")
	}
}

#[derive(Clone, Debug)]
pub struct NonNullGraph<C: Constrainer>(C);

impl<C: Constrainer> NonNullGraph<C>
{
	/// Constrains the given graph.
	///
	/// The given graph must be non-null. This is not checked by this function.
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> Constrainer for NonNullGraph<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>
	{
		if g.graph().all_vertices().next().is_some()
		{
			Ok(Self::new(g))
		}
		else
		{
			Err(())
		}
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveVertex for NonNullGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if self.all_vertices().nth(1).is_some()
		{
			self.0.graph_mut().remove_vertex(v)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Constrainer> NonNull for NonNullGraph<C> {}

impl_constraints! {
	NonNullGraph<C>: NonNull, RemoveVertex
}
