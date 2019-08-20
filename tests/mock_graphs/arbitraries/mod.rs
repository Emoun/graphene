
mod mock_graph;

pub use self::{
	mock_graph::*,
};

use graphene::core::trait_aliases::Id;
use quickcheck::{Arbitrary, Gen};
use crate::mock_graphs::{MockVertex, MockT, MockGraph};
use graphene::core::Graph;
use rand::{ Rng };

///
/// Trait alias for arbitrary identifiers.
///
pub trait ArbVertex: Arbitrary + Id{}
impl<T> ArbVertex for T where T: Arbitrary + Id{}

impl Arbitrary for MockVertex
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

pub trait ArbT: Arbitrary{}
impl<T> ArbT for T where T: Arbitrary + Id{}

impl Arbitrary for MockT
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

///
/// An arbitrary graph and two vertices in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndTwoVertices(pub MockGraph, pub MockVertex, pub MockVertex);
impl Arbitrary for ArbGraphAndTwoVertices {
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		// Create a graph with at least 1 vertex
		let graph = {
			let mut candGraph = MockGraph::arbitrary(g);
			while candGraph.all_vertices::<Vec<_>>().len()==0 {
				candGraph = MockGraph::arbitrary(g);
			}
			candGraph
		};
		
		let verts: Vec<_> = graph.all_vertices();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = verts[g.gen_range(0, verts.len())];
		
		ArbGraphAndTwoVertices(graph, v1, v2)
	}
	
	fn shrink(&self) -> Box<Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		
		if self.0.all_vertices::<Vec<_>>().len() > 1 {
			/*	First we shrink the graph, using only the shrunk graphs where the vertices are valid
			*/
			result.extend(
				self.0.shrink()
					.filter(|g|{
						let verts: Vec<_> = g.all_vertices();
						verts.contains(&self.1) && verts.contains(&self.2)
					})
					.map(|g| ArbGraphAndTwoVertices(g, self.1, self.2))
			);
			
			/*	We shrink vertices by updating the graph's vertices too
			*/
			result.extend(
				// Get all the shrunk values
				self.1.shrink()
					// Remove any that match an existing edge in the graph
					.filter(|v| !self.0.all_vertices::<Vec<_>>().contains(&v))
					// Output the shrunk value by updating it in the graph and the vertex
					.map(|v| {
						let mut gClone= self.0.clone();
						gClone.replace_vertex(self.1, v);
						ArbGraphAndTwoVertices(gClone, v, self.2)
					})
			);
			// Do the same for the second vertex
			result.extend(
				self.2.shrink()
					.filter(|v| !self.0.all_vertices::<Vec<_>>().contains(&v))
					.map(|v| {
						let mut gClone= self.0.clone();
						gClone.replace_vertex(self.2, v);
						ArbGraphAndTwoVertices(gClone, self.1, v)
					})
			);
			
			// Lastly, if the graph has only 2 vertices, remove one and update the corresponding
			// vertex to the remaining on
			if self.0.all_vertices::<Vec<_>>().len() == 2 && self.0.all_edges::<Vec<_>>().len() == 0{
				let mut clone1 = self.0.clone();
				clone1.vertices.remove(0);
				let v = clone1.vertices[0].0;
				result.push(ArbGraphAndTwoVertices(clone1, v, v));
				
				let mut clone1 = self.0.clone();
				clone1.vertices.remove(1);
				let v = clone1.vertices[0].0;
				result.push(ArbGraphAndTwoVertices(clone1, v, v));
			}
		}
		
		Box::new(result.into_iter())
	}
	
}

///
/// An arbitrary graph and a vertex in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndVertex(pub MockGraph, pub MockVertex);
impl Arbitrary for ArbGraphAndVertex {
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let arb = ArbGraphAndTwoVertices::arbitrary(g);
		ArbGraphAndVertex(arb.0, arb.1)
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		Box::new(ArbGraphAndTwoVertices(self.0.clone(), self.1, self.1).shrink()
			.map(|ArbGraphAndTwoVertices(g, v, _)| ArbGraphAndVertex(g, v)))
	}
}

///
/// An arbitrary graph and a vertex that is guaranteed to not be in it.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndInvalidVertex(pub MockGraph, pub MockVertex);
impl Arbitrary for ArbGraphAndInvalidVertex {
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = MockGraph::arbitrary(g);
		let mut v = MockVertex::arbitrary(g);
		while graph.all_vertices::<Vec<_>>().contains(&v) {
			v = MockVertex::arbitrary(g)
		}
		
		ArbGraphAndInvalidVertex(graph, v)
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	First shrink the graph, keeping only the shrunk ones where the vertex
			stays invalid
		*/
		result.extend(
			self.0.shrink().filter(|g| !g.all_vertices::<Vec<_>>().contains(&self.1))
				.map(|g| ArbGraphAndInvalidVertex(g, self.1))
		);
		
		// We then shrink the vertex, keeping only the shrunk values
		// that are invalid in the graph
		let verts: Vec<_> = self.0.all_vertices();
		result.extend(
			self.1.shrink().filter(|v| !verts.contains(v))
				.map(|v| ArbGraphAndInvalidVertex(self.0.clone(), v))
		);
		
		Box::new(result.into_iter())
	}
}

///
/// An arbitrary graph and two vertices where at least one is not in the graph.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndInvalidEdge(pub MockGraph, pub MockVertex, pub MockVertex);
impl Arbitrary for ArbGraphAndInvalidEdge{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let single_invalid = ArbGraphAndInvalidVertex::arbitrary(g);
		Self(single_invalid.0, single_invalid.1, MockVertex::arbitrary(g))
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	Shrink the graph, keeping only the shrunk graphs where the edge is still invalid.
		*/
		result.extend(
			self.0.shrink().filter(|g| {
				let verts = g.all_vertices::<Vec<_>>();
				!verts.contains(&self.1) || !verts.contains(&self.2)
			})
			.map(|g| Self(g, self.1, self.2))
		);
		
		/*	We then shrink the vertices, ensuring that at least one of them stays invalid
		*/
		result.extend(
			self.1.shrink().filter(|v| {
				let verts = self.0.all_vertices::<Vec<_>>();
				!verts.contains(&v) || !verts.contains(&self.2)
			})
			.map(|v| Self(self.0.clone(), v, self.2))
		);
		result.extend(
			self.2.shrink().filter(|v| {
				let verts = self.0.all_vertices::<Vec<_>>();
				!verts.contains(&self.1) || !verts.contains(&v)
			})
				.map(|v| Self(self.0.clone(), self.1, v))
		);
		Box::new(result.into_iter())
	}
}