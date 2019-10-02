use crate::core::{Graph, EdgeWeighted, AddVertex, Constrainer, GraphMut, AddEdge};
use delegate::delegate;

///
/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is  apath from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
///
pub trait Connected: Graph
{}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<G: Graph>(G);

impl<G:Graph> ConnectedGraph<G>
{
	///
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	///
	pub fn new(g: G) -> Self
	{
		Self(g)
	}
}

impl<G: Graph> Graph for ConnectedGraph<G>
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

impl<G: GraphMut> GraphMut for ConnectedGraph<G>
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

impl<G: AddVertex> AddVertex for ConnectedGraph<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
			
		}
	}
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		Err(())
	}
}

impl<G: AddEdge> AddEdge for ConnectedGraph<G>
{
	delegate! {
		target self.0 {
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
	fn remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		unimplemented!()
	}
}

impl<G: Graph> Connected for ConnectedGraph<G>{}

impl_constraints!{
	ConnectedGraph<G>: Connected
}

impl<C: Constrainer> Constrainer for ConnectedGraph<C>
{
	type BaseGraph = C::BaseGraph;
	type Constrained = C;
	
	fn constrain_single(_: Self::Constrained) -> Result<Self, ()>{
		unimplemented!()
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}