//!
//! Tests the `Constrainer` and `BaseGraph` combination for constraining graphs.
//!

use crate::mock_graph::{MockDirectedness, MockGraph, MockEdgeWeight, MockVertexWeight};
use graphene::core::{Graph, Constrainer, BaseGraph, EdgeWeighted, GraphMut, NewVertex, AddEdge, ImplGraphMut, ImplGraph, RemoveVertex};

///
/// A mock constraint simply to test.
///
/// To have it do something, we'll say this trait ensures the graph doesn't have 5 or more vertices.
///
trait MockConstraint: Graph
{
	fn mock_constraint_count(&self) -> usize
	{
		self.all_vertices().count()
	}
}
struct MockConstrainer<C: Constrainer>(pub C);

impl<C: Constrainer> ImplGraph for MockConstrainer<C> {
	type Graph = Self;
	
	fn graph(&self) -> &Self::Graph {
		self
	}
}
impl<C: Constrainer> ImplGraphMut for MockConstrainer<C>  {
	fn graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constrainer> Constrainer for MockConstrainer<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> {
		
		if g.graph().all_vertices().count() < 5 {
			Ok(Self(g))
		} else {
			Err(())
		}
	}
	
	fn unconstrain_single(self) -> Self::Constrained {
		self.0
	}
}

impl<C: Constrainer> Graph for MockConstrainer<C>
{
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Directedness = <C::Graph as Graph>::Directedness;
	
	
	fn all_vertices_weighted<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.0.graph().all_vertices_weighted()
	}
	
	fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.0.graph().all_edges()
	}

}

impl<C: Constrainer + ImplGraphMut>  GraphMut for MockConstrainer<C>
	where C::Graph: GraphMut
{
	
	fn all_vertices_weighted_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.0.graph_mut().all_vertices_weighted_mut()
	}

	
	fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.0.graph_mut().all_edges_mut()
	}

}

impl<C: Constrainer + ImplGraphMut> NewVertex for MockConstrainer<C>
	where C::Graph: NewVertex
{
	fn new_vertex_weighted(&mut self, w: Self::VertexWeight)
						   -> Result<Self::Vertex, ()>
	{
		if self.0.graph().all_vertices().count() < 4 {
			self.0.graph_mut().new_vertex_weighted(w)
		} else {
			Err(())
		}
	}
}
impl<C: Constrainer + ImplGraphMut>  RemoveVertex for MockConstrainer<C>
	where C::Graph: RemoveVertex
{
	fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>
	{
		self.0.graph_mut().remove_vertex(v)
	}

}

impl<C: Constrainer + ImplGraphMut>  AddEdge for MockConstrainer<C>
	where C::Graph: AddEdge
{

	fn remove_edge_where<F>(&mut self, f: F) -> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	{
		self.0.graph_mut().remove_edge_where(f)
	}

	
	fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>
	{
		self.0.graph_mut().add_edge_weighted(e)
	}
}

impl<C: Constrainer> MockConstraint for MockConstrainer<C>{}


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

