use crate::mock_graph::{
	utilities::auto_copy_from, MockEdgeWeight, MockGraph, MockVertex, MockVertexWeight,
};
use graphene::{common::AdjListGraph, core::Directedness};
use std::collections::HashMap;

mod impl_graph;

fn adj_list_from_mock<D: Directedness>(
	mock: &MockGraph<D>,
) -> (
	AdjListGraph<MockVertexWeight, MockEdgeWeight, D>,
	HashMap<MockVertex, usize>,
)
{
	let mut g = AdjListGraph::new();
	let map = auto_copy_from(&mut g, mock);
	(g, map)
}
