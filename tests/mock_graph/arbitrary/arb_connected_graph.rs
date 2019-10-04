use graphene::core::{Directedness, EdgeWeighted, Graph, Edge, Constrainer, AddVertex, GraphMut, AddEdge,};
use graphene::core::constraint::{ConnectedGraph, DirectedGraph};
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockGraph, MockVertexWeight, MockVertex, MockEdgeWeight};
use rand::Rng;
use delegate::delegate;
use crate::mock_graph::arbitrary::GuidedArbGraph;

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
impl<D: Directedness> Graph for ArbConnectedGraph<D>
{
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	type Directedness = D;
	
	delegate! {
		target self.0 {
	
			fn all_vertices_weighted<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>;
		
			fn all_edges<'a>(&'a self)
				-> Box<dyn 'a + Iterator<Item=(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>;
			
		}
	}
}

impl<D: Directedness> GraphMut for ArbConnectedGraph<D>
{
	delegate! {
		target self.0 {
			fn all_vertices_weighted_mut<'a>(&'a mut self)
				-> Box<dyn 'a +Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>;
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>;
			
		}
	}
}

impl<D: Directedness> AddVertex for ArbConnectedGraph<D>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
				
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
		}
	}
}

impl<D: Directedness> AddEdge for ArbConnectedGraph<D>
{
	delegate! {
		target self.0 {
	
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool ;
			
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
		}
	}
}

impl<D: Directedness> Arbitrary for ArbConnectedGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		let mut graph = MockGraph::arbitrary_guided(g, .., ..(2*g.size()));
		let vertex_count = graph.all_vertices().count();
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			// Create a 'ring' with edges, ensuring the graph is connected
			let mut verts= graph.all_vertices().collect::<Vec<_>>().into_iter(); // Collect such that we no longer borrow graph
			if let Some(mut v_prev) = verts.next() {
				for v_next in verts.chain(vec![v_prev]){
					graph.add_edge_weighted((v_prev, v_next, MockEdgeWeight::arbitrary(g))).unwrap();
					v_prev = v_next;
				}
			}
			
			// We now try to remove a random number of edges yet preserve connectedness
			// If an edge can't be removed, we don't care
			for e in graph.all_edges().map(|e| (e.source(), e.sink())).collect::<Vec<_>>().into_iter() {
				if g.gen_bool(0.5) {
					if let Ok(e_weight) = graph.remove_edge(e) {
						// We check that all vertices still have paths to each other.
						// If not, we return the edge
						if !is_connected(&graph)	{
							graph.add_edge_weighted((e.source(), e.sink(), e_weight)).unwrap();
						}
					}
				}
			}
		}
		assert!(is_connected(&graph));
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
		if self.all_vertices().count() > 1 {
			result.extend(
				self.all_vertices().filter(|&v| {
					if let Ok(g) = <DirectedGraph<&Self>>::constrain(self) {
						g.edges_sourced_in(v).count() == 1 &&
							g.edges_sinked_in(v).count() == 1
					} else {
						self.edges_incident_on(v).count() == 2
					}
				})
					.flat_map(|v| {
						let mut clone = self.0.clone().unconstrain_single();
						let (e_in, e_out)=
							if let Ok(g) = <DirectedGraph<&Self>>::constrain(self) {
								(g.edges_sinked_in(v).next().unwrap().split().0,
								 g.edges_sourced_in(v).next().unwrap().split().0)
							} else {
								let mut edges = self.edges_incident_on(v);
								(edges.next().unwrap().split().0, edges.next().unwrap().split().0)
							};
						let weight1 = clone.remove_edge(e_in).unwrap();
						let weight2 = clone.remove_edge(e_out).unwrap();
						clone.remove_vertex(v).unwrap();
						
						let mut clone2 = clone.clone();
						
						clone.add_edge_weighted((e_in.source(), e_out.sink(), weight1)).unwrap();
						clone2.add_edge_weighted((e_in.source(), e_out.sink(), weight2)).unwrap();

//					assert!(is_connected(&clone));
//					assert!(is_connected(&clone2));
						
						vec![Self(ConnectedGraph::new(clone)),
							 Self(ConnectedGraph::new(clone2))].into_iter()
					})
			);
		}
		Box::new(result.into_iter())
	}
}