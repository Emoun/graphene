use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::{ImplGraph, Graph, GraphMut, ImplGraphMut, AddEdge, Edge, EdgeDeref, EdgeWeighted, RemoveEdge};
use crate::mock_graph::arbitrary::{GuidedArbGraph, ArbTwoVerticesIn};
use rand::Rng;

///
/// An arbitrary graph with an edge that is guaranteed to be in the graph (the weight is a clone)
///
#[derive(Clone, Debug)]
pub struct ArbEdgeIn<G>(pub G, pub (MockVertex, MockVertex, MockEdgeWeight))
	where
		G: Arbitrary + ImplGraph,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbEdgeIn<Gr>
	where
		Gr: GuidedArbGraph + ImplGraphMut,
		Gr::Graph: GraphMut<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight> + AddEdge + RemoveEdge
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let arb_graph = Gr::arbitrary_guided(g, .. , 1..);
		let graph = arb_graph.graph();
		let edge = graph.all_edges().nth(g.gen_range(0, graph.all_edges().count())).unwrap();
		let edge_clone = (edge.source(),edge.sink(),edge.weight().clone());
		Self(arb_graph, edge_clone)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		/*	First, we can simply shrink the weight
		*/
		result.extend(
			(self.1).2.shrink().map(|shrunk| {
				let mut clone = self.0.clone();
				let edge = clone.graph_mut().all_edges_mut()
					.find(|e|
						e.source() == self.1.source() &&
							e.sink() == self.1.sink() &&
							e.weight() == self.1.weight_ref()
					).unwrap().2;
				*edge = shrunk.clone();
				Self(clone, ((self.1).0, (self.1).1, shrunk))
			})
		);
		
		/* We shrink each vertex in the edge
		*/
		let mut without_edge = self.0.clone();
		without_edge.graph_mut().remove_edge_where(|e|
			e.source() == self.1.source() &&
				e.sink() == self.1.sink() &&
				e.weight() == self.1.weight_ref()
		).unwrap();
		result.extend(
			ArbTwoVerticesIn(without_edge, (self.1).0, (self.1).1).shrink()
				.map(|ArbTwoVerticesIn(mut g, v1, v2)| {
					g.graph_mut().add_edge_weighted((v1,v2,(self.1).2.clone())).unwrap();
					Self(g, (v1,v2,(self.1).2.clone()))
				})
		);
		
		Box::new(result.into_iter())
	}
}