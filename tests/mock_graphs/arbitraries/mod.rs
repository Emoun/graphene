
mod mock_graph;

pub use self::{
	mock_graph::*,
};

use graphene::core::trait_aliases::Id;
use quickcheck::{Arbitrary, Gen};
use mock_graphs::{MockVertex, MockT, MockGraph};
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
		
		/*	A Graph with only 1 vertex can't be shrunk (as we cannot use one with no vertices).
			The vertices also can't be shrunk, since they both refer
		 	to the single vertex in the graph.
		*/
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
						let idx = self.0.vertices.iter().enumerate()
							.find(|(_,(v,_,_))| v.value == self.1.value).unwrap().0;
						gClone.vertices[idx].0 = v;
						ArbGraphAndTwoVertices(gClone, v, self.2)
					})
			);
			// Do the same for the second vertex
			result.extend(
				self.2.shrink()
					.filter(|v| !self.0.all_vertices::<Vec<_>>().contains(&v))
					.map(|v| {
						let mut gClone= self.0.clone();
						let idx = self.0.vertices.iter().enumerate()
							.find(|(_,(v,_,_))| v.value == self.2.value).unwrap().0;
						gClone.vertices[idx].0 = v;
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
