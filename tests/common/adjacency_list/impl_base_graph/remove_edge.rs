use super::*;

fn decreases_edge_count(desc: GraphDescription<u32,u32>,
						edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	AdjListGraph_init_and_remove_edge(&desc, edge_index, |g, _|{
		(desc.edges.len() -1) == g.all_edges().len()
	})
}

fn maintains_vertices(desc: GraphDescription<u32,u32>,
					  edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	AdjListGraph_init_and_remove_edge(&desc, edge_index, |g, _|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

fn after_graph_is_equals_to_desc_minus_edge(desc: GraphDescription<u32,u32>,
											edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	AdjListGraph_init_and_remove_edge(&desc, edge_index, |g, (i,_)|{
		let mut desc_clone = desc.clone();
		desc_clone.edges.remove(i);
		equal_description_and_graph_edges(&desc_clone, &g)
	})
}

fn rejects_non_edge(desc: GraphDescription<u32,u32>,
					source_i_cand:usize, sink_i_cand: usize, weight:u32)
					-> bool
{
	holds_if!(desc.values.len() == 0);
	AdjListGraph_init(&desc, |mut g|{
		let v_nr = desc.values.len();
		let mut source_i = source_i_cand % v_nr;
		let mut sink_i = sink_i_cand % v_nr;
		
		let mut i = 0;
		while desc.edges.contains(&(source_i, sink_i,weight)) && i<v_nr {
			source_i += 1;
			source_i %= v_nr;
			let mut j = 0;
			while desc.edges.contains(&(source_i,sink_i, weight )) && j < v_nr{
				sink_i += 1;
				sink_i %= v_nr;
				j += 1;
			}
			i += 1;
		}
		if desc.edges.contains(&(source_i, sink_i,weight)) {
			//The core contains all edge possibilities.
			//Since we cannot find an edge that is not present,
			//the property must hold
			return true;
		}
		let edge = BaseEdge::new(desc.values[source_i], desc.values[sink_i], weight);
		g.remove_edge(edge).is_err()
	})
}

fn rejects_invalid_source(desc: GraphDescription<u32,u32>,
						  source:u32, sink: u32, weight: u32)
						  -> bool
{
	AdjListGraph_init(&desc, |mut g|{
		let invalid_source = invalidate_vertice(source, &desc);
		
		g.remove_edge(BaseEdge::new(invalid_source, sink,weight)).is_err()
	})
}

fn rejects_invalid_sink(desc: GraphDescription<u32,u32>,
						source:u32, sink: u32, weight: u32)
						-> bool
{
	AdjListGraph_init(&desc, |mut g|{
		let invalid_sink = invalidate_vertice(sink, &desc);
		
		g.remove_edge(BaseEdge::new(source, invalid_sink,weight)).is_err()
	})
}

quickcheck!{
	fn PROP_decreases_edge_count
	(desc: GraphDescription<u32,u32>, edge_index: usize)
	-> bool{
		decreases_edge_count(desc, edge_index)
	}
	
	fn PROP_maintains_vertices
	(desc: GraphDescription<u32,u32>, edge_index: usize)
	-> bool{
		maintains_vertices(desc, edge_index)
	}
	
	fn PROP_after_graph_is_equals_to_desc_minus_edge
	(desc: GraphDescription<u32,u32>, edge_index: usize)
	-> bool{
		after_graph_is_equals_to_desc_minus_edge(desc, edge_index)
	}

	fn PROP_rejects_non_edge
	(desc: GraphDescription<u32,u32>, source: usize, sink:usize, weight: u32)
	-> bool{
		rejects_non_edge(desc, source, sink, weight)
	}
	
	fn PROP_rejects_invalid_source
	(desc: GraphDescription<u32,u32>,source:u32, sink: u32, weight: u32)
	-> bool
	{
		rejects_invalid_source(desc, source, sink, weight)
	}
	
	fn PROP_rejects_invalid_sink
	(desc: GraphDescription<u32,u32>,source:u32, sink: u32, weight: u32)
	-> bool
	{
		rejects_invalid_sink(desc, source, sink, weight)
	}
}