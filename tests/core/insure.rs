//! Tests the `Insure` and `BaseGraph` combination for insuring graphs.
//!

use crate::mock_graph::{MockDirectedness, MockGraph, MockVertexWeight};
use graphene::core::{BaseGraph, Graph, GraphDeref, GraphDerefMut, GraphMut, Insure, Release};

/// A mock property that doesn't use mutability.
///
/// Its requirement is that the graph has exactly 1 vertex with a weight
trait MockProperty: Graph
{
	/// Return the value of the vertex weight
	fn mock_weight_value(&self) -> &Self::VertexWeight;
}
/// A mock property that uses mutability.
trait MockPropertyMut: MockProperty
{
	/// Set the value of the vertex weight
	fn mock_set_weight(&mut self, w: Self::VertexWeight);
}

/// A mock insurer.
struct MockInsurer<C: Insure>(pub C);

impl<C: Insure> GraphDeref for MockInsurer<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Insure> GraphDerefMut for MockInsurer<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Insure> Insure for MockInsurer<C>
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Insured) -> bool
	{
		c.graph().all_vertices().count() == 1
	}
}

impl<C: Insure> Release for MockInsurer<C>
{
	type Base = C::Base;
	type Insured = C;

	fn release(self) -> Self::Insured
	{
		self.0
	}
}

impl<C: Insure> Graph for MockInsurer<C>
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

impl<C: Insure + GraphDerefMut> GraphMut for MockInsurer<C>
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

impl<C: Insure> MockProperty for MockInsurer<C>
{
	fn mock_weight_value(&self) -> &Self::VertexWeight
	{
		self.all_vertices_weighted().next().unwrap().1
	}
}

impl<C: Insure + GraphDerefMut> MockPropertyMut for MockInsurer<C>
where
	C::Graph: GraphMut,
{
	fn mock_set_weight(&mut self, w: Self::VertexWeight)
	{
		*self.all_vertices_weighted_mut().next().unwrap().1 = w;
	}
}

