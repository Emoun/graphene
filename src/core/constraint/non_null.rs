use crate::core::{constraint::RemoveVertex, Constrainer, Graph, GraphDeref, GraphDerefMut};
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

impl<C: Constrainer> GraphDeref for NonNullGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for NonNullGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
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

impl<C: Constrainer> Graph for NonNullGraph<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;

			fn all_edges<'a>(
				&'a self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
		}
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
