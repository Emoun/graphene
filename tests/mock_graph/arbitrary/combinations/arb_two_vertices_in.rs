use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::{ImplGraph, Graph, ImplGraphMut, AddVertex};
use rand::Rng;
use crate::mock_graph::arbitrary::{GuidedArbGraph, Limit};
use std::collections::HashSet;

///
/// An arbitrary graph and two vertices in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbTwoVerticesIn<G>(pub G, pub MockVertex, pub MockVertex)
	where
		G: Arbitrary + ImplGraph,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;

impl<Gr> Arbitrary for ArbTwoVerticesIn<Gr>
	where
		Gr: GuidedArbGraph + ImplGraphMut,
		Gr::Graph: AddVertex<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		// Create a graph with at least 1 vertex
		let graph = Gr::arbitrary_guided(g, 1.., ..);
		let verts: Vec<_> = graph.graph().all_vertices().collect();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = verts[g.gen_range(0, verts.len())];
		
		ArbTwoVerticesIn(graph, v1, v2)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let arb_graph = &self.0;
		let graph = arb_graph.graph();
		
		/*	First we shrink the graph without touching the two designated vertices
		*/
		let mut limits = HashSet::new();
		limits.insert(Limit::VertexKeep(self.1));
		limits.insert(Limit::VertexKeep(self.2));
		result.extend(
			arb_graph.shrink_guided(limits)
				.map(|g| ArbTwoVerticesIn(g, self.1, self.2))
		);
		
		// Lastly, simply remove one of the vertices and use the other for both positions
		if self.1 != self.2 {
			let mut clone = arb_graph.clone();
			result.push(ArbTwoVerticesIn(clone, self.1, self.1));
			
			let mut clone = arb_graph.clone();
			result.push(ArbTwoVerticesIn(clone, self.2, self.2));
		}
		Box::new(result.into_iter())
	}
	
}
impl<G> ImplGraph for ArbTwoVerticesIn<G>
	where
		G: Arbitrary + ImplGraph,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	type Graph = G::Graph;
	
	fn graph(&self) -> &Self::Graph {
		self.0.graph()
	}
}
impl<G> ImplGraphMut for ArbTwoVerticesIn<G>
	where
		G: Arbitrary + ImplGraphMut,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self.0.graph_mut()
	}
}