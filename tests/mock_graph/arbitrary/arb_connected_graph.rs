use graphene::core::{Directedness, EdgeWeighted, ExactGraph, Graph, ManualGraph, Edge, Directed, Constrainer, trait_aliases::*};
use graphene::core::constraint::ConnectedGraph;
use quickcheck::{Arbitrary, Gen};
use crate::mock_graph::{MockGraph, MockVertexWeight, MockVertex, MockEdgeWeight};
use rand::Rng;
use delegate::delegate;

///
/// Returns whether there is a path from the first vertex given to the second (on the given graph).
///
/// Does not check for whether the vertices actually are in the graph.
///
fn has_path_to(graph: &MockGraph<Directed>, start: MockVertex, end: MockVertex) -> bool
{
	fn dfs_rec(graph: &MockGraph<Directed>, start: MockVertex,
			   end: MockVertex, visited: &mut Vec<MockVertex>)
			   -> bool
	{
		if start == end {
			return true
		}
		visited.push(start);
		for e in graph.edges_sourced_in::<Vec<_>>(start).into_iter() {
			if !visited.contains(&e.sink()) {
				if dfs_rec(graph, e.sink(), end, visited) {
					return true
				}
			}
		}
		false
	}
	let mut visited = Vec::new();
	dfs_rec(graph, start, end, &mut visited)
}

fn is_connected(graph: &MockGraph<Directed>) -> bool
{
	let v_all = graph.all_vertices::<Vec<_>>();
	v_all.iter().flat_map(|&v| v_all.iter().map(move |&v_other| (v, v_other)))
		.all(|(v, v_other)| has_path_to(&graph, v, v_other))
}

///
/// An arbitrary graph that is connected
///
#[derive(Clone, Debug)]
pub struct ArbConnectedGraph<D: Directedness + Clone>(
	pub ConnectedGraph<MockGraph<D>>,
);
impl<D: Directedness + Clone> Graph for ArbConnectedGraph<D>
{
	type Vertex = MockVertex;
	type VertexWeight = MockVertexWeight;
	type EdgeWeight = MockEdgeWeight;
	type Directedness = D;
	
	delegate! {
		target self.0 {
	
			fn all_vertices<I: IntoFromIter<Self::Vertex>>(&self) -> I;
			
			fn vertex_weight(&self, v: Self::Vertex) -> Option<&Self::VertexWeight> ;
			
			fn vertex_weight_mut(&mut self, v: Self::Vertex) -> Option<&mut Self::VertexWeight>;
			
			fn all_edges<'a, I>(&'a self) -> I
				where I: EdgeIntoFromIter<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn all_edges_mut<'a, I>(&'a mut self) -> I
				where I: EdgeIntoFromIterMut<'a, Self::Vertex, Self::EdgeWeight>;
			
			fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
				where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>;
			
			fn remove_edge_where<F>(&mut self, f: F)
				-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;
		}
	}
}

impl<D: Directedness + Clone> ManualGraph for ArbConnectedGraph<D>
{
	delegate!{
		target self.0{
			fn add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight)
				-> Result<(), ()>;
				
			// We delegate this method because it maintains connectivity
			// The default implementation will call unimplemented methods
			fn replace_vertex(&mut self, to_replace: Self::Vertex, replacement: Self::Vertex)
					  -> Result<(),()>;
		}
	}
}

impl Arbitrary for ArbConnectedGraph<Directed>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		
		// Set the maximum amount of vertices and edges
		let mut graph = MockGraph::empty();
		
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0, g.size()/5);
		
		/* If the amount of vertices is 0, no edges can be created.
		 */
		if vertex_count > 0 {
			// Add all vertices to the graph
			while graph.vertex_count() < vertex_count {
				let v_weight = MockVertexWeight::arbitrary(g);
				let mut v_candidate = MockVertex::arbitrary(g);
				while graph.add_vertex_weighted(v_candidate,v_weight.clone()).is_err(){
					v_candidate = MockVertex::arbitrary(g);
				}
			}
			// Create a 'ring' with edges, ensuring the graph is connected
			let mut verts = graph.all_vertices::<Vec<_>>().into_iter();
			if let Some(mut v_prev) = verts.next() {
				for v_next in verts.chain(vec![v_prev]){
					graph.add_edge_weighted((v_prev, v_next, MockEdgeWeight::arbitrary(g))).unwrap();
					v_prev = v_next;
				}
			}
			
			// We now have a connected graph.
			// We add a random set of additional edges for good measure.
			let v_all = graph.all_vertices::<Vec<_>>();
			for _ in 0..g.gen_range(0, vertex_count+1) {
				let source = v_all[g.gen_range(0,v_all.len())];
				let sink = v_all[g.gen_range(0,v_all.len())];
				let e_weight = MockEdgeWeight::arbitrary(g);
				graph.add_edge_weighted((source, sink, e_weight)).unwrap();
			}
			
			// We now try to remove a random number of edges yet preserve connectedness
			// If an edge can't be removed, we don't care
			for _ in 0..g.gen_range(0, vertex_count*2) {
				let source = v_all[g.gen_range(0,v_all.len())];
				let sink = v_all[g.gen_range(0,v_all.len())];
				if let Ok(e_weight) = graph.remove_edge((source, sink)) {
					// We check that all vertices still have paths to each other.
					// If not, we return the edge
					if !is_connected(&graph)	{
						graph.add_edge_weighted((source, sink, e_weight)).unwrap();
					}
				}
			}
		}
		assert!(is_connected(&graph));
		println!("ConnectedGraph created.");
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
		if self.all_vertices::<Vec<_>>().len() > 1 {
			result.extend(
				self.all_vertices::<Vec<_>>().into_iter()
					.filter(|&v| self.edges_sourced_in::<Vec<_>>(v).len() == 1 &&
						self.edges_sinked_in::<Vec<_>>(v).len() == 1)
					.flat_map(|v| {
						let mut clone = self.0.clone().unconstrain_single();
						let e_in = self.edges_sinked_in::<Vec<_>>(v)[0];
						let e_out = self.edges_sourced_in::<Vec<_>>(v)[0];
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