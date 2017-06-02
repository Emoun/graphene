use super::*;

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
pub trait BaseGraph<'a,>
{
	/// Type of the vertices in the graph.
	type Vertex: Copy + Eq;
	/// Type of the weights in the graph.
	type Weight: Copy + Eq;
	/// Type of the collection returned with vertices.
	type VertexIter: IntoIterator<Item=Self::Vertex>;
	// Type of the collection returned with edges.
	type EdgeIter: IntoIterator<Item=BaseEdge<Self::Vertex,Self::Weight>>;
	
	///
	/// Returns copies of all current vertices in the graph.
	///
	fn all_vertices(&'a self) -> Self::VertexIter;
	
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges(&'a self) -> Self::EdgeIter;
	
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
	fn add_vertex(&'a mut self, v: Self::Vertex) -> Result<(),()>;
	
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
	fn remove_vertex(&'a mut self, v: Self::Vertex) -> Result<(),()>;
	
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
	fn add_edge(&'a mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
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
	fn remove_edge(&'a mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
}

