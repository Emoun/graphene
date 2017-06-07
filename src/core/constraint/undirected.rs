use super::*;

///
/// A marker trait for graphs that is effectively undirected.
///
/// Formally, the trait guarantees that for any edge connecting two vertices , (v1,v2,w),
/// there is an edge connecting the same two vertices in the other directed, (v2,v1,w), with
/// the same weight.
/// All `BasicGraph` methods must upholds the undirected invariant. This means adding an edge
/// must automatically add an edge in the other direction, and removing one must also remove on in
/// the other direction.
///
/// Consumers of the trait must manually treat the two edges as one undirected edge where needed.
///
///
pub trait Undirected<V,W,Vi,Ei>: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
{}

///
/// A graph wrapper that enforces the `Undirected` constraint on any graph its given.
///
/// See <INSERT LINK TO `Undirected`> for a complete description.
///
#[derive(Clone,Debug)]
pub struct UndirectedGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	graph: G
}

impl<V,W,Vi,Ei,G> BaseGraph for UndirectedGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	type Vertex = V;
	type Weight = W;
	type VertexIter = Vi;
	type EdgeIter = Ei;
	
	fn empty_graph() -> Self {
		UndirectedGraph{graph: G::empty_graph()}
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
		unimplemented!()
	}
	
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		unimplemented!()
	}
}

impl<V,W,Vi,Ei,G> ConstrainedGraph for UndirectedGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	fn invariant_holds(&self) -> bool {
		unimplemented!()
	}
	
	unsafe fn uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		unimplemented!()
	}
	
	unsafe fn uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()> {
		unimplemented!()
	}
	
	unsafe fn uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		unimplemented!()
	}
	
	unsafe fn uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		unimplemented!()
	}
}

