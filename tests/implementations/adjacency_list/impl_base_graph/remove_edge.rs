use super::*;

fn decreases_edge_count(desc: ArbitraryGraphDescription<u32>,
									edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	after_init_and_remove_edge(&desc, edge_index, |g, _|{
		(desc.edges.len() -1) == g.all_edges().len()
	})
}

fn maintains_vertices(desc: ArbitraryGraphDescription<u32>,
								  edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	after_init_and_remove_edge(&desc, edge_index, |g, _|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

fn after_graph_is_equals_to_desc_minus_edge(desc: ArbitraryGraphDescription<u32>,
														edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	after_init_and_remove_edge(&desc, edge_index, |g, (i,_)|{
		let mut desc_clone = desc.clone();
		desc_clone.edges.remove(i);
		equal_description_and_graph_edges(&desc_clone, &g)
	})
}

fn rejects_non_edge(	desc: ArbitraryGraphDescription<u32>,
									source_i_cand:usize, sink_i_cand: usize)
									-> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_graph_init(&desc, |mut g|{
		let v_nr = desc.vertex_values.len();
		let mut source_i = source_i_cand % v_nr;
		let mut sink_i = sink_i_cand % v_nr;
		
		let mut i = 0;
		while desc.edges.contains(&(source_i, sink_i)) && i<v_nr {
			source_i += 1;
			source_i %= v_nr;
			let mut j = 0;
			while desc.edges.contains(&(source_i,sink_i)) && j < v_nr{
				sink_i += 1;
				sink_i %= v_nr;
				j += 1;
			}
			i += 1;
		}
		if desc.edges.contains(&(source_i, sink_i)) {
			//The graph contains all edge possibilities.
			//Since we cannot find an edge that is not present,
			//the property must hold
			return true;
		}
		let edge = BaseEdge::new(desc.vertex_values[source_i], desc.vertex_values[sink_i],());
		g.remove_edge(edge).is_err()
	})
}

fn rejects_invalid_source(	desc: ArbitraryGraphDescription<u32>,
										  source:u32, sink: u32)
										  -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_source = invalidate_vertice(source, &desc);
		
		g.remove_edge(BaseEdge::new(invalid_source, sink,())).is_err()
	})
}

fn rejects_invalid_sink(	desc: ArbitraryGraphDescription<u32>,
										source:u32, sink: u32)
										-> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_sink = invalidate_vertice(sink, &desc);
		
		g.remove_edge(BaseEdge::new(source, invalid_sink,())).is_err()
	})
}

quickcheck!{
	fn PROP_decreases_edge_count
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		decreases_edge_count(desc, edge_index)
	}
	
	fn PROP_maintains_vertices
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		maintains_vertices(desc, edge_index)
	}
	
	fn PROP_after_graph_is_equals_to_desc_minus_edge
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		after_graph_is_equals_to_desc_minus_edge(desc, edge_index)
	}

	fn PROP_rejects_non_edge
	(desc: ArbitraryGraphDescription<u32>, source: usize, sink:usize)
	-> bool{
		rejects_non_edge(desc, source, sink)
	}
	
	fn PROP_rejects_invalid_source
	(desc: ArbitraryGraphDescription<u32>,source:u32, sink: u32)
	-> bool
	{
		rejects_invalid_source(desc, source, sink)
	}
	
	fn PROP_rejects_invalid_sink
	(desc: ArbitraryGraphDescription<u32>,source:u32, sink: u32)
	-> bool
	{
		rejects_invalid_sink(desc, source, sink)
	}
}