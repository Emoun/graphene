
use super::*;


///
/// A marker trait for graphs containing only unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting two vertices.
/// If the graph is directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2,_) and (v2,v1,_).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2,_) or (v1,v2,_).
/// Regardless of directedness, only one loop is allowed for each vertex,
/// i.e. only one (v,v,_).
///
///
///
pub trait Unique: ConstrainedGraph
	where
		<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{}

graph_wrapper!{
	///
	/// A graph wrapper that enforces the `Unique` constraint on any graph its given.
	///
	/// See <INSERT LINK TO `Unique`> for a complete description.
	///
	struct UniqueGraph
}
impl_constraints_for_wrapper!{UniqueGraph : Unique}

impl<G> BaseGraph for UniqueGraph<G>
	where
		G: ConstrainedGraph,
		<G as BaseGraph>::Vertex: Vertex,
		<G as BaseGraph>::Weight: Weight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	type Vertex = <G as BaseGraph>::Vertex;
	type Weight = <G as BaseGraph>::Weight;
	type VertexIter = <G as BaseGraph>::VertexIter;
	type EdgeIter = <G as BaseGraph>::EdgeIter;
	
	fn empty_graph() -> Self {
		UniqueGraph::wrap(G::empty_graph())
	}
	
	wrapped_method!{all_vertices(&self) -> Self::VertexIter}
	
	wrapped_method!{all_edges(&self) -> Self::EdgeIter}
	
	wrapped_method!{add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrapped_method!{remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		// If the edge is already present in the graph (ignoring weight)
		if let Some(_) = self.all_edges().into_iter().position(
			|edge| edge.source == e.source && edge.sink == e.sink )
		{
			// Disallow the addition
			return Err(());
		}
		self.wraps.add_edge(e)
	}
	
	wrapped_method!{remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
}

impl<G> ConstrainedGraph for UniqueGraph<G>
	where
		G: ConstrainedGraph,
		<G as BaseGraph>::Vertex: Vertex,
		<G as BaseGraph>::Weight: Weight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	fn invariant_holds(&self) -> bool {
		
		// For each vertex
		for v1 in self.all_vertices(){
			
			for v2 in self.all_vertices(){
				// Find all edges from v1 to v2
				let mut v1_to_v2 = self.all_edges().into_iter().filter(|edge|{
					edge.source == v1 && edge.sink == v2
				});
				
				// Make sure there is at most 1
				v1_to_v2.next();
				
				// If there is another one
				if let Some(_) = v1_to_v2.next(){
					// Invariant doesn't hold
					return false;
				}
			}
		}
		// Invariant holds, make sure the inner graph's invariant also holds
		self.wraps.invariant_holds()
	}
	wrapped_uncon_methods!{}
}

