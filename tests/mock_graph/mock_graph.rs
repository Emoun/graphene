use crate::mock_graph::{MockEdgeWeight, MockType, MockVertex, MockVertexWeight};
use graphene::{
	base_graph,
	core::{
		property::{AddEdge, EdgeCount, NewVertex, RemoveEdge, RemoveVertex, VertexCount},
		Directedness, Edge, Graph, GraphMut,
	},
};
use std::{
	borrow::Borrow,
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
pub struct MockGraph<D: Directedness, Ew: MockType = MockEdgeWeight>
{
	/// The number to give the next new vertex.
	pub next_id: usize,
	/// The weights of the vertices in the graph.
	pub vertices: HashMap<usize, MockVertexWeight>,
	/// All edges in the graph, regardless of directedness.
	pub edges: Vec<(usize, usize, Ew)>,
	phantom: PhantomData<D>,
}

impl<D: Directedness, Ew: MockType> MockGraph<D, Ew>
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
			EdgeWeight = Ew,
			Directedness = D,
		>,
	{
		let mut v_map: HashMap<MockVertex, MockVertex> = HashMap::new();

		for (v, w) in other.all_vertices_weighted()
		{
			let new_v = self.new_vertex_weighted(w.clone()).unwrap();
			v_map.insert(v, new_v);

			// Insert all edge to/from the finished vertices
			for (v_done, new_v_done) in v_map.iter()
			{
				for e_w in other.edges_between(&v, v_done)
				{
					self.add_edge_weighted(&new_v, new_v_done, e_w.borrow().clone())
						.unwrap();
				}
				if D::directed() && *v_done != v
				{
					for e_w in other.edges_between(v_done, &v)
					{
						self.add_edge_weighted(new_v_done, &new_v, e_w.borrow().clone())
							.unwrap();
					}
				}
			}
		}

		v_map
	}
}

impl<D: Directedness, Ew: MockType> Debug for MockGraph<D, Ew>
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
			f.write_fmt(format_args!("({:?}, {:?}, {:?}), ", so, si, w))?;
		}
		f.write_str("] }")?;
		Ok(())
	}
}

impl<D: Directedness, Ew: MockType> Graph for MockGraph<D, Ew>
{
	type Directedness = D;
	type EdgeWeight = Ew;
	type EdgeWeightRef<'a>
		= &'a Ew
	where
		Self: 'a;
	/// We hide u32 behind a struct to ensure our tests aren't dependent
	/// on graphs using usize as ids
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;

	fn all_vertices_weighted(&self) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>
	{
		self.vertices
			.iter()
			.map(|(&v, w)| (MockVertex { value: v }, w))
	}

	fn all_edges(&self) -> impl Iterator<Item = (Self::Vertex, Self::Vertex, &Self::EdgeWeight)>
	{
		self.edges
			.iter()
			.map(|(so, si, w)| (MockVertex { value: *so }, MockVertex { value: *si }, w))
	}

	fn edges_between(
		&self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = &Self::EdgeWeight>
	{
		let source = source.borrow().value;
		let sink = sink.borrow().value;
		self.edges.iter().filter_map(move |(so, si, w)| {
			if (source == *so && sink == *si)
				|| (!Self::Directedness::directed() && (source == *si && sink == *so))
			{
				Some(w)
			}
			else
			{
				None
			}
		})
	}
}

impl<D: Directedness, Ew: MockType> GraphMut for MockGraph<D, Ew>
{
	fn all_vertices_weighted_mut(
		&mut self,
	) -> impl Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>
	{
		self.vertices
			.iter_mut()
			.map(|(&v, w)| (MockVertex { value: v }, w))
	}

	fn edges_between_mut(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
	) -> impl Iterator<Item = &mut Self::EdgeWeight>
	{
		let source = source.borrow().value;
		let sink = sink.borrow().value;
		self.edges.iter_mut().filter_map(move |(so, si, w)| {
			if (source == *so && sink == *si)
				|| (!Self::Directedness::directed() && (source == *si && sink == *so))
			{
				Some(w)
			}
			else
			{
				None
			}
		})
	}
}

impl<D: Directedness, Ew: MockType> NewVertex for MockGraph<D, Ew>
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
impl<D: Directedness, Ew: MockType> RemoveVertex for MockGraph<D, Ew>
{
	fn remove_vertex(&mut self, mock_v: impl Borrow<Self::Vertex>)
		-> Result<Self::VertexWeight, ()>
	{
		let v = mock_v.borrow().value;
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

impl<D: Directedness, Ew: MockType> AddEdge for MockGraph<D, Ew>
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		let source = source.borrow().value;
		let sink = sink.borrow().value;
		if self.vertices.contains_key(&source) && self.vertices.contains_key(&sink)
		{
			self.edges.push((source, sink, weight));
			self.validate_is_graph();
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<D: Directedness, Ew: MockType> RemoveEdge for MockGraph<D, Ew>
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
		if let Some((idx, _)) = self.edges.iter().enumerate().find(|(_, (so, si, w))| {
			let source = source.borrow().value;
			let sink = sink.borrow().value;
			((*so == source && *si == sink)
				|| !Self::Directedness::directed() && (*so == sink && *si == source))
				&& f(w)
		})
		{
			let (_, _, w) = self.edges.remove(idx);
			self.validate_is_graph();
			Ok(w)
		}
		else
		{
			Err(())
		}
	}
}

impl<D: Directedness, Ew: MockType> VertexCount for MockGraph<D, Ew>
{
	type Count = usize;

	fn vertex_count(&self) -> Self::Count
	{
		self.vertices.len()
	}
}

impl<D: Directedness, Ew: MockType> EdgeCount for MockGraph<D, Ew>
{
	type Count = usize;

	fn edge_count(&self) -> Self::Count
	{
		self.edges.len()
	}
}

base_graph! {
	use<D,Ew> MockGraph<D,Ew>
	where D: Directedness, Ew: MockType
}
