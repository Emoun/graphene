use super::*;
use std::iter::FromIterator;

pub trait Vertex: Copy + Eq{}
impl<T> Vertex for T
	where T: Copy + Eq
{}

pub trait Weight: Copy + Eq{}
impl<T> Weight for T
	where T: Copy + Eq
{}

pub trait VertexIter<V>: IntoIterator<Item=V> + FromIterator<V>
	where
		V: Vertex,
		Self::IntoIter : ExactSizeIterator<Item=V>
		
{}
impl<T,V> VertexIter<V> for T
	where
		T: IntoIterator<Item=V> + FromIterator<V>,
		T::IntoIter: ExactSizeIterator<Item=V>,
		V: Vertex,
{}

pub trait EdgeIter<V,W>: IntoIterator<Item=BaseEdge<V,W>> + FromIterator<BaseEdge<V,W>>
	where
		V: Vertex,
		W: Weight,
		Self::IntoIter : ExactSizeIterator<Item=BaseEdge<V,W>>
{}
impl<T,V,W> EdgeIter<V,W> for T
	where
		T: IntoIterator<Item=BaseEdge<V,W>> + FromIterator<BaseEdge<V,W>>,
		T::IntoIter: ExactSizeIterator<Item=BaseEdge<V,W>>,
		V: Vertex,
		W: Weight,
{}

///
/// The basic graph interface.
///
/// This is the main trait for all types graphs.
///
/// The vertices in a graph are identified by their value, and must therefore be unique
/// in the graph.
///
/// The edges in a graph are identified by the vertices they connect and their weight.
/// Edges do not have to be unique, but if there are duplicates (i.e. two or more edges connecting
/// the same vertices with the same weights) then any operation intended for one of the edges
/// may happen on any one of them. E.g. If one of the edges is to be removed, then any
/// one of them will be so.
///
///
///
///
///
pub trait BaseGraph
	where
		<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	/// Type of the vertices in the graph.
	type Vertex: Vertex;
	/// Type of the weights in the graph.
	type Weight: Weight;
	/// Type of the collection returned with vertices.
	type VertexIter: VertexIter<Self::Vertex>;
	// Type of the collection returned with edges.
	type EdgeIter: EdgeIter<Self::Vertex, Self::Weight>;
	
	///
	/// Creates an empty graph. I.e a graph with no vertices and no edges.
	///
	/// Properties:
	///
	/// - `empty().all_vertices().into_iter().next() == None`
	/// - `empty().all_edges().into_iter().next() == None`
	/// - `empty().add_vertex(v) == Ok(())`
	/// - `empty().add_edge(e) == Err(())`
	/// - `empty().remove_vertex(v) == Err(())`
	/// - `empty().remove_edge(e) == Err(())`
	///
	fn empty_graph() -> Self;
	
	///
	/// Returns copies of all current vertices in the graph.
	///
	fn all_vertices(&self) -> Self::VertexIter;
	
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges(&self) -> Self::EdgeIter;
	
	///
	/// Adds the given vertex to graph as long as no equal vertex is already
	/// present i the graph and the graph is capable of storing it.
	///
	/// ###Returns:
	///
	/// - `Ok` if the vertex is valid and has been added.
	/// - `Err` if the vertex is already present in the graph or
	/// the graph doesn't have capacity for it.
	///
	/// ###`Ok` properties :
	///
	/// - All vertices present before the call are also present after it.
	/// - All edges present before the call are also present after it.
	/// - No edge weights are changed.
	/// - No new edges are introduced.
	/// - Only the given vertex is added to the graph.
	///
	/// ###`Err` properties :
	///
	/// - The graph is unchanged.
	///
	///
	fn add_vertex(&mut self, v: Self::Vertex) -> Result<(),()>;
	
	///
	/// Removes the given vertex from the graph, assuming it is present.
	///
	/// ###Returns:
	///
	/// - `Ok` if the vertex was removed.
	/// - `Err` if the vertex was not present in the graph.
	///
	/// ###`Ok` properties:
	///
	/// - Only the given vertex is removed from the graph.
	/// - Any edge connecting to the removed vertex is also removed.
	/// - All other edges are unchanged.
	/// - No new vertices are introduced.
	/// - No new edges are introduced.
	/// - No edge weights are changed.
	///
	/// ###`Err` properties:
	///
	/// - The graph is unchanged.
	///
	///
	///
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<(),()>;
	
	///
	/// Adds the given edge to the graph assuming it connects to valid vertices.
	///
	/// ###Returns
	/// - `Ok` if the edge connects to valid vertices and the edge was added successfully.
	/// - `Err` if the edge connects to invalid vertices or was not added.
	///
	/// ###`Ok` properties:
	///
	/// - Only the given edge is added to the graph.
	/// - Existing edges are unchanged.
	/// - No vertices are introduced or removed.
	///
	/// ###`Err` properties:
	///
	/// - The graph is unchanged.
	///
	fn add_edge(&mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
	///
	/// Removes the given edge from the graph, assuming it is already present.
	///
	/// ###Returns
	/// - `Ok` if the edge was present before the call and was removed successfully.
	/// - `Err` if the edge was not found in the graph.
	///
	/// ###`Ok` properties:
	///
	/// - Only the given edge is removed.
	/// - No new edges are introduced.
	/// - No edge weights are changed.
	/// - No new vertices are introduced or removed.
	///
	/// ###`Err` properties:
	///
	/// - The graph is unchanged.
	///
	fn remove_edge(&mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
	///
	/// Creates a graph containing the given vertices and edges. There can be no
	/// duplicate vertices and all edges must connect to the given vertices.
	///
	/// ###Returns:
	///
	/// - `Ok`: If the given graph description is valid, the created graph is returned.
	/// - `Err`: If the given graph description is invalid.
	///
	fn graph(	vertices: Vec<Self::Vertex>,
			 	edges: Vec<(Self::Vertex, Self::Vertex,Self::Weight)>)
		-> Result<Self,()>
		where
			Self: Sized,
	{
		let mut g = Self::empty_graph();
		
		/* Add all vertices
		 */
		for v in vertices {
			//Make sure the vertex is added
			g.add_vertex(v)?;
		}
		
		/* Add all edges
		 */
		for (so,si,w) in edges {
			// Make sure the edge is added
			g.add_edge(BaseEdge::new(so,si,w))?;
		}
		
		Ok(g)
	}
	
	///
	/// Returns all edges that are connect to both the given vertices.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	///
	fn edges_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Self::EdgeIter{
		let all_edges = self.all_edges().into_iter();
		
		// Filter out any edge that is not connected to both vertices
		let relevant = all_edges.filter(|edge|{
			(edge.source == v1 && edge.sink == v2) ||
				(edge.source == v2 && edge.sink == v1)
		});
		
		// Return the result
		relevant.collect()
	}
	
}

