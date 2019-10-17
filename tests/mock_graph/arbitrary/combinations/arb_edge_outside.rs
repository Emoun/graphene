use crate::mock_graph::{MockVertex, MockVertexWeight, MockEdgeWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::Graph;
use crate::mock_graph::arbitrary::ArbVertexOutside;

///
/// An arbitrary graph and two vertices where at least one is not in the graph.
///
#[derive(Clone, Debug)]
pub struct ArbEdgeOutside<G>(pub G, pub MockVertex, pub MockVertex)
	where
		G: Arbitrary + Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbEdgeOutside<Gr>
	where
		Gr: Arbitrary + Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let single_invalid = ArbVertexOutside::arbitrary(g);
		Self(single_invalid.0, single_invalid.1, MockVertex::arbitrary(g))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	Shrink the graph, keeping only the shrunk graphs where the edge is still invalid.
		*/
		result.extend(
			self.0.shrink().filter(|g| {
				!g.contains_vertex(self.1) || !g.contains_vertex(self.2)
			})
				.map(|g| Self(g, self.1, self.2))
		);
		
		/*	We then shrink the vertices, ensuring that at least one of them stays invalid
		*/
		result.extend(
			self.1.shrink().filter(|v| {
				!self.0.contains_vertex(*v) || !self.0.contains_vertex(self.2)
			})
				.map(|v| Self(self.0.clone(), v, self.2))
		);
		result.extend(
			self.2.shrink().filter(|v| {
				!self.0.contains_vertex(self.1) || !self.0.contains_vertex(*v)
			})
				.map(|v| Self(self.0.clone(), self.1, v))
		);
		Box::new(result.into_iter())
	}
}