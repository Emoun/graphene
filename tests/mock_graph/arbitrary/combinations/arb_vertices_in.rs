use std::collections::HashSet;
use crate::mock_graph::{MockVertex, MockEdgeWeight, MockVertexWeight};
use quickcheck::{Arbitrary, Gen};
use graphene::core::{ImplGraph, Graph, ImplGraphMut, AddVertex};
use rand::Rng;

///
/// An arbitrary graph and an arbitrary set of vertices in it.
///
#[derive(Clone, Debug)]
pub struct ArbVerticesIn<G>(pub G, pub HashSet<MockVertex>)
	where
		G: Arbitrary + ImplGraph,
		G::Graph: Graph<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>;
impl<Gr> Arbitrary for ArbVerticesIn<Gr>
	where
		Gr: Arbitrary + ImplGraphMut,
		Gr::Graph: AddVertex<Vertex=MockVertex, VertexWeight=MockVertexWeight,
			EdgeWeight=MockEdgeWeight>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = Gr::arbitrary(g);
		let v_count = graph.graph().all_vertices().count();
		
		let mut set = HashSet::new();
		
		if v_count > 0 {
			let v_expected = g.gen_range(0, v_count + 1);
			let v_saturation = v_expected as f64/v_count as f64;
			for v in graph.graph().all_vertices() {
				if g.gen_bool(v_saturation) {
					set.insert(v);
				}
			}
		}
		Self(graph, set)
	}
	
}