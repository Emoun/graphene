use crate::core::{Graph, EdgeWeighted, AutoGraph, Constrainer, BaseGraph};
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

impl<G: Graph> Graph for NoLoopsGraph<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	
	delegate! {
		target self.0 {
	
			fn all_vertices_weighted<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>;
				
			fn all_vertices_weighted_mut<'a>(&'a mut self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
			
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
			
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
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

impl<G: AutoGraph> AutoGraph for NoLoopsGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
		}
	}
}

impl<G: Graph> NoLoops for NoLoopsGraph<G>{}

impl_constraints!{
	NoLoopsGraph<G>: NoLoops
}

impl<B, C>  Constrainer for NoLoopsGraph<C>
	where B: BaseGraph, C: Graph + Constrainer<BaseGraph=B>
{
	type BaseGraph = B;
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