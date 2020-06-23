use crate::core::{property::RemoveVertex, Ensure, Graph, GraphDerefMut};
use std::{
	borrow::Borrow,
	fmt::{Debug, Error, Formatter},
};

/// A marker trait for graphs with at least 1 vertex.
pub trait HasVertex: Graph
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
pub struct HasVertexGraph<C: Ensure>(C);

impl<C: Ensure> Ensure for HasVertexGraph<C>
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

impl<C: Ensure + GraphDerefMut> RemoveVertex for HasVertexGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: &Self::Vertex) -> Result<Self::VertexWeight, ()>
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

impl<C: Ensure> HasVertex for HasVertexGraph<C>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.all_vertices()
			.next()
			.expect("HasVertexGraph has no vertices.")
	}
}

impl_ensurer! {
	use<C> HasVertexGraph<C>: Ensure, HasVertex, RemoveVertex
	as (self.0) : C
}

/// Ensures a specific vertex is in the underlying graph.
///
/// That vertex is guaranteed to be returned by any call to `get_vertex` and
/// cannot be removed from the graph.
#[derive(Clone)]
pub struct VertexInGraph<C: Ensure, V: Borrow<<C::Graph as Graph>::Vertex>>(C, V);

impl<C, V> Debug for VertexInGraph<C, V>
where
	C: Ensure + Debug,
	<C::Graph as Graph>::Vertex: Debug,
	V: Borrow<<C::Graph as Graph>::Vertex> + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
	{
		f.debug_tuple("VertexInGraph")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<C: Ensure, V: Borrow<<C::Graph as Graph>::Vertex>> Ensure for VertexInGraph<C, V>
{
	fn ensure_unvalidated(c: Self::Ensured, v: V) -> Self
	{
		Self(c, v)
	}

	fn validate(c: &Self::Ensured, p: &V) -> bool
	{
		c.graph().contains_vertex(p.borrow())
	}
}

impl<C: Ensure + GraphDerefMut, V: Borrow<<C::Graph as Graph>::Vertex>> RemoveVertex
	for VertexInGraph<C, V>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: &Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		if self.1.borrow() != v.borrow()
		{
			self.0.graph_mut().remove_vertex(v)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure, V: Borrow<<C::Graph as Graph>::Vertex>> HasVertex for VertexInGraph<C, V>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.1.borrow().clone()
	}
}

impl_ensurer! {
	use<C,V> VertexInGraph<C,V>: Ensure, HasVertex, RemoveVertex
	as (self.0) : C
	as (self.1) : V
	where V: Borrow<<C::Graph as Graph>::Vertex>
}