/// Creates a graph that can be insured by MockInsure.
macro_rules! insurable_graph {
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
macro_rules! assert_implements_mock_property {
	{$graph:ident} =>{
		assert_eq!($graph.mock_weight_value().value, $graph.mock_weight_value().value);
	}
}
/// Test given graph implements MockConstraintMut by using its method to
/// increment the weight and afterwards test that it was incremented.
macro_rules! assert_implements_mock_property_mut {
	{$graph:ident} =>{
		let old_weight = $graph.mock_weight_value().value;
		$graph.mock_set_weight(MockVertexWeight{value: old_weight + 1});
		assert_eq!(old_weight + 1, $graph.mock_weight_value().value);
	}
}

/// Test that defining a type alias allows for easy insuring of a base graph
#[test]
fn pretyped_insuring()
{
	type InsuredGraph = MockInsurer<MockInsurer<MockGraph<MockDirectedness>>>;

	// Test can use `Insure.insure_all` on a base graph without needing type
	// annotation
	let g = InsuredGraph::insure_all(insurable_graph!()).unwrap();

	// Test that `BaseGraph.insure_all` can be used where the property is defined
	// elsewhere (in this case by an annotation, but could also be elsewhere and
	// then solved by type inference)
	let g2: InsuredGraph = insurable_graph!().insure_all().unwrap();

	// Test can remove 1 property
	let _: MockInsurer<MockGraph<MockDirectedness>> = g.release();

	// Test can remove all properties
	let _: MockGraph<MockDirectedness> = g2.release_all();
}

/// Test that can define properties inline
#[test]
fn inline_insuring()
{
	// Test can use `Insure.insure_all` on a base graph using inline properties
	let g = <MockInsurer<MockInsurer<MockGraph<MockDirectedness>>>>::insure_all(insurable_graph!())
		.unwrap();

	// Test that `BaseGraph.insure_all` can be used where the property is defined
	// elsewhere (in this case by an annotation, but could also be elsewhere and
	// then solved by type inference)
	let g2: MockInsurer<MockInsurer<MockGraph<MockDirectedness>>> =
		insurable_graph!().insure_all().unwrap();

	// Test can remove 1 property
	let _: MockInsurer<MockGraph<MockDirectedness>> = g.release();

	// Test can remove all properties
	let _: MockGraph<MockDirectedness> = g2.release_all();
}

#[test]
fn insurer_insuring_base()
{
	type InsuredGraphRef<'a> = MockInsurer<MockInsurer<&'a MockGraph<MockDirectedness>>>;

	let mut g = insurable_graph!();

	// Test insuring reference to graph
	let c_ref = InsuredGraphRef::insure_all(&g).unwrap();
	assert_implements_mock_property!(c_ref);

	// Test still is a MockInsure after 1 release_all
	let c_ref_uncon = c_ref.release();
	assert_implements_mock_property!(c_ref_uncon);

	// By reusing 'g' below, we test that the previous property
	// is dropped when it i no longer used.

	type InsuredGraphMut<'a> = MockInsurer<MockInsurer<&'a mut MockGraph<MockDirectedness>>>;

	// Test insuring mutable reference to graph
	let mut c_ref_mut = InsuredGraphMut::insure_all(&mut g).unwrap();
	assert_implements_mock_property_mut!(c_ref_mut);

	// Test still is a MockInsure after 1 release_all
	let mut c_ref_mut_uncon = c_ref_mut.release();
	assert_implements_mock_property_mut!(c_ref_mut_uncon);

	// We don't test release_all() explicitly now, because it happens automatically
	// when insurer is no longer used and the reference is freed.

	type InsuredGraph<'a> = MockInsurer<MockInsurer<MockInsurer<MockGraph<MockDirectedness>>>>;

	// Test insuring graph directly
	let mut c_owned = InsuredGraph::insure_all(g).unwrap();
	assert_implements_mock_property_mut!(c_owned);

	// Test still is a MockInsure after 1 release_all
	let mut c_owned_uncon = c_owned.release();
	assert_implements_mock_property_mut!(c_owned_uncon);

	// Test all properties can be removed at once
	c_owned_uncon.release_all().validate_is_graph();
}

#[test]
fn inline_insurer_insuring_base()
{
	let mut g = insurable_graph!();

	let c_ref = <MockInsurer<MockInsurer<&MockGraph<MockDirectedness>>>>::insure_all(&g).unwrap();
	assert_implements_mock_property!(c_ref);

	let mut c_ref_mut =
		<MockInsurer<MockInsurer<&mut MockGraph<MockDirectedness>>>>::insure_all(&mut g).unwrap();
	assert_implements_mock_property_mut!(c_ref_mut);

	let mut c_owned =
		<MockInsurer<MockInsurer<MockGraph<MockDirectedness>>>>::insure_all(g).unwrap();
	assert_implements_mock_property_mut!(c_owned);
}

#[test]
fn base_insures_self_by_inference()
{
	type InsuredGraph<G> = MockInsurer<MockInsurer<G>>;

	let mut g = insurable_graph!();

	let c_ref: InsuredGraph<&MockGraph<MockDirectedness>> = (&g).insure_all().unwrap();
	assert_implements_mock_property!(c_ref);
	let c_ref_unc = c_ref.release();
	assert_implements_mock_property!(c_ref_unc);

	let mut c_ref_mut: InsuredGraph<&mut MockGraph<MockDirectedness>> =
		(&mut g).insure_all().unwrap();
	assert_implements_mock_property_mut!(c_ref_mut);

	let mut c_owned: InsuredGraph<MockGraph<MockDirectedness>> = g.insure_all().unwrap();
	assert_implements_mock_property_mut!(c_owned);
}

#[test]
fn base_insures_self_by_inline_inference()
{
	let mut g = insurable_graph!();

	let c_ref: MockInsurer<MockInsurer<&MockGraph<MockDirectedness>>> = (&g).insure_all().unwrap();
	assert_implements_mock_property!(c_ref);
	let c_ref_unc = c_ref.release();
	assert_implements_mock_property!(c_ref_unc);

	let mut c_ref_mut: MockInsurer<MockInsurer<&mut MockGraph<MockDirectedness>>> =
		(&mut g).insure_all().unwrap();
	assert_implements_mock_property_mut!(c_ref_mut);
	let mut c_ref_mut_unc = c_ref_mut.release();
	assert_implements_mock_property_mut!(c_ref_mut_unc);

	let c_owned: MockInsurer<MockInsurer<MockGraph<MockDirectedness>>> = g.insure_all().unwrap();
	assert_implements_mock_property!(c_owned);
	let mut c_owned_unc = c_owned.release();
	assert_implements_mock_property_mut!(c_owned_unc);

	c_owned_unc.release_all().validate_is_graph()
}
