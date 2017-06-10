//!
//! Testing for graphene::core.
//!
//! All tests will use the GraphMock struct to test default implementations.
//! There is no need to test the required implementation, as the graph GraphMock wraps
//! is assumed to be tested in that regard.
//!
//!
//!
//!
use arbitraries::*;
use graphene::core::*;
use graphene::common::*;
use quickcheck::*;
use self::utilities::*;

mod base_graph;
mod utilities;


///
/// Used to test the default implementations of Graph traits.
///
/// Wraps AdjListGraph which provides an implementation for the required methods
/// while the rest of the methods use the default implementation even though AdjListGraph
/// may have its own.
///
///
pub struct GraphMock<V,W>{
	graph: AdjListGraph<V,W>
}

impl<V,W> BaseGraph for GraphMock<V,W>
	where
		V: Vertex,
		W: Weight,
{
	type Vertex = V;
	type Weight = W;
	type VertexIter = <AdjListGraph<V,W> as BaseGraph>::VertexIter;
	type EdgeIter = <AdjListGraph<V,W> as BaseGraph>::EdgeIter;
	
	fn empty_graph() -> Self {
		GraphMock{graph: AdjListGraph::empty_graph()}
	}
	
	wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
	wrap!{graph.all_edges(&self) -> Self::EdgeIter }
	
	wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> }
	
	wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> }
	
	wrap!{graph.remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> }
}