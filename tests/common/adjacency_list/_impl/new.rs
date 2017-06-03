use super::*;

fn correct_vertex_count(desc: GraphDescription<u32,u32>) -> bool{
	after_graph_init(&desc, |g|{
		g.all_vertices().len() == desc.values.len()
	})
}

fn correct_edge_count(desc: GraphDescription<u32,u32>) -> bool{
	after_graph_init(&desc, |g|{
		g.all_edges().len() == desc.edges.len()
	})
}

fn expected_vertices_subsetof_graph(desc: GraphDescription<u32,u32>) -> bool{
	after_graph_init(&desc, |g|{
		unordered_sublist_equal(&desc.values, &g.all_vertices())
	})
}

fn graph_vertices_subsetof_expected(desc: GraphDescription<u32,u32>) -> bool{
	after_graph_init(&desc, |g|{
		unordered_sublist_equal(&g.all_vertices(), &desc.values)
	})
}

fn expected_edges_subsetof_graph(desc: GraphDescription<u32,u32>) -> bool{
	original_edges_maintained_subsetof_graph_after(desc, |_,_|())
}

fn graph_edges_subsetof_expected(desc: GraphDescription<u32,u32>) -> bool{
	after_graph_init(&desc, |g|{
		graph_subsetof_edges(&g, &edges_by_value(&desc))
	})
}


//Test runners
quickcheck!{
	fn PROP_correct_vertex_count(g: GraphDescription<u32,u32>) -> bool {
		correct_vertex_count(g)
	}
	fn PROP_correct_edge_count(g: GraphDescription<u32,u32>) -> bool {
		correct_edge_count(g)
	}
	fn PROP_expected_vertices_subsetof_graph(g: GraphDescription<u32,u32>) -> bool {
		expected_vertices_subsetof_graph(g)
	}
	
	fn PROP_graph_vertices_subsetof_expected(g: GraphDescription<u32,u32>) -> bool{
		graph_vertices_subsetof_expected(g)
	}
	
	fn PROP_expected_edges_subsetof_graph(g: GraphDescription<u32,u32>) -> bool {
		expected_edges_subsetof_graph(g)
	}
	
	fn PROP_graph_edges_subsetof_expected(g: GraphDescription<u32,u32>) -> bool {
		graph_edges_subsetof_expected(g)
	}
	
}