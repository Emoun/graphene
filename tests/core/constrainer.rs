//! Tests the `Constrainer` and `BaseGraph` combination for constraining graphs.
//!

use crate::mock_graph::{MockDirectedness, MockGraph, MockVertexWeight};
use graphene::core::{BaseGraph, Constrainer, Graph, GraphDeref, GraphDerefMut, GraphMut};

/// A mock constraint that doesn't use mutability.
///
/// Its requirement is that the graph has exactly 1 vertex with a weight
trait MockConstraint: Graph
{
	/// Return the value of the vertex weight
	fn mock_weight_value(&self) -> &Self::VertexWeight;
}
/// A mock constraint that uses mutability.
trait MockConstraintMut: MockConstraint
{
	/// Set the value of the vertex weight
	fn mock_set_weight(&mut self, w: Self::VertexWeight);
}

/// A mock constrainer.
struct MockConstrainer<C: Constrainer>(pub C);

impl<C: Constrainer> GraphDeref for MockConstrainer<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Constrainer> GraphDerefMut for MockConstrainer<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Constrainer> Constrainer for MockConstrainer<C>
{
	type Base = C::Base;
	type Constrained = C;

	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>
	{
		if g.graph().all_vertices().count() == 1
		{
			Ok(Self(g))
		}
		else
		{
			Err(())
		}
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self.0
	}
}

impl<C: Constrainer> Graph for MockConstrainer<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	fn all_vertices_weighted<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a Self::VertexWeight)>>
	{
		self.0.graph().all_vertices_weighted()
	}

	fn all_edges<'a>(
		&'a self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>
	{
		self.0.graph().all_edges()
	}
}

impl<C: Constrainer + GraphDerefMut> GraphMut for MockConstrainer<C>
where
	C::Graph: GraphMut,
{
	fn all_vertices_weighted_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, &'a mut Self::VertexWeight)>>
	{
		self.0.graph_mut().all_vertices_weighted_mut()
	}

	fn all_edges_mut<'a>(
		&'a mut self,
	) -> Box<dyn 'a + Iterator<Item = (Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>
	{
		self.0.graph_mut().all_edges_mut()
	}
}

impl<C: Constrainer> MockConstraint for MockConstrainer<C>
{
	fn mock_weight_value(&self) -> &Self::VertexWeight
	{
		self.all_vertices_weighted().next().unwrap().1
	}
}

impl<C: Constrainer + GraphDerefMut> MockConstraintMut for MockConstrainer<C>
where
	C::Graph: GraphMut,
{
	fn mock_set_weight(&mut self, w: Self::VertexWeight)
	{
		*self.all_vertices_weighted_mut().next().unwrap().1 = w;
	}
}

/// Creates a graph that can be constrainted by MockConstrainer.
macro_rules! constrainable_graph {
	{} =>{
		{
			let mut g = MockGraph::empty();
			g.vertices.insert(0, MockVertexWeight{value: 0});
			g.next_id += 1;
			g
		}
	}
}
/// Test given graph implements MockConstraint by calling its method.
macro_rules! assert_implements_mock_constraint {
	{$graph:ident} =>{
		assert_eq!($graph.mock_weight_value().value, $graph.mock_weight_value().value);
	}
}
/// Test given graph implements MockConstraintMut by using its method to
/// increment the weight and afterwards test that it was incremented.
macro_rules! assert_implements_mock_constraint_mut {
	{$graph:ident} =>{
		let old_weight = $graph.mock_weight_value().value;
		$graph.mock_set_weight(MockVertexWeight{value: old_weight + 1});
		assert_eq!(old_weight + 1, $graph.mock_weight_value().value);
	}
}

/// Test that defining a type alias allows for easy constraining of a base graph
#[test]
fn pretyped_constraining()
{
	type ConstrainedGraph = MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>;

	// Test can use `Constrainer.constrain` on a base graph without needing type
	// annotation
	let g = ConstrainedGraph::constrain(constrainable_graph!()).unwrap();

	// Test that `BaseGraph.constrain` can be used where the constraint is defined
	// elsewhere (in this case by an annotation, but could also be elsewhere and
	// then solved by type inference)
	let g2: ConstrainedGraph = constrainable_graph!().constrain().unwrap();

	// Test can remove 1 constraint
	let _: MockConstrainer<MockGraph<MockDirectedness>> = g.unconstrain_single();

	// Test can remove all constraints
	let _: MockGraph<MockDirectedness> = g2.unconstrain();
}

/// Test that can define constraints inline
#[test]
fn inline_constraining()
{
	// Test can use `Constrainer.constrain` on a base graph using inline constraints
	let g = <MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>>::constrain(
		constrainable_graph!(),
	)
	.unwrap();

	// Test that `BaseGraph.constrain` can be used where the constraint is defined
	// elsewhere (in this case by an annotation, but could also be elsewhere and
	// then solved by type inference)
	let g2: MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>> =
		constrainable_graph!().constrain().unwrap();

	// Test can remove 1 constraint
	let _: MockConstrainer<MockGraph<MockDirectedness>> = g.unconstrain_single();

	// Test can remove all constraints
	let _: MockGraph<MockDirectedness> = g2.unconstrain();
}

