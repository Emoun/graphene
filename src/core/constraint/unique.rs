use super::*;

///
/// A marker trait for graphs containing unique edges.
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
pub trait Unique<V,W,Vi,Ei>: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
{}

#[derive(Clone,Debug)]
pub struct UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		G: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	graph: G
}

impl<V,W,Vi,Ei,G> BaseGraph for UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		G: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	type Vertex = V;
	type Weight = W;
	type VertexIter = Vi;
	type EdgeIter = Ei;
	
	fn empty_graph() -> Self {
		UniqueGraph{graph: G::empty_graph()}
	}
	
	fn all_vertices(&self) -> Self::VertexIter {
		self.graph.all_vertices()
	}
	
	fn all_edges(&self) -> Self::EdgeIter {
		self.graph.all_edges()
	}
	
	fn add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.graph.add_vertex(v)
	}
	
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.graph.remove_vertex(v)
	}
	
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
	
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		self.graph.remove_edge(e)
	}
}

impl<V,W,Vi,Ei,G> ConstrainedGraph for UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
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
	
	unsafe fn uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.graph.uncon_add_vertex(v)
	}
	
	unsafe fn uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		self.graph.uncon_remove_vertex(v)
	}
	
	unsafe fn uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		self.graph.uncon_add_edge(e)
	}
	
	unsafe fn uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		self.graph.uncon_remove_edge(e)
	}
}