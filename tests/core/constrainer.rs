//!
//! Tests the `Constrainer` and `BaseGraph` combination for constraining graphs.
//!

use crate::mock_graph::{MockDirectedness, MockGraph};
use graphene::core::{Directedness, Graph, Constrainer, BaseGraph, EdgeWeighted, GraphMut, AddVertex, AddEdge, Edge};
use delegate::delegate;

struct MockConstrainer<G: Graph>(pub G);

impl<G: Graph> Graph for MockConstrainer<G>
{
	type Vertex = G::Vertex;
	type VertexWeight = G::VertexWeight;
	type EdgeWeight = G::EdgeWeight;
	type Directedness = G::Directedness;
	delegate!{
		target self.0 {
			fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a Self::VertexWeight)>> ;
			
			fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>> ;
		}
	}
}

impl<G: GraphMut> GraphMut for MockConstrainer<G>
{
	delegate!{
		target self.0 {
			fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, &'a mut Self::VertexWeight)>> ;
	
			
			fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
				(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>> ;
	
		}
	}
}

impl<G: AddVertex> AddVertex for MockConstrainer<G>
{
	delegate! {
		target self.0 {
			fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
				-> Result<Self::Vertex, ()>;
			
			fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()> ;
		}
	}
}

impl<G: AddEdge> AddEdge for MockConstrainer<G>
{
	delegate! {
		target self.0 {
			fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
				where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool;

		}
	}
	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		if Self::Directedness::directed() {
			if self.edges_between(e.source(), e.sink())
				.any(|edge| e.source() == edge.source() && e.sink() == edge.sink()){
				return Err(());
			}
		} else {
			if self.edges_between(e.source(), e.sink()).next().is_some() {
				return Err(());
			}
		}
		self.0.add_edge_weighted(e)
	}
}

impl<G:Graph + Constrainer<BaseGraph=B>, B: BaseGraph> Constrainer for MockConstrainer<G>
{
	type BaseGraph = B;
	type Constrained = G;
	
	fn unconstrain_single(self) -> Self::Constrained {
		self.0
	}
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> {
		Ok(Self(g))
	}
}

///
/// Test that defining a type alias allows for easy constraining of a base graph
///
#[test]
fn pretyped_constraining() {
	
	type ConstrainedGraph =
	MockConstrainer<
		MockConstrainer<
				MockGraph<MockDirectedness>
			>>;
	
	// Test can use `Constrainer.constrain` on a base graph without needing type annotation
	let g = ConstrainedGraph::constrain(MockGraph::empty()).unwrap();
	
	// Test that `BaseGraph.constrain` can be used where the constraint is defined elsewhere
	// (in this case by an annotation, but could also be elsewhere and then solved by type inference)
	let g2: ConstrainedGraph = MockGraph::empty().constrain().unwrap();
	
	// Test can remove 1 constraint
	let _: MockConstrainer<MockGraph<MockDirectedness>>  = g.unconstrain_single();
	
	// Test can remove all constraints
	let _: MockGraph<MockDirectedness> = g2.unconstrain();
}

///
/// Test that can define constraints inline
///
#[test]
fn inline_constraining() {
	
	// Test can use `Constrainer.constrain` on a base graph using inline constraints
	let g = <MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>>
		::constrain(MockGraph::empty()).unwrap();
	
	// Test that `BaseGraph.constrain` can be used where the constraint is defined elsewhere
	// (in this case by an annotation, but could also be elsewhere and then solved by type inference)
	let g2: MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>> =
		MockGraph::empty().constrain().unwrap();
	
	// Test can remove 1 constraint
	let _: MockConstrainer<MockGraph<MockDirectedness>>  = g.unconstrain_single();
	
	// Test can remove all constraints
	let _: MockGraph<MockDirectedness> = g2.unconstrain();
}