
use mock_graphs::*;
use common::utilities::*;
use graphene::common::AdjListGraph;

pub fn AdjListGraph_init<F>(mock: &MockGraph, holds: F) -> bool
	where F: Fn(&mut AdjListGraph<MockVertex, MockVertexWeight, MockEdgeWeight>) -> bool
{
	graph_init(&mut AdjListGraph::new(), mock, holds)
}

pub fn AdjListGraph_init_and_add_edge<F>(
	mock: &MockGraph, source_i_cand: usize,
	sink_i_cand:usize, weight: MockEdgeWeight,
	holds: F)
	-> bool
	where F: FnOnce(&mut AdjListGraph<MockVertex, MockVertexWeight, MockEdgeWeight>,
					(MockVertex, MockVertex, MockEdgeWeight))
				-> bool
{
	graph_init_and_add_edge(&mut AdjListGraph::new(), mock, source_i_cand, sink_i_cand, weight, holds)
}
/*
pub fn AdjListGraph_init_and_remove_edge<V,W,F>(
	desc: &GraphDescription<V,W>,
	edge_index: usize, holds: F)
	-> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		F: Fn(AdjListGraph<V,W>,(usize,BaseEdge<V,W>)) -> bool,
{
	graph_init_and_remove_edge::<AdjListGraph<_,_>,_>(desc, edge_index, holds)
}
*/

