
use graphene::core::trait_aliases::Id;
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{
	MockVertex, MockT, MockGraph, MockEdgeWeight, MockVertexWeight
};
use graphene::core::{Graph, Directedness};
use rand::{ Rng };
use std::marker::PhantomData;

impl Arbitrary for MockVertex
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

impl Arbitrary for MockT
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self{value: u32::arbitrary(g)}
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		Box::new(self.value.shrink().map(|v| Self{value: v}))
	}
}

impl<D> Arbitrary for MockGraph<D>
	where D: Directedness + Clone + Send + 'static
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let COUNT = 10;
		let mut vertices:  Vec<(MockVertex, MockVertexWeight, _)> = Vec::new();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0,COUNT);
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			//Decide the amount of edges
			let edge_count = g.gen_range(0, COUNT);
			
			/* Create vertex values ensuring that
			 * each vertex has a unique value
			 */
			let mut next_value = MockVertex::arbitrary(g);
			//For each vertex
			for _ in 0..vertex_count {
				// As long as the created value is already used by another vertex
				while vertices.iter().any( |&(id,_,_)| id.value == next_value.value) {
					// Create a new value
					next_value = MockVertex::arbitrary(g);
				}
				vertices.push((next_value, MockVertexWeight::arbitrary(g), Vec::new()));
			}
			
			/* Create edges
			 */
			//For each edge
			for _ in 0..edge_count {
				/* Create a valid edge
				 */
				let t_source = usize::arbitrary(g) % vertex_count;
				let t_sink = usize::arbitrary(g) % vertex_count;
				let t_weight = MockEdgeWeight::arbitrary(g);
				vertices[t_source].2.push((t_sink, t_weight));
			}
		}
		Self::new(vertices)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		
		/* Base case
		 */
		if self.vertices.len() == 0 {
			return Box::new(std::iter::empty());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertices
		 */
		//For each vertex
		self.vertices.iter().enumerate()
			//Get all possible shrinkages
			.flat_map(|(idx,(id,_,_))| id.shrink().map(move|s| (idx,s)))
			//Remove any that are equal to existing vertices
			.filter(|(_,shrunk_id)|
				self.vertices.iter().all(|(id,_,_)| shrunk_id.value != id.value))
			//copy the graph, and change the id to the shrunk id
			.for_each(|(idx, shrunk_id)| {
				let mut new_id = self.vertices.clone();
				new_id[idx].0 = shrunk_id;
				result.push(Self::new(new_id));
			});
		
		/* Shrink by shrinking vertex weight
		 */
		self.vertices.iter().enumerate()
			//Get all possible shrinkages
			.flat_map(|(idx, (_,weight,_))| weight.shrink().map(move|s| (idx,s)))
			//For each shrunk weight,
			//create a new graph where the vertex has that weight
			.for_each(|(idx, shrunk_weight)|{
				let mut new_graph = self.clone();
				new_graph.vertices[idx].1 = shrunk_weight;
				result.push(new_graph);
			});
		
		/* Shrink by shrinking edge weight
		 */
		//For each edge
		self.all_edges::<Vec<_>>().into_iter().for_each(|(source,sink,ref weight)|{
			let shrunk_weights = weight.shrink();
			
			shrunk_weights.for_each( |s_w| {
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_edge_where_weight((source, sink),
													  |ref w| w.value == weight.value
				).unwrap();
				shrunk_graph.add_edge_weighted((source, sink, s_w)).unwrap();
				result.push(shrunk_graph);
			});
		});
		
		/* Shrink by removing an edge
		 */
		//For each edge
		for e in self.all_edges::<Vec<_>>(){
			/* Add to the result a copy of the graph
			 * without the edge
			 */
			let mut shrunk_graph = self.clone();
			shrunk_graph.remove_edge(e).unwrap();
			result.push(shrunk_graph);
		}
		
		/* Shrink by removing a vertex that has no edges.
		 * We don't remove any edges in this step (to be able to remove a vertex)
		 * because we are already shrinking by removing edges, which means, there
		 * should be a set of edge shrinkages that result in a removable vertex.
		 */
		for v in self.all_vertices::<Vec<_>>(){
			let edges: Vec<_> = self.edges_incident_on(v);
			if edges.len() == 0 {
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_vertex(v).unwrap();
				result.push(shrunk_graph);
			}
		}
		
		Box::new(result.into_iter())
	}
}

///
/// An arbitrary graph and two vertices in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndTwoVertices<D: Directedness + Clone>(pub MockGraph<D>, pub MockVertex, pub MockVertex);
impl<D> Arbitrary for ArbGraphAndTwoVertices<D>
	where D: Directedness + Clone + Send + 'static
{
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
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
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
pub struct ArbGraphAndVertex<D: Directedness + Clone>(pub MockGraph<D>, pub MockVertex);
impl<D> Arbitrary for ArbGraphAndVertex<D>
	where D: Directedness + Clone + Send + 'static
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let arb = ArbGraphAndTwoVertices::arbitrary(g);
		ArbGraphAndVertex(arb.0, arb.1)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		Box::new(ArbGraphAndTwoVertices(self.0.clone(), self.1, self.1).shrink()
			.map(|ArbGraphAndTwoVertices(g, v, _)| ArbGraphAndVertex(g, v)))
	}
}

///
/// An arbitrary graph and a vertex that is guaranteed to not be in it.
///
#[derive(Clone, Debug)]
pub struct ArbGraphAndInvalidVertex<D: Directedness + Clone>(pub MockGraph<D>, pub MockVertex);
impl<D> Arbitrary for ArbGraphAndInvalidVertex<D>
	where D: Directedness + Clone + Send + 'static
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = MockGraph::arbitrary(g);
		let mut v = MockVertex::arbitrary(g);
		while graph.all_vertices::<Vec<_>>().contains(&v) {
			v = MockVertex::arbitrary(g)
		}
		
		ArbGraphAndInvalidVertex(graph, v)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
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
pub struct ArbGraphAndInvalidEdge<D: Directedness + Clone>(pub MockGraph<D>, pub MockVertex, pub MockVertex);
impl<D> Arbitrary for ArbGraphAndInvalidEdge<D>
	where D: Directedness + Clone + Send + 'static
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let single_invalid = ArbGraphAndInvalidVertex::arbitrary(g);
		Self(single_invalid.0, single_invalid.1, MockVertex::arbitrary(g))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
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