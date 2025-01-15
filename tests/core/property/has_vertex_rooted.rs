/// Tests `HasVertexGraph`, VertexInGraph`, and `RootedGraph`
use crate::mock_graph::arbitrary::Unique;
use crate::mock_graph::{
	arbitrary::{Arb, TwoVerticesIn},
	MockGraph, MockVertexWeight,
};
use duplicate::duplicate_item;
use graphene::core::{
	property::{
		HasVertex, HasVertexGraph, NewVertex, RemoveVertex, Rooted, RootedGraph, VertexInGraph,
	},
	Directed, Undirected,
};

#[duplicate_item(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;
	mod has_vertex
	{
		use super::*;
		use graphene::core::{Guard, Release};

		/// Tests that null graphs are rejected.
		#[test]
		fn reject_null()
		{
			let null_graph = MockGraph::<directedness>::empty();

			assert!(!HasVertexGraph::can_guard(&null_graph));
		}

		/// Tests that graphs with at least 1 vertex are accepted.
		#[quickcheck]
		fn accept_has_vertex(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			HasVertexGraph::can_guard(&g.release_all())
		}

		/// Tests cannot remove a vertex if it's the only one in the graph.
		#[test]
		fn reject_remove_vertex()
		{
			let mut g = MockGraph::<directedness>::empty();
			let v = g
				.new_vertex_weighted(MockVertexWeight { value: 0 })
				.unwrap();

			let mut g = HasVertexGraph::guard(g).unwrap();

			assert!(g.remove_vertex(v).is_err())
		}

		/// Tests that can remove a vertex if there are at least 2.
		#[quickcheck]
		fn accept_remove_vertex(Arb(g): Arb<TwoVerticesIn<MockGraph<directedness>, Unique>>)
			-> bool
		{
			let v = g.get_vertex();
			let mut g = HasVertexGraph::guard(g.release_all()).unwrap();

			g.remove_vertex(v).is_ok()
		}
	}

	#[duplicate_item(
		GraphStruct 		get_method 		set_method		ensure_wrap(v);
		[ VertexInGraph ] 	[ get_vertex ] 	[ set_vertex ]	[[v]];
		[ RootedGraph ] 	[ root ] 		[ set_root ]	[v];
	)]
	mod __
	{
		use super::*;
		use crate::mock_graph::arbitrary::VertexOutside;
		use graphene::core::{Ensure, ReleasePayload};

		/// Tests that graphs with at least 1 vertex are accepted.
		#[quickcheck]
		fn accept_in_graph(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			GraphStruct::can_ensure(&g, &ensure_wrap([g.get_vertex()]))
		}

		/// Tests that vertices not in the graph are rejected.
		#[quickcheck]
		fn reject_not_in_graph(Arb(g): Arb<VertexOutside<MockGraph<directedness>>>) -> bool
		{
			!GraphStruct::can_ensure(&g.0, &ensure_wrap([g.1]))
		}

		/// Tests that can remove a vertex if its not the one guaranteed by
		/// the graph
		#[quickcheck]
		fn vertex_in_accept_remove_vertex(
			Arb(g): Arb<TwoVerticesIn<MockGraph<directedness>, Unique>>,
		) -> bool
		{
			let (v1, v2) = g.get_both();
			let mut g = GraphStruct::ensure_unchecked(g.release_all().0, ensure_wrap([v1]));

			g.remove_vertex(v2).is_ok()
		}

		/// Tests cannot remove a vertex if its the one guaranteed by
		/// the graph
		#[quickcheck]
		fn reject_remove_vertex(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			let v = g.get_vertex();
			let mut g = GraphStruct::ensure_unchecked(g, ensure_wrap([v]));

			g.remove_vertex(v).is_err()
		}

		/// Tests the graph can get the underlying vertex
		#[quickcheck]
		fn get_vertex(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			let v = g.get_vertex();
			let g = GraphStruct::ensure_unchecked(g.release_all().0, ensure_wrap([v]));

			g.get_method() == v
		}

		/// Tests that the graph can change the specific underlying
		/// vertex
		#[quickcheck]
		fn set_vertex(Arb(g): Arb<TwoVerticesIn<MockGraph<directedness>, Unique>>) -> bool
		{
			let (v1, v2) = g.get_both();
			let mut g = GraphStruct::ensure_unchecked(g.release_all().0, ensure_wrap([v1]));

			g.set_method(ensure_wrap([v2])).is_ok() && g.get_method() == v2
		}

		/// Tests that the graph rejects changing the underlying vertex
		/// to one that isn't in the graph.
		#[quickcheck]
		fn set_vertex_wrong(
			Arb(g): Arb<VertexOutside<VertexInGraph<MockGraph<directedness>>>>,
		) -> bool
		{
			let v1 = g.0.get_vertex();
			let v2 = g.1;
			let mut g = GraphStruct::ensure_unchecked(g.release_all().0, ensure_wrap([v1]));

			g.set_method(ensure_wrap([v2])).is_err()
		}
	}

	/// Tests that RootedGraphs `is_root` returns true if given the root
	#[quickcheck]
	fn is_root_true(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
	{
		use graphene::core::{Ensure, Release};
		let v = g.get_vertex();
		let g = RootedGraph::ensure_unchecked(g.release_all(), v);

		g.is_root(v)
	}

	/// Tests that RootedGraphs `is_root` returns false when not given the root
	#[quickcheck]
	fn is_root_false(Arb(g): Arb<TwoVerticesIn<MockGraph<directedness>, Unique>>) -> bool
	{
		use graphene::core::{Ensure, Release};
		let (v1, v2) = g.get_both();
		let g = RootedGraph::ensure_unchecked(g.release_all(), v1);

		!g.is_root(v2)
	}
}
