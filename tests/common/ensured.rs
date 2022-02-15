//! Tests `Ensured` and `EnsuredGraph` to make sure
//! the new versions of each method give the same result
//! as the original.
use crate::mock_graph::{arbitrary::Arb, MockGraph, MockVertex, MockVertexWeight};
use duplicate::duplicate_item;
use graphene::{
	common::Ensured,
	core::{
		property::{HasVertex, NewVertex},
		Directed, Graph,
	},
};

/// Tests that `contains_vertex` for EnsureGraph is identical to the original.
///
/// We use duplicate here simply to ensure the same name is used
/// for both calls. This will ensure even through one of them is
/// renamed (using a tool to find and rename usages automatically), this
/// will fail to compile if the other isn't renamed too.
#[duplicate_item(
	contains_vertex; [contains_vertex]
)]
#[quickcheck]
fn contains_vertex(Arb(graph): Arb<MockGraph<Directed>>, v: MockVertex) -> bool
{
	let graph_clone = graph.clone();

	let expected = graph.contains_vertex(v);
	let result = graph_clone.ensured().contains_vertex(v);
	let result_is_some = result.is_some();
	let result_v = result.map_or(v, |graph| graph.get_vertex());

	(expected == result_is_some) && (v == result_v)
}

/// Tests that `new_vertex` and `new_vertex_weighted` for EnsureGraph are
/// identical to their originals.
#[duplicate_item(
	new_vertex				arguments;
	[new_vertex_weighted]	[_w.clone()];
	[new_vertex]			[]
)]
#[quickcheck]
fn new_vertex(Arb(mut graph): Arb<MockGraph<Directed>>, _w: MockVertexWeight) -> bool
{
	let graph_clone = graph.clone();

	let expected = graph.new_vertex(arguments);
	let expected_is_ok = expected.is_ok();
	let expected_v = expected.unwrap_or(MockVertex { value: 0 });
	let result = graph_clone.ensured().new_vertex(arguments);
	let result_is_ok = result.is_ok();
	let result_v = result.map_or(expected_v, |graph| graph.get_vertex());

	(expected_is_ok == result_is_ok) && (expected_v == result_v)
}
