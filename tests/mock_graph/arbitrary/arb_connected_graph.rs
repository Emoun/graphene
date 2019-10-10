use graphene::core::{Directedness, EdgeWeighted, Graph, Edge, Constrainer, AddVertex, AddEdge, ImplGraph, ImplGraphMut};
use graphene::core::constraint::{ConnectedGraph, DirectedGraph};
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockGraph, MockVertex, MockEdgeWeight};
use rand::Rng;
use crate::mock_graph::arbitrary::GuidedArbGraph;
use std::collections::HashMap;
use std::ops::RangeBounds;

fn dfs_rec<G: Graph<Vertex=MockVertex>>(graph: &G, start: MockVertex,
	end: Option<MockVertex>, visited: &mut Vec<MockVertex>)
	-> bool
{
	if let Some(end) = end {
		if start == end {
			return true
		}
	}
	visited.push(start);
	if G::Directedness::directed() {
		for e in graph.edges_incident_on(start).filter(|e| e.source() == start) {
			if !visited.contains(&e.sink()) {
				if dfs_rec(graph, e.sink(), end, visited) {
					return true //early return of 'end' is found
				}
			}
		}
	} else {
		for e in graph.edges_incident_on(start) {
			let v_other = if e.source() == start {e.sink()} else {e.source()};
			if !visited.contains(&v_other) {
				dfs_rec(graph, v_other, end, visited);
			}
		}
	}
	false
}

/// This is very inefficient (but never the less correct)
fn is_connected<D: Directedness>(graph: &MockGraph<D>) -> bool
{
	if let Ok(graph) = <DirectedGraph<&MockGraph<D>>>::constrain(graph) {
		let v_all = graph.all_vertices().collect::<Vec<_>>();
		v_all.iter().flat_map(|&v| v_all.iter().map(move |&v_other| (v, v_other)))
			.all(|(v, v_other)| {
				let mut visited = Vec::new();
				dfs_rec(&graph, v, Some(v_other), &mut visited)
			})
	} else {
		let v_count = graph.all_vertices().count();
		if v_count > 0 {
			let mut visited = Vec::new();
			dfs_rec(graph, graph.all_vertices().next().unwrap(), None, &mut visited);
			visited.len() == v_count
		} else {
			true
		}
	}
}

///
/// An arbitrary graph that is connected
///
#[derive(Clone, Debug)]
pub struct ArbConnectedGraph<D: Directedness>(
	pub ConnectedGraph<MockGraph<D>>,
);

impl<D: Directedness> ImplGraph for ArbConnectedGraph<D>
{
	type Graph = ConnectedGraph<MockGraph<D>>;
	
	fn graph(&self) -> &Self::Graph {
		&self.0
	}
}
impl<D: Directedness> ImplGraphMut for ArbConnectedGraph<D>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		&mut self.0
	}
}

impl<D: Directedness> GuidedArbGraph for ArbConnectedGraph<D>
{
	fn arbitrary_guided<G: Gen>(g: &mut G, v_range: impl RangeBounds<usize>,
								e_range: impl RangeBounds<usize>)
								-> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);
		// If we are asked to make the singleton or empty graph, we just do
		if v_max <= 2 {
			return Self(ConnectedGraph::new(MockGraph::arbitrary_guided(g,
				v_min..v_max, e_min..e_max)))
		}
		
		// If the exact size of the graph hasn't been decided yet, do so.
		if (v_min + 1) != v_max {
			let v_count = g.gen_range(v_min, v_max);
			return Self::arbitrary_guided(g, v_count..v_count + 1, e_min..e_max)
		}
		
		let (v_count_1, v_count_2) = ((v_min/2) + (v_min%2), v_min/2);
		let e_min_2 = e_min / 2;
		let e_min_1 = e_min_2 + (e_min%2);
		let e_max_2 = e_max / 2;
		let e_max_1 = e_max_2 + (e_max%2);
		
		// We create two smaller connected graphs
		let mut graph = Self::arbitrary_guided(g, v_count_1..v_count_1+1, e_min_1..e_max_1).0.unconstrain_single();
		let g2 = Self::arbitrary_guided(g, v_count_2..v_count_2+1, e_min_2..e_max_2).0;
		
		// We find random vertices in each graph for later use
		let v11 = graph.all_vertices().nth(g.gen_range(0, graph.all_vertices().count())).unwrap();
		let v12 = graph.all_vertices().nth(g.gen_range(0, graph.all_vertices().count())).unwrap();
		let v21 = g2.all_vertices().nth(g.gen_range(0, g2.all_vertices().count())).unwrap();
		let v22 = g2.all_vertices().nth(g.gen_range(0, g2.all_vertices().count())).unwrap();
		
		// Join the second into the first making an unconnected graph with the 2 components
		let mut v_map: HashMap<MockVertex, MockVertex> = HashMap::new();
		for (v,w) in g2.all_vertices_weighted() {
			let new_v = graph.new_vertex_weighted(w.clone()).unwrap();
			v_map.insert(v, new_v);
		}
		for (so,si, w) in g2.all_edges() {
			graph.add_edge_weighted((v_map[&so], v_map[&si], w.clone())).unwrap();
		}
		
		// Add vertices connecting the two components
		graph.add_edge_weighted((v11,v_map[&v21], MockEdgeWeight::arbitrary(g))).unwrap();
		if D::directed() {
			graph.add_edge_weighted((v_map[&v22],v12, MockEdgeWeight::arbitrary(g))).unwrap();
		}
		
		Self(ConnectedGraph::new(graph))
	}
}

