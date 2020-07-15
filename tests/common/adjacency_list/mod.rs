use crate::mock_graph::{utilities::auto_copy_from, MockEdgeWeight, MockVertex, MockVertexWeight};
use graphene::{common::AdjListGraph, core::Graph};
use std::collections::HashMap;

mod impl_graph;

fn adj_list_from_mock<G>(
	mock: &G,
) -> (
	AdjListGraph<MockVertexWeight, MockEdgeWeight, G::Directedness>,
	HashMap<MockVertex, usize>,
)
where
	G: Graph<Vertex = MockVertex, EdgeWeight = MockEdgeWeight, VertexWeight = MockVertexWeight, VertexRef = MockVertex>,
{
	let mut g = AdjListGraph::new();
	let map = auto_copy_from(&mut g, mock);
	(g, map)
}
