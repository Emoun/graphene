use crate::core::{
	AddEdge, Constrainer, Directed, Directedness, EdgeWeighted, Graph, GraphMut, ImplGraph,
	ImplGraphMut, NewVertex, RemoveEdge, RemoveVertex,
};
use delegate::delegate;

/// A marker trait for graphs who's esges are directed.
pub trait DirectedConstraint: Graph
{
}

#[derive(Clone, Debug)]
pub struct DirectedGraph<C: Constrainer>(C);

impl<C: Constrainer> DirectedGraph<C>
{
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> ImplGraph for DirectedGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> ImplGraphMut for DirectedGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for DirectedGraph<C>
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

impl<C: Constrainer> Graph for DirectedGraph<C>
{
	type Directedness = Directed;
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

impl<C: Constrainer + ImplGraphMut> GraphMut for DirectedGraph<C>
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

impl<C: Constrainer + ImplGraphMut> NewVertex for DirectedGraph<C>
where
	C::Graph: NewVertex,
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveVertex for DirectedGraph<C>
where
	C::Graph: RemoveVertex,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for DirectedGraph<C>
where
	C::Graph: AddEdge,
{
	delegate! {
		to self.0.graph_mut() {
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
			where
				E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for DirectedGraph<C>
where
	C::Graph: RemoveEdge,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_edge_where<F>(
				&mut self,
				f: F,
			) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
			where
				F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
		}
	}
}

impl<C: Constrainer> DirectedConstraint for DirectedGraph<C> {}

impl_constraints! {
	DirectedGraph<C>: DirectedConstraint
}
