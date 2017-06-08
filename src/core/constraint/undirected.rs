use super::*;

///
/// A marker trait for graphs that are effectively undirected.
///
/// Formally, the trait guarantees that for any edge connecting two vertices , (v1,v2,w),
/// there is an edge connecting the same two vertices in the other direction, (v2,v1,w), with
/// the same weight.
///
/// All implementations must upholds the undirected invariant by assuming any edge it receives
/// as a parameter is undirected and is therefore equal to receiving the two corresponding directed
/// edges. When the implementer outputs edges it must be in the directed pair form. I.e for every
/// undirected edge (v1,v2,w) in the undirected graph, outputs must provide the two directed edges
/// (v1,v2,w) and (v2,v1,w).
///
/// All consumers of this trait specifically, must handle the input to and output from the graph
/// in a way consistent with the above specification.
///
/// It is the responsibility of the owner of the graph to make sure that any method
/// which does not specifically require a `Undirected` graph can logically handle it.
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
	
	wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
	wrap!{graph.all_edges(&self) -> Self::EdgeIter}
	
	wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		unsafe {
			self.uncon_add_edge(e)?;
			self.uncon_add_edge(BaseEdge::new(e.sink, e.source, e.weight))?;
			Ok(())
		}
	}
	
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		unsafe {
			self.uncon_remove_edge(e)?;
			self.uncon_remove_edge(BaseEdge::new(e.sink, e.source, e.weight))?;
			Ok(())
		}
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
	
	
	
		
	}
	
	wrap_uncon_methods!{graph}
}

