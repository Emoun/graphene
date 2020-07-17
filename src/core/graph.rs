use crate::core::{trait_aliases::Id, Directedness};
use std::{borrow::Borrow, iter::Iterator};

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

	type VertexRef: Clone + Borrow<Self::Vertex>;

	/// Returns copies of all current vertex values in the graph.
	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::VertexRef, &'a Self::VertexWeight)>>;
	/// Returns the weights of all edges that are sourced in v1 and sinked in
	/// v2. I.e. all edges where e == (v1,v2,_).
	///
	/// If the graph is undirected, returns all edges connecting the two
	/// vertices I.e. all edges where e == (v1,v2,_) or e == (v2,v1,_)
	fn edges_between<'a: 'b, 'b>(
		&'a self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a Self::EdgeWeight>>;

	// Optional methods
	fn vertex_ref(&self, v: impl Borrow<Self::Vertex>) -> Option<Self::VertexRef>
	{
		self.all_vertices()
			.find(|v_ref| v_ref.borrow() == v.borrow())
	}
	fn all_vertices<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::VertexRef>>
	{
		Box::new(self.all_vertices_weighted().map(|(v, _)| v))
	}
	fn vertex_weight(&self, v: impl Borrow<Self::Vertex>) -> Option<&Self::VertexWeight>
	{
		self.all_vertices_weighted()
			.find(|(candidate, _)| candidate.borrow() == v.borrow())
			.map(|(_, w)| w)
	}
	fn contains_vertex(&self, v: impl Borrow<Self::Vertex>) -> bool
	{
		self.vertex_weight(v).is_some()
	}

	/// Returns the sink and weight of any edge sourced in the given vertex.
	/// I.e. all edges where `e == (v,_,_)`
	///
	/// If the graph is undirected, is semantically equivalent to
	/// `edges_incident_on`.
	fn edges_sourced_in<'a: 'b, 'b>(
		&'a self,
		v: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = (Self::VertexRef, &'a Self::EdgeWeight)>>
	{
		Box::new(self.all_vertices().flat_map(move |v2| {
			self.edges_between(v.borrow().clone(), v2.borrow().clone())
				.map(move |w| (v2.clone(), w))
		}))
	}

	/// Returns the source and weight of any edge sinked in the given vertex.
	/// I.e. all edges where `e == (_,v,_)`
	///
	/// If the graph is undirected, is semantically equivalent to
	/// `edges_incident_on`.
	fn edges_sinked_in<'a: 'b, 'b>(
		&'a self,
		v: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = (Self::VertexRef, &'a Self::EdgeWeight)>>
	{
		Box::new(self.all_vertices().flat_map(move |v2| {
			self.edges_between(v2.borrow().clone(), v.borrow().clone())
				.map(move |w| (v2.clone(), w))
		}))
	}

	/// Returns the neighboring vertex and the weight of any edge incident
	/// on the given vertex.
	///
	/// If the graph is directed, edge directions are ignored.
	fn edges_incident_on<'a: 'b, 'b>(
		&'a self,
		v: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = (Self::VertexRef, &'a Self::EdgeWeight)>>
	{
		Box::new(
			self.edges_sourced_in(v.borrow().clone()).chain(
				self.edges_sinked_in(v.borrow().clone())
					.filter(move |(v2, _)| {
						Self::Directedness::directed() && v.borrow() != v2.borrow()
					}),
			),
		)
	}

	/// Returns any vertices connected to the given one with an edge regardless
	/// of direction.
	fn vertex_neighbors<'a: 'b, 'b>(
		&'a self,
		v: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = Self::VertexRef>>
	{
		Box::new(
			self.all_vertices()
				.filter(move |other| self.neighbors(v.borrow(), other.borrow())),
		)
	}

	/// Returns whether the two vertices are connected by an edge in any
	/// direction.
	fn neighbors(&self, v1: impl Borrow<Self::Vertex>, v2: impl Borrow<Self::Vertex>) -> bool
	{
		self.edges_between(v1.borrow(), v2.borrow())
			.next()
			.is_some() || (Self::Directedness::directed()
			&& self
				.edges_between(v2.borrow(), v1.borrow())
				.next()
				.is_some())
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
	) -> Box<dyn 'a + Iterator<Item = (Self::VertexRef, &'a mut Self::VertexWeight)>>;

	fn edges_between_mut<'a: 'b, 'b>(
		&'a mut self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> Box<dyn 'b + Iterator<Item = &'a mut Self::EdgeWeight>>;

	// Optional methods
	fn vertex_weight_mut(&mut self, v: impl Borrow<Self::Vertex>)
		-> Option<&mut Self::VertexWeight>
	{
		self.all_vertices_weighted_mut()
			.find(|(candidate, _)| candidate.borrow() == v.borrow())
			.map(|(_, w)| w)
	}
}
