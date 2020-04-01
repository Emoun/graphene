use crate::core::{
	property::{NewVertex, RemoveVertex},
	BaseGraph, Directed, Ensure, Graph, GraphDeref, GraphDerefMut, GraphMut, Undirected,
};
use delegate::delegate;

/// A proxy that acts as an undirected version of the underlying directed graph
pub struct UndirectedProxy<C: Ensure>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Ensure> UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	pub fn new(underlying: C) -> Self
	{
		Self(underlying)
	}
}

impl<C: Ensure> Graph for UndirectedProxy<C>
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

impl<C: Ensure + GraphDerefMut> GraphMut for UndirectedProxy<C>
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

impl<C: Ensure + GraphDerefMut> NewVertex for UndirectedProxy<C>
where
	C::Graph: NewVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for UndirectedProxy<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Ensure> GraphDeref for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Ensure> GraphDerefMut for UndirectedProxy<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Ensure> BaseGraph for UndirectedProxy<C> where C::Graph: Graph<Directedness = Directed> {}
