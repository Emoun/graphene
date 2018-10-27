
use core::{
	Edge, EdgeWeighted,
	trait_aliases::{
		Id, IntoFromIter
	}
};

#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! edges_between {
		($e:expr, $v1:expr, $v2:expr) => {
			{
				let all_edges = $e.into_iter();
			
				// Filter out any edge that is not connected to both vertices
				let relevant = all_edges.filter(|edge|{
					(edge.source() == $v1 && edge.sink() == $v2) ||
						(edge.source() == $v2 && edge.sink() == $v1)
				});
				
				// Return the result
				relevant.collect()
			}
		}
	}
	#[macro_export]
	macro_rules! edges_incident_on {
		($e:expr, $v:expr, $i:ident) => {
			$e.into_iter().filter(|e| e.$i() == $v).collect()
		};
		($e:expr, $v:expr) => {
			$e.into_iter().filter(|(so,si,_)| (*so == $v) || (*si == $v)).collect()
		}
	}
}


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
pub trait Graph<'a>
{
	///
	/// Type of the graph's vertex value.
	///
	type Vertex: Id;
	type VertexWeight;
	type EdgeWeight:'a;
	/// Type of the collection returned with vertices.
	type VertexIter: IntoFromIter<Self::Vertex>;
	/// Type of the collection returned with edges.
	type EdgeIter: IntoFromIter<(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>;
	type EdgeMutIter: IntoFromIter<(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>;
	
	///
	/// Returns copies of all current vertex values in the graph.
	///
	fn all_vertices(&self) -> Self::VertexIter;
	
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges(&'a self) -> Self::EdgeIter;
	fn all_edges_mut(&'a mut self) -> Self::EdgeMutIter;
	
	///
	/// Removes the given vertex from the graph.
	/// If the vertex still has edges incident on it, no changes are made and an error is returned.
	///
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>;
	
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>;
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	;
	
	///
	/// Adds a copy of the given edge to the graph
	///
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
	
	///
	/// Adds the given edge to the graph, regardless of whether there are existing,
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
	fn add_edge<E>(&mut self, e: E) -> Result<(),()>
		where
			E: Edge<Self::Vertex>,
			Self::EdgeWeight: Default,
	{
		self.add_edge_weighted((e.source(), e.sink(), Self::EdgeWeight::default()))
	}
	
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
	fn remove_edge<E>(&mut self, e: E) -> Result<Self::EdgeWeight,()>
		where E: Edge<Self::Vertex>
	{
		self.remove_edge_where_weight(e, |_| true)
	}
	
	fn remove_edge_where_weight<E,F>(&mut self, e: E, f: F) -> Result<Self::EdgeWeight,()>
		where
			E: Edge<Self::Vertex>,
			F: Fn(&Self::EdgeWeight) -> bool,
	{
		self.remove_edge_where(|(so,si, w)|
			(so == e.source()) && (si == e.sink()) && f(w))
			.map(|removed_edge| removed_edge.2)
	}
	
	///
	/// Returns all edges that are incident on both the given vertices, regardless of direction.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	///
	fn edges_between(&'a self, v1: Self::Vertex, v2: Self::Vertex) -> Self::EdgeIter
	{
		edges_between!(self.all_edges(), v1, v2)
	}
	fn edges_between_mut(&'a mut self, v1: Self::Vertex, v2: Self::Vertex) -> Self::EdgeMutIter
	{
		edges_between!(self.all_edges_mut(), v1, v2)
	}
	
	///
	/// Returns all edges that are sourced in the given vertex.
	///
	/// I.e. all edges where `e == (v,_,_)`
	///
	fn edges_sourced_in(&'a self, v: Self::Vertex) -> Self::EdgeIter
	{
		edges_incident_on!(self.all_edges(), v, source)
	}
	fn edges_sourced_in_mut(&'a mut self, v: Self::Vertex) -> Self::EdgeMutIter
	{
		edges_incident_on!(self.all_edges_mut(), v, source)
	}
	
	///
	/// Returns all edges that are sinked in the given vertex.
	///
	/// I.e. all edges where `e == (_,v,_)`
	///
	fn edges_sinked_in(&'a self, v: Self::Vertex) -> Self::EdgeIter
	{
		edges_incident_on!(self.all_edges(), v, sink)
	}
	fn edges_sinked_in_mut(&'a mut self, v: Self::Vertex) -> Self::EdgeMutIter
	{
		edges_incident_on!(self.all_edges_mut(), v, sink)
	}
	
	fn edges_incident_on(&'a self, v: Self::Vertex) -> Self::EdgeIter
	{
		edges_incident_on!(self.all_edges(),v)
	}
	fn edges_incident_on_mut(&'a mut self, v: Self::Vertex) -> Self::EdgeMutIter
	{
		edges_incident_on!(self.all_edges_mut(),v)
	}
	
	/* Currently not implementable.
		Explanation: https://stackoverflow.com/questions/38713228/cannot-borrow-variable-when-borrower-scope-ends
		When generic associated types are introduced to Rust, the 'a lifetime can be removed
		as generic to Graph (i.e. Graph<'a>) and instead be generic on EdgeIter and EdgeIterMut.
		Luckily, until then, this can be implemented by the user as:
		let mut to_remove = Vec::new();
		g.edges_sourced_in(m0).into_iter().for_each(|(so,si,_)| to_remove.push((so,si)));
		g.edges_sinked_in(m0).into_iter().for_each(|(so,si,_)| to_remove.push((so,si)));
		to_remove.iter().for_each(|&e| {g.remove_edge(e).unwrap();});
		g.remove_vertex(m0).unwrap();
	///
	/// Removes the given vertex. If there are edges incident on it, they are removed too.
	/// Returns the weight of the removed vertex.
	///
	fn remove_vertex_forced(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>
	{
		let mut edges_to_remove: Vec<(Self::Vertex, Self::Vertex)> = Vec::new();
		{
			let sourced = self.edges_sourced_in(v).into_iter();
			for (so,si,_) in sourced {
				edges_to_remove.push((so.clone(),si.clone()));
			}
		}
		self.edges_sinked_in(v).into_iter().for_each(|(so, si, _)| edges_to_remove.push((so, si)));
		edges_to_remove.iter().for_each(|e| { self.remove_edge(*e).unwrap(); });
		
		for &e in edges_to_remove.iter() {
			self.remove_edge(e).unwrap();
		}
		
		self.remove_vertex(v)
	}*/
}

///
/// A graph where the vertex ids can be provided by the user.
///
///
pub trait ManualGraph<'a>: Graph<'a>
{
	
	///
	/// Adds the given vertex to the graph with the given weight.
	///
	fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(),()>;
	
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
	fn add_vertex(&mut self, v: Self::Vertex) -> Result<(),()>
		where Self::VertexWeight: Default
	{
		self.add_vertex_weighted(v, Self::VertexWeight::default())
	}
	
}

///
/// A graph where the vertex ids are assigned automatically.
///
pub trait AutoGraph<'a>: Graph<'a>
{
	///
	/// Adds a new vertex with the given weight to the graph.
	/// Returns the id of the new vertex.
	///
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex,()>;
	
	///
	/// Adds a new vertex to the graph.
	/// Returns the id of the new vertex.
	/// The weight of the vertex is the default.
	///
	fn new_vertex(&mut self) -> Result<Self::Vertex,()>
		where Self::VertexWeight: Default
	{
		self.new_vertex_weighted(Self::VertexWeight::default())
	}
	
}

///
/// Graph that at all times has a finite set of vertices and edges that
/// can be counted.
///
pub trait ExactGraph<'a>: Graph<'a>
	where
		<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<Self::EdgeMutIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	
	///
	/// Returns the number of vertices in the graph.
	///
	fn vertex_count(&'a self) -> usize {
		self.all_vertices().into_iter().len()
	}
	
	///
	/// Returns the number of edges in the graph.
	///
	fn edge_count(&'a self) -> usize {
		self.all_edges().into_iter().len()
	}
	
}