use crate::core::{Graph, EdgeWeighted, AddVertex, Constrainer, GraphMut, AddEdge, ConstrainerMut, BaseGraph};

///
/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is  apath from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
///
pub trait Connected: Graph
{}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<C: Constrainer>(C);

impl<C: Constrainer> ConnectedGraph<C>
{
	///
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	///
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> Constrainer for ConnectedGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(_: Self::Constrained) -> Result<Self, ()>{
		unimplemented!()
	}
	
	fn constrained(&self) -> &Self::Constrained {
		&self.0
	}
	
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}
impl<C: ConstrainerMut> ConstrainerMut for ConnectedGraph<C>
{
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut {
		&mut self.0
	}
}

impl<C: Constrainer> Graph for ConnectedGraph<C>
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

impl<C: ConstrainerMut> GraphMut for ConnectedGraph<C>
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

impl<C: ConstrainerMut> AddVertex for ConnectedGraph<C>
	where <C::Base as BaseGraph>::Graph: AddVertex
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
	   -> Result<Self::Vertex, ()>
	{
		self.base_mut().new_vertex_weighted(w)
	}
	
	fn remove_vertex(&mut self, _v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		Err(())
	}
}

impl<C: ConstrainerMut> AddEdge for ConnectedGraph<C>
	where <C::Base as BaseGraph>::Graph: AddEdge
{

	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.base_mut().add_edge_weighted(e)
	}

	fn remove_edge_where<F>(&mut self, _f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		unimplemented!()
	}
}

impl<C: Constrainer> Connected for ConnectedGraph<C>{}

impl_constraints!{
	ConnectedGraph<C>: Connected
}

