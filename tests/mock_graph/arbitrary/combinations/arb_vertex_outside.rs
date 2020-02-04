use crate::mock_graph::{MockEdgeWeight, MockVertex, MockVertexWeight};
use graphene::core::Graph;
use quickcheck::{Arbitrary, Gen};

/// An arbitrary graph and a vertex that is guaranteed to not be in it.
#[derive(Clone, Debug)]
pub struct ArbVertexOutside<G>(pub G, pub MockVertex)
where
	G: Arbitrary
		+ Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVertexOutside<Gr>
where
	Gr: Arbitrary
		+ Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		let graph = Gr::arbitrary(g);
		let mut v = MockVertex::arbitrary(g);
		while graph.all_vertices().any(|existing| existing == v)
		{
			v = MockVertex::arbitrary(g)
		}

		ArbVertexOutside(graph, v)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();
		// 	First shrink the graph, keeping only the shrunk ones where the vertex
		// stays invalid
		result.extend(
			self.0
				.shrink()
				.filter(|g| !g.contains_vertex(self.1))
				.map(|g| ArbVertexOutside(g, self.1)),
		);

		// We then shrink the vertex, keeping only the shrunk values
		// that are invalid in the graph
		result.extend(
			self.1
				.shrink()
				.filter(|&v| self.0.contains_vertex(v))
				.map(|v| ArbVertexOutside(self.0.clone(), v)),
		);

		Box::new(result.into_iter())
	}
}
