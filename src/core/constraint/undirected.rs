use super::*;

///
/// A marker trait for graphs that are effectively undirected.
///
/// Formally, the trait guarantees that for any edge connecting two vertices , (v1,v2,w),
/// there is an edge connecting the same two vertices in the other direction, (v2,v1,w), with
/// the same weight.
/// This is not the case for loops, i.e. a loop is automatically considered as undirected.
///
/// All implementations must upholds the undirected invariant by assuming any edge it receives
/// as a parameter is undirected and is therefore equal to receiving the two corresponding directed
/// edges. When the implementer outputs edges it must be in the directed pair form. I.e for every
/// undirected edge (v1,v2,w) in the undirected graph, outputs must provide the two directed edges
/// (v1,v2,w) and (v2,v1,w).
/// If the implementer receives a loop edge, it is considered intrinsically undirected and is
/// therefore only treated at a single edge. Likewise, a loop is only output once for each.
///
/// All consumers of this trait specifically, must handle the input to and output from the graph
/// in a way consistent with the above specification.
///
/// It is the responsibility of the owner of the graph to make sure that any method
/// which does not specifically require a `Undirected` graph can logically handle it.
///
pub trait Undirected: ConstrainedGraph
	where
		<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
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
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{
	pub graph: G
}

impl<V,W,Vi,Ei,G> Undirected for UndirectedGraph<V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>,
{}

impl<V,W,Vi,Ei,G> BaseGraph for UndirectedGraph<V,W,Vi,Ei,G>
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
		UndirectedGraph{graph: G::empty_graph()}
	}
	
	wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
	wrap!{graph.all_edges(&self) -> Self::EdgeIter}
	
	wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		let mut c = self.unconstrained().add_edge(e);
		if e.source != e.sink {
			c = c.add_edge(BaseEdge::new(e.sink, e.source, e.weight));
		}
		c.constrain()
	}
	
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()> {
		let mut c = self.unconstrained().remove_edge(e);
		if e.source != e.sink {
			c = c.remove_edge(BaseEdge::new(e.sink, e.source, e.weight));
		}
		c.constrain()
	}
}

impl<V,W,Vi,Ei,G> ConstrainedGraph for UndirectedGraph<V,W,Vi,Ei,G>
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
	
		let edges = self.all_edges().into_iter();
		
		/* Keep track of edges that have yet to be matched with
		 * a corresponding edge in the other direction
		 */
		let mut unmatched_edges: Vec<BaseEdge<V,W>> = Vec::new();
		
		// For each edge
		for e in edges{
			/* If the edge is a loop, it is intrinsically matched
			 */
			if e.source == e.sink {
				continue;
			}
			// If the edge can be matched with a previously unmatched edge
			if let Some(i) = unmatched_edges.iter().position(|temp| {
				temp.source == e.sink &&
					temp.sink == e.source &&
					temp.weight == e.weight
			} ){
				/* Discard both edges
				 */
				unmatched_edges.swap_remove(i);
			}else{
				// Add the edge to the unmatched set
				unmatched_edges.push(e);
			}
		}
		
		// The invariant holds if there are no unmatched edges left
		// and the wrapped graph's invariant holds
		unmatched_edges.is_empty() &&
			self.graph.invariant_holds()
	}
	
	wrap_uncon_methods!{graph}
}

