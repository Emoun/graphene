use graphene::core::{Directedness, EdgeWeighted, Graph, Edge, Constrainer, RemoveVertex, AddEdge, ImplGraph, ImplGraphMut, EdgeDeref};
use graphene::core::constraint::{ConnectedGraph, DirectedGraph};
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockGraph, MockVertex, MockEdgeWeight};
use rand::Rng;
use crate::mock_graph::arbitrary::{GuidedArbGraph, Limit};
use std::collections::{HashSet};
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
	let v_all = graph.all_vertices().collect::<Vec<_>>();
	let v_count = v_all.len();
	
	if v_count <= 1 {
		// Trivial cases.
		return true
	}
	
	if let Ok(graph) = <DirectedGraph<&MockGraph<D>>>::constrain(graph) {
		
		v_all.iter().flat_map(|&v| v_all.iter().map(move |&v_other| (v, v_other)))
			.all(|(v, v_other)| {
				let mut visited = Vec::new();
				dfs_rec(&graph, v, Some(v_other), &mut visited)
			})
	} else {
		let mut visited = Vec::new();
		dfs_rec(graph, v_all[0], None, &mut visited);
		visited.len() == v_count
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
		let v_map = graph.join(&g2);
		
		// Add edges connecting the two components
		graph.add_edge_weighted((v11,v_map[&v21], MockEdgeWeight::arbitrary(g))).unwrap();
		if D::directed() {
			graph.add_edge_weighted((v_map[&v22],v12, MockEdgeWeight::arbitrary(g))).unwrap();
		}
		
		Self(ConnectedGraph::new(graph))
	}
	
	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item=Self>>
	{
		let mut result = Vec::new();
		
		// Shrink by removing any edge that isn't critical for connectedness
		if	!limits.contains(&Limit::EdgeRemove) &&
			!limits.contains(&Limit::EdgeMin(self.0.all_edges().count()))
		{
			let mut count = 0;
			result.extend(
				self.0.all_edges()
					.map(|e| {
						let mut g = self.0.clone().unconstrain_single();
						g.remove_edge_where_weight(e, |w| w == e.weight()).unwrap();
						g
					})
					.filter(|g| {if !is_connected(&g) {count +=1; false} else { true } })
			);
		}
		
		// Shrink by removing vertices and replacing them with edges between
		// any of their neighbors
		if	!limits.contains(&Limit::VertexRemove) &&
			!limits.contains(&Limit::EdgeRemove) &&
			!limits.contains(&Limit::VertexMin(self.0.all_vertices().count()))
		{
			for v in self.0.all_vertices().filter(|v| !limits.contains(&Limit::VertexKeep(*v))) {
				let mut clone = self.0.clone().unconstrain_single();
				clone.remove_vertex(v).unwrap();
				if let Ok(g) = DirectedGraph::constrain_single(&self.0) {
					for (sink, w1) in g.edges_sourced_in(v).map(|e| (e.sink(), e.weight_owned())) {
						if sink == v { continue }
						for (source, w2) in g.edges_sinked_in(v).map(|e| (e.source(), e.weight_owned())) {
							if source == v { continue }
							clone.add_edge_weighted((source, sink, w1.clone())).unwrap();
							clone.add_edge_weighted((source, sink, w2.clone())).unwrap();
						}
					}
				} else {
					let neighbors: Vec<_> = self.0.edges_incident_on(v)
						.map(|e| (e.other(v), e.weight_owned())).collect();
					let mut neighbor_iter = neighbors.iter();
					while let Some(&(v1, w1)) = neighbor_iter.next() {
						if v1 == v { continue }
						let rest = neighbor_iter.clone();
						for &(v2, w2) in rest {
							if v2 == v { continue }
							clone.add_edge_weighted((v1, v2, w1.clone())).unwrap();
							clone.add_edge_weighted((v1, v2, w2.clone())).unwrap();
						}
					}
				}
				result.push(clone);
			}
		}
		
		// We shrink the MockGraph values, not removing any vertices or edges
		limits.insert(Limit::EdgeRemove);
		limits.insert(Limit::VertexRemove);
		result.extend(
			self.0.clone().unconstrain_single().shrink_guided(limits)
		);
		Box::new(result.into_iter().map(|g| Self(ConnectedGraph::new(g))))
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
		self.shrink_guided(HashSet::new())
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
		
		// Join the two graph into one graph with no edges between the two parts
		graph.join(&g2);
		
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