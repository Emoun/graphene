
use crate::core::{Edge, EdgeWeighted, Directedness, trait_aliases::{
	Id,
}, Directed, Constrainer, BaseGraph};
use std::iter::Iterator;
use crate::core::constraint::{DirectedGraph, UndirectedGraph};

#[macro_use]
mod macros {
	#[macro_export]
	macro_rules! edges_between {
		($e:expr, $v1:expr, $v2:expr) => {
			{
				// Filter out any edge that is not connected to both vertices
				let relevant = $e.filter(move|edge|{
					(edge.source() == $v1 && edge.sink() == $v2) ||
						(edge.source() == $v2 && edge.sink() == $v1)
				});
				
				// Return the result
				Box::new(relevant)
			}
		}
	}
	#[macro_export]
	macro_rules! edges_incident_on {
		($e:expr, $v:expr, $i:ident) => {
			Box::new($e.into_iter().filter(move|e| e.$i() == $v))
		};
		($e:expr, $v:expr) => {
			Box::new($e.into_iter().filter(move|edge| (edge.source() == $v) || (edge.sink() == $v)))
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
/// TODO: When Rust supports 'impl trait' consider having some of the iterators be clone too
/// (those that don't include mutable references). This allows cloning the iterators mid-iteration,
/// which can be useful when comparing each item to the ones after it.
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
	fn all_vertices_weighted<'a>(&'a self)
		 -> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>;
	///
	/// Returns copies of all current edges in the graph.
	///
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
	
// Optional methods

	fn all_vertices<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=Self::Vertex>>
	{
		Box::new(self.all_vertices_weighted().map(|(v, _)| v))
	}
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>
	{
		self.all_vertices_weighted().find(|&(candidate,_)| candidate == v).map(|(_,w)| w)
	}
	fn contains_vertex(&self, v: Self::Vertex) -> bool
	{
		self.vertex_weight(v).is_some()
	}
	fn all_vertex_weights<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Self::VertexWeight>>
	{
		Box::new(self.all_vertices_weighted().map(|(_,w)| w))
	}
	
	///
	/// Returns all edges that are incident on both of the given vertices, regardless of direction.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	///
	fn edges_between<'a>(&'a self, v1: Self::Vertex, v2: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		edges_between!(self.all_edges(), v1, v2)
	}
	///
	/// Returns all edges that are sourced in the given vertex.
	///
	/// I.e. all edges where `e == (v,_,_)`
	///
	/// Only available for directed graphs
	///
	fn edges_sourced_in<'a>(&'a self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
		where Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges(), v, source)
	}
	///
	/// Returns all edges that are sinked in the given vertex.
	///
	/// I.e. all edges where `e == (_,v,_)`
	///
	fn edges_sinked_in<'a>(&'a self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
		where Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges(), v, sink)
	}
	fn edges_incident_on<'a>(&'a self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		edges_incident_on!(self.all_edges(),v)
	}
	
	fn constrain_directedness(&self) -> DirectednessVariants<&Self>
		where Self: BaseGraph
	{
		if let Ok(g) = self.constrain() {
			DirectednessVariants::Directed(g)
		} else {
			DirectednessVariants::Undirected(UndirectedGraph::unchecked(self))
		}
	}
	
	fn edge_valid<E>(&self, e: E) -> bool
		where E: Edge<Self::Vertex>
	{
		self.contains_vertex(e.source()) && self.contains_vertex(e.sink())
	}
}

