use crate::core::{Graph, EdgeWeighted, NewVertex, Constrainer, AddEdge, GraphMut, ImplGraph, ImplGraphMut, RemoveVertex, RemoveEdge};
use delegate::delegate;

///
/// A marker trait for graphs containing no graph loops.
///
/// In graph theory, a loop is an edge that connects a vertex to itself.
/// This trait guarantees that there are no loops in the graph and that no loops
/// can be added to it.
///
pub trait NoLoops: Graph
{
	fn no_loops_func(&self){}
	
}

pub struct NoLoopsGraph<C: Constrainer>(C);

impl<C: Constrainer> ImplGraph for NoLoopsGraph<C> {
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for NoLoopsGraph<C>  {
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> Constrainer for NoLoopsGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>{
		if c.graph().all_vertices()
			.any(|v| c.graph().edges_between(v,v).next().is_some()){
			Err(())
		} else {
			Ok(NoLoopsGraph(c))
		}
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}

impl<C: Constrainer> Graph for NoLoopsGraph<C>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Directedness = <C::Graph as Graph>::Directedness;
	
	delegate!{
		to self.0.graph() {
			fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a Self::VertexWeight)>>;
				
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut>  GraphMut for NoLoopsGraph<C>
	where C::Graph: GraphMut
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a mut Self::VertexWeight)>>;
				
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> NewVertex for NoLoopsGraph<C>
	where C::Graph: NewVertex
{
	delegate! {
		to self.0.graph_mut() {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveVertex for NoLoopsGraph<C>
	where C::Graph: RemoveVertex
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for NoLoopsGraph<C>
	where C::Graph: AddEdge
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if e.source() == e.sink(){
			Err(())
		} else {
			self.0.graph_mut().add_edge_weighted(e)
		}
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for NoLoopsGraph<C>
	where C::Graph: RemoveEdge
{
	delegate! {
		to self.0.graph_mut() {
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
		}
	}
}

impl<C: Constrainer> NoLoops for NoLoopsGraph<C>{}

impl_constraints!{
	NoLoopsGraph<C>: NoLoops
}
