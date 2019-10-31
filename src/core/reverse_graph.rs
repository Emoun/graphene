use crate::core::{Graph, Directed, Edge, GraphMut, NewVertex, AddEdge, EdgeWeighted, EdgeDeref, Constrainer, ImplGraphMut, ImplGraph, BaseGraph, RemoveVertex, RemoveEdge};

#[derive(Debug)]
pub struct ReverseGraph<C: Constrainer>(C)
	where C::Graph: Graph<Directedness=Directed>;

impl<C: Constrainer> ReverseGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	///
	/// Creates the a reversed graph from the given graph.
	///
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> Graph for ReverseGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Directedness = Directed;
	
	fn all_vertices_weighted<'a>(&'a self)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.0.graph().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(self.0.graph().all_edges().map(|e| (e.sink(), e.source(), e.weight_owned())))
	}
		
}

impl<C: Constrainer + ImplGraphMut> GraphMut for ReverseGraph<C>
	where C::Graph: GraphMut<Directedness=Directed>
{
	fn all_vertices_weighted_mut<'a>(&'a mut self)
		-> Box<dyn 'a +Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.0.graph_mut().all_vertices_weighted_mut()
	}

	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(self.0.graph_mut().all_edges_mut().map(|e| (e.sink(), e.source(), e.weight_owned())))
	}
}

impl<C: Constrainer + ImplGraphMut> NewVertex for ReverseGraph<C>
	where C::Graph: NewVertex<Directedness=Directed>
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		self.0.graph_mut().new_vertex_weighted(w)
	}
}
impl<C: Constrainer + ImplGraphMut> RemoveVertex for ReverseGraph<C>
	where C::Graph: RemoveVertex<Directedness=Directed>
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.0.graph_mut().remove_vertex(v)
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for ReverseGraph<C>
	where C::Graph: AddEdge<Directedness=Directed>
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.0.graph_mut().add_edge_weighted((e.sink(), e.source(), e.weight_owned()))
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for ReverseGraph<C>
	where C::Graph: RemoveEdge<Directedness=Directed>
{
	fn remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.0.graph_mut().remove_edge_where(|e| f((e.sink(), e.source(), e.weight())))
	}
}

impl<C: Constrainer> ImplGraph for ReverseGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer + ImplGraphMut> ImplGraphMut for ReverseGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> BaseGraph for ReverseGraph<C>
	where C::Graph: Graph<Directedness=Directed> {}