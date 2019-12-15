use crate::core::{Graph, EdgeWeighted, Constrainer, GraphMut, AddEdge, ImplGraph, ImplGraphMut, ReverseGraph, RemoveVertex, RemoveEdge};
use crate::algo::DFS;
use crate::core::constraint::{DirectedGraph, UnilaterallyConnected, proxy_remove_edge_where, proxy_remove_vertex};

///
/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is a path from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
///
pub trait Connected: UnilaterallyConnected
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

impl<C: Constrainer> ImplGraph for ConnectedGraph<C> {
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for ConnectedGraph<C>  {
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> Constrainer for ConnectedGraph<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>{
		let g = c.graph();
		let v_count = g.all_vertices().count();
		
		if v_count > 0 {
			let v = g.all_vertices().next().unwrap();
			let dfs_count = DFS::new(g, v).count();
			if dfs_count == v_count {
				// If its undirected, no more needs to be done
				if let Ok(g) = <DirectedGraph<&C::Graph>>::constrain(g) {
					let reverse = ReverseGraph::new(g);
					if DFS::new(&reverse, v).count() != v_count {
						return Err(())
					}
				}
				return Ok(Self(c))
			}
			return Err(())
		}
		Ok(Self(c))
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}

impl<C: Constrainer> Graph for ConnectedGraph<C>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Directedness = <C::Graph as Graph>::Directedness;
	
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

impl<C: Constrainer + ImplGraphMut> GraphMut for ConnectedGraph<C>
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

impl<C: Constrainer + ImplGraphMut> RemoveVertex for ConnectedGraph<C>
	where C::Graph: RemoveVertex
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<ConnectedGraph<_>,_>(self.0.graph_mut(), v)
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for ConnectedGraph<C>
	where C::Graph: AddEdge
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for ConnectedGraph<C>
	where C::Graph: RemoveEdge
{
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		proxy_remove_edge_where::<ConnectedGraph<_>,_,_>(self.0.graph_mut(), f)
	}
}

impl<C: Constrainer> UnilaterallyConnected for ConnectedGraph<C>{}
impl<C: Constrainer> Connected for ConnectedGraph<C>{}

impl_constraints!{
	ConnectedGraph<C>: Connected, UnilaterallyConnected
}

