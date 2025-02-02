use crate::core::{
	property::{HasVertex, VertexInGraph},
	Ensure, Graph, ReleasePayload,
};
use std::borrow::Borrow;

/// A maker trait for graphs that are rooted.
///
/// A rooted graph has a distinguished node, called the root of the graph.
/// A rooted graph always has a root, which cannot be removed (unless another
/// vertex is designated as the root first).
///
/// Even though rooted graphs always implement `HasVertex`, the `any_vertex`
/// method is not required to always return the root of the graph.
/// To always get the root, the `root` method can be used.
pub trait Rooted: HasVertex
{
	/// Returns the root of the graph.
	///
	/// If the root of the graph changes between successive calls to this
	/// method, so does the vertex returned.
	/// However, if the root doesn't change, the vertex is guaranteed to be
	/// returned by successive calls.
	fn root(&self) -> Self::Vertex;

	/// Designates the given vertex is the root of the graph.
	///
	/// Returns error if it was unable to change the root of the graph.
	/// E.g. if the given vertex is not in the graph.
	fn set_root(&mut self, v: impl Borrow<Self::Vertex>) -> Result<(), ()>;

	/// Return true of the given vertex is the root of the graph.
	/// Otherwise, returns false.
	fn is_root(&self, v: impl Borrow<Self::Vertex>) -> bool
	{
		self.root() == *v.borrow()
	}
}

/// Ensures a specific vertex is the root of the graph.
pub struct RootedGraph<C: Ensure>(VertexInGraph<C>);

impl<C: Ensure> Clone for RootedGraph<C>
where
	VertexInGraph<C>: Clone,
{
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
}

impl<C: Ensure> ReleasePayload for RootedGraph<C>
{
	type Base = C::Base;
	type Ensured = C;
	type Payload = (<C::Graph as Graph>::Vertex, C::Payload);

	fn release(self) -> (Self::Ensured, <C::Graph as Graph>::Vertex)
	{
		let (r1, r2) = self.0.release();
		(r1, r2[0])
	}
}

impl<C: Ensure> Ensure for RootedGraph<C>
{
	fn ensure_unchecked(c: Self::Ensured, v: <C::Graph as Graph>::Vertex) -> Self
	{
		Self(VertexInGraph::ensure_unchecked(c, [v]))
	}

	fn can_ensure(c: &Self::Ensured, p: &<C::Graph as Graph>::Vertex) -> bool
	{
		VertexInGraph::<C>::can_ensure(c, &[*p])
	}
}

impl<C: Ensure> Rooted for RootedGraph<C>
{
	fn root(&self) -> Self::Vertex
	{
		self.0.any_vertex()
	}

	fn set_root(&mut self, v: impl Borrow<Self::Vertex>) -> Result<(), ()>
	{
		self.0.set_vertex(&[*v.borrow()])
	}
}

impl_ensurer! {
	use<C> RootedGraph<C>: ReleasePayload, Ensure, Rooted
	as (self.0) : VertexInGraph<C>
	where C: Ensure
}
