use super::*;

fn increases_vertex_count(desc: GraphDescription<u32,u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		(desc.values.len() + 1) == g.all_vertices().len()
	})
}

fn maintains_original_vertices(desc: GraphDescription<u32,u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		unordered_sublist_equal(&desc.values, &g.all_vertices())
	})
}

fn contains_added_value(desc: GraphDescription<u32,u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		let new_v = add_appropriate_value(&mut g,v);
		g.all_vertices().contains(&new_v)
	})
}

fn rejects_existing_value(desc: GraphDescription<u32,u32>, v: usize) -> bool{
	after_graph_init(&desc, | mut g|{
		holds_if!(g.all_vertices().len() == 0);
		
		let verts = g.all_vertices();
		let i =  v % verts.len();
		
		g.add_vertex(verts[i]).is_err()
	})
}

fn maintains_edge_count(desc: GraphDescription<u32,u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		desc.edges.len() == g.all_edges().len()
	})
}

fn maintains_original_edges(desc: GraphDescription<u32,u32>, v: u32) -> bool{
	
	original_edges_maintained_subsetof_graph_after(desc, |_, g|{
		add_appropriate_value(g,v);
	})
}


quickcheck!{
	fn PROP_increases_vertex_count(desc: GraphDescription<u32,u32>, v: u32) -> bool{
		increases_vertex_count(desc, v)
	}
	
	fn PROP_maintains_original_vertices(desc: GraphDescription<u32,u32>, v: u32) -> bool{
		maintains_original_vertices(desc, v)
	}

	fn PROP_contains_added_value(desc: GraphDescription<u32,u32>, v: u32) -> bool{
		contains_added_value(desc,v)
	}
	
	fn PROP_rejects_existing_value(desc: GraphDescription<u32,u32>, v: usize) -> bool{
		rejects_existing_value(desc, v)
	}
	
	fn PROP_maintains_edge_count(desc: GraphDescription<u32,u32>, v: u32) -> bool{
		maintains_edge_count(desc, v)
	}
	
	fn PROP_maintains_original_edges(desc: GraphDescription<u32,u32>, v: u32) -> bool{
		maintains_original_edges(desc, v)
	}
	
}