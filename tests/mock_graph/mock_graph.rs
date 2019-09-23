
use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight,};
use graphene::{
	core::{
		Graph, Edge, EdgeWeighted,
	},
};
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};
use graphene::core::{Directedness, ExactGraph, BaseGraph, AutoGraph};
use std::collections::HashMap;

///
/// A simple graph implementation used for testing.
///
/// Vertex ids are maintained across vertex creation and removal.
/// Vertex ids of previously removed vertices won't be reused unless `pack()` is called.
///
/// Will panic if it runs out of ids.
///
#[derive(Clone)]
pub struct MockGraph<D>
	where D: Directedness + Clone
{
	/// The number to give the next new vertex.
	pub next_id: usize,
	///
	/// The weights of the vertices in the graph.
	///
	pub vertices: HashMap<usize, MockVertexWeight>,
	/// All edges in the graph, regardless of directedness.
	pub edges: Vec<(usize, usize, MockEdgeWeight)>,
	phantom: PhantomData<D>
}

impl<D: Directedness + Clone> MockGraph<D> {
	
	/// Validates the internal integrity of the graph.
	///
	/// I.e:
	/// * All edges are incident on currently vertices that are still in the graph.
	/// * All vertex ids are less that the next id to be used
	pub fn validate(&self)
	{
		if let Some(v) = self.vertices.keys().find(|&&v| v >= self.next_id ) {
			panic!("Found a vertex with id larger than 'next_id'({}): {}", self.next_id, v);
		}
		if let Some(e) = self.edges.iter().find(|e|
			!self.vertices.contains_key(&e.source()) ||
			!self.vertices.contains_key(&e.sink()) )
		{
			panic!("Found an edge incident on invalid vertices: {:?}", e);
		}
	}
	
	pub fn empty() -> Self
	{
		Self{next_id: 0, vertices: HashMap::new(), edges: Vec::new(), phantom: PhantomData}
	}
	
	///
	/// Reassigns vertex ids such that there are no spaces between them.
	///
	/// I.e. if the vertices are {0,1,3,4,6} they become {0,1,2,3,4} and all edges are
	/// corrected accordingly.
	///
	pub fn pack(&mut self)
	{
		let mut old_verts = self.vertices.keys().collect::<Vec<_>>();
		old_verts.sort();
		let vert_map: HashMap<usize, usize> = old_verts.into_iter().enumerate()
			.map(|(idx, &old_v)| (old_v, idx)).collect();

		self.next_id = 0;

		// Move all vertex weight to new vertex map
		let mut new_vertices = HashMap::new();
		for (old_v, &new_v) in &vert_map {
			new_vertices.insert(new_v, self.vertices.remove(old_v).unwrap());
		}
		self.vertices = new_vertices;

		// Correct all edges
		for e in self.edges.iter_mut() {
			e.0 = vert_map[&e.0];
			e.1 = vert_map[&e.1];
		}

		self.validate()
	}
}

impl<D: Directedness + Clone> Debug for MockGraph<D> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		f.write_str("MockGraph { vertices: [ ")?;
		for (v,w) in &self.vertices {
			f.write_fmt(format_args!("({:?}, {:?}), ", v, w.value))?;
		}
		f.write_str("], edges: [ ")?;
		for (so,si,w) in &self.edges {
			f.write_fmt(format_args!("({:?}, {:?}, {:?}), ", so, si, w.value))?;
		}
		f.write_str("] }")?;
		Ok(())
	}
}

impl<D: Directedness + Clone> Graph for MockGraph<D>
{
	/// We hide u32 behind a struct to ensure our tests aren't dependent
	/// on graphs using usize as ids
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	type Directedness = D;
	
	fn all_vertices_weighted<'a>(&'a self)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>
	{
		Box::new(self.vertices.iter().map(|(&v,w)| (MockVertex{value: v},w)))
	}
	
	fn all_vertices_weighted_mut<'a>(&'a mut self)
		-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		Box::new(self.vertices.iter_mut().map(|(&v,w)| (MockVertex{value: v},w)))
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		Box::new(self.edges.iter().map(|(so, si, w)|
			(MockVertex{value: *so}, MockVertex{value: *si}, w)))
	}
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		Box::new(self.edges.iter_mut().map(|(so, si, w)|
			(MockVertex{value: *so}, MockVertex{value: *si}, w)))
	}
	
	fn remove_vertex(&mut self, mock_v: Self::Vertex) -> Result<Self::VertexWeight,()>{
		let v = mock_v.value;
		if let Some(weight) = self.vertices.remove(&v){
			self.edges.retain(|e| e.source() != v && e.sink() != v);
			self.validate();
			Ok(weight)
		} else {
			Err(())
		}
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where
			E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>,
	{
		if self.vertices.contains_key(&e.source().value) &&
			self.vertices.contains_key(&e.sink().value)
		{
			self.edges.push(
				(e.source().value, e.sink().value, e.weight_owned())
			);
			self.validate();
			Ok(())
		} else {
			Err(())
		}
	}
	
	fn remove_edge_where<F>(&mut self, f: F)
		-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		if let Some((idx,_)) = self.edges.iter().enumerate()
			.find(|(_, (so,si,w))|
				f((MockVertex{value: *so},MockVertex{value: *si}, w)))
		{
			let (so,si,w) = self.edges.remove(idx);
			self.validate();
			Ok((MockVertex{value: so},MockVertex{value: si}, w))
		} else {
			Err(())
		}
	}
}

impl<D: Directedness + Clone> AutoGraph for MockGraph<D>
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex, ()> {
		if self.vertices.insert(self.next_id, w).is_some() {
			panic!("'next_id' was already in use.");
		} else {
			self.next_id += 1;
			self.validate();
			Ok(MockVertex{ value: self.next_id - 1})
		}
	}
}

impl<D: Directedness + Clone> ExactGraph for MockGraph<D>{}

impl<D: Directedness + Clone> BaseGraph for MockGraph<D>
{}