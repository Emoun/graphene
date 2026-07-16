use crate::core::{
	property::{AddEdge, EdgeCount, NewVertex, RemoveEdge, VertexCount},
	Graph, GraphMut,
};
use std::{borrow::Borrow, collections::HashMap, hash::Hash, iter::empty};

/// A graph that may have arbitrary vertices (V).
///
/// The [`add_vertex_weighted`](VertexMapGraph::add_vertex_weighted) method is
/// provided to add arbitrary vertices to the graph. All vertices must be
/// unique.
///
/// For example, can be used with [`AdjListGraph`](crate::common::AdjListGraph)
/// as a backing graph.
///
/// It is backed by an underlying graph (G), with a map from the arbitrary
/// vertices to vertices of the underlying graph.
///
/// Does not implement [`RemoveVertex`] because doing so does not guarantee that
/// the vertices stay the same in the underlying graph.
#[derive(Clone, Debug)]
pub struct VertexMapGraph<V: Copy + Eq + Hash, G: Graph>
{
	pub graph: G,
	pub map: HashMap<V, G::Vertex>,
}

impl<V: Copy + Eq + Hash, G: Graph> VertexMapGraph<V, G>
{
	pub fn new() -> Self
	where
		G: Default,
	{
		Self {
			graph: G::default(),
			map: HashMap::new(),
		}
	}

	/// Inserts the given vertex into the graph with the given weight
	pub fn add_vertex_weighted(&mut self, v: V, w: <Self as Graph>::VertexWeight) -> Result<(), ()>
	where
		G: NewVertex,
	{
		let new_v = self.graph.new_vertex_weighted(w)?;
		assert!(self.map.insert(v, new_v).is_none());
		Ok(())
	}

	/// Inserts the given vertex into the graph
	pub fn add_vertex(&mut self, v: V) -> Result<(), ()>
	where
		G: NewVertex,
		G::VertexWeight: Default,
	{
		self.add_vertex_weighted(v, Default::default())
	}

	fn get_backing_vertices<const NR: usize>(&self, vs: [&V; NR]) -> Result<[G::Vertex; NR], ()>
	{
		let accesses = vs.map(|v| self.map.get(v));
		if accesses.iter().any(|v| v.is_none())
		{
			Err(())
		}
		else
		{
			Ok(accesses.map(|v| *v.unwrap()))
		}
	}
}

impl<V: Copy + Eq + Hash, G: Graph> Graph for VertexMapGraph<V, G>
{
	type Directedness = G::Directedness;
	type EdgeWeight = G::EdgeWeight;
	type EdgeWeightRef<'a>
		= G::EdgeWeightRef<'a>
	where
		Self: 'a;
	type Vertex = V;
	type VertexWeight = G::VertexWeight;

	fn all_vertices_weighted(&self) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>
	{
		self.map
			.iter()
			.map(|(v, b)| (*v, self.graph.vertex_weight(b).unwrap()))
	}

	fn edges_between(
		&self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = Self::EdgeWeightRef<'_>>
	{
		let boxed: Box<dyn Iterator<Item = Self::EdgeWeightRef<'_>>> =
			if let Ok([backing_so, backing_si]) =
				self.get_backing_vertices([source.borrow(), sink.borrow()])
			{
				Box::new(self.graph.edges_between(backing_so, backing_si))
			}
			else
			{
				Box::new(empty())
			};
		boxed.into_iter()
	}
}

impl<V: Copy + Eq + Hash, G: GraphMut> GraphMut for VertexMapGraph<V, G>
{
	fn all_vertices_weighted_mut(
		&mut self,
	) -> impl Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>
	{
		self.graph.all_vertices_weighted_mut().map(|(b, w)| {
			(
				*self
					.map
					.iter()
					.find(|(_, b2)| **b2 == b)
					.map(|(v, _)| v)
					.unwrap(),
				w,
			)
		})
	}

	fn edges_between_mut(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = &mut Self::EdgeWeight>
	{
		let boxed: Box<dyn Iterator<Item = &mut Self::EdgeWeight>> =
			if let Ok([backing_so, backing_si]) =
				self.get_backing_vertices([source.borrow(), sink.borrow()])
			{
				Box::new(self.graph.edges_between_mut(backing_so, backing_si))
			}
			else
			{
				Box::new(empty())
			};
		boxed.into_iter()
	}
}

impl<V: Copy + Eq + Hash, G: AddEdge> AddEdge for VertexMapGraph<V, G>
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		let [backing_so, backing_si] =
			self.get_backing_vertices([source.borrow(), sink.borrow()])?;
		self.graph.add_edge_weighted(backing_so, backing_si, weight)
	}
}

impl<V: Copy + Eq + Hash, G: RemoveEdge> RemoveEdge for VertexMapGraph<V, G>
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
		let [backing_so, backing_si] =
			self.get_backing_vertices([source.borrow(), sink.borrow()])?;

		self.graph
			.remove_edge_where_weight(backing_so, backing_si, f)
	}
}

impl<V: Copy + Eq + Hash, G: VertexCount> VertexCount for VertexMapGraph<V, G>
{
	type Count = G::Count;
}

impl<V: Copy + Eq + Hash, G: EdgeCount> EdgeCount for VertexMapGraph<V, G>
{
	type Count = G::Count;
}

base_graph! {
	use<V, G> VertexMapGraph<V, G> where V: Copy + Eq + Hash, G: Graph
}
