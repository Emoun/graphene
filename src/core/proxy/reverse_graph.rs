use crate::core::{
	property::{AddEdge, RemoveEdge},
	Directed, Ensure, Graph, GraphDerefMut, GraphMut,
};
use delegate::delegate;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct ReverseGraph<C: Ensure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Ensure> ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	/// Creates the a reversed graph from the given graph.
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Ensure> Graph for ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Directedness = Directed;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type EdgeWeightRef<'a>
		= <C::Graph as Graph>::EdgeWeightRef<'a>
	where
		Self: 'a;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted(&self) ->
				impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>;
		}
	}

	fn edges_between(
		&self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = Self::EdgeWeightRef<'_>>
	{
		self.0.graph().edges_between(sink, source)
	}
}

impl<C: Ensure + GraphDerefMut> GraphMut for ReverseGraph<C>
where
	C::Graph: GraphMut<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut(&mut self)
				-> impl Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>;
		}
	}

	fn edges_between_mut(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = &mut Self::EdgeWeight>
	{
		self.0.graph_mut().edges_between_mut(sink, source)
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for ReverseGraph<C>
where
	C::Graph: AddEdge<Directedness = Directed>,
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		self.0.graph_mut().add_edge_weighted(sink, source, weight)
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for ReverseGraph<C>
where
	C::Graph: RemoveEdge<Directedness = Directed>,
{
	fn remove_edge_where_weight<F>(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		f: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		self.0.graph_mut().remove_edge_where_weight(source, sink, f)
	}
}

base_graph! {
	use<C> ReverseGraph<C>: NewVertex, RemoveVertex, HasVertex
	as (self.0): C
	where
		C: Ensure,
		C::Graph: Graph<Directedness = Directed>
}
