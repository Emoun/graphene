use crate::mock_graph::{MockVertex, MockVertexWeight, MockEdgeWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::{ImplGraph, Graph, ImplGraphMut, AddVertex};
use crate::mock_graph::arbitrary::{ArbTwoVerticesIn, GuidedArbGraph};

///
/// An arbitrary graph and a vertex in it.
///
/// Note: All graphs will have at least 1 vertex, meaning this type never includes
/// the empty graph.
///
#[derive(Clone, Debug)]
pub struct ArbVertexIn<G>(pub G, pub MockVertex)
	where
		G: Arbitrary + ImplGraph,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVertexIn<Gr>
	where
		Gr: GuidedArbGraph + ImplGraphMut,
		Gr::Graph: AddVertex<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let arb = ArbTwoVerticesIn::arbitrary(g);
		ArbVertexIn(arb.0, arb.1)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		Box::new(ArbTwoVerticesIn(self.0.clone(), self.1, self.1).shrink()
			.map(|ArbTwoVerticesIn(g, v, _)| ArbVertexIn(g, v)))
	}
}


impl<G> ImplGraph for ArbVertexIn<G>
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
impl<G> ImplGraphMut for ArbVertexIn<G>
	where
		G: Arbitrary + ImplGraphMut,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self.0.graph_mut()
	}
}