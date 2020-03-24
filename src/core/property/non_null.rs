use crate::core::{property::RemoveVertex, Graph, GraphDerefMut, Insure};

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

impl<C: Insure> NonNull for NonNullGraph<C>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.all_vertices()
			.next()
			.expect("NonNull graph is null (has no vertices).")
	}
}

impl_insurer! {
	NonNullGraph<C>: NonNull, RemoveVertex
	for C as (self.0)
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
/// 2. If an instance was created as an insurer (using `insure_unvalidated` or
/// `insure` etc.) then an arbitrary vertex in the graph is chosen, with no
/// guarantees as to how this choice is made.
#[derive(Clone)]
pub struct VertexInGraph<C: Insure>(C, <C::Graph as Graph>::Vertex);

impl<C: Insure> VertexInGraph<C>
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

impl<C: Insure> Insure for VertexInGraph<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		let v = c.graph().all_vertices().next().unwrap();
		Self(c, v)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		NonNullGraph::validate(c)
	}
}

impl<C: Insure + GraphDerefMut> RemoveVertex for VertexInGraph<C>
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

impl<C: Insure> NonNull for VertexInGraph<C>
{
	fn get_vertex(&self) -> Self::Vertex
	{
		self.1
	}
}

impl_insurer! {
	VertexInGraph<C>: NonNull, RemoveVertex
	for C as (self.0)
}
