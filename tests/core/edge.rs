//! Tests the edge traits' API
//!
#![allow(unused_must_use)]

use crate::mock_graph::{MockEdgeWeight, MockVertex};
use graphene::core::Edge;

#[test]
fn edge_test()
{
	let weight = MockEdgeWeight { value: 2 };
	let mut weight_mut = MockEdgeWeight { value: 2 };

	let no_weight_edge = (MockVertex { value: 0 }, MockVertex { value: 1 });
	let _: MockVertex = no_weight_edge.source();
	let _: MockVertex = no_weight_edge.sink();

	let weight_edge = (
		MockVertex { value: 0 },
		MockVertex { value: 1 },
		MockEdgeWeight { value: 2 },
	);
	let _: MockVertex = weight_edge.source();
	let _: MockVertex = weight_edge.sink();

	let weight_ref_edge = (MockVertex { value: 0 }, MockVertex { value: 1 }, &weight);
	let _: MockVertex = weight_ref_edge.source();
	let _: MockVertex = weight_ref_edge.sink();

	let weight_ref_mut_edge = (
		MockVertex { value: 0 },
		MockVertex { value: 1 },
		&mut weight_mut,
	);
	let _: MockVertex = weight_ref_mut_edge.source();
	let _: MockVertex = weight_ref_mut_edge.sink();
}
