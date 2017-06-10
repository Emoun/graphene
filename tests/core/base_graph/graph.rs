use super::*;

fn edges_equal_to_description(desc: GraphDescription<u32,u32>) -> bool {
	GraphMock_init(&desc, |g|{
		equal_description_and_graph_edges(&desc, &g)
	})
}

fn vertices_equal_to_description(desc: GraphDescription<u32,u32>) -> bool {
	GraphMock_init(&desc, |g|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

quickcheck!{
	fn PROP_edges_equal_to_description(desc: GraphDescription<u32,u32>) -> bool {
		edges_equal_to_description(desc)
	}
	
	fn PROP_vertices_equal_to_description(desc: GraphDescription<u32,u32>) -> bool {
		vertices_equal_to_description(desc)
	}
}