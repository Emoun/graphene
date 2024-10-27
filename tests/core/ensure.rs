//! Tests the `Ensure` and `BaseGraph` combination for ensuring graphs.

use crate::mock_graph::MockVertex;
use delegate::delegate;
use graphene::core::{Ensure, Graph, GraphDeref, GraphDerefMut, GraphMut, Release};
use std::borrow::Borrow;

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

/// A mock ensurer with payload.
struct MockEnsurer<C: Ensure>(pub C, MockVertex);
impl<C: Ensure> GraphDeref for MockEnsurer<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Ensure> GraphDerefMut for MockEnsurer<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Ensure> Ensure for MockEnsurer<C>
{
	fn ensure_unvalidated(c: Self::Ensured, p: MockVertex) -> Self
	{
		Self(c, p)
	}

	fn validate(c: &Self::Ensured, p: &MockVertex) -> bool
	{
		c.graph().all_vertices().count() == p.value as usize
	}
}
impl<C: Ensure> Release for MockEnsurer<C>
{
	type Base = C::Base;
	type Ensured = C;
	type Payload = (MockVertex, C::Payload);

	fn release(self) -> (Self::Ensured, MockVertex)
	{
		(self.0, self.1)
	}
}
impl<C: Ensure> Graph for MockEnsurer<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted(
				&self,
			) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>;

			fn all_edges(
				&self,
			) -> impl '_ + Iterator<Item = (Self::Vertex, Self::Vertex, &Self::EdgeWeight)>;

			fn edges_between<'a: 'b, 'b>(
				&'a self,
				source: impl 'b + Borrow<Self::Vertex>,
				sink: impl 'b + Borrow<Self::Vertex>,
			) -> impl 'b + Iterator<Item = &'a Self::EdgeWeight>;
		}
	}
}
impl<C: Ensure + GraphDerefMut> GraphMut for MockEnsurer<C>
where
	C::Graph: GraphMut,
{
	delegate! {
		to self.0.graph_mut() {
			fn all_vertices_weighted_mut(
				&mut self,
			) -> impl '_ + Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>;

			fn edges_between_mut<'a: 'b, 'b>(
				&'a mut self,
				source: impl 'b + Borrow<Self::Vertex>,
				sink: impl 'b + Borrow<Self::Vertex>,
			) -> impl 'b + Iterator<Item = &'a mut Self::EdgeWeight>;
		}
	}
}
impl<C: Ensure> MockProperty for MockEnsurer<C>
{
	fn mock_weight_value(&self) -> &Self::VertexWeight
	{
		self.all_vertices_weighted().next().unwrap().1
	}
}
impl<C: Ensure + GraphDerefMut> MockPropertyMut for MockEnsurer<C>
where
	C::Graph: GraphMut,
{
	fn mock_set_weight(&mut self, w: Self::VertexWeight)
	{
		*self.all_vertices_weighted_mut().next().unwrap().1 = w;
	}
}

/// A mock ensurer with no payload.
struct MockUnloadedEnsurer<C: Ensure>(pub C);
impl<C: Ensure> GraphDeref for MockUnloadedEnsurer<C>
{
	type Graph = Self;

	fn graph(&self) -> &Self::Graph
	{
		self
	}
}
impl<C: Ensure> GraphDerefMut for MockUnloadedEnsurer<C>
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		self
	}
}
impl<C: Ensure> Ensure for MockUnloadedEnsurer<C>
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		c.graph().all_vertices().count() == 1
	}
}
impl<C: Ensure> Release for MockUnloadedEnsurer<C>
{
	type Base = C::Base;
	type Ensured = C;
	type Payload = C::Payload;

	fn release(self) -> (Self::Ensured, ())
	{
		(self.0, ())
	}
}
impl<C: Ensure> Graph for MockUnloadedEnsurer<C>
{
	type Directedness = <C::Graph as Graph>::Directedness;
	type EdgeWeight = <C::Graph as Graph>::EdgeWeight;
	type Vertex = <C::Graph as Graph>::Vertex;
	type VertexWeight = <C::Graph as Graph>::VertexWeight;

