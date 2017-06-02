use super::*;


fn increases_edge_count(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
								 -> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	
	after_init_and_add_edge(&desc, source_i_cand, sink_i_cand, |g,_|{
		g.all_edges().len() == (desc.edges.len() + 1)
	})
}

fn maintain_original_edges(desc: ArbitraryGraphDescription<u32>,
									source_i_cand: usize, sink_i_cand:usize)
									-> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	
	original_edges_maintained_subsetof_graph_after(desc, |d, g|{
		add_appropriate_edge(d, g, source_i_cand,sink_i_cand);
	})
}

fn graph_subsetof_original_edges_and_added_edge(desc: ArbitraryGraphDescription<u32>,
														 source_i_cand: usize, sink_i_cand:usize)
														 -> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_graph_init(&desc, |mut g|{
		let edge = add_appropriate_edge(&desc,&mut g, source_i_cand, sink_i_cand);
		let mut original_edges_v = edges_by_value(&desc);
		original_edges_v.push((edge.source(), edge.sink()));
		graph_subsetof_edges(&g, &original_edges_v)
	})
}

fn maintains_vertices(desc: ArbitraryGraphDescription<u32>,
							   source_i_cand: usize, sink_i_cand:usize)
							   -> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_init_and_add_edge(&desc, source_i_cand, sink_i_cand, |g, _|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

fn reject_invalid_source(desc: ArbitraryGraphDescription<u32>,
								  source: u32, sink: u32) -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_source = invalidate_vertice(source, &desc);
		
		g.add_edge(BaseEdge::new(invalid_source, sink,())).is_err()
	})
}

fn reject_invalid_sink(desc: ArbitraryGraphDescription<u32>,
								source: u32, sink: u32) -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_sink = invalidate_vertice(sink, &desc);
		
		g.add_edge(BaseEdge::new(source ,invalid_sink,())).is_err()
	})
}


quickcheck!{
	fn PROP_increases_edge_count(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		increases_edge_count(desc, source_i_cand, sink_i_cand)
	}
	
	fn PROP_maintain_original_edges(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		maintain_original_edges(desc, source_i_cand, sink_i_cand)
	}
	
	fn PROP_graph_subsetof_original_edges_and_added_edge
	(desc: ArbitraryGraphDescription<u32>,source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		graph_subsetof_original_edges_and_added_edge(desc, source_i_cand, sink_i_cand)
	}
	
	fn PROP_maintains_vertices
	(desc: ArbitraryGraphDescription<u32>,source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		maintains_vertices(desc, source_i_cand, sink_i_cand)
	}
	
	fn PROP_reject_invalid_source
	(desc: ArbitraryGraphDescription<u32>,source: u32, sink:u32)
	-> bool{
		reject_invalid_source(desc, source, sink)
	}
	
	fn PROP_reject_invalid_sink
	(desc: ArbitraryGraphDescription<u32>,source: u32, sink:u32)
	-> bool{
		reject_invalid_sink(desc, source, sink)
	}

}