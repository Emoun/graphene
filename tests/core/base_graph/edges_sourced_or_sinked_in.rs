use super::*;

///
/// Tests that the `edges_sourced_in` optional method for `BaseGraph`.
///
/// Ensured that all the returned edges are sourced in the given vertice.
///
fn all_edges_sourced_in_the_vertice(
	desc: GraphDescription<u32,u32>,
	v_cand: u32)
	-> bool
{
	GraphMock_init(&desc, |g|{
		
		if desc.values.len() == 0 {
			// If the description says the graph has no vertices,
			// then there can be no edges sourced in the given vertice,
			// as it is not part of the graph.
			g.edges_sourced_in(v_cand).len() == 0
		} else {
			let v = appropriate_vertex_value_from_index(&desc, v_cand as usize);
			
			let sourced_edges = g.edges_sourced_in(v);
			let sourced_edges_len = sourced_edges.len();
			
			let valid_edges = sourced_edges.into_iter().filter(|e|{
				e.source == v
			}).collect::<Vec<_>>();
			
			sourced_edges_len == valid_edges.len()
		}
	})
}

///
/// Tests that the `edges_sinked_in` optional method for `BaseGraph`.
///
/// Ensured that all the returned edges are sinked in the given vertice.
///
fn all_edges_sinked_in_the_vertice(
	desc: GraphDescription<u32,u32>,
	v_cand: u32)
	-> bool
{
	GraphMock_init(&desc, |g|{
		
		if desc.values.len() == 0 {
			// If the description says the graph has no vertices,
			// then there can be no edges sourced in the given vertice,
			// as it is not part of the graph.
			g.edges_sinked_in(v_cand).len() == 0
		} else {
			let v = appropriate_vertex_value_from_index(&desc, v_cand as usize);
			
			let sinked_edges = g.edges_sinked_in(v);
			let sinked_edges_len = sinked_edges.len();
			
			let valid_edges = sinked_edges.into_iter().filter(|e|{
				e.sink == v
			}).collect::<Vec<_>>();
			
			sinked_edges_len == valid_edges.len()
		}
	})
}

quickcheck!{
	fn PROP_all_edges_sourced_in_the_vertice(
		desc: GraphDescription<u32,u32>,
		v_cand: u32)
		-> bool
	{
		all_edges_sourced_in_the_vertice(desc, v_cand)
	}
	
	fn PROP_all_edges_sinked_in_the_vertice(
		desc: GraphDescription<u32,u32>,
		v_cand: u32)
		-> bool
	{
		all_edges_sinked_in_the_vertice(desc, v_cand)
	}
}