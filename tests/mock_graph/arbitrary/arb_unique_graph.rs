use graphene::core::constraint::UniqueGraph;
use graphene::core::{Directedness, Graph, AutoGraph, Edge, EdgeWeighted, Constrainer};
use crate::mock_graph::{MockGraph, MockVertex, MockVertexWeight, MockEdgeWeight};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use crate::mock_graph::arbitrary::{max_vertex_count, GuidedArbGraph, Limit};
use delegate::delegate;
use std::ops::{RangeBounds, Bound};
use std::collections::HashSet;

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

impl<D: Directedness> GuidedArbGraph for ArbUniqueGraph<D>
{
	///
	/// Generates a Unique graph with a vertex count within the given range.
	///
	/// The range for edges is only upheld if the lower bound is 1 and the lower bound of
	/// vertices is 1, in which case the graph is guaranteed to have at least 1 edge.
	///
	fn arbitrary_guided<G: Gen, R: RangeBounds<usize>>(g: &mut G, v_range: R, e_range: R) -> Self {
		let mut graph = MockGraph::empty();
		
		//Decide the amount of vertices
		let v_min = match v_range.start_bound() {
			Bound::Included(x) =>  x ,
			x => panic!("Unsupported lower vertex bound: {:?}", x)
		};
		let v_max = match v_range.end_bound() {
			Bound::Included(x) =>  x + 1 ,
			Bound::Excluded(x) => *x,
			x => panic!("Unsupported upper vertex bound: {:?}", x)
			
		};
		let vertex_count = g.gen_range(v_min, v_max);
		
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
			match e_range.start_bound() {
				Bound::Included(&x) if x == 1 && graph.all_edges().count() < 1 =>
					graph.add_edge_weighted((verts[g.gen_range(0, verts.len())],
											 verts[g.gen_range(0, verts.len())],
											 MockEdgeWeight::arbitrary(g))).unwrap(),
				_ => ()
			}
		}
		Self(UniqueGraph::unchecked(graph))
	}
}

impl<D: Directedness> Arbitrary for ArbUniqueGraph<D>
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let v_max = max_vertex_count(g);
		Self::arbitrary_guided(g, 0..v_max, 0..v_max)
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
		let v_max = max_vertex_count(g);
		// Ensure there are at least 1 edge (so that we can duplicate)
		let mut graph = ArbUniqueGraph::arbitrary_guided(g, 1..v_max, 1..v_max).0.unconstrain();
		
		// Duplicate a arbitrary number of additional edges (at least 1)
		let original_edges: Vec<_> = graph.all_edges().map(|e| (e.source(), e.sink())).collect();
		let duplicate_count = g.gen_range(1, original_edges.len()+1);
		for _ in 0..duplicate_count{
			let dup_edge = original_edges[g.gen_range(0, original_edges.len())];
			graph.add_edge_weighted((dup_edge.source(),
									 dup_edge.sink(),
									 MockEdgeWeight::arbitrary(g)))
				.unwrap();
		}
		Self(graph, duplicate_count)
	}
	
	fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
		
		let mut result = Vec::new();
		
		// Allow MockGraph to shrink everything except removing edges.
		let mut limits = HashSet::new();
		limits.insert(Limit::EdgeRemove);
		result.extend(self.0.shrink_guided(limits).map(|g| Self(g,self.1)));
		
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
		
		Box::new(result.into_iter())
	}
}