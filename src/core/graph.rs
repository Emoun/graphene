use crate::core::{trait_aliases::Id, Directed, Directedness, Edge};
use std::iter::Iterator;

#[macro_use]
mod macros
{
	#[macro_export]
	macro_rules! edges_between {
		($e:expr, $v1:expr, $v2:expr) => {{
			// Filter out any edge that is not connected to both vertices
			let relevant = $e.filter(move |edge| {
				(edge.source() == $v1 && edge.sink() == $v2)
					|| (edge.source() == $v2 && edge.sink() == $v1)
			});

			// Return the result
			Box::new(relevant)
			}};
	}
	#[macro_export]
	macro_rules! edges_incident_on {
		($e:expr, $v:expr, $i:ident) => {
			Box::new($e.into_iter().filter(move |e| e.$i() == $v))
		};
		($e:expr, $v:expr) => {
			Box::new(
				$e.into_iter()
					.filter(move |edge| (edge.source() == $v) || (edge.sink() == $v)),
				)
		};
	}
}

/// The basic graph trait, providing vertex and edge inspection.
///
/// A graphs vertices are identified by the unique value of the associated type
/// [`Vertex`](#associatedtype.Vertex).
/// For example, [`Vertex`] could be `usize`, in which case every vertex is
/// identified by a unique integer value.
/// In addition to its identifier, each vertex has a weight of type
/// [`VertexWeight`]. A vertex's weight can be anything or can be omitted by
/// using `()`.
///
/// Edges are identified primarily by which two vertices they connect. They can
/// also have a weight (of type [`EdgeWeight`]) that can be anything, but may
/// also in some circumstances be used to differentiate between two edges that
/// connect the same vertices. The associated type [`Directedness`] defined
/// whether a graph is directed or undirected. If it is assigned to
/// [`Directed`](struct.Directed.html), we say the graph is directed, meaning
/// the order of vertices in the edge matter. `(v1,v2)` would be an edge
/// "pointing" from `v1` to `v2` but not the other way around, which means
/// `(v2,v1)` will always be seen as a strictly different edge.
/// When [`Directedness`] is assigned to [`Undirected`](struct.Undirected.html)
/// we say the graph is undirected and the order of vertices in an edge is not
/// important. `(v1,v2)` therefore connects `v1` and `v2` and is synonymous with
/// `(v2,v1)`.
///
/// TODO: When Rust supports 'impl trait' consider having some of the iterators
/// be clone too (those that don't include mutable references). This allows
/// cloning the iterators mid-iteration, which can be useful when comparing each
/// item to the ones after it.
///
/// ### Related
///
/// - [`GraphMut`](trait.GraphMut.html):
/// Provides methods for accessing weights through mutable references.
/// - [`NewVertex`](property/trait.NewVertex.html):
/// Provides methods for adding vertices to a graph.
/// - [`RemoveVertex`](property/trait.RemoveVertex.html):
/// Provides methods for removing vertices from a graph.
/// - [`AddEdge`](property/trait.AddEdge.html):
/// Provides methods for adding edges to a graph.
/// - [`RemoveEdge`](property/trait.RemoveEdge.html):
/// Provides methods for removing edges to a graph.
///
/// [`Vertex`]: #associatedtype.Vertex
/// [`VertexWeight`]: #associatedtype.VertexWeight
/// [`EdgeWeight`]: #associatedtype.EdgeWeight
/// [`Directedness`]: #associatedtype.Directedness
pub trait Graph
{
	/// Type of the graphs vertices.
	///
	/// This type should be lightweight, as its passed around by-value
	/// (therefore must implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)).
	/// Whether two vertices are equal is also a very common operation and
	/// should therefore also be light-weight.
	type Vertex: Id;

	/// The weight associated with each vertex.
	///
	/// `()` can be used if no weight is needed.
	type VertexWeight;

	/// The weight associated with each edge.
	///
	/// `()` can be used if no weight is needed.
	type EdgeWeight;

	/// Whether the graph is directed or not.
	///
	/// Only [`Directed`](struct.Directed.html) and
	/// [`Undirected`](struct.Undirected.html)
	/// are valid assignments. Using any other type is undefined behaviour.
	type Directedness: Directedness;

	/// Returns copies of all current vertex values in the graph.
	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>;
	/// Returns copies of all current edges in the graph.
	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;

	// Optional methods

	fn all_vertices<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::Vertex>>
	{
		Box::new(self.all_vertices_weighted().map(|(v, _)| v))
	}
	fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight>
	{
		self.all_vertices_weighted()
			.find(|&(candidate, _)| candidate == v)
			.map(|(_, w)| w)
	}
	fn contains_vertex(&self, v: Self::Vertex) -> bool
	{
		self.vertex_weight(v).is_some()
	}
	fn all_vertex_weights<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Self::VertexWeight>>
	{
		Box::new(self.all_vertices_weighted().map(|(_, w)| w))
	}

	/// Returns all edges that are incident on both of the given vertices,
	/// regardless of direction.
	///
	/// I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	fn edges_between<'a>(
		&'a self,
		v1: Self::Vertex,
		v2: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		edges_between!(self.all_edges(), v1, v2)
	}
	/// Returns all edges that are sourced in the given vertex.
	///
	/// I.e. all edges where `e == (v,_,_)`
	///
	/// Only available for directed graphs
	fn edges_sourced_in<'a>(
		&'a self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	where
		Self: Graph<Directedness = Directed>,
	{
		edges_incident_on!(self.all_edges(), v, source)
	}
	/// Returns all edges that are sinked in the given vertex.
	///
	/// I.e. all edges where `e == (_,v,_)`
	fn edges_sinked_in<'a>(
		&'a self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	where
		Self: Graph<Directedness = Directed>,
	{
		edges_incident_on!(self.all_edges(), v, sink)
	}
	fn edges_incident_on<'a>(
		&'a self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		edges_incident_on!(self.all_edges(), v)
	}

	fn edge_valid<E>(&self, e: E) -> bool
	where
		E: Edge<Self::Vertex>,
	{
		self.contains_vertex(e.source()) && self.contains_vertex(e.sink())
	}
}

/// A graph with mutable vertex and edge weights.
///
/// [`Graph`](trait.Graph.html) provides methods that return references to
/// vertex and edge weight. However, it can't provide mutable references to the.
/// This trait provides mutable variants of [`Graph`](trait.Graph.html)'s
/// methods plus some additional utilities.
pub trait GraphMut: Graph
{
	fn all_vertices_weighted_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>;

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;

	// Optional methods

	fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>
	{
		self.all_vertices_weighted_mut()
			.find(|&(candidate, _)| candidate == v)
			.map(|(_, w)| w)
	}

	fn edges_between_mut<'a>(
		&'a mut self,
		v1: Self::Vertex,
		v2: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		edges_between!(self.all_edges_mut(), v1, v2)
	}
	fn edges_sourced_in_mut<'a>(
		&'a mut self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	where
		Self: Graph<Directedness = Directed>,
	{
		edges_incident_on!(self.all_edges_mut(), v, source)
	}
	fn edges_sinked_in_mut<'a>(
		&'a mut self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	where
		Self: Graph<Directedness = Directed>,
	{
		edges_incident_on!(self.all_edges_mut(), v, sink)
	}
	fn edges_incident_on_mut<'a>(
		&'a mut self,
		v: Self::Vertex,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		edges_incident_on!(self.all_edges_mut(), v)
	}
}