	delegate! {
		to self.0.graph() {
			fn all_vertices_weighted(
				&self,
			) -> impl Iterator<Item = (Self::Vertex, &Self::VertexWeight)>;

			fn all_edges(
				&self,
			) -> impl '_ + Iterator<Item = (Self::Vertex, Self::Vertex, &Self::EdgeWeight)>;

			fn edges_between<'a: 'b, 'b>(
				&'a self,
				source: impl 'b + Borrow<Self::Vertex>,
				sink: impl 'b + Borrow<Self::Vertex>,
			) -> impl 'b + Iterator<Item = &'a Self::EdgeWeight>;
		}
	}
}
impl<C: Ensure + GraphDerefMut> GraphMut for MockUnloadedEnsurer<C>
where
	C::Graph: GraphMut,
{
	fn all_vertices_weighted_mut(
		&mut self,
	) -> impl '_ + Iterator<Item = (Self::Vertex, &mut Self::VertexWeight)>
	{
		self.0.graph_mut().all_vertices_weighted_mut()
	}

	fn edges_between_mut<'a: 'b, 'b>(
		&'a mut self,
		source: impl 'b + Borrow<Self::Vertex>,
		sink: impl 'b + Borrow<Self::Vertex>,
	) -> impl 'b + Iterator<Item = &'a mut Self::EdgeWeight>
	{
		self.0.graph_mut().edges_between_mut(source, sink)
	}
}
impl<C: Ensure> MockProperty for MockUnloadedEnsurer<C>
{
	fn mock_weight_value(&self) -> &Self::VertexWeight
	{
		self.all_vertices_weighted().next().unwrap().1
	}
}
impl<C: Ensure + GraphDerefMut> MockPropertyMut for MockUnloadedEnsurer<C>
where
	C::Graph: GraphMut,
{
	fn mock_set_weight(&mut self, w: Self::VertexWeight)
	{
		*self.all_vertices_weighted_mut().next().unwrap().1 = w;
	}
}

