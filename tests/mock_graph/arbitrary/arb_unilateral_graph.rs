use graphene::core::constraint::UnilaterallyConnectedGraph;
use crate::mock_graph::{MockGraph, MockEdgeWeight, MockVertexWeight};
use graphene::core::{Directed, ImplGraph, ImplGraphMut, RemoveEdge, Constrainer, Graph, AddEdge, NewVertex, Edge};
use crate::mock_graph::arbitrary::{GuidedArbGraph, Limit, ArbEdgeIn, ArbTwoVerticesIn, ArbVertexIn};
use std::collections::hash_map::RandomState;
use std::ops::RangeBounds;
use std::collections::HashSet;
use quickcheck::{Gen, Arbitrary};
use rand::Rng;

///
/// An arbitrary graph that is unilaterally connected
///
#[derive(Clone, Debug)]
pub struct ArbUnilatralGraph(pub UnilaterallyConnectedGraph<MockGraph<Directed>>);

impl ImplGraph for ArbUnilatralGraph
{
	type Graph = UnilaterallyConnectedGraph<MockGraph<Directed>>;
	
	fn graph(&self) -> &Self::Graph {
		&self.0
	}
}
impl ImplGraphMut for ArbUnilatralGraph
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		&mut self.0
	}
}

impl GuidedArbGraph for ArbUnilatralGraph
{
	fn arbitrary_guided<G: Gen>(g: &mut G, v_range: impl RangeBounds<usize>,
								e_range: impl RangeBounds<usize>) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		// If we are asked to make the empty graph, we just do
		if v_max <= 1{
			return Self(UnilaterallyConnectedGraph::new(MockGraph::arbitrary_guided(g,
				v_min..v_max, v_min..v_max)))
		}
		
		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max {
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max)
		}
		
		let mut graph;
		let v;
		// If the target is exactly 1 vertex, just create it
		if v_min == 1 {
			graph = MockGraph::empty();
			v = graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
		} else {
			// For larger graphs, we start by making a unilateral graph with 1 less vertex
			// and get an edge
			let arb = ArbVertexIn::<Self>::arbitrary_guided(g, v_min-1..v_min, e_min..e_max);
			graph = (arb.0).0.unconstrain_single();
			let v_original = arb.1;
			
			// Add a new vertex to the graph
			v = graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
			
			// Add an edge to/from the new and old vertices
			if g.gen_bool(0.5) {
				// To ensure unilateralism, take all outgoing edges from the original vertex
				// and move them to the new one.
				let outgoing_sinks  = graph.edges_sourced_in(v_original)
					.map(|e|  e.sink()).collect::<Vec<_>>();
				for sink in outgoing_sinks {
					let weight = graph.remove_edge((v_original, sink)).unwrap();
					graph.add_edge_weighted((v, sink, weight)).unwrap();
				}
				
				graph.add_edge_weighted((v_original, v, MockEdgeWeight::arbitrary(g))).unwrap();
			} else {
				// To ensure unilateralism, take all the incoming edges from the original vertex
				// and move them to the new one.
				let sources  = graph.edges_sinked_in(v_original)
					.map(|e|  e.source()).collect::<Vec<_>>();
				for source in sources {
					let weight = graph.remove_edge((source, v_original)).unwrap();
					graph.add_edge_weighted((source, v, weight)).unwrap();
				}
				
				graph.add_edge_weighted((v, v_original, MockEdgeWeight::arbitrary(g))).unwrap();
				
			}
		}
		
		// We now randomly create additional edges from/to the new vertex
		let p = 0.5/(v_min as f64);
		for v_other in graph.all_vertices().collect::<Vec<_>>() {
			if g.gen_bool(p) {
				graph.add_edge_weighted((v, v_other, MockEdgeWeight::arbitrary(g))).unwrap();
			}
			if g.gen_bool(p) {
				graph.add_edge_weighted((v_other, v, MockEdgeWeight::arbitrary(g))).unwrap();
			}
		}
		
		Self(UnilaterallyConnectedGraph::new(graph))
	}
	
	fn shrink_guided(&self, _limits: HashSet<Limit, RandomState>) -> Box<dyn Iterator<Item=Self>> {
		unimplemented!()
	}
}

impl Arbitrary for ArbUnilatralGraph
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = Self::arbitrary_guided(g, .., ..).0.unconstrain_single();
		Self(UnilaterallyConnectedGraph::new(graph))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		self.shrink_guided(HashSet::new())
	}
}















