pub trait GraphMut: Graph
{
	fn all_vertices_weighted_mut<'a>(&'a mut self)
									 -> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>;
	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
	(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
	
// Optional methods
	
	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		self.all_vertices_weighted_mut().find(|&(candidate,_)| candidate == v).map(|(_,w)| w)
	}
	
	fn edges_between_mut<'a>(&'a mut self, v1: Self::Vertex, v2: Self::Vertex)
		 -> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		edges_between!(self.all_edges_mut(), v1, v2)
	}
	fn edges_sourced_in_mut<'a>(&'a mut self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
		where Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges_mut(), v, source)
	}
	fn edges_sinked_in_mut<'a>(&'a mut self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
		where Self: Graph<Directedness=Directed>
	{
		edges_incident_on!(self.all_edges_mut(), v, sink)
	}
	fn edges_incident_on_mut<'a>(&'a mut self, v: Self::Vertex)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		edges_incident_on!(self.all_edges_mut(),v)
	}
	
	#[allow(unreachable_code)]
	fn constrain_directedness_mut(&mut self) -> DirectednessVariants<& mut Self>
		where Self: BaseGraph
	{
		unimplemented!("I suspect the below implementation to work, but needs review.");
		/*  We use this unsafe block to allow us to use 'self' in the 'else' branch below.
			This is safe because 'self' is not used by 'g' if the 'else' branch is taken and
			therefore the reference is not shared with anyone.
			However, the compiler cannot see this and there doesn't seem to a fix in the pipeline:
			https://github.com/rust-lang/rust/issues/53528
			
			If/when this issue is fixed, this code can be updated to remove the unsafe block.
		*/
		let self_2: &mut Self = unsafe {  (self as *mut Self).as_mut().unwrap() } ;
		
		if let Ok(g) = self.constrain() {
			DirectednessVariants::Directed(g)
		} else {
			DirectednessVariants::Undirected(UndirectedGraph::unchecked(self_2))
		}
	}
}

///
/// A graph where new vertices can be added
///
pub trait NewVertex: Graph
{
	///
	/// Adds a new vertex with the given weight to the graph.
	/// Returns the id of the new vertex.
	///
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex,()>;
	
// Optional methods
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
/// A graph where vertices can be removed.
///
pub trait RemoveVertex: Graph
{
	///
	/// Removes the given vertex from the graph, returning its weight.
	/// If the vertex still has edges incident on it, they are also removed,
	/// dropping their weights.
	///
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>;
}

pub trait AddEdge: Graph
{
	///
	/// Adds a copy of the given edge to the graph
	///
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
	
// Optional methods

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
	
}

pub trait RemoveEdge: Graph {
	///
	/// Removes an edge that matches the given predicate closure.
	/// If no edge is found to match and successfully removed, returns error
	/// but otherwise doesn't change the graph.
	///
	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
	
// Optional methods
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
		self.remove_edge_where(|(so,si, w)| f(w) &&
			((so == e.source()) && (si == e.sink()) ||
				(!Self::Directedness::directed() && (so == e.sink()) && (si == e.source()))) )
			.map(|removed_edge| removed_edge.2)
	}
	
	///
	/// Tries to removes all edges that match the given predicate.
	/// Stops either when no more edges match, or an edge that matched couldn't be removed.
	///
	/// Returns the removed edges regardless.
	///
	fn remove_edge_where_all<F>(&mut self, f: F) -> Vec<(Self::Vertex, Self::Vertex, Self::EdgeWeight)>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		let mut result = Vec::new();
		while let Ok(e) = self.remove_edge_where(|e| f(e)) {
			result.push(e);
		}
		result
	}
}

///
/// Graph that at all times has a finite set of vertices and edges.
///
/// TODO: redesign. What about edges? even through there are less than usize::MAX vertices
/// there can easily be be more than usize::MAX edges. Maybe this should be 2 traits.
///
pub trait ExactGraph: Graph
{
	
	///
	/// Returns the number of vertices in the graph.
	///
	fn vertex_count(&self) -> usize {
		self.all_vertices().count()
	}
	
	///
	/// Returns the number of edges in the graph.
	///
	fn edge_count(&self) -> usize {
		self.all_edges().count()
	}
}

pub enum DirectednessVariants<C: Constrainer>
{
	Directed(DirectedGraph<C>),
	Undirected(UndirectedGraph<C>),
}