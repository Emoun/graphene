use crate::core::{property::RemoveVertex, Graph, GraphDerefMut, Insure};
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
pub struct NonNullGraph<C: Insure>(C);

impl<C: Insure> Insure for NonNullGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		c.graph().all_vertices().next().is_some()
	}
}

impl<C: Insure + GraphDerefMut> RemoveVertex for NonNullGraph<C>
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

impl<C: Insure> NonNull for NonNullGraph<C> {}

impl_insurer! {
	NonNullGraph<C>: NonNull, RemoveVertex
}
