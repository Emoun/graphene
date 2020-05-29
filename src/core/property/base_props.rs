use crate::core::{Directedness, Edge, EdgeWeighted, Graph};
use num_traits::{One, PrimInt, Unsigned, Zero};

/// A graph where new vertices can be added
pub trait NewVertex: Graph
{
	/// Adds a new vertex with the given weight to the graph.
	/// Returns the id of the new vertex.
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>;

	// Optional methods
	/// Adds a new vertex to the graph.
	/// Returns the id of the new vertex.
	/// The weight of the vertex is the default.
	fn new_vertex(&mut self) -> Result<Self::Vertex, ()>
	where
		Self::VertexWeight: Default,
	{
		self.new_vertex_weighted(Self::VertexWeight::default())
	}
}

/// A graph where vertices can be removed.
pub trait RemoveVertex: Graph
{
	/// Removes the given vertex from the graph, returning its weight.
	/// If the vertex still has edges incident on it, they are also removed,
	/// dropping their weights.
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
}

pub trait AddEdge: Graph
{
	/// Adds a copy of the given edge to the graph
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;

	// Optional methods

	/// Adds the given edge to the graph, regardless of whether there are
	/// existing, identical edges in the graph.
	/// The vertices the new edge is incident on must exist in the graph and the
	/// id must be valid.
	///
	/// ###Returns
	/// - `Ok` if the edge is valid and was added to the graph.
	/// - `Err` if the edge is invalid or the graph was otherwise unable to
	///   store it.
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
	fn add_edge<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: Edge<Self::Vertex>,
		Self::EdgeWeight: Default,
	{
		self.add_edge_weighted((e.source(), e.sink(), Self::EdgeWeight::default()))
	}
}

pub trait RemoveEdge: Graph
{
	/// Removes an edge that matches the given predicate closure.
	/// If no edge is found to match and successfully removed, returns error
	/// but otherwise doesn't change the graph.
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;

	// Optional methods
	/// Removes the given edge from the graph if it exists.
	///
	/// ###Returns
	/// - `Ok` if the edge was present before the call and was removed.
	/// - `Err` if the edge was not found in the graph or it was otherwise
	///   unable to remove it.
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
	fn remove_edge<E>(&mut self, e: E) -> Result<Self::EdgeWeight, ()>
	where
		E: Edge<Self::Vertex>,
	{
		self.remove_edge_where_weight(e, |_| true)
	}

	fn remove_edge_where_weight<E, F>(&mut self, e: E, f: F) -> Result<Self::EdgeWeight, ()>
	where
		E: Edge<Self::Vertex>,
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		self.remove_edge_where(|(so, si, w)| {
			f(w) && ((so == e.source()) && (si == e.sink())
				|| (!Self::Directedness::directed() && (so == e.sink()) && (si == e.source())))
		})
		.map(|removed_edge| removed_edge.2)
	}

	/// Tries to removes all edges that match the given predicate.
	/// Stops either when no more edges match, or an edge that matched couldn't
	/// be removed.
	///
	/// Returns the removed edges regardless.
	fn remove_edge_where_all<F>(
		&mut self,
		f: F,
	) -> Vec<(Self::Vertex, Self::Vertex, Self::EdgeWeight)>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		let mut result = Vec::new();
		while let Ok(e) = self.remove_edge_where(|e| f(e))
		{
			result.push(e);
		}
		result
	}
}

/// A graph with a finite number of vertices that can be counted.
pub trait VertexCount: Graph
{
	type Count: PrimInt + Unsigned;

	/// Returns the number of vertices in the graph.
	fn vertex_count(&self) -> Self::Count
	{
		let mut count = Self::Count::zero();
		let mut verts = self.all_vertices();
		while let Some(_) = verts.next()
		{
			count = count + Self::Count::one();
		}
		count
	}
}

/// A graph with a finite number of edges that can be counted.
pub trait EdgeCount: Graph
{
	type Count: PrimInt + Unsigned;

	/// Returns the number of vertices in the graph.
	fn edge_count(&self) -> Self::Count
	{
		let mut count = Self::Count::zero();
		let mut inc = || count = count + Self::Count::one();
		let verts: Vec<_> = self.all_vertices().collect();

		let mut iter = verts.iter();
		let mut rest_iter = iter.clone();
		while let Some(v) = iter.next()
		{
			for v2 in rest_iter
			{
				self.edges_between(&v, &v2).for_each(|_| inc());
				if Self::Directedness::directed()
				{
					self.edges_between(&v2, &v).for_each(|_| inc());
				}
			}
			rest_iter = iter.clone();
		}
		count
	}
}
