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
	fn get_vertex(&self) -> Self::VertexRef;
}

/// Ensures the underlying graph has at least 1 vertex.
///
/// Gives no guarantees on which vertex is returned by any given call to
/// `get_vertex` if the the graph has multiple vertices.
#[derive(Clone)]
pub struct HasVertexGraph<C: Ensure>(C, <C::Graph as Graph>::VertexRef);

impl<C: Ensure> Ensure for HasVertexGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		let v = c
			.graph()
			.all_vertices()
			.next()
			.expect("Does not have a vertex.");
		Self(c, v)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		c.graph().all_vertices().next().is_some()
	}
}

impl<C> Debug for HasVertexGraph<C>
where
	C: Ensure + Debug,
	<C::Graph as Graph>::VertexRef: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
	{
		f.debug_tuple("VertexInGraph")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for HasVertexGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if self.all_vertices().nth(1).is_some()
		{
			let weight = self.0.graph_mut().remove_vertex(v.borrow())?;
			if v.borrow() == self.1.borrow()
			{
				self.1 = self
					.0
					.graph()
					.all_vertices()
					.next()
					.expect("Does not have a vertex.");
			}
			Ok(weight)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> HasVertex for HasVertexGraph<C>
{
	fn get_vertex(&self) -> Self::VertexRef
	{
		self.1.clone()
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
pub struct VertexInGraph<C: Ensure>(C, <C::Graph as Graph>::VertexRef);

impl<C> Debug for VertexInGraph<C>
where
	C: Ensure + Debug,
	<C::Graph as Graph>::Vertex: Debug,
	<C::Graph as Graph>::VertexRef: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
	{
		f.debug_tuple("VertexInGraph")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<C: Ensure> Ensure for VertexInGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, v: <C::Graph as Graph>::VertexRef) -> Self
	{
		Self(c, v)
	}

	fn validate(c: &Self::Ensured, p: &<C::Graph as Graph>::VertexRef) -> bool
	{
		c.graph().contains_vertex(p.borrow())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for VertexInGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
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

impl<C: Ensure> HasVertex for VertexInGraph<C>
{
	fn get_vertex(&self) -> Self::VertexRef
	{
		self.1.clone()
	}
}

impl_ensurer! {
	use<C> VertexInGraph<C>: Ensure, HasVertex, RemoveVertex
	as (self.0) : C
	as (self.1) : <C::Graph as Graph>::VertexRef
}
