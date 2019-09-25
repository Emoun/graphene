use graphene::core::constraint::UniqueGraph;
use graphene::core::{Directedness, Graph, AutoGraph, Edge, EdgeWeighted, Constrainer};
use crate::mock_graph::{MockGraph, MockVertex, MockVertexWeight, MockEdgeWeight};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use crate::mock_graph::arbitrary::max_vertex_count;
use delegate::delegate;

///
/// An arbitrary graph that is unique
///
#[derive(Clone, Debug)]
pub struct ArbUniqueGraph<D:Directedness>(
	pub UniqueGraph<MockGraph<D>>
);

impl<D: Directedness> Graph for ArbUniqueGraph<D>
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

impl<D: Directedness> Arbitrary for ArbUniqueGraph<D>
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
			let mut maybe_add_edge = |source, sink|{
				if g.gen_bool(edge_saturation) {
					graph.add_edge_weighted((source, sink, MockEdgeWeight::arbitrary(g))).unwrap();
				}
			};
			if D::directed() {
				for &source in verts.iter() {
					for &sink in verts.iter() {
						maybe_add_edge(source, sink)
					}
				}
			} else {
				let mut iter = verts.iter();
				let mut iter_rest = iter.clone();
				while let Some(&source) = iter.next() {
					for &sink in iter_rest{
						maybe_add_edge(source, sink)
					}
					iter_rest = iter.clone()
				}
			}
		}
		Self(UniqueGraph::unchecked(graph))
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		Box::new(self.0.clone().unconstrain().shrink().map(|g| Self(UniqueGraph::unchecked(g))))
	}
}

///
/// An arbitrary graph that is __not__ unique
///
#[derive(Clone, Debug)]
pub struct ArbNonUniqueGraph<D:Directedness>(
	pub MockGraph<D>,
	usize
);

impl<D: Directedness> Graph for ArbNonUniqueGraph<D>
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

impl<D: Directedness> Arbitrary for ArbNonUniqueGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let mut graph = {
			let mut cand = ArbUniqueGraph::arbitrary(g).0.unconstrain();
			while cand.all_edges().count() < 1 {
				cand = ArbUniqueGraph::arbitrary(g).0.unconstrain();
			}
			cand
		};
		let original_edges: Vec<_> = graph.all_edges().map(|e| (e.source(), e.sink())).collect();
		
		// Duplicate a arbitrary number of additional edges (at least 1)
		let duplicate_count = g.gen_range(1, original_edges.len()+1);
		for _ in 0..duplicate_count{
			let dup_edge = original_edges[g.gen_range(0, original_edges.len())];
			graph.add_edge_weighted((dup_edge.source(), dup_edge.sink(), MockEdgeWeight::arbitrary(g)));
		}
		Self(graph, duplicate_count)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		/* Base case
		 */
		if self.all_vertices().count() == 0 {
			return Box::new(std::iter::empty());
		}
		
		let mut result = Vec::new();
		
		/* Shrink by shrinking vertex weight
		 */
		self.all_vertices_weighted()
			//Get all possible shrinkages
			.flat_map(|(v,weight)| weight.shrink().map(move|shrunk| (v,shrunk)))
			//For each shrunk weight,
			//create a new graph where the vertex has that weight
			.for_each(|(v, shrunk_weight)|{
				let mut new_graph = self.0.clone();
				new_graph.vertices.insert(v.value, shrunk_weight);
				result.push(Self(new_graph, self.1));
			});
		
		/* Shrink by shrinking edge weight
		 */
		//For each edge
		self.all_edges().for_each(|(source,sink,ref weight)|{
			let shrunk_weights = weight.shrink();
			
			shrunk_weights.for_each( |s_w| {
				let mut shrunk_graph = self.clone();
				if let Some(w) = shrunk_graph.all_edges_mut()
					.find(|(so,si,w)| source == *so && sink == *si && w.value == weight.value)
					.map(|(_,_,w)| w)
				{
					*w = s_w
				}
				result.push(shrunk_graph);
			});
		});
		
		/* Shrink by removing an edge.
		 * Can only remove an edge if there are more than 2 (must have at least 2 edges duplicating
		 * each other.
		 */
		if self.all_edges().count() > 2 {
			for e in self.all_edges() {
				/* Add to the result a copy of the graph
				 * without the edge
				 */
				let mut shrunk_graph = self.0.clone();
				let mut shrunk_dup_count = self.1;
				if D::directed() {
					if self.edges_between(e.source(), e.sink())
						.filter(|&(so, _, _)| so == e.source())
						.count() > 1
					{
						// Trying to remove a duplicate edge
						if shrunk_dup_count > 1 {
							shrunk_dup_count -= 1;
							shrunk_graph.remove_edge(e).unwrap();
							result.push(Self(shrunk_graph, shrunk_dup_count));
						} else {
							// Cant remove this edge, since it would result in a unique graph
						}
					} else {
						// A non-duplicate edge can be removed
						shrunk_graph.remove_edge(e).unwrap();
						result.push(Self(shrunk_graph, shrunk_dup_count));
					}
				} else {
					if self.edges_between(e.source(), e.sink()).count() > 1 {
						// Trying to remove a duplicate edge
						if shrunk_dup_count > 1 {
							shrunk_dup_count -= 1;
							shrunk_graph.remove_edge(e).unwrap();
							result.push(Self(shrunk_graph, shrunk_dup_count));
						} else {
							// Cant remove this edge, since it would result in a unique graph
						}
					} else {
						// A non-duplicate edge can be removed
						shrunk_graph.remove_edge(e).unwrap();
						result.push(Self(shrunk_graph, shrunk_dup_count));
					}
				}
			}
		}
		/* Shrink by removing a vertex that has no edges.
		 * We don't remove any edges in this step (to be able to remove a vertex)
		 * because we are already shrinking by removing edges, which means, there
		 * should be a set of edge shrinkages that result in a removable vertex.
		 */
		for v in self.all_vertices(){
			if self.edges_incident_on(v).next().is_none(){
				let mut shrunk_graph = self.clone();
				shrunk_graph.remove_vertex(v).unwrap();
				result.push(shrunk_graph);
			}
		}
		
		Box::new(result.into_iter())
	}
}