use crate::mock_graph::{MockEdgeWeight, MockVertex, MockVertexWeight};
use graphene::core::{
	property::{AddEdge, NewVertex, RemoveEdge, RemoveVertex},
	BaseGraph, Directedness, Edge, EdgeWeighted, Graph, GraphDeref, GraphDerefMut, GraphMut,
};
use std::{
	collections::HashMap,
	fmt::{Debug, Error, Formatter},
	marker::PhantomData,
};

/// A simple graph implementation used for testing.
///
/// Vertex ids are maintained across vertex creation and removal.
/// Vertex ids of previously removed vertices won't be reused unless `pack()` is
/// called.
///
/// Will panic if it runs out of ids.
#[derive(Clone)]
pub struct MockGraph<D: Directedness>
{
	/// The number to give the next new vertex.
	pub next_id: usize,
	/// The weights of the vertices in the graph.
	pub vertices: HashMap<usize, MockVertexWeight>,
	/// All edges in the graph, regardless of directedness.
	pub edges: Vec<(usize, usize, MockEdgeWeight)>,
	phantom: PhantomData<D>,
}

impl<D: Directedness> MockGraph<D>
{
	/// Validates the internal integrity of the graph.
	///
	/// I.e:
	/// * All edges are incident on currently vertices that are still in the
	///   graph.
	/// * All vertex ids are less that the next id to be used
	pub fn validate_is_graph(&self)
	{
		if let Some(v) = self.vertices.keys().find(|&&v| v >= self.next_id)
		{
			panic!(
				"Found a vertex with id larger than 'next_id'({}): {}",
				self.next_id, v
			);
		}
		if let Some(e) = self.edges.iter().find(|e| {
			!self.vertices.contains_key(&e.source()) || !self.vertices.contains_key(&e.sink())
		})
		{
			panic!("Found an edge incident on invalid vertices: {:?}", e);
		}
	}

	pub fn empty() -> Self
	{
		Self {
			next_id: 0,
			vertices: HashMap::new(),
			edges: Vec::new(),
			phantom: PhantomData,
		}
	}

	/// Reassigns vertex ids such that there are no spaces between them.
	///
	/// I.e. if the vertices are {0,1,3,4,6} they become {0,1,2,3,4} and all
	/// edges are corrected accordingly.
	#[allow(dead_code)]
	pub fn pack(&mut self)
	{
		let mut old_verts = self.vertices.keys().collect::<Vec<_>>();
		old_verts.sort();
		let vert_map: HashMap<usize, usize> = old_verts
			.into_iter()
			.enumerate()
			.map(|(idx, &old_v)| (old_v, idx))
			.collect();

		self.next_id = 0;

		// Move all vertex weight to new vertex map
		let mut new_vertices = HashMap::new();
		for (old_v, &new_v) in &vert_map
		{
			new_vertices.insert(new_v, self.vertices.remove(old_v).unwrap());
		}
		self.vertices = new_vertices;

		// Correct all edges
		for e in self.edges.iter_mut()
		{
			e.0 = vert_map[&e.0];
			e.1 = vert_map[&e.1];
		}

		self.validate_is_graph()
	}

	/// Inserts the given graph into this one, creating new vertices and edges
	/// to match.
	///
	/// Returns a mapping from the vertex id's in the given graph to their new
	/// counterparts
	pub fn join<G>(&mut self, other: &G) -> HashMap<MockVertex, MockVertex>
	where
		G: Graph<
			Vertex = MockVertex,
			VertexWeight = MockEdgeWeight,
			EdgeWeight = MockEdgeWeight,
			Directedness = D,
		>,
	{
		let mut v_map: HashMap<MockVertex, MockVertex> = HashMap::new();

		for (v, w) in other.all_vertices_weighted()
		{
			let new_v = self.new_vertex_weighted(w.clone()).unwrap();
			v_map.insert(v, new_v);
		}
		for (so, si, w) in other.all_edges()
		{
			self.add_edge_weighted((v_map[&so], v_map[&si], w.clone()))
				.unwrap();
		}

		v_map
	}
}

impl<D: Directedness> Debug for MockGraph<D>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		f.write_str("MockGraph { vertices: [ ")?;
		let mut verts: Vec<_> = self.vertices.iter().collect();
		verts.sort_by_key(|(&v, _)| v);
		for (v, w) in &verts
		{
			f.write_fmt(format_args!("({:?}, {:?}), ", v, w.value))?;
		}
		f.write_str("], edges: [ ")?;

		let mut edges = self.edges.clone();
		edges.sort_by_key(|(v, _, _)| *v);
		for (so, si, w) in &edges
		{
			f.write_fmt(format_args!("({:?}, {:?}, {:?}), ", so, si, w.value))?;
		}
		f.write_str("] }")?;
		Ok(())
	}
}

impl<D: Directedness> Graph for MockGraph<D>
{
	type Directedness = D;
	type EdgeWeight = MockEdgeWeight;
	/// We hide u32 behind a struct to insure our tests aren't dependent
	/// on graphs using usize as ids
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;

	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>
	{
		Box::new(
			self.vertices
				.iter()
				.map(|(&v, w)| (MockVertex { value: v }, w)),
		)
	}

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(
			self.edges
				.iter()
				.map(|(so, si, w)| (MockVertex { value: *so }, MockVertex { value: *si }, w)),
		)
	}
}

impl<D: Directedness> GraphMut for MockGraph<D>
{
	fn all_vertices_weighted_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		Box::new(
			self.vertices
				.iter_mut()
				.map(|(&v, w)| (MockVertex { value: v }, w)),
		)
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(
			self.edges
				.iter_mut()
				.map(|(so, si, w)| (MockVertex { value: *so }, MockVertex { value: *si }, w)),
		)
	}
}

impl<D: Directedness> NewVertex for MockGraph<D>
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()>
	{
		if self.vertices.insert(self.next_id, w).is_some()
		{
			panic!("'next_id' was already in use.");
		}
		else
		{
			self.next_id += 1;
			self.validate_is_graph();
			Ok(MockVertex {
				value: self.next_id - 1,
			})
		}
	}
}
impl<D: Directedness> RemoveVertex for MockGraph<D>
{
	fn remove_vertex(&mut self, mock_v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		let v = mock_v.value;
		if let Some(weight) = self.vertices.remove(&v)
		{
			self.edges.retain(|e| e.source() != v && e.sink() != v);
			self.validate_is_graph();
			Ok(weight)
		}
		else
		{
			Err(())
		}
	}
}

impl<D: Directedness> AddEdge for MockGraph<D>
{
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
	where
		E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		if self.vertices.contains_key(&e.source().value)
			&& self.vertices.contains_key(&e.sink().value)
		{
			self.edges
				.push((e.source().value, e.sink().value, e.weight_owned()));
			self.validate_is_graph();
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<D: Directedness> RemoveEdge for MockGraph<D>
{
	fn remove_edge_where<F>(
		&mut self,
		f: F,
	) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
	where
		F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool,
	{
		if let Some((idx, _)) =
			self.edges.iter().enumerate().find(|(_, (so, si, w))| {
				f((MockVertex { value: *so }, MockVertex { value: *si }, w))
			})
		{
			let (so, si, w) = self.edges.remove(idx);
			self.validate_is_graph();
			Ok((MockVertex { value: so }, MockVertex { value: si }, w))
		}
		else
		{
			Err(())
		}
	}
}

impl<D: Directedness> GraphDeref for MockGraph<D>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<D: Directedness> GraphDerefMut for MockGraph<D>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<D: Directedness> BaseGraph for MockGraph<D> {}
