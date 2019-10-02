use crate::core::{Graph, EdgeWeighted, AddVertex, Constrainer, AddEdge, GraphMut};
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

pub struct NoLoopsGraph<G: Graph>(G);

//delegate_graph!{
//	NoLoopsGraph<G>
//	{

//	}
//}

impl<G: Graph> Graph for NoLoopsGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	delegate!{
		target self.0 {
			fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a Self::VertexWeight)>> ;
			
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>> ;
		}
	}
}

impl<G: GraphMut> GraphMut for NoLoopsGraph<G>
{
	delegate!{
		target self.0 {
			fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a mut Self::VertexWeight)>> ;
	
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>> ;
	
		}
	}
}

impl<G: AddVertex> AddVertex for NoLoopsGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
		}
	}
}

impl<G: AddEdge> AddEdge for NoLoopsGraph<G>
{
	delegate! {
		target self.0 {
			fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;

		}
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if e.source() == e.sink(){
			Err(())
		} else {
			self.0.add_edge_weighted(e)
		}
	}
}

impl<G: Graph> NoLoops for NoLoopsGraph<G>{}

impl_constraints!{
	NoLoopsGraph<G>: NoLoops
}

impl<C: Constrainer>  Constrainer for NoLoopsGraph<C>
{
	type BaseGraph = C::BaseGraph;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{

		if g.all_vertices()
			.any(|v| g.edges_between(v,v).next().is_some()){
			Err(())
		} else {
			Ok(NoLoopsGraph(g))
		}
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}