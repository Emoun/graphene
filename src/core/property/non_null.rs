use crate::core::{property::RemoveVertex, Ensure, Graph, GraphDerefMut};
use std::fmt::{Debug, Error, Formatter};

/// A marker trait for graphs with at least 1 vertex.
pub trait NonNull: Graph
{
	/// Returns a vertex in the graph.
	///
	/// Successive calls do not have to return the same vertex,
	/// even though the graph hasn't changed.
	///
	/// This trait doesn't provide a default implementation for `get_vertex`
	/// to ensure that "wrapping" ensurers don't accidentally use it, instead
	/// of actively delegating to the inner class, who might have its own
	/// implementation.
	fn get_vertex(&self) -> Self::Vertex;
}

/// Ensures the underlying graph has at least 1 vertex.
///
/// Gives no guarantees on which vertex is returned by any given call to
/// `get_vertex` if the the graph has multiple vertices.
#[derive(Clone, Debug)]
pub struct NonNullGraph<C: Ensure>(C);

impl<C: Ensure> Ensure for NonNullGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		c.graph().all_vertices().next().is_some()
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for NonNullGraph<C>
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

impl<C: Ensure> NonNull for NonNullGraph<C>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.all_vertices()
			.next()
			.expect("NonNull graph is null (has no vertices).")
	}
}

impl_ensurer! {
	use<C> NonNullGraph<C>: Ensure, NonNull, RemoveVertex
	as (self.0) : C
}

/// Ensures a specific vertex is in the underlying graph.
///
/// That vertex is guaranteed to be returned by any call to `get_vertex` and
/// cannot be removed from the graph.
///
/// Which vertex is guaranteed by this type depends on how an instance was
/// created:
///
/// 1. If `new` was used, the vertex it was given is guaranteed.
/// 2. If an instance was created as an ensurer (using `ensure_unvalidated` or
/// `ensure` etc.) then an arbitrary vertex in the graph is chosen, with no
/// guarantees as to how this choice is made.
#[derive(Clone)]
pub struct VertexInGraph<C: Ensure>(C, <C::Graph as Graph>::Vertex);

impl<C: Ensure> Debug for VertexInGraph<C>
where
	C: Debug,
	<C::Graph as Graph>::Vertex: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
	{
		f.debug_tuple("VertexInGraph")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<C: Ensure> VertexInGraph<C>
{
	pub fn new(graph: C, v: <C::Graph as Graph>::Vertex) -> Option<Self>
	{
		if graph.graph().contains_vertex(v)
		{
			Some(Self(graph, v))
		}
		else
		{
			None
		}
	}

	pub fn new_unvalidated(graph: C, v: <C::Graph as Graph>::Vertex) -> Self
	{
		Self(graph, v)
	}
}

impl<C: Ensure> Ensure for VertexInGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, v: <C::Graph as Graph>::Vertex) -> Self
	{
		Self(c, v)
	}

	fn validate(c: &Self::Ensured, p: &<C::Graph as Graph>::Vertex) -> bool
	{
		c.graph().contains_vertex(*p)
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for VertexInGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if self.1 != v
		{
			self.0.graph_mut().remove_vertex(v)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> NonNull for VertexInGraph<C>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.1
	}
}

impl_ensurer! {
	use<C> VertexInGraph<C>: Ensure, NonNull, RemoveVertex
	as (self.0) : C
	as (self.1) : <C::Graph as Graph>::Vertex
}
