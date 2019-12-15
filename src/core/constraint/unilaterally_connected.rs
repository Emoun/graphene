use crate::core::{Directed, Graph, Constrainer, ImplGraph, ImplGraphMut, GraphMut, RemoveVertex, AddEdge, EdgeWeighted, RemoveEdge};
use crate::core::constraint::{proxy_remove_edge_where, proxy_remove_vertex};
use crate::algo::DFS;

///
/// A marker trait for graphs that are unilaterally connected.
///
/// A graph is unilaterally connected if, for each pair of vertices, there exists at least 1 path
/// from one of them to the other. This is contrasted with strongly
/// connected graphs, where there must exist a path from either to the other (i.e. a path in each
/// direction).
///
/// The distinction between unilaterally and strongly connected only exists for directed graphs,
/// for undirected ones, they are equal. For this reason, the companion constrainer graph
/// `UnilaterallyConnectedGraph` only allows directed graphs. For undirected graph, simply use
/// `ConnectedGraph`.
///
/// For type safety reasons, the trait itself does not restrict directedness.
///
pub trait UnilaterallyConnected: Graph
{}

#[derive(Clone, Debug)]
pub struct UnilaterallyConnectedGraph<C: Constrainer>(C)
	where C::Graph: Graph<Directedness=Directed>
;

impl<C: Constrainer> UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	///
	/// Creates a new unilaterally connected graph.
	/// The given graph *must* be unilaterally connected.
	/// This method does not check for this!!
	///
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Constrainer> ImplGraph for UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> Constrainer for UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(c: Self::Constrained) -> Result<Self, ()>{
		// TODO: An efficient algorithm: https://stackoverflow.com/questions/30642383/determine-if-a-graph-is-semi-connected-or-not
		
		let graph = c.graph();
		
		let verts = graph.all_vertices().collect::<Vec<_>>();
		let mut iter = verts.iter();
		
		while let Some(&v1) = iter.next() {
			let iter_rest = iter.clone();
			for &v2 in iter_rest {
				if !DFS::new(graph, v1).any(|v| v == v2) && !DFS::new(graph, v2).any(|v| v == v1) {
					return Err(())
				}
			}
		}
		
		Ok(Self::new(c))
	}
	
	fn unconstrain_single(self) -> Self::Constrained{
		self.0
	}
}

impl<C: Constrainer> Graph for UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
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

impl<C: Constrainer + ImplGraphMut> GraphMut for UnilaterallyConnectedGraph<C>
	where C::Graph: GraphMut<Directedness=Directed>
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

impl<C: Constrainer + ImplGraphMut> RemoveVertex for UnilaterallyConnectedGraph<C>
	where C::Graph: RemoveVertex<Directedness=Directed>
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<UnilaterallyConnectedGraph<_>,_>(self.0.graph_mut(), v)
	}
}

impl<C: Constrainer + ImplGraphMut> AddEdge for UnilaterallyConnectedGraph<C>
	where C::Graph: AddEdge<Directedness=Directed>
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer + ImplGraphMut> RemoveEdge for UnilaterallyConnectedGraph<C>
	where C::Graph: RemoveEdge<Directedness=Directed>
{
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		proxy_remove_edge_where::<UnilaterallyConnectedGraph<_>,_,_>(self.0.graph_mut(), f)
	}
}

impl<C: Constrainer> UnilaterallyConnected for UnilaterallyConnectedGraph<C>
	where C::Graph: Graph<Directedness=Directed>
{}

impl_constraints!{
	UnilaterallyConnectedGraph<C>: UnilaterallyConnected
	where C::Graph: Graph<Directedness=Directed>
}