/// Creates a graph that can be ensured by MockEnsure.
macro_rules! ensurable_graph {
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

mod test_with_payload
{
	use crate::{
		core::ensure::{MockEnsurer, MockProperty, MockPropertyMut, MockUnloadedEnsurer},
		mock_graph::{MockDirectedness, MockGraph, MockVertex, MockVertexWeight},
	};
	use graphene::core::{BaseGraph, Ensure, Release};

	/// Test that defining a type alias allows for easy ensuring of a base graph
	#[test]
	fn pretyped_ensuring()
	{
		type EnsuredGraph = MockEnsurer<MockEnsurer<MockGraph<MockDirectedness>>>;
		let payload = MockVertex { value: 1 };
		// Test can use `Ensure.ensure_all` on a base graph without needing type
		// annotation
		let g = EnsuredGraph::ensure_all(ensurable_graph!(), (payload, (payload, ()))).unwrap();

		// Test that `BaseGraph.ensure_all` can be used where the property is defined
		// elsewhere (in this case by an annotation, but could also be elsewhere and
		// then solved by type inference)
		let g2: EnsuredGraph = ensurable_graph!()
			.ensure_all((payload, (payload, ())))
			.unwrap();

		// Test can remove 1 property
		let (_, released_payload): (MockEnsurer<MockGraph<MockDirectedness>>, _) = g.release();
		assert_eq!(released_payload, payload);

		// Test can remove all properties
		let (_, released_payload): (MockGraph<MockDirectedness>, _) = g2.release_all();
		assert_eq!(released_payload, (payload, (payload, ())));
	}

	/// Test that can define properties inline
	#[test]
	fn inline_ensuring()
	{
		let payload = MockVertex { value: 1 };
		// Test can use `Ensure.ensure_all` on a base graph using inline properties
		let g = <MockUnloadedEnsurer<MockEnsurer<MockGraph<MockDirectedness>>>>::ensure_all(
			ensurable_graph!(),
			(payload, ()),
		)
		.unwrap();

		// Test that `BaseGraph.ensure_all` can be used where the property is defined
		// elsewhere (in this case by an annotation, but could also be elsewhere and
		// then solved by type inference)
		let g2: MockEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>> =
			ensurable_graph!().ensure_all((payload, ())).unwrap();

		// Test can remove 1 property
		let _: (MockEnsurer<MockGraph<MockDirectedness>>, ()) = g.release();

		// Test can remove all properties
		let (_, released_payload): (MockGraph<MockDirectedness>, _) = g2.release_all();
		assert_eq!(released_payload, (payload, ()));
	}

	#[test]
	fn ensurer_ensuring_base()
	{
		type EnsuredGraphRef<'a> =
			MockUnloadedEnsurer<MockEnsurer<MockEnsurer<&'a MockGraph<MockDirectedness>>>>;
		let payload = MockVertex { value: 1 };

		let mut g = ensurable_graph!();

		// Test ensuring reference to graph
		let c_ref = EnsuredGraphRef::ensure_all(&g, (payload, (payload, ()))).unwrap();
		assert_implements_mock_property!(c_ref);

		// Test still is a MockEnsure after 1 release_all
		let (c_ref_uncon, ()) = c_ref.release();
		assert_implements_mock_property!(c_ref_uncon);

		// By reusing 'g' below, we test that the previous property
		// is dropped when it i no longer used.

		type EnsuredGraphMut<'a> = MockEnsurer<
			MockUnloadedEnsurer<MockUnloadedEnsurer<&'a mut MockGraph<MockDirectedness>>>,
		>;

		// Test ensuring mutable reference to graph
		let mut c_ref_mut = EnsuredGraphMut::ensure_all(&mut g, (payload, ())).unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		// Test still is a MockEnsure after 1 release_all
		let (mut c_ref_mut_uncon, released_payload) = c_ref_mut.release();
		assert_eq!(payload, released_payload);
		assert_implements_mock_property_mut!(c_ref_mut_uncon);

		// We don't test release_all() explicitly now, because it happens automatically
		// when ensurer is no longer used and the reference is freed.

		type EnsuredGraph<'a> = MockUnloadedEnsurer<
			MockEnsurer<MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>>,
		>;

		// Test ensuring graph directly
		let mut c_owned = EnsuredGraph::ensure_all(g, (payload, ())).unwrap();
		assert_implements_mock_property_mut!(c_owned);

		// Test still is a MockEnsure after 1 release_all
		let (mut c_owned_uncon, ()) = c_owned.release();
		assert_implements_mock_property_mut!(c_owned_uncon);

		// Test all properties can be removed at once
		let (g, released_payload) = c_owned_uncon.release_all();
		assert_eq!((payload, ()), released_payload);
		g.validate_is_graph();
	}

	#[test]
	fn inline_ensurer_ensuring_base()
	{
		let mut g = ensurable_graph!();
		let payload = MockVertex { value: 1 };

		let c_ref = <MockEnsurer<MockEnsurer<&MockGraph<MockDirectedness>>>>::ensure_all(
			&g,
			(payload, (payload, ())),
		)
		.unwrap();
		assert_implements_mock_property!(c_ref);

		let mut c_ref_mut =
			<MockUnloadedEnsurer<MockEnsurer<&mut MockGraph<MockDirectedness>>>>::ensure_all(
				&mut g,
				(payload, ()),
			)
			.unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		let mut c_owned =
			<MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>>::ensure_all(
				g,
				(),
			)
			.unwrap();
		assert_implements_mock_property_mut!(c_owned);
	}

	#[test]
	fn base_ensures_self_by_inference()
	{
		type EnsuredGraph<G> = MockEnsurer<MockUnloadedEnsurer<MockUnloadedEnsurer<G>>>;
		let payload = MockVertex { value: 1 };

		let mut g = ensurable_graph!();

		let c_ref: EnsuredGraph<&MockGraph<MockDirectedness>> =
			(&g).ensure_all((payload, ())).unwrap();
		assert_implements_mock_property!(c_ref);
		let (c_ref_unc, released_payload) = c_ref.release();
		assert_eq!(payload, released_payload);
		assert_implements_mock_property!(c_ref_unc);

		let mut c_ref_mut: EnsuredGraph<&mut MockGraph<MockDirectedness>> =
			(&mut g).ensure_all((payload, ())).unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		let mut c_owned: EnsuredGraph<MockGraph<MockDirectedness>> =
			g.ensure_all((payload, ())).unwrap();
		assert_implements_mock_property_mut!(c_owned);
	}

	#[test]
	fn base_ensures_self_by_inline_inference()
	{
		let mut g = ensurable_graph!();
		let payload = MockVertex { value: 1 };

		let c_ref: MockEnsurer<
			MockUnloadedEnsurer<MockUnloadedEnsurer<MockEnsurer<&MockGraph<MockDirectedness>>>>,
		> = (&g).ensure_all((payload, (payload, ()))).unwrap();
		assert_implements_mock_property!(c_ref);
		let (c_ref_unc, released_payload) = c_ref.release();
		assert_eq!(released_payload, payload);
		assert_implements_mock_property!(c_ref_unc);

		let mut c_ref_mut: MockUnloadedEnsurer<
			MockEnsurer<MockEnsurer<MockUnloadedEnsurer<&mut MockGraph<MockDirectedness>>>>,
		> = (&mut g).ensure_all((payload, (payload, ()))).unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);
		let (mut c_ref_mut_unc, ()) = c_ref_mut.release();
		assert_implements_mock_property_mut!(c_ref_mut_unc);

		let c_owned: MockEnsurer<
			MockEnsurer<MockUnloadedEnsurer<MockEnsurer<MockGraph<MockDirectedness>>>>,
		> = g.ensure_all((payload, (payload, (payload, ())))).unwrap();
		assert_implements_mock_property!(c_owned);
		let (mut c_owned_unc, released_payload) = c_owned.release();
		assert_eq!(released_payload, payload);
		assert_implements_mock_property_mut!(c_owned_unc);

		let (g, released_payload) = c_owned_unc.release_all();
		g.validate_is_graph();
		assert_eq!(released_payload, (payload, (payload, ())));
	}
}

