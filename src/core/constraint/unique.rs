
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

///
/// A graph wrapper that enforces the `Unique` constraint on any graph its given.
///
/// See <INSERT LINK TO `Unique`> for a complete description.
///
#[derive(Clone,Debug)]
pub struct UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	graph: G
}

impl<V,W,Vi,Ei,G> Unique for UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{}

impl<V,W,Vi,Ei,G> BaseGraph for UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	type Vertex = V;
	type Weight = W;
	type VertexIter = Vi;
	type EdgeIter = Ei;
	
	fn empty_graph() -> Self {
		UniqueGraph{graph: G::empty_graph()}
	}
	
	wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
	wrap!{graph.all_edges(&self) -> Self::EdgeIter}
	
	wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		// If the edge is already present in the graph (ignoring weight)
		if let Some(_) = self.all_edges().into_iter().position(
			|edge| edge.source == e.source && edge.sink == e.sink )
		{
			// Disallow the addition
			return Err(());
		}
		self.graph.add_edge(e)
	}
	
	wrap!{graph.remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
}

impl<V,W,Vi,Ei,G> ConstrainedGraph for UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
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
		self.graph.invariant_holds()
	}
	wrap_uncon_methods!{graph}
}

