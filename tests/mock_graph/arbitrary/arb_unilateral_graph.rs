use graphene::core::constraint::UnilaterallyConnectedGraph;
use crate::mock_graph::{MockGraph, MockEdgeWeight, MockVertexWeight};
use graphene::core::{Directed, ImplGraph, ImplGraphMut, Constrainer, Graph, AddEdge, NewVertex, Edge};
use crate::mock_graph::arbitrary::{GuidedArbGraph, Limit, ArbEdgeIn};
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
		// If we are asked to make the singleton or empty graph, we just do
		if v_max <= 2 {
			return Self(UnilaterallyConnectedGraph::new(MockGraph::arbitrary_guided(g,
				v_min..v_max, e_min..e_max)))
		}
		
		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max {
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max)
		}
		
		let mut graph;
		// If the target is exactly 2 vertices, just create it
		if v_min == 2 {
			graph = MockGraph::empty();
			
			let v1 = graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
			let v2 = graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
			
			// Create a path between the edges to guarantee unilateralism.
			if g.gen_bool(0.5) {
				graph.add_edge_weighted((v1,v2,MockEdgeWeight::arbitrary(g))).unwrap();
			} else {
				graph.add_edge_weighted((v2,v1,MockEdgeWeight::arbitrary(g))).unwrap();
			}
			
			// Randomly add another edge
			if g.gen_bool(0.5) {
				if g.gen_bool(0.5) {
					graph.add_edge_weighted((v1,v2,MockEdgeWeight::arbitrary(g))).unwrap();
				} else {
					graph.add_edge_weighted((v2,v1,MockEdgeWeight::arbitrary(g))).unwrap();
				}
			}
			
		} else {
			// For larger graphs, we start by making a unilateral graph with 1 less vertex
			// and get an edge
			let ArbEdgeIn(graph, edge) = ArbEdgeIn::<Self>::
				arbitrary_guided(g, v_min-1..v_min, e_min..e_max);
			let mut graph = graph.0.unconstrain_single();
			
			// Add a new vertex to the graph
			let v = graph.new_vertex_weighted(MockVertexWeight::arbitrary(g)).unwrap();
			
			// Break the edge between the existing vertices, and substiture it with a path that
			// connects the existing vertices through the new vertex.
			
			unimplemented!()
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















































