/// Tests `HasVertexGraph` and `VertexInGraph`
use crate::mock_graph::arbitrary::Unique;
use crate::mock_graph::{
	arbitrary::{Arb, TwoVerticesIn},
	MockDirectedness, MockGraph, MockVertexWeight,
};
use duplicate::duplicate;
use graphene::core::{
	property::{HasVertex, HasVertexGraph, NewVertex, RemoveVertex, VertexInGraph},
	Directed, Undirected,
};

#[duplicate(
	directedness; [ Directed ]; [ Undirected ]
)]
mod __
{
	use super::*;
	mod non_null
	{
		use super::*;
		use graphene::core::{EnsureUnloaded, ReleaseUnloaded};

		/// Tests that null graphs are rejected.
		#[test]
		fn reject_null()
		{
			let null_graph = MockGraph::<directedness>::empty();

			assert!(!HasVertexGraph::validate(&null_graph));
		}

		/// Tests that graphs with at least 1 vertex are accepted.
		#[quickcheck]
		fn accept_non_null(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			HasVertexGraph::validate(&g.release_all())
		}

		/// Tests cannot remove a vertex if its the only one in the graph.
		#[test]
		fn reject_remove_vertex()
		{
			let mut g = MockGraph::<directedness>::empty();
			let v = g
				.new_vertex_weighted(MockVertexWeight { value: 0 })
				.unwrap();

			let mut g = HasVertexGraph::ensure(g).unwrap();

			assert!(g.remove_vertex(v).is_err())
		}

		/// Tests that can remove a vertex if there are at least 2.
		#[quickcheck]
		fn non_null_accept_remove_vertex(
			Arb(g): Arb<TwoVerticesIn<MockGraph<directedness>, Unique>>,
		) -> bool
		{
			let v = g.get_vertex();
			let mut g = HasVertexGraph::ensure(g.release_all()).unwrap();

			g.remove_vertex(v).is_ok()
		}
	}

	mod vertex_in
	{
		use super::*;
		use crate::mock_graph::arbitrary::VertexOutside;
		use graphene::core::{Ensure, Release};

		/// Tests that graphs with at least 1 vertex are accepted.
		#[quickcheck]
		fn accept_in_graph(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>) -> bool
		{
			VertexInGraph::validate(&g, &g.get_vertex())
		}

		/// Tests that vertices not in the graph are rejected.
		#[quickcheck]
		fn reject_not_in_graph(Arb(g): Arb<VertexOutside<MockGraph<directedness>>>) -> bool
		{
			!VertexInGraph::validate(&g.0, &g.1)
		}

		/// Tests that can remove a vertex if its not the one guaranteed by
		/// VertexInGraph
		#[quickcheck]
		fn vertex_in_accept_remove_vertex(
			Arb(g): Arb<TwoVerticesIn<MockGraph<MockDirectedness>, Unique>>,
		) -> bool
		{
			let (v1, v2) = g.get_both();
			let mut g = VertexInGraph::ensure_unvalidated(g.release_all().0, v1);

			g.remove_vertex(v2).is_ok()
		}

		/// Tests cannot remove a vertex if its the one guaranteed by
		/// VertexInGraph
		#[quickcheck]
		fn reject_remove_vertex(Arb(g): Arb<VertexInGraph<MockGraph<directedness>>>)
		{
			let v = g.get_vertex();
			let mut g = VertexInGraph::ensure_unvalidated(g, v);

			assert!(g.remove_vertex(v).is_err())
		}
	}
}