mod test_no_payload
{
	use crate::{
		core::ensure::{MockProperty, MockPropertyMut, MockUnloadedEnsurer},
		mock_graph::{MockDirectedness, MockGraph, MockVertexWeight},
	};
	use graphene::core::{BaseGraphUnloaded, EnsureUnloaded, ReleaseUnloaded};

	/// Test that defining a type alias allows for easy ensuring of a base graph
	#[test]
	fn pretyped_ensuring()
	{
		type EnsuredGraph = MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>;

		// Test can use `Ensure.ensure_all` on a base graph without needing type
		// annotation
		let g = EnsuredGraph::ensure_all(ensurable_graph!()).unwrap();

		// Test that `BaseGraph.ensure_all` can be used where the property is defined
		// elsewhere (in this case by an annotation, but could also be elsewhere and
		// then solved by type inference)
		let g2: EnsuredGraph = ensurable_graph!().ensure_all().unwrap();

		// Test can remove 1 property
		let _: MockUnloadedEnsurer<MockGraph<MockDirectedness>> = g.release();

		// Test can remove all properties
		let _: MockGraph<MockDirectedness> = g2.release_all();
	}

	/// Test that can define properties inline
	#[test]
	fn inline_ensuring()
	{
		// Test can use `Ensure.ensure_all` on a base graph using inline properties
		let g =
			<MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>>::ensure_all(
				ensurable_graph!(),
			)
			.unwrap();

		// Test that `BaseGraph.ensure_all` can be used where the property is defined
		// elsewhere (in this case by an annotation, but could also be elsewhere and
		// then solved by type inference)
		let g2: MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>> =
			ensurable_graph!().ensure_all().unwrap();

		// Test can remove 1 property
		let _: MockUnloadedEnsurer<MockGraph<MockDirectedness>> = g.release();

		// Test can remove all properties
		let _: MockGraph<MockDirectedness> = g2.release_all();
	}

