/// Tests `NonNullGraph` and `VertexInGraph`
use crate::mock_graph::arbitrary::{ArbTwoUniqueVerticesIn, ArbVertexIn};
use crate::mock_graph::{MockDirectedness, MockGraph, MockVertexWeight};
use duplicate::duplicate;
use graphene::core::{
	property::{NewVertex, NonNullGraph, RemoveVertex, VertexInGraph},
	Directed, Insure, Undirected,
};

#[duplicate(
	[
		module			[ directed ]
		directedness 	[ Directed ]
	]
	[
		module			[ undirected ]
		directedness 	[ Undirected ]
	]
)]
mod module
{
	use super::*;

	#[duplicate(
		[
			module2		[ non_null_graph ]
			ensurer 	[ NonNullGraph ]
		]
		[
			module2		[ vertex_in_graph ]
			ensurer 	[ VertexInGraph ]
		]
	)]
	mod module2
	{
		use super::*;

		/// Tests that null graphs are rejected.
		#[test]
		fn reject_null()
		{
			let null_graph = MockGraph::<directedness>::empty();

			assert!(!ensurer::validate(&null_graph));
		}

		/// Tests that graphs with at least 1 vertex are accepted.
		#[quickcheck]
		fn accept_non_null(g: ArbVertexIn<MockGraph<directedness>>) -> bool
		{
			ensurer::validate(&g)
		}

		/// Tests cannot remove a vertex if its the only one in the graph.
		#[test]
		fn reject_remove_vertex()
		{
			// Create a graph with examp
			let mut g = MockGraph::<directedness>::empty();
			let v = g
				.new_vertex_weighted(MockVertexWeight { value: 0 })
				.unwrap();

			let mut g = ensurer::insure(g).unwrap();

			assert!(g.remove_vertex(v).is_err())
		}
	}

	/// Tests that can remove a vertex from NonNullGraph if there are at least
	/// 2.
	#[quickcheck]
	fn non_null_accept_remove_vertex(
		ArbTwoUniqueVerticesIn(g, v, _): ArbTwoUniqueVerticesIn<MockGraph<MockDirectedness>>,
	) -> bool
	{
		let mut g = NonNullGraph::insure(g).unwrap();

		g.remove_vertex(v).is_ok()
	}

	/// Tests that can remove a vertex if its not the one guaranteed by
	/// VertexInGraph
	#[quickcheck]
	fn vertex_in_accept_remove_vertex(
		ArbTwoUniqueVerticesIn(g, v1, v2): ArbTwoUniqueVerticesIn<MockGraph<MockDirectedness>>,
	) -> bool
	{
		let mut g = VertexInGraph::new(g, v1).unwrap();

		g.remove_vertex(v2).is_ok()
	}
}
