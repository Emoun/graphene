use super::*;

fn all_edges_incident_on_the_vertices(
	desc: GraphDescription<u32,u32>,
	v1_cand: u32, v2_cand: u32)
	-> bool
{
	GraphMock_init(&desc, |g|{
		if desc.values.len() == 0 {
			return g.edges_between(v1_cand,v2_cand).len() == 0;
		}
		
		let v1 = appropriate_vertex_value_from_index(&desc, v1_cand as usize);
		let v2 = appropriate_vertex_value_from_index(&desc, v2_cand as usize);
		
		let edges_between = g.edges_between(v1,v2);
		let edges_between_len = edges_between.len();
		
		let valid_edges = edges_between.into_iter().filter(|e| {
			(e.source == v1 && e.sink == v2) ||
				(e.source == v2 && e.sink == v1)
		}).collect::<Vec<_>>();
		
		edges_between_len == valid_edges.len()
	})
}

fn all_incident_edges_in_desc_found_in_graph(
	desc: GraphDescription<u32,u32>,
	v1_cand: u32, v2_cand: u32)
	-> bool
{
	holds_if!(desc.values.len() == 0);
	
	GraphMock_init(&desc, |g|{
		let v1 = appropriate_vertex_value_from_index(&desc, v1_cand as usize);
		let v2 = appropriate_vertex_value_from_index(&desc, v2_cand as usize);
		
		let inc_edges = desc.edges_by_value().into_iter().filter(|&(so,si,_)|{
			(so == v1 && si == v2) ||
				(so == v2 && si == v2)
		}).collect();
		
		edges_sublistof_graph(&inc_edges, &g)
	})
}



quickcheck!{
	fn PROP_all_edges_incident_on_the_vertices(
		desc: GraphDescription<u32,u32>,
		v1_cand: u32, v2_cand: u32)
		-> bool
	{
		all_edges_incident_on_the_vertices(desc,v1_cand, v2_cand)
	}
	
	fn PROP_all_incident_edges_in_desc_found_in_graph(
		desc: GraphDescription<u32,u32>,
		v1_cand: u32, v2_cand: u32)
		-> bool
	{
		all_incident_edges_in_desc_found_in_graph(desc,v1_cand, v2_cand)
	}
}