	#[test]
	fn ensurer_ensuring_base()
	{
		type EnsuredGraphRef<'a> =
			MockUnloadedEnsurer<MockUnloadedEnsurer<&'a MockGraph<MockDirectedness>>>;

		let mut g = ensurable_graph!();

		// Test ensuring reference to graph
		let c_ref = EnsuredGraphRef::ensure_all(&g).unwrap();
		assert_implements_mock_property!(c_ref);

		// Test still is a MockEnsure after 1 release_all
		let c_ref_uncon = c_ref.release();
		assert_implements_mock_property!(c_ref_uncon);

		// By reusing 'g' below, we test that the previous property
		// is dropped when it i no longer used.

		type EnsuredGraphMut<'a> =
			MockUnloadedEnsurer<MockUnloadedEnsurer<&'a mut MockGraph<MockDirectedness>>>;

		// Test ensuring mutable reference to graph
		let mut c_ref_mut = EnsuredGraphMut::ensure_all(&mut g).unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		// Test still is a MockEnsure after 1 release_all
		let mut c_ref_mut_uncon = c_ref_mut.release();
		assert_implements_mock_property_mut!(c_ref_mut_uncon);

		// We don't test release_all() explicitly now, because it happens automatically
		// when ensurer is no longer used and the reference is freed.

		type EnsuredGraph<'a> = MockUnloadedEnsurer<
			MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>,
		>;

		// Test ensuring graph directly
		let mut c_owned = EnsuredGraph::ensure_all(g).unwrap();
		assert_implements_mock_property_mut!(c_owned);

		// Test still is a MockEnsure after 1 release_all
		let mut c_owned_uncon = c_owned.release();
		assert_implements_mock_property_mut!(c_owned_uncon);

		// Test all properties can be removed at once
		c_owned_uncon.release_all().validate_is_graph();
	}

	#[test]
	fn inline_ensurer_ensuring_base()
	{
		let mut g = ensurable_graph!();

		let c_ref =
			<MockUnloadedEnsurer<MockUnloadedEnsurer<&MockGraph<MockDirectedness>>>>::ensure_all(
				&g,
			)
			.unwrap();
		assert_implements_mock_property!(c_ref);

		let mut c_ref_mut = <MockUnloadedEnsurer<
			MockUnloadedEnsurer<&mut MockGraph<MockDirectedness>>,
		>>::ensure_all(&mut g)
		.unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		let mut c_owned =
			<MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>>>::ensure_all(g)
				.unwrap();
		assert_implements_mock_property_mut!(c_owned);
	}

	#[test]
	fn base_ensures_self_by_inference()
	{
		type EnsuredGraph<G> = MockUnloadedEnsurer<MockUnloadedEnsurer<G>>;

		let mut g = ensurable_graph!();

		let c_ref: EnsuredGraph<&MockGraph<MockDirectedness>> = (&g).ensure_all().unwrap();
		assert_implements_mock_property!(c_ref);
		let c_ref_unc = c_ref.release();
		assert_implements_mock_property!(c_ref_unc);

		let mut c_ref_mut: EnsuredGraph<&mut MockGraph<MockDirectedness>> =
			(&mut g).ensure_all().unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);

		let mut c_owned: EnsuredGraph<MockGraph<MockDirectedness>> = g.ensure_all().unwrap();
		assert_implements_mock_property_mut!(c_owned);
	}

	#[test]
	fn base_ensures_self_by_inline_inference()
	{
		let mut g = ensurable_graph!();

		let c_ref: MockUnloadedEnsurer<MockUnloadedEnsurer<&MockGraph<MockDirectedness>>> =
			(&g).ensure_all().unwrap();
		assert_implements_mock_property!(c_ref);
		let c_ref_unc = c_ref.release();
		assert_implements_mock_property!(c_ref_unc);

		let mut c_ref_mut: MockUnloadedEnsurer<
			MockUnloadedEnsurer<&mut MockGraph<MockDirectedness>>,
		> = (&mut g).ensure_all().unwrap();
		assert_implements_mock_property_mut!(c_ref_mut);
		let mut c_ref_mut_unc = c_ref_mut.release();
		assert_implements_mock_property_mut!(c_ref_mut_unc);

		let c_owned: MockUnloadedEnsurer<MockUnloadedEnsurer<MockGraph<MockDirectedness>>> =
			g.ensure_all().unwrap();
		assert_implements_mock_property!(c_owned);
		let mut c_owned_unc = c_owned.release();
		assert_implements_mock_property_mut!(c_owned_unc);

		c_owned_unc.release_all().validate_is_graph()
	}
}
