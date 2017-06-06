use super::*;

///
/// A marker trait for graphs containing unique edges.
///
/// An edge is unique if it is the only edge in the graph
/// connecting vertex v1 to vertex v2.
///
/// If the graph is also directed then between two vertices v1 and v2
/// two edges are allowed: (v1,v2) and (v2,v1).
/// If the graph is undirected, there may only be one edge of either
/// (v1,v2) or (v1,v2)
///
///
pub trait Unique<V,W,Vi,Ei>: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
	where
		V: Copy + Eq,
		W: Copy + Eq,
		Vi: IntoIterator<Item=V>,
		Ei: IntoIterator<Item=BaseEdge<V,W>>
{}

pub struct UniqueGraph<V,W,Vi,Ei,G>
	where
		V: Copy + Eq,
		W: Copy + Eq,
		Vi: IntoIterator<Item=V>,
		Ei: IntoIterator<Item=BaseEdge<V,W>>,
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