#[test]
fn constrainer_constraining_base()
{
	type ConstrainedGraphRef<'a> =
		MockConstrainer<MockConstrainer<&'a MockGraph<MockDirectedness>>>;

	let mut g = constrainable_graph!();

	// Test constraining reference to graph
	let c_ref = ConstrainedGraphRef::constrain(&g).unwrap();
	assert_implements_mock_constraint!(c_ref);

	// Test still is a MockConstrainer after 1 unconstrain
	let c_ref_uncon = c_ref.unconstrain_single();
	assert_implements_mock_constraint!(c_ref_uncon);

	// By reusing 'g' below, we test that the previous constraint
	// is dropped when it i no longer used.

	type ConstrainedGraphMut<'a> =
		MockConstrainer<MockConstrainer<&'a mut MockGraph<MockDirectedness>>>;

	// Test constraining mutable reference to graph
	let mut c_ref_mut = ConstrainedGraphMut::constrain(&mut g).unwrap();
	assert_implements_mock_constraint_mut!(c_ref_mut);

	// Test still is a MockConstrainer after 1 unconstrain
	let mut c_ref_mut_uncon = c_ref_mut.unconstrain_single();
	assert_implements_mock_constraint_mut!(c_ref_mut_uncon);

	// We don't test unconstrain() explicitly now, because it happens automatically
	// when constrainer is no longer used and the reference is freed.

	type ConstrainedGraph<'a> =
		MockConstrainer<MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>>;

	// Test constraining graph directly
	let mut c_owned = ConstrainedGraph::constrain(g).unwrap();
	assert_implements_mock_constraint_mut!(c_owned);

	// Test still is a MockConstrainer after 1 unconstrain
	let mut c_owned_uncon = c_owned.unconstrain_single();
	assert_implements_mock_constraint_mut!(c_owned_uncon);

	// Test all constraints can be removed at once
	c_owned_uncon.unconstrain().validate();
}

#[test]
fn inline_constrainer_constraining_base()
{
	let mut g = constrainable_graph!();

	let c_ref =
		<MockConstrainer<MockConstrainer<&MockGraph<MockDirectedness>>>>::constrain(&g).unwrap();
	assert_implements_mock_constraint!(c_ref);

	let mut c_ref_mut =
		<MockConstrainer<MockConstrainer<&mut MockGraph<MockDirectedness>>>>::constrain(&mut g)
			.unwrap();
	assert_implements_mock_constraint_mut!(c_ref_mut);

	let mut c_owned =
		<MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>>>::constrain(g).unwrap();
	assert_implements_mock_constraint_mut!(c_owned);
}

#[test]
fn base_constrains_self_by_constraint_inference()
{
	type ConstrainedGraph<G> = MockConstrainer<MockConstrainer<G>>;

	let mut g = constrainable_graph!();

	let c_ref: ConstrainedGraph<&MockGraph<MockDirectedness>> = (&g).constrain().unwrap();
	assert_implements_mock_constraint!(c_ref);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_implements_mock_constraint!(c_ref_unc);

	let mut c_ref_mut: ConstrainedGraph<&mut MockGraph<MockDirectedness>> =
		(&mut g).constrain().unwrap();
	assert_implements_mock_constraint_mut!(c_ref_mut);

	let mut c_owned: ConstrainedGraph<MockGraph<MockDirectedness>> = g.constrain().unwrap();
	assert_implements_mock_constraint_mut!(c_owned);
}

#[test]
fn base_constrains_self_by_inline_constraint_inference()
{
	let mut g = constrainable_graph!();

	let c_ref: MockConstrainer<MockConstrainer<&MockGraph<MockDirectedness>>> =
		(&g).constrain().unwrap();
	assert_implements_mock_constraint!(c_ref);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_implements_mock_constraint!(c_ref_unc);

	let mut c_ref_mut: MockConstrainer<MockConstrainer<&mut MockGraph<MockDirectedness>>> =
		(&mut g).constrain().unwrap();
	assert_implements_mock_constraint_mut!(c_ref_mut);
	let mut c_ref_mut_unc = c_ref_mut.unconstrain_single();
	assert_implements_mock_constraint_mut!(c_ref_mut_unc);

	let c_owned: MockConstrainer<MockConstrainer<MockGraph<MockDirectedness>>> =
		g.constrain().unwrap();
	assert_implements_mock_constraint!(c_owned);
	let mut c_owned_unc = c_owned.unconstrain_single();
	assert_implements_mock_constraint_mut!(c_owned_unc);

	c_owned_unc.unconstrain().validate()
}
