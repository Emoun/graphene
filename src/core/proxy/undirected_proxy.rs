use crate::core::{
	property::{NewVertex, RemoveVertex},
	BaseGraph, Directed, Graph, GraphDeref, GraphDerefMut, GraphMut, Insure, Undirected,
};
use delegate::delegate;

/// A proxy that acts as an undirected version of the underlying directed graph
pub struct UndirectedProxy<C: Insure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Insure> UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	pub fn new(underlying: C) -> Self
	{
		Self(underlying)
	}
}

impl<C: Insure> Graph for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
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

impl<C: Insure + GraphDerefMut> GraphMut for UndirectedProxy<C>
where
	C::Graph: GraphMut<Directedness = Directed>,
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

impl<C: Insure + GraphDerefMut> NewVertex for UndirectedProxy<C>
where
	C::Graph: NewVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}

impl<C: Insure + GraphDerefMut> RemoveVertex for UndirectedProxy<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Insure> GraphDeref for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Insure> GraphDerefMut for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Insure> BaseGraph for UndirectedProxy<C> where C::Graph: Graph<Directedness = Directed> {}
