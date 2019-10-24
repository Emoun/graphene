use crate::core::{Graph, Constrainer, ImplGraph, ImplGraphMut, GraphMut, NewVertex, AddEdge, EdgeWeighted, Directedness, Directed, RemoveVertex};

///
/// A marker trait for graphs who's esges are directed.
///
///
pub trait DirectedConstraint: Graph
{}

#[derive(Clone, Debug)]
pub struct DirectedGraph<C: Constrainer>(C);

impl<C: Constrainer> DirectedGraph<C>
{
	///
	/// Constrains the given graph.
	///
	/// The given graph must be unique. This is not checked by this function.
	///
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> ImplGraph for DirectedGraph<C> {
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for DirectedGraph<C>  {
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> Constrainer for DirectedGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		if <<C::Graph as Graph>::Directedness as Directedness>::directed() {
			Ok(Self::unchecked(g))
		} else {
			Err(())
		}
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}

impl<C: Constrainer> Graph for DirectedGraph<C>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Directedness = Directed;
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.0.graph().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.0.graph().all_edges()
	}
}

impl<C: Constrainer + ImplGraphMut>  GraphMut for DirectedGraph<C>
	where C::Graph: GraphMut
{
	fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.0.graph_mut().all_vertices_weighted_mut()
	}
	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.0.graph_mut().all_edges_mut()
	}
}

impl<C: Constrainer + ImplGraphMut> NewVertex for DirectedGraph<C>
	where C::Graph: NewVertex
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		self.0.graph_mut().new_vertex_weighted(w)
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveVertex for DirectedGraph<C>
	where C::Graph: RemoveVertex
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.0.graph_mut().remove_vertex(v)
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for DirectedGraph<C>
	where C::Graph: AddEdge
{
	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.0.graph_mut().remove_edge_where(f)
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer> DirectedConstraint for DirectedGraph<C>{}

impl_constraints!{
	DirectedGraph<C>: DirectedConstraint
}