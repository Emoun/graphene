use crate::mock_graph::{
	arbitrary::{Arb, NonTreeGraph, VertexOutside},
	MockEdgeWeight, MockGraph, MockVertexWeight,
};
use duplicate::duplicate_item;
use graphene::core::{
	property::{
		Acyclic, AddEdge, HasVertex, NewVertex, RemoveEdge, RemoveVertex, Tree, TreeGraph, Unique,
		VertexCount, VertexIn, VertexInGraph, Weak,
	},
	Directed, Graph, Guard, Release, Undirected,
};
use static_assertions::{assert_impl_all, assert_not_impl_any};

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;
	use graphene::core::property::NewLeafUndirected;

	/// Tests that TreeGraph correctly identifies its own graphs.
	#[quickcheck]
	fn accept_tree(g: Arb<TreeGraph<MockGraph<directedness, MockEdgeWeight>>>) -> bool
	{
		TreeGraph::can_guard(&g.0.release_all())
	}

	/// Tests that TreeGraph correctly rejects non-trees
	#[quickcheck]
	fn reject_non_tree(g: Arb<NonTreeGraph<directedness, MockEdgeWeight>>) -> bool
	{
		!TreeGraph::can_guard(&g.0.release_all())
	}

	/// Tests can add leaf to existing parent
	#[quickcheck]
	fn new_leaf(
		Arb(g): Arb<VertexInGraph<TreeGraph<MockGraph<directedness, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let mut clone = g.0.clone();
		let result = clone.new_leaf_weighted(g.vertex_at::<0>(), v_weight, e_weight);

		result.is_ok()
			&& clone.vertex_count() == g.0.vertex_count() + 1
			&& TreeGraph::can_guard(&clone.release_all())
	}

	/// Tests cannot add leaf to non-existent parent
	#[quickcheck]
	fn new_leaf_reject(
		Arb(g): Arb<VertexOutside<TreeGraph<MockGraph<directedness, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		let mut clone = g.0.clone();
		let result = clone.new_leaf_weighted(g.1, v_weight, e_weight);

		result.is_err()
			&& clone.vertex_count() == g.0.vertex_count()
			&& TreeGraph::can_guard(&g.0.release_all())
	}

	/// Tests can remove leaf
	#[quickcheck]
	fn remove_leaf(
		Arb(g): Arb<VertexInGraph<TreeGraph<MockGraph<directedness, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		// Start by adding a leaf
		let mut clone = g.0.clone();
		let leaf = clone
			.new_leaf_weighted(g.vertex_at::<0>(), v_weight, e_weight)
			.unwrap();

		// Then try to remove it
		clone.remove_vertex(leaf).is_ok()
			&& clone.vertex_count() == g.0.vertex_count()
			&& TreeGraph::can_guard(&clone.release_all())
	}

	/// Tests cannot remove non-leaf
	#[quickcheck]
	fn remove_non_leaf(
		Arb(g): Arb<VertexInGraph<TreeGraph<MockGraph<directedness, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
	) -> bool
	{
		// Start by adding 2 leaves to one vertex, ensuring it can no longer be a leaf
		let mut clone = g.0.clone();
		clone
			.new_leaf_weighted(g.vertex_at::<0>(), v_weight.clone(), e_weight.clone())
			.unwrap();
		clone
			.new_leaf_weighted(g.vertex_at::<0>(), v_weight, e_weight)
			.unwrap();

		// Then try to remove the parent
		clone.remove_vertex(g.vertex_at::<0>()).is_err()
			&& clone.vertex_count() == g.0.vertex_count() + 2
	}

	assert_impl_all!(TreeGraph<MockGraph<directedness, MockEdgeWeight>>:
		Tree, HasVertex, Unique, Weak, Acyclic);
	assert_not_impl_any!(TreeGraph<MockGraph<directedness, MockEdgeWeight>>:
		NewVertex, AddEdge, RemoveEdge);
}

mod directed
{
	use graphene::core::property::NewLeafDirected;

	use super::*;
	/// Tests can add leaf to existing parent
	#[quickcheck]
	fn new_leaf_to_new(
		Arb(g): Arb<VertexInGraph<TreeGraph<MockGraph<Directed, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
		to_new: bool,
	) -> bool
	{
		let mut clone = g.0.clone();
		let result = clone.new_leaf_weighted(g.vertex_at::<0>(), to_new, v_weight, e_weight);

		result.is_ok()
			&& clone.vertex_count() == g.0.vertex_count() + 1
			&& clone.edges_sinked_in(result.unwrap()).count() == (to_new as usize)
			&& TreeGraph::can_guard(&clone.release_all())
	}

	/// Tests cannot add leaf to non-existent parent
	#[quickcheck]
	fn new_leaf_reject_directed(
		Arb(g): Arb<VertexOutside<TreeGraph<MockGraph<Directed, MockEdgeWeight>>>>,
		v_weight: MockVertexWeight,
		e_weight: MockEdgeWeight,
		to_new: bool,
	) -> bool
	{
		let mut clone = g.0.clone();
		let result = clone.new_leaf_weighted(g.1, to_new, v_weight, e_weight);

		result.is_err()
			&& clone.vertex_count() == g.0.vertex_count()
			&& TreeGraph::can_guard(&g.0.release_all())
	}
}
