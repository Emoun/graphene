use crate::core::{
	AddEdge, Constrainer, Directedness, Edge, EdgeWeighted, Graph, GraphMut, ImplGraph,
	ImplGraphMut, NewVertex, RemoveEdge, RemoveVertex,
};
use delegate::delegate;

/// A marker trait for graphs containing only unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting two vertices.
/// If the graph is directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2,_) and (v2,v1,_).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2,_) or (v1,v2,_).
/// Regardless of directedness, only one loop is allowed for each vertex,
/// i.e. only one (v,v,_).
pub trait Unique: Graph
{
	fn edge_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Option<&Self::EdgeWeight>
	{
		self.edges_between(v1, v2).next().map(|(_, _, w)| w)
	}
}

#[derive(Clone, Debug)]
pub struct UniqueGraph<C: Constrainer>(C);

impl<C: Constrainer> UniqueGraph<C>
{
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> ImplGraph for UniqueGraph<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> ImplGraphMut for UniqueGraph<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for UniqueGraph<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>
	{
		let edges: Vec<_> = g.graph().all_edges().collect();
		let mut iter = edges.iter();
		while let Some(e) = iter.next()
		{
			for e2 in iter.clone()
			{
				if (e.source() == e2.source() && e.sink() == e2.sink())
					|| (e.source() == e2.sink()
						&& e.sink() == e2.source()
						&& !<C::Graph as Graph>::Directedness::directed())
				{
					return Err(());
				}
			}
		}

		Ok(UniqueGraph(g))
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer> Graph for UniqueGraph<C>
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

impl<C: Constrainer + ImplGraphMut> GraphMut for UniqueGraph<C>
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

impl<C: Constrainer + ImplGraphMut> NewVertex for UniqueGraph<C>
where
	C::Graph: NewVertex,
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveVertex for UniqueGraph<C>
where
	C::Graph: RemoveVertex,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for UniqueGraph<C>
where
	C::Graph: AddEdge,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		if Self::Directedness::directed()
		{
			if self
				.edges_between(e.source(), e.sink())
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink())
			{
				return Err(());
			}
		}
		else
		{
			if self.edges_between(e.source(), e.sink()).next().is_some()
			{
				return Err(());
			}
		}
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for UniqueGraph<C>
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

impl<C: Constrainer> Unique for UniqueGraph<C> {}

impl_constraints! {
	UniqueGraph<C>: Unique
}
