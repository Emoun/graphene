use super::*;

fn decreases_vertex_count(desc: GraphDescription<u32,u32>, i: usize) -> bool{
	holds_if!{desc.values.len() == 0};
	
	AdjListGraph_init(&desc, |mut g|{
		remove_appropriate_vertex(&desc,&mut g,i);
		(desc.values.len() - 1) == g.all_vertices().len()
	})
}

fn maintains_unremoved_vertices(desc: GraphDescription<u32,u32>, i: usize) -> bool{
	holds_if!{desc.values.len() == 0};
	
	AdjListGraph_init(&desc, |mut g|{
		let (rem_i, _) = remove_appropriate_vertex(&desc,&mut g,i);
		let mut vertex_clones = desc.values.clone();
		vertex_clones.remove(rem_i);
		unordered_sublist_equal(&vertex_clones, &g.all_vertices())
	})
}

fn removes_vertex_from_graph(desc: GraphDescription<u32,u32>, i: usize) -> bool{
	holds_if!{desc.values.len() == 0};
	
	AdjListGraph_init(&desc, |mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		
		!g.all_vertices().contains(&removed_v)
	})
}

fn after_independent_edges_subsetof_graph(desc: GraphDescription<u32,u32>, i: usize) -> bool{
	holds_if!{desc.values.len() == 0};
	
	AdjListGraph_init(&desc, |mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		let indy_edges = edges_not_incident_on_vertex(&desc, removed_v);
		
		unordered_sublist(&indy_edges, &g.all_edges(), |&(e_source, e_sink, _), g_edge|{
			e_source == g_edge.source() &&
				e_sink == g_edge.sink()
		})
	})
}

fn after_graph_subsetof_independent_edges(desc: GraphDescription<u32,u32>, i: usize) -> bool{
	holds_if!{desc.values.len() == 0};
	
	AdjListGraph_init(&desc, |mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		
		let indy_edges = edges_not_incident_on_vertex(&desc, removed_v);
		
		unordered_sublist(&g.all_edges(), &indy_edges, |g_edge, &(e_source, e_sink, _)|{
			e_source == g_edge.source() &&
				e_sink == g_edge.sink()
		})
	})
}

fn rejects_absent_vertex(desc: GraphDescription<u32,u32>, v:u32) -> bool{
	
	AdjListGraph_init(&desc, |mut g|{
		let mut value = v;
		while g.all_vertices().contains(&value){
			value += 1;
		}
		
		g.remove_vertex(value).is_err()
	})
}


quickcheck!{
	fn PROP_decreases_vertex_count(desc: GraphDescription<u32,u32>, i: usize) -> bool{
		decreases_vertex_count(desc,i)
	}
	
	fn PROP_maintains_unremoved_vertices(desc: GraphDescription<u32,u32>, i: usize) -> bool{
		maintains_unremoved_vertices(desc, i)
	}
	
	fn PROP_removes_vertex_from_graph(desc: GraphDescription<u32,u32>, i: usize) -> bool{
		removes_vertex_from_graph(desc, i)
	}
	
	fn PROP_after_independent_edges_subsetof_graph(desc: GraphDescription<u32,u32>, i: usize) -> bool{
		after_independent_edges_subsetof_graph(desc, i)
	}
	
	fn PROP_after_graph_subsetof_independent_edges(desc: GraphDescription<u32,u32>, i: usize) -> bool{
		after_graph_subsetof_independent_edges(desc, i)
	}
	
	fn PROP_rejects_absent_vertex(desc: GraphDescription<u32,u32>, v:u32) -> bool{
		rejects_absent_vertex(desc, v)
	}
}