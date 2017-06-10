use super::*;

fn equals_to_nr_of_vertices(desc: GraphDescription<u32,u32>) -> bool {
	GraphMock_init(&desc, |g|{
		g.vertex_count() == desc.values.len()
	})
}


quickcheck!{
	fn PROP_equals_to_nr_of_vertices(desc: GraphDescription<u32,u32>) -> bool {
		equals_to_nr_of_vertices(desc)
	}
}