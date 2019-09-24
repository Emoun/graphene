use graphene::core::constraint::UniqueGraph;
use graphene::core::{Directedness, Graph, Directed, AutoGraph, EdgeWeighted, Constrainer};
use crate::mock_graph::{MockGraph, MockVertex, MockVertexWeight, MockEdgeWeight};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use crate::mock_graph::arbitrary::max_vertex_count;
use delegate::delegate;

///
/// An arbitrary graph that is unique
///
#[derive(Clone, Debug)]
pub struct ArbUniqueGraph<D:Directedness  + Clone>(
	pub UniqueGraph<MockGraph<D>>
);
impl<D: Directedness + Clone> Graph for ArbUniqueGraph<D>
{
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	type Directedness = D;
	
	delegate! {
		target self.0 {
	
			fn all_vertices_weighted<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>;
		
			fn all_vertices_weighted_mut<'a>(&'a mut self)
				-> Box<dyn 'a +Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
			
			fn all_edges<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
			
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool ;
			
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
}

impl Arbitrary for ArbUniqueGraph<Directed>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let mut graph = MockGraph::empty();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0, max_vertex_count(g));
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			// Add all vertices to the graph
			for _ in 0..vertex_count {
				let v_weight = MockVertexWeight::arbitrary(g);
				graph.new_vertex_weighted(v_weight.clone()).unwrap();
			}
			// Collect vertices such that we don't borrow graph
			let verts: Vec<_>= graph.all_vertices().collect();
			
			/* For each vertex pair (in each direction), maybe create an edge
			 */
			let edge_saturation = g.gen_range(0.0, 1.0);
			for &source in verts.iter() {
				for &sink in verts.iter() {
					if g.gen_bool(edge_saturation) {
						graph.add_edge_weighted((source, sink, MockEdgeWeight::arbitrary(g))).unwrap();
					}
				}
			}
		}
		Self(UniqueGraph::unchecked(graph))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		
		Box::new(self.0.clone().unconstrain().shrink().map(|g| Self(UniqueGraph::unchecked(g))))
	}
}
