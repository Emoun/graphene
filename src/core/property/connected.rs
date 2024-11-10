use crate::{
	algo::{Dfs, DijkstraShortestPaths},
	core::{
		property::{
			proxy_remove_edge_where_weight, proxy_remove_vertex, DirectedGraph, EdgeCount,
			HasVertex, HasVertexGraph, RemoveEdge, RemoveVertex, Unilateral, VertexInGraph, Weak,
		},
		proxy::ReverseGraph,
		Ensure, Graph, GraphDerefMut,
	},
};
use num_traits::{PrimInt, Unsigned};
use std::borrow::Borrow;

/// A marker trait for graphs that are connected.
///
/// A graph is connected if there is a path from any vertex to any other vertex.
/// Graphs with one or zero vertices count as connected.
pub trait Connected: Unilateral
{
	/// Calculates the maximum distance between the designated vertex and any other vertex ([the eccentricity](https://mathworld.wolfram.com/GraphEccentricity.html)).
	///
	/// Takes a closure that converts an edge's weight into a distance value.
	/// The distance between two vertices is equal to the distance of the
	/// edge(s) between them.
	fn eccentricity_weighted<W: PrimInt + Unsigned>(
		&self,
		get_distance: fn(&Self::EdgeWeight) -> W,
	) -> W
	where
		Self: EdgeCount + HasVertex + Sized,
	{
		// We search for all the shortest paths, the eccentricity is the longest one
		DijkstraShortestPaths::distances(self, get_distance).fold(W::zero(), |max_dist, (_, d2)| {
			if max_dist < d2
			{
				d2
			}
			else
			{
				max_dist
			}
		})
	}

	/// Calculates the maximum eccentricity of the graph ([the diameter](https://mathworld.wolfram.com/GraphDiameter.html)).
	///
	/// Takes a closure that converts an edge's weight into a distance value.
	/// The distance between two vertices is equal to the distance of the
	/// edge(s) between them.
	fn diameter_weighted<W: PrimInt + Unsigned>(
		&self,
		get_distance: fn(&Self::EdgeWeight) -> W,
	) -> W
	where
		Self: EdgeCount + Sized,
	{
		self.all_vertices().fold(W::zero(), |max_ecc, v| {
			let new_ecc =
				VertexInGraph::ensure_unvalidated(self, v).eccentricity_weighted(get_distance);
			if new_ecc > max_ecc
			{
				new_ecc
			}
			else
			{
				max_ecc
			}
		})
	}

	/// Calculates the minimum eccentricity of the graph ([the radius](https://mathworld.wolfram.com/GraphDiameter.html)).
	///
	/// Takes a closure that converts an edge's weight into a distance value.
	/// The distance between two vertices is equal to the distance of the
	/// edge(s) between them.
	fn radius_weighted<W: PrimInt + Unsigned>(&self, get_distance: fn(&Self::EdgeWeight) -> W) -> W
	where
		Self: EdgeCount + Sized,
	{
		self.all_vertices().fold(W::zero(), |min_ecc, v| {
			let new_ecc =
				VertexInGraph::ensure_unvalidated(self, v).eccentricity_weighted(get_distance);
			if new_ecc < min_ecc
			{
				new_ecc
			}
			else
			{
				min_ecc
			}
		})
	}

	/// Returns the vertices with eccentricity equal to the radius ([the centers](https://mathworld.wolfram.com/GraphCenter.html)).
	///
	/// Takes a closure that converts an edge's weight into a distance value.
	/// The distance between two vertices is equal to the distance of the
	/// edge(s) between them.
	fn centers_weighted<'a, W: 'a + PrimInt + Unsigned>(
		&'a self,
		get_distance: fn(&Self::EdgeWeight) -> W,
	) -> impl Iterator<Item = Self::Vertex> + '_
	where
		Self: EdgeCount + Sized,
	{
		let radius = self.radius_weighted(get_distance);
		self.all_vertices().filter(move |v| {
			VertexInGraph::ensure_unvalidated(self, *v).eccentricity_weighted(get_distance)
				== radius
		})
	}
}

#[derive(Clone, Debug)]
pub struct ConnectedGraph<C: Ensure>(C);

impl<C: Ensure> ConnectedGraph<C>
{
	/// Creates a new connected graph. The given graph *must* be connected.
	/// This method does not check for this!!
	pub fn new(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Ensure> Ensure for ConnectedGraph<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		let g = c.graph();
		let v_count = g.all_vertices().count();

		if let Ok(g) = HasVertexGraph::ensure(g, ())
		{
			let dfs_count = Dfs::new_simple(&g).count();
			if (dfs_count + 1) == v_count
			{
				// If its undirected, no more needs to be done
				if let Ok(g) = DirectedGraph::ensure(g, ())
				{
					let reverse = ReverseGraph::new(g);
					if (Dfs::new_simple(&reverse).count() + 1) != v_count
					{
						return false;
					}
				}
				return true;
			}
			return false;
		}
		true
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for ConnectedGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		proxy_remove_vertex::<ConnectedGraph<_>, _>(self.0.graph_mut(), v.borrow())
	}
}

impl<C: Ensure + GraphDerefMut> RemoveEdge for ConnectedGraph<C>
where
	C::Graph: RemoveEdge,
{
	fn remove_edge_where_weight<F>(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		f: F,
	) -> Result<Self::EdgeWeight, ()>
	where
		F: Fn(&Self::EdgeWeight) -> bool,
	{
		proxy_remove_edge_where_weight::<ConnectedGraph<_>, _, _>(
			self.0.graph_mut(),
			source.borrow(),
			sink.borrow(),
			f,
		)
	}
}

impl<C: Ensure> Weak for ConnectedGraph<C> {}
impl<C: Ensure> Unilateral for ConnectedGraph<C> {}
impl<C: Ensure> Connected for ConnectedGraph<C> {}

impl_ensurer! {
	use<C> ConnectedGraph<C>: Ensure, Connected, Unilateral, Weak, RemoveVertex, RemoveEdge,
	// A new vertex wouldn't be connected to the rest of the graph
	NewVertex
	as (self.0) : C
}
