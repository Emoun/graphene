use crate::core::{
	Constrainer, Directedness, Graph, GraphDeref, GraphDerefMut, GraphMut, Undirected,
};
use delegate::delegate;

/// A marker trait for graphs who's edges are undirected.
pub trait UndirectedConstraint: Graph
{
}

#[derive(Clone, Debug)]
pub struct UndirectedGraph<C: Constrainer>(C);

impl<C: Constrainer> UndirectedGraph<C>
{
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> GraphDeref for UndirectedGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for UndirectedGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for UndirectedGraph<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>
	{
		if <<C::Graph as Graph>::Directedness as Directedness>::directed()
		{
			Ok(Self::unchecked(g))
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

impl<C: Constrainer> Graph for UndirectedGraph<C>
{
	type Directedness = Undirected;
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

impl<C: Constrainer + GraphDerefMut> GraphMut for UndirectedGraph<C>
where
	C::Graph: GraphMut,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut<'a>(
				&'a mut self,
			) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>;

			fn all_edges_mut<'a>(
				&'a mut self,
			) -> Box<
				dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>
			>;
		}
	}
}

impl<C: Constrainer> UndirectedConstraint for UndirectedGraph<C> {}

impl_constraints! {
	UndirectedGraph<C>: UndirectedConstraint
}
