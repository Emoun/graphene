use crate::core::{Graph, EdgeWeighted, AddVertex, Constrainer, AddEdge, GraphMut, ConstrainerMut, BaseGraph};


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

impl<C: Constrainer> Constrainer for NoLoopsGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>{
		if c.base().all_vertices()
			.any(|v| c.base().edges_between(v,v).next().is_some()){
			Err(())
		} else {
			Ok(NoLoopsGraph(c))
		}
	}
	
	fn constrained(&self) -> &Self::Constrained {
		&self.0
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
impl<C: ConstrainerMut> ConstrainerMut for NoLoopsGraph<C>
{
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut {
		&mut self.0
	}
}

impl<C: Constrainer> Graph for NoLoopsGraph<C>
{
	type Vertex = <<C::Base as BaseGraph>::Graph as Graph>::Vertex;
	type VertexWeight = <<C::Base as BaseGraph>::Graph as Graph>::VertexWeight;
	type EdgeWeight = <<C::Base as BaseGraph>::Graph as Graph>::EdgeWeight;
	type Directedness = <<C::Base as BaseGraph>::Graph as Graph>::Directedness;
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.base().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.base().all_edges()
	}
}

impl<C: ConstrainerMut>  GraphMut for NoLoopsGraph<C>
	where <C::Base as BaseGraph>::Graph: GraphMut
{
	fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.base_mut().all_vertices_weighted_mut()
	}
	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.base_mut().all_edges_mut()
	}
}

impl<C: ConstrainerMut> AddVertex for NoLoopsGraph<C>
	where <C::Base as BaseGraph>::Graph: AddVertex
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
		-> Result<Self::Vertex, ()>
	{
		self.base_mut().new_vertex_weighted(w)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.base_mut().remove_vertex(v)
	}
}

impl<C: ConstrainerMut> AddEdge for NoLoopsGraph<C>
	where <C::Base as BaseGraph>::Graph: AddEdge
{
	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.base_mut().remove_edge_where(f)
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if e.source() == e.sink(){
			Err(())
		} else {
			self.base_mut().add_edge_weighted(e)
		}
	}
}

impl<C: Constrainer> NoLoops for NoLoopsGraph<C>{}

impl_constraints!{
	NoLoopsGraph<C>: NoLoops
}
