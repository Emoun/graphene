use crate::mock_graph::{utilities::auto_copy_from, MockEdgeWeight, MockVertex, MockVertexWeight};
use graphene::{
	common::{AdjListGraph, VertexMapGraph},
	core::{property::NewVertex, Graph},
};
use std::collections::HashMap;

mod ensured;
mod impl_graph;

fn adj_list_from_mock<G>(
	mock: &G,
) -> (
	AdjListGraph<MockVertexWeight, MockEdgeWeight, G::Directedness>,
	HashMap<MockVertex, usize>,
)
where
	G: Graph<Vertex = MockVertex, EdgeWeight = MockEdgeWeight, VertexWeight = MockVertexWeight>,
{
	let mut g = AdjListGraph::new();
	let map = auto_copy_from(&mut g, mock, |g, _, w| g.new_vertex_weighted(w).unwrap());
	(g, map)
}

pub fn vertex_map_from_mock<G>(
	mock: &G,
) -> (
	VertexMapGraph<MockVertex, AdjListGraph<MockVertexWeight, MockEdgeWeight, G::Directedness>>,
	HashMap<MockVertex, MockVertex>,
)
where
	G: Graph<Vertex = MockVertex, EdgeWeight = MockEdgeWeight, VertexWeight = MockVertexWeight>,
{
	let mut g = VertexMapGraph::<MockVertex, AdjListGraph<_, _, _>>::new();
	let map = auto_copy_from(&mut g, mock, |g, v, w| {
		g.add_vertex_weighted(v.clone(), w).unwrap();
		v
	});
	(g, map)
}