impl<D: Directedness> Arbitrary for ArbConnectedGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let graph = Self::arbitrary_guided(g, .., ..).0.unconstrain_single();
//		assert!(is_connected(&graph));
		Self(ConnectedGraph::new(graph))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		
		// We shrink the MockGraph, keeping only the shrunk graphs that are still connected
		result.extend(
			self.0.clone().unconstrain_single().shrink().filter( |g| is_connected(&g))
				.map(|g| Self(ConnectedGraph::new(g)))
		);
		
		// We also shrink by replacing any vertex with in- and outdegree of 1 with an edge
		if self.0.all_vertices().count() > 1 {
			result.extend(
				self.0.all_vertices().filter(|&v| {
					if let Ok(g) = <DirectedGraph<&<Self as ImplGraph>::Graph>>::constrain(&(self.0)) {
						g.edges_sourced_in(v).count() == 1 &&
							g.edges_sinked_in(v).count() == 1
					} else {
						self.0.edges_incident_on(v).count() == 2
					}
				})
				.flat_map(|v| {
					let mut clone = self.0.clone().unconstrain_single();
					let (e_in, e_out)=
						if let Ok(g) = <DirectedGraph<&<Self as ImplGraph>::Graph>>::constrain(&self.0) {
							(g.edges_sinked_in(v).next().unwrap().split().0,
							 g.edges_sourced_in(v).next().unwrap().split().0)
						} else {
							let mut edges = self.0.edges_incident_on(v);
							let v1 = edges.next().unwrap().split().0.other(v);
							let v2 = edges.next().unwrap().split().0.other(v);
							((v1, v), (v, v2))
						};
					let weight1 = clone.remove_edge(e_in).unwrap();
					let weight2 = clone.remove_edge(e_out).unwrap();
					clone.remove_vertex(v).unwrap();
					
					let mut clone2 = clone.clone();
					clone.add_edge_weighted((e_in.source(), e_out.sink(), weight1)).unwrap();
					clone2.add_edge_weighted((e_in.source(), e_out.sink(), weight2)).unwrap();

					assert!(is_connected(&clone));
					assert!(is_connected(&clone2));
					
					vec![Self(ConnectedGraph::new(clone)),
						 Self(ConnectedGraph::new(clone2))].into_iter()
				})
			);
		}
		Box::new(result.into_iter())
	}
}

///
/// An arbitrary graph that is connected
///
#[derive(Clone, Debug)]
pub struct ArbUnconnectedGraph<D: Directedness>(
	pub MockGraph<D>,
);

impl<D: Directedness> ImplGraph for ArbUnconnectedGraph<D>
{
	type Graph = MockGraph<D>;
	
	fn graph(&self) -> &Self::Graph {
		&self.0
	}
}
impl<D: Directedness> ImplGraphMut for ArbUnconnectedGraph<D>
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		&mut self.0
	}
}

impl<D: Directedness> Arbitrary for ArbUnconnectedGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// We merge 2 graphs into 1. This will ensure they are not connected.
		// They must each have at least 1 vertex, otherwise the result
		// might be trivially connected (<2 vertices)
		let mut graph = MockGraph::arbitrary_guided(g, 1..(g.size()/2), ..);
		let g2 = <MockGraph<D>>::arbitrary_guided(g, 1..(g.size()/2), ..);
		
		// Map of vertices in 'g2', to their new counterparts in 'graph'
		let mut v_map: HashMap<MockVertex,MockVertex> = HashMap::new();
		for (v,w) in g2.all_vertices_weighted() {
			let new_v = graph.new_vertex_weighted(w.clone()).unwrap();
			v_map.insert(v, new_v);
		}
		for (so,si, w) in g2.all_edges() {
			graph.add_edge_weighted((v_map[&so], v_map[&si], w.clone())).unwrap();
		}
		
		assert!(!is_connected(&graph));
		Self(graph)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		let mut result = Vec::new();
		
		// We shrink the MockGraph, keeping only the shrunk graphs that are still unconnected
		result.extend(
			self.0.shrink().filter( |g| !is_connected(&g))
				.map(|g| Self(g))
		);
		
		Box::new(result.into_iter())
	}
}