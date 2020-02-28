use crate::core::{
	constraint::{AddEdge, NewVertex, RemoveEdge, RemoveVertex},
	BaseGraph, Constrainer, Directed, Edge, EdgeDeref, EdgeWeighted, Graph, GraphDeref,
	GraphDerefMut, GraphMut,
};
use delegate::delegate;

#[derive(Debug)]
pub struct ReverseGraph<C: Constrainer>(C)
where
	C::Graph: Graph<Directedness = Directed>;

impl<C: Constrainer> ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	/// Creates the a reversed graph from the given graph.
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> Graph for ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Directedness = Directed;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a Self::VertexWeight)>>;
		}
	}

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(
			self.0
				.graph()
				.all_edges()
				.map(|e| (e.sink(), e.source(), e.weight_owned())),
		)
	}
}

impl<C: Constrainer + GraphDerefMut> GraphMut for ReverseGraph<C>
where
	C::Graph: GraphMut<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a mut Self::VertexWeight)>>;
		}
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(
			self.0
				.graph_mut()
				.all_edges_mut()
				.map(|e| (e.sink(), e.source(), e.weight_owned())),
		)
	}
}

impl<C: Constrainer + GraphDerefMut> NewVertex for ReverseGraph<C>
where
	C::Graph: NewVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}
impl<C: Constrainer + GraphDerefMut> RemoveVertex for ReverseGraph<C>
where
	C::Graph: RemoveVertex<Directedness = Directed>,
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Constrainer + GraphDerefMut> AddEdge for ReverseGraph<C>
where
	C::Graph: AddEdge<Directedness = Directed>,
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		self.0
			.graph_mut()
			.add_edge_weighted((e.sink(), e.source(), e.weight_owned()))
	}
}

impl<C: Constrainer + GraphDerefMut> RemoveEdge for ReverseGraph<C>
where
	C::Graph: RemoveEdge<Directedness = Directed>,
{
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		self.0
			.graph_mut()
			.remove_edge_where(|e| f((e.sink(), e.source(), e.weight())))
	}
}

impl<C: Constrainer> GraphDeref for ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer + GraphDerefMut> GraphDerefMut for ReverseGraph<C>
where
	C::Graph: Graph<Directedness = Directed>,
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> BaseGraph for ReverseGraph<C> where C::Graph: Graph<Directedness = Directed> {}
