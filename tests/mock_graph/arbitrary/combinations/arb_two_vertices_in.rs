use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::{ImplGraph, Graph, ImplGraphMut, AddVertex};
use rand::Rng;

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
		Gr: Arbitrary + ImplGraphMut,
		Gr::Graph: AddVertex<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		// Create a graph with at least 1 vertex
		let arb_graph = {
			let mut candidate_graph = Gr::arbitrary(g);
			while candidate_graph.graph().all_vertices().count() == 0 {
				candidate_graph = Gr::arbitrary(g);
			}
			candidate_graph
		};
		let graph = arb_graph.graph();
		let verts: Vec<_> = graph.all_vertices().collect();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = verts[g.gen_range(0, verts.len())];
		
		ArbTwoVerticesIn(arb_graph, v1, v2)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		let arb_graph = &self.0;
		let graph = arb_graph.graph();
		if graph.all_vertices().count() > 1 {
			/*	First we shrink the graph, using only the shrunk graphs where the vertices are valid
			*/
			result.extend(
				arb_graph.shrink()
					.filter(|g|{
						let g = g.graph();
						g.contains_vertex(self.1) && g.contains_vertex(self.2)
					})
					.map(|g| ArbTwoVerticesIn(g, self.1, self.2))
			);
			// Lastly, if the graph has only 2 vertices and no edges, remove one and
			// update the corresponding vertex to the remaining one
			if graph.all_vertices().count() == 2 &&
				graph.all_edges().next().is_none()
			{
				let mut verts = graph.all_vertices();
				let v1 = verts.next().unwrap();
				let v2 = verts.next().unwrap();
				
				let mut clone = arb_graph.clone();
				clone.graph_mut().remove_vertex(v1).unwrap();
				result.push(ArbTwoVerticesIn(clone, v2, v2));
				
				let mut clone = arb_graph.clone();
				clone.graph_mut().remove_vertex(v2).unwrap();
				result.push(ArbTwoVerticesIn(clone, v1, v1));
			}
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