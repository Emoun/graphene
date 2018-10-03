
use mock_graphs::*;
use common::adjacency_list::utilities::*;
use graphene::common::AdjListGraph;
use graphene::core::*;

fn increases_edge_count(mock: MockGraph, source_i_cand: usize,
						sink_i_cand:usize, weight: MockEdgeWeight)
	-> bool
{
	holds_if!(mock.vertices.len() == 0);
	
	AdjListGraph_init_and_add_edge(&mock, source_i_cand, sink_i_cand, weight, |g, _|{
		g.all_edges().len() == (mock.all_edges().len() + 1)
	})
}
/*
fn maintain_original_edges(mock: MockGraph,
						   source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
						   -> bool
{
	holds_if!(mock.values.len() == 0);
	
	original_edges_maintained_sublistof_graph_after::<AdjListGraph<_,_>,_>(mock, |d, g|{
		add_appropriate_edge(d, g, source_i_cand,sink_i_cand,weight);
	})
}

fn graph_subsetof_original_edges_and_added_edge(mock: MockGraph,
												source_i_cand: usize, sink_i_cand:usize,
												weight: MockEdgeWeight)
												-> bool
{
	holds_if!(mock.values.len() == 0);
	AdjListGraph_init(&mock, |mut g|{
		let edge = add_appropriate_edge(&mock,&mut g, source_i_cand, sink_i_cand,weight);
		let mut original_edges_v = mock.edges_by_value();
		original_edges_v.push((edge.source, edge.sink,edge.weight));
		graph_sublistof_edges(&g, &original_edges_v)
	})
}

fn maintains_vertices(mock: MockGraph,
					  source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
					  -> bool
{
	holds_if!(mock.values.len() == 0);
	AdjListGraph_init_and_add_edge(&mock, source_i_cand, sink_i_cand, weight, |g, _|{
		equal_mockription_and_graph_vertices(&mock, &g)
	})
}

fn reject_invalid_source(mock: MockGraph,
						 source: u32, sink: u32, weight: MockEdgeWeight) -> bool
{
	AdjListGraph_init(&mock, |mut g|{
		let invalid_source = invalidate_vertice(source, &mock);
		
		g.add_edge(BaseEdge::new(invalid_source, sink,weight)).is_err()
	})
}

fn reject_invalid_sink(mock: MockGraph,
					   source: u32, sink: u32, weight: MockEdgeWeight) -> bool
{
	AdjListGraph_init(&mock, |mut g|{
		let invalid_sink = invalidate_vertice(sink, &mock);
		
		g.add_edge(BaseEdge::new(source ,invalid_sink,weight)).is_err()
	})
}
*/

quickcheck!{
	fn PROP_increases_edge_count(mock: MockGraph,
								 source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
	-> bool{
		increases_edge_count(mock, source_i_cand, sink_i_cand, weight)
	}
	/*
	fn PROP_maintain_original_edges(mock: MockGraph,
								 source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
	-> bool{
		maintain_original_edges(mock, source_i_cand, sink_i_cand, weight)
	}
	
	fn PROP_graph_subsetof_original_edges_and_added_edge
	(mock: MockGraph,source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
	-> bool{
		graph_subsetof_original_edges_and_added_edge(mock, source_i_cand, sink_i_cand, weight)
	}
	
	fn PROP_maintains_vertices
	(mock: MockGraph,source_i_cand: usize, sink_i_cand:usize, weight: MockEdgeWeight)
	-> bool{
		maintains_vertices(mock, source_i_cand, sink_i_cand, weight)
	}
	
	fn PROP_reject_invalid_source
	(mock: MockGraph,source: u32, sink:u32, weight: MockEdgeWeight)
	-> bool{
		reject_invalid_source(mock, source, sink, weight)
	}
	
	fn PROP_reject_invalid_sink
	(mock: MockGraph,source: u32, sink:u32, weight: MockEdgeWeight)
	-> bool{
		reject_invalid_sink(mock, source, sink, weight)
	}
	*/
}