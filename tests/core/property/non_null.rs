/// Tests `NonNullGraph`
use crate::mock_graph::arbitrary::ArbVertexIn;
use crate::mock_graph::{MockDirectedness, MockGraph, MockVertexWeight};
use graphene::core::{
	property::{NewVertex, NonNull, NonNullGraph, RemoveVertex},
	Graph, Insure,
};

duplicate_for_directedness! {
	$directedness

	/// Tests that null graphs are rejected.
	#[test]
	fn reject_null()
	{
		let null_graph = MockGraph::<directedness>::empty();

		assert!(!NonNullGraph::validate(&null_graph));
	}

	/// Tests that graphs with at least 1 vertex are accepted.
	#[quickcheck]
	fn accept_non_null(ArbVertexIn(g,_): ArbVertexIn<MockGraph<directedness>>) -> bool
	{
		NonNullGraph::validate(&g)
	}

	/// Tests that can remove a vertex if there are at least 2.
	#[quickcheck]
	fn accept_remove_vertex(
		ArbVertexIn(g,v): ArbVertexIn<MockGraph<MockDirectedness>>,
		w: MockVertexWeight
	) -> bool
	{
		let mut g = NonNullGraph::insure(g).unwrap();
		g.new_vertex_weighted(w).unwrap();

		g.remove_vertex(v).is_ok()
	}

	/// Tests cannot remmove a vertex if its the only one in the graph.
	#[test]
	fn reject_remove_vertex()
	{
		// Create a graph with examp
		let mut g = MockGraph::<directedness>::empty();
		let v = g.new_vertex_weighted(MockVertexWeight{value: 0}).unwrap();

		let mut g = NonNullGraph::insure(g).unwrap();

		assert!(g.remove_vertex(v).is_err())
	}

	/// Tests that `get_vertex()` returns a vertex in the graph.
	#[quickcheck]
	fn get_vertex(
		ArbVertexIn(g,_): ArbVertexIn<MockGraph<MockDirectedness>>
	) -> bool
	{
		let g = NonNullGraph::insure(g).unwrap();

		g.contains_vertex(g.get_vertex())
	}
}