#[test]
fn constrainer_constraining_base() {
	type ConstrainedGraphRef<'a> =
	MockConstrainer<
		MockConstrainer<
			&'a MockGraph<MockDirectedness>
		>>;
	
	let mut g = MockGraph::empty();
	assert_eq!(g.all_vertices().count(), 0);
	
	// Test constraining reference to graph
	let c_ref = ConstrainedGraphRef::constrain(&g).unwrap();
	assert_eq!(c_ref.mock_constraint_count(), 0);
	assert_eq!(c_ref.all_vertices().count(), 0);
	
	// Test still is a MockConstrainer after 1 unconstrain
	let c_ref_uncon = c_ref.unconstrain_single();
	assert_eq!(c_ref_uncon.mock_constraint_count(), 0);
	
	// By reusing 'g' below, we test that the previous constraint is dropped when the it in no
	// longer used.
	
	type ConstrainedGraphMut<'a> =
	MockConstrainer<
		MockConstrainer<
			&'a mut MockGraph<MockDirectedness>
		>>;
	
	// Test constraining mutable reference to graph
	let mut c_ref_mut = ConstrainedGraphMut::constrain(&mut g).unwrap();
	let vertex = c_ref_mut.new_vertex_weighted(MockVertexWeight { value: 32 }).unwrap();
	c_ref_mut.add_edge_weighted((vertex, vertex, MockEdgeWeight { value: 32 })).unwrap();
	assert_eq!(c_ref_mut.mock_constraint_count(), 1);
	
	// Test still is a MockConstrainer after 1 unconstrain
	let mut c_ref_mut_uncon = c_ref_mut.unconstrain_single();
	let vertex = c_ref_mut_uncon.new_vertex_weighted(MockVertexWeight { value: 32 }).unwrap();
	c_ref_mut_uncon.add_edge_weighted((vertex, vertex, MockEdgeWeight { value: 32 })).unwrap();
	assert_eq!(c_ref_mut_uncon.mock_constraint_count(), 2);
	
	// We don't test unconstraint, because it happens automatically when constrainer is
	// no longer used and the reference is freed.
	
	type ConstrainedGraph<'a> =
	MockConstrainer<
		MockConstrainer<
			MockConstrainer<
				MockGraph<MockDirectedness>
			>>>;
	
	// Test constraining graph directly
	let mut c_owned = ConstrainedGraph::constrain(g).unwrap();
	let vertex = c_owned.new_vertex_weighted(MockVertexWeight { value: 32 }).unwrap();
	c_owned.add_edge_weighted((vertex, vertex, MockEdgeWeight { value: 32 })).unwrap();
	assert_eq!(c_owned.mock_constraint_count(), 3);
	assert_eq!(c_owned.all_vertices().count(), 3);
	assert_eq!(c_owned.all_edges().count(), 3);
	
	// Test still is a MockConstrainer after 1 unconstrain
	let mut c_owned_uncon = c_owned.unconstrain_single();
	let vertex = c_owned_uncon.new_vertex_weighted(MockVertexWeight { value: 32 }).unwrap();
	c_owned_uncon.add_edge_weighted((vertex, vertex, MockEdgeWeight { value: 32 })).unwrap();
	assert_eq!(c_owned_uncon.mock_constraint_count(), 4);
	assert_eq!(c_owned_uncon.all_edges().count(), 4);
	
	// Test that the constraint is upheld (less than 5 vertices)
	assert!(c_owned_uncon.new_vertex_weighted(MockVertexWeight { value: 32 }).is_err());
	
	// Test all constraints can be removed at once
	let mut g = c_owned_uncon.unconstrain();
	let vertex = g.new_vertex_weighted(MockVertexWeight { value: 32 }).unwrap();
	g.add_edge_weighted((vertex, vertex, MockEdgeWeight { value: 32 })).unwrap();
	assert_eq!(g.all_vertices().count(), 5);
	assert_eq!(g.all_edges().count(), 5);
	
	// Test can no longer constrain it
	assert!(ConstrainedGraph::constrain(g).is_err());
}

#[test]
fn inline_constrainer_constraining_base(){

	let mut g = MockGraph::empty();
	assert_eq!(g.all_vertices().count(), 0);

	let c_ref = <MockConstrainer<MockConstrainer<&MockGraph<MockDirectedness>>>>::constrain(&g).unwrap();
	assert_eq!(c_ref.all_vertices().count(), 0);

	let mut c_ref_mut = <MockConstrainer<MockConstrainer<&mut MockGraph<MockDirectedness>>>>::constrain(&mut g).unwrap();
	c_ref_mut.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_ref_mut.all_vertices().count(), 1);

	let mut c_owned = <MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>>::constrain(g).unwrap();
	c_owned.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_owned.all_vertices().count(), 2);

}

#[test]
fn base_constrains_self_by_constraint_inference(){
	type ConstrainedGraph<G> = MockConstrainer< MockConstrainer<G> >;
	
	let mut g = MockGraph::empty();
	assert_eq!(g.all_vertices().count(), 0);

	let c_ref: ConstrainedGraph<&MockGraph<MockDirectedness>> = (&g).constrain().unwrap();
	assert_eq!(c_ref.all_vertices().count(), 0);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_eq!(c_ref_unc.all_vertices().count(), 0);

	let mut c_ref_mut: ConstrainedGraph<&mut MockGraph<MockDirectedness>> = (&mut g).constrain().unwrap();
	c_ref_mut.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_ref_mut.all_vertices().count(), 1);

	let mut c_owned: ConstrainedGraph<MockGraph<MockDirectedness>> = g.constrain().unwrap();
	c_owned.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_owned.all_vertices().count(), 2);
}

#[test]
fn base_constrains_self_by_inline_constraint_inference(){
	
	let mut g = MockGraph::empty();
	assert_eq!(g.all_vertices().count(), 0);

	let c_ref: MockConstrainer<MockConstrainer<&MockGraph<MockDirectedness>>> = (&g).constrain().unwrap();
	assert_eq!(c_ref.all_vertices().count(), 0);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_eq!(c_ref_unc.all_vertices().count(), 0);

	let mut c_ref_mut: MockConstrainer<MockConstrainer<&mut MockGraph<MockDirectedness>>> = (&mut g).constrain().unwrap();
	c_ref_mut.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_ref_mut.all_vertices().count(), 1);
	let mut c_ref_mut_unc = c_ref_mut.unconstrain_single();
	c_ref_mut_unc.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_ref_mut_unc.all_vertices().count(), 2);

	let c_owned: MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>> = g.constrain().unwrap();
	assert_eq!(c_owned.all_vertices().count(), 2);
	let mut c_owned_unc = c_owned.unconstrain_single();
	c_owned_unc.new_vertex_weighted(MockVertexWeight{value: 10}).unwrap();
	assert_eq!(c_owned_unc.all_vertices().count(), 3);
	let g2 = c_owned_unc.unconstrain();
	assert_eq!(g2.all_vertices().count(), 3);
}