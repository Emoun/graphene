use super::*;

fn equals_to_nr_of_edges(desc: GraphDescription<u32,u32>) -> bool {
	GraphMock_init(&desc, |g|{
		g.edge_count() == desc.edges.len()
	})
}


quickcheck!{
	fn PROP_equals_to_nr_of_edges(desc: GraphDescription<u32,u32>) -> bool {
		equals_to_nr_of_edges(desc)
	}
}