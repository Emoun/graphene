
use core::{
	Edge,
	trait_aliases::{
		Id, IntoFromIter
	}
};

///
/// The basic graph interface. This is the main trait for all types of graphs.
///
/// For all graphs, vertices are identified by their [value](#associatedType.Vertex), and must be unique
/// in the graph. Edges are identified by the two vertices they are incident on, the direction,
/// and their [Id](#associatedType.EdgeId); I.e. the triple `(v1,v2,e)` is an edge from `v1` to `v2`
/// with an edge id `e`. All edges are directed but need not be unique. Therefore, multiple edges with
/// the same source (`v1`), sink (`v2`), and id (`e`) may be present in a graph at the same time.
/// Edges with the same source, sink, and id are identical and must be interchangeable. E.g. if any
/// one of two or more identical edges is to be removed, then any one of them may be removed.
///
pub trait BaseGraph
{
	///
	/// Type of the graph's vertex value.
	///
	type Vertex: Id;
	///
	/// Type of the graph's edge id.
	///
	type EdgeId: Id;
	
	/// Type of the collection returned with vertices.
	type VertexIter: IntoFromIter<Self::Vertex>;
	/// Type of the collection returned with edges.
	type EdgeIter: IntoFromIter<(Self::Vertex, Self::Vertex, Self::EdgeId)>;
	
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
	/// Returns copies of all current vertex values in the graph.
	///
	fn all_vertices(&self) -> Self::VertexIter;
	
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges(&self) -> Self::EdgeIter;
	
	///
	/// Adds the given vertex to graph as long as no identical vertex is already
	/// present i the graph and the graph is capable of storing it.
	///
	/// ###Returns:
	///
	/// - `Ok` if the vertex is valid and has been added.
	/// - `Err` if the vertex is already present in the graph or
	/// it is otherwise unable to store it.
	///
	/// ###`Ok` properties :
	///
	/// - All vertices present before the call are also present after it.
	/// - All edges present before the call are also present after it.
	/// - No edges are changed.
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
	/// Removes the given vertex from the graph along with any incident edges.
	///
	/// ###Returns:
	///
	/// - `Ok` if the vertex was removed.
	/// - `Err` if the vertex was not present in the graph or it was otherwise unable to remove it.
	///
	/// ###`Ok` properties:
	///
	/// - Only the given vertex is removed from the graph.
	/// - Any edge incident on the vertex is also removed.
	/// - All other edges are unchanged.
	/// - No new vertices are introduced.
	/// - No new edges are introduced.
	/// - No edges are changed.
	///
	/// ###`Err` properties:
	///
	/// - The graph is unchanged.
	///
	///
	///
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<(),()>;
	
	///
	/// Adds a copy of the given edge to the graph, regardless of whether there are existing
	/// identical edges in the graph.
	/// The vertices the new edge is incident on must exist in the graph and the id must be valid.
	///
	/// ###Returns
	/// - `Ok` if the edge is valid and was added to the graph.
	/// - `Err` if the edge is invalid or the graph was otherwise unable to store it.
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
	fn add_edge_copy<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>;
	
	///
	/// Removes the given edge from the graph if it exists.
	///
	/// ###Returns
	/// - `Ok` if the edge was present before the call and was removed.
	/// - `Err` if the edge was not found in the graph or it was otherwise unable to remove it.
	///
	/// ###`Ok` properties:
	///
	/// - One edge identical to the given edge is removed.
	/// - No new edges are introduced.
	/// - No edges are changed.
	/// - No new vertices are introduced or removed.
	///
	/// ###`Err` properties:
	///
	/// - The graph is unchanged.
	///
	fn remove_edge<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, Self::EdgeId>;
	
	///
	/// Returns all edges that are incident on both the given vertices, regardless of direction.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	///
	fn edges_between(&self, v1: Self::Vertex, v2: Self::Vertex) -> Self::EdgeIter
	{
		let all_edges = self.all_edges().into_iter();
		
		// Filter out any edge that is not connected to both vertices
		let relevant = all_edges.filter(|edge|{
			(*edge.source() == v1 && *edge.sink() == v2) ||
				(*edge.source() == v2 && *edge.sink() == v1)
		});
		
		// Return the result
		relevant.collect()
	}
	
	///
	/// Returns all edges that are sourced in the given vertex.
	///
	/// I.e. all edges where `e == (v,_,_)`
	///
	fn edges_sourced_in(&self, v: Self::Vertex) -> Self::EdgeIter
	{
		self.all_edges().into_iter().filter(|e| *e.source() == v).collect::<Self::EdgeIter>()
	}
	
	///
	/// Returns all edges that are sinked in the given vertex.
	///
	/// I.e. all edges where `e == (_,v,_)`
	///
	fn edges_sinked_in(&self, v: Self::Vertex) -> Self::EdgeIter
	{
		self.all_edges().into_iter().filter(|e| *e.sink() == v).collect::<Self::EdgeIter>()
	}
	
}
