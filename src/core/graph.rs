
use crate::core::{Edge, EdgeWeighted, Directedness, trait_aliases::{
	Id, IntoFromIter, EdgeIntoFromIter, EdgeIntoFromIterMut
}, Directed};

use std::iter::Iterator;

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
			$e.into_iter().filter(|edge| (edge.source() == $v) || (edge.sink() == $v)).collect()
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
pub trait Graph
{
	///
	/// Type of the graph's vertex value.
	///
	type Vertex: Id;
	type VertexWeight;
	type EdgeWeight;
	type Directedness: Directedness;
	
	///
	/// Returns copies of all current vertex values in the graph.
	///
	fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I;
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>;
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
	///
	/// Removes the given vertex from the graph, returning its weight.
	/// If the vertex still has edges incident on it, they are also removed,
	/// dropping their weights.
	///
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>;
	
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges<'a, I>(&'a self) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>;
	fn all_edges_mut<'a, I>(&'a mut self) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>;
	///
	///
	/// TODO: Remove all edges that match, or just 1?
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
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
	/// Returns all edges that are incident on both of the given vertices, regardless of direction.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	///
	fn edges_between<'a, I>(&'a self, v1: Self::Vertex, v2: Self::Vertex) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>
	{
		edges_between!(self.all_edges::<I>(), v1, v2)
	}
	fn edges_between_mut<'a, I>(&'a mut self, v1: Self::Vertex, v2: Self::Vertex) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>
	{
		edges_between!(self.all_edges_mut::<I>(), v1, v2)
	}
	///
	/// Returns all edges that are sourced in the given vertex.
	///
	/// I.e. all edges where `e == (v,_,_)`
	///
	/// Only available for directed graphs
	///
	fn edges_sourced_in<'a, I>(&'a self, v: Self::Vertex) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>,
			  Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges::<I>(), v, source)
	}
	fn edges_sourced_in_mut<'a, I>(&'a mut self, v: Self::Vertex) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>,
			  Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges_mut::<I>(), v, source)
	}
	///
	/// Returns all edges that are sinked in the given vertex.
	///
	/// I.e. all edges where `e == (_,v,_)`
	///
	fn edges_sinked_in<'a, I>(&'a self, v: Self::Vertex) ->  I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>,
			  Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges::<I>(), v, sink)
	}
	fn edges_sinked_in_mut<'a, I>(&'a mut self, v: Self::Vertex) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>,
			  Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges_mut::<I>(), v, sink)
	}
	fn edges_incident_on<'a, I>(&'a self, v: Self::Vertex) -> I
		where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>
	{
		edges_incident_on!(self.all_edges::<I>(),v)
	}
	fn edges_incident_on_mut<'a, I>(&'a mut self, v: Self::Vertex) -> I
		where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>
	{
		edges_incident_on!(self.all_edges_mut::<I>(),v)
	}
}

///
/// A graph where the vertex ids can be provided by the user.
///
///
pub trait ManualGraph: Graph
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
pub trait AutoGraph: Graph
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
/// Graph that at all times has a finite set of vertices and edges.
///
pub trait ExactGraph: Graph
{
	
	///
	/// Returns the number of vertices in the graph.
	///
	fn vertex_count(&self) -> usize {
		self.all_vertices::<Vec<_>>().into_iter().count()
	}
	
	///
	/// Returns the number of edges in the graph.
	///
	fn edge_count(&self) -> usize {
		self.all_edges::<Vec<_>>().into_iter().count()
	}
}