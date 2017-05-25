
#![allow(non_snake_case)]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use graphene::implementations::adjacency_list::*;
use graphene::graph::*;
use quickcheck::{Arbitrary,Gen};
use std::collections::{HashMap};
use std::hash::Hash;

#[derive(Clone,Debug)]
struct ArbitraryGraphDescription<V> where V: Arbitrary{
	pub vertex_values: Vec<V>,
	pub edges: Vec<(usize,usize)>,
}

impl Arbitrary for ArbitraryGraphDescription<u32>{
	fn arbitrary<G: Gen>(g: &mut G) -> Self {
		let MAX_VALUES = 10;
		let mut vertex_values = Vec::new();
		let mut edges = Vec::new();
		//Decide the amount of vertices
		let vertex_count = g.gen_range(0,MAX_VALUES);
		
		if vertex_count > 0 {
			//Decide the amount of edges
			let edge_count = g.gen_range(0, MAX_VALUES);
			
			//Create vertex values
			let mut next_value = g.gen_range(0, MAX_VALUES);
			for _ in 0..vertex_count {
				//Make sure the values are unique
				while vertex_values.contains(&next_value) {
					next_value = g.gen_range(0, MAX_VALUES);
				}
				vertex_values.push(next_value);
			}
			
			//Create edges
			
			let mut t_source;
			let mut t_sink;
			for _ in 0..edge_count {
				t_source = g.next_u32() % vertex_count;
				t_sink = g.next_u32() % vertex_count;
				
				edges.push((t_source as usize, t_sink as usize))
			}
		}
		ArbitraryGraphDescription{vertex_values, edges}
	}
	
	fn shrink(&self) -> Box<Iterator<Item=Self>> {
		
		//Base case
		if self.vertex_values.len() == 0 {
			return Box::new(Vec::new().into_iter());
		}
		
		let mut result = Vec::new();
		
		//Shrink by reducing a vertex value
		let mut new_values;
		for (i,&val) in self.vertex_values.iter().enumerate(){
			if val > 0 {
				new_values = self.vertex_values.clone();
				new_values[i] = val - 1;
				result.push(ArbitraryGraphDescription { vertex_values: new_values, edges: self.edges.clone() });
			}
		}
		
		//Shrink by removing an edge
		let mut new_edges;
		for (i, _ ) in self.edges.iter().enumerate(){
			new_edges = self.edges.clone();
			new_edges.remove(i);
			result.push(ArbitraryGraphDescription {
				vertex_values: self.vertex_values.clone(),
				edges: new_edges });
		}
		
		let mut t_edge;
		//Shrink by removing a vertex (v)
		for (i,_) in self.vertex_values.iter().enumerate(){
			new_values = self.vertex_values.clone();
			new_edges = Vec::new();
			
			//For all edges
			for &e in self.edges.iter(){
				//Remove any pointing to or from v
				if e.0 != i && e.1 != i {
					t_edge = e;
					
					//Any pointing to or from the last edge
					//now point to v
					if e.0 == self.vertex_values.len() {
						t_edge.0 = i;
					}
					if e.1 == self.vertex_values.len() {
						t_edge.1 = i;
					}
					new_edges.push(t_edge);
				}
			}
			
			//Replace v with the last vertex
			new_values.swap_remove(i);
			
			result.push(ArbitraryGraphDescription{vertex_values: new_values, edges: new_edges});
		}
		
		Box::new(result.into_iter())
	}
}

//Helper functions
/*
quickcheck! {
	fn test_arbitrary_graph(Ag: ArbitraryGraphDescription<u32>) -> bool{
		println!("Original: {:?}", Ag);
		
		for a in Ag.shrink(){
			println!("Shrunk: {:?}", a);
		}
		true
	}
}
*/

///
/// Returns all the edges in the given description
/// by the value of the vertices they point to and from
///
fn edges_by_value<V>(desc: ArbitraryGraphDescription<V>)
	-> (ArbitraryGraphDescription<V>, Vec<(V, V)>)
where
	V: Arbitrary
{
	let mut edges = Vec::new();
	
	for e in &desc.edges{
		let t_source = desc.vertex_values[e.0].clone();
		let t_sink = desc.vertex_values[e.1].clone();
		edges.push((t_source, t_sink));
	}
	(desc, edges)
}

fn unordered_sublist<B,P,F>(sublist:&Vec<B>, superlist:&Vec<P>, equal: F) -> bool
where F: Fn(&B, &P) -> bool,
{
	//Track whether each element in the superlist has been used
	// to match an element of the sublist
	let mut used = Vec::new();
	used.resize(superlist.len(),false);
	
	//For each sublist element
	'outer:
	for sub_e in sublist{
		//Look through all superelements
		for (i, super_e) in superlist.iter().enumerate(){
			//If the element is unused
			if !used[i] {
				//if they are equal
				if equal(&sub_e,super_e) {
					//Assign the super element as used and move to the nex subelement
					used[i] = true;
					continue 'outer;
				}
			}
		}
		//The subelement was not found
		return false;
	}
	//All subelement were found
	true
}

fn after_graph_init<V,F>(desc: ArbitraryGraphDescription<V>, holds: F) -> bool
where 	F: Fn(ArbitraryGraphDescription<V>, AdjListGraph<V>) -> bool,
		V: Arbitrary + Clone + Eq
{
	if let Some(g) = AdjListGraph::new(
		desc.vertex_values.clone(), desc.edges.clone())
	{
		return holds(desc, g);
	}
	false
}

fn expected_edges_for_vertices<V>(desc: ArbitraryGraphDescription<V>, outgoing: bool)
								  -> (ArbitraryGraphDescription<V>, HashMap<V,Vec<V>>)
where
	V:Arbitrary + Clone + Eq + Hash
{
	let (desc, edges_by_value) = edges_by_value(desc);
	
	//Construct expected outgoing/incoming edges for each vertex.
	//Create the map
	let mut edges_for = HashMap::new();
	//Initialize each vertex for empty outgoing
	for v in desc.vertex_values.iter().cloned(){
		edges_for.insert(v, Vec::new());
	}
	//For each edge
	for e in edges_by_value{
		if outgoing {
			//If the outgoing edges are wanted,
			//Put the sink of the edge in the source's
			//map
			edges_for.get_mut(&e.0).unwrap().push(e.1);
		}else{
			//If the incoming edges are wanted,
			//Put the source of the edge in the sink's
			//map
			edges_for.get_mut(&e.1).unwrap().push(e.0);
		}
	}
	(desc, edges_for)
}

//Property functions

fn init_correct_vertex_count(desc:ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d, g|{
		g.vertex_count() == d.vertex_values.len()
	})
}

fn init_correct_edge_count(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d, g|{
		g.edge_count() == d.edges.len()
	})
}

fn init_expected_vertices_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d,g|{
		unordered_sublist(&d.vertex_values, &g.all_vertices(), |e_v, g_v|{
			e_v == g_v
		})
	})
}

fn init_graph_vertices_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d,g|{
		unordered_sublist(&g.all_vertices(), &d.vertex_values, |g_v, e_v|{
			e_v == g_v
		})
	})
}

fn init_expected_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d,g|{
		unordered_sublist(&edges_by_value(d).1, &g.all_edges(), |&expected, ref g_edge|{
			expected.0 == g_edge.source() &&
				expected.1 == g_edge.sink()
		})
	})
}

fn init_graph_edges_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(desc, |d,g|{
		unordered_sublist(&g.all_edges(), &edges_by_value(d).1, |ref g_edge, &expected|{
			expected.0 == g_edge.source() &&
				expected.1 == g_edge.sink()
		})
	})
}

fn init_expected_outgoing_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	
	let (desc, expected_outgoing) = expected_edges_for_vertices(desc, true);
	
	after_graph_init(desc, |_, g|{
		//For each vertex in the graph
		for v in g.all_vertices() {
			if let (Ok(v_out), Some(v_expected_out)) = (g.outgoing_edges(v) , expected_outgoing.get(&v)){
				
				if !unordered_sublist(v_expected_out, &v_out, |&e_v, g_edge| {
					e_v == g_edge.sink()
				} ){
					return false;
				}
				
			}else {
				unreachable!();
			}
		}
		//For all vertices, the expected edges were found
		return true;
	})
}

fn init_graph_outgoing_edges_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	
	let (desc, expected_outgoing) = expected_edges_for_vertices(desc,true);
	
	after_graph_init(desc, |_, g|{
		//For each vertex in the graph
		for v in g.all_vertices() {
			if let (Ok(v_out), Some(v_expected_out)) = (g.outgoing_edges(v) , expected_outgoing.get(&v)){
				
				if !unordered_sublist(&v_out, v_expected_out, |g_edge, &e_v| {
					e_v == g_edge.sink()
				} ){
					return false;
				}
				
			}else {
				unreachable!();
			}
		}
		//For all vertices, the expected edges were found
		return true;
	})
}

fn init_expected_incoming_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	
	let (desc, expected_incoming) = expected_edges_for_vertices(desc, false);
	
	after_graph_init(desc, |_, g|{
		//For each vertex in the graph
		for v in g.all_vertices() {
			if let (Ok(v_in), Some(v_expected_in)) = (g.incoming_edges(v) , expected_incoming.get(&v)){
				
				if !unordered_sublist(v_expected_in, &v_in, |&e_v, g_edge| {
					e_v == g_edge.source()
				} ){
					return false;
				}
				
			}else {
				unreachable!();
			}
		}
		//For all vertices, the expected edges were found
		return true;
	})
}

fn init_graph_incoming_edges_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	
	let (desc, expected_incoming) = expected_edges_for_vertices(desc, false);
	
	after_graph_init(desc, |_, g|{
		//For each vertex in the graph
		for v in g.all_vertices() {
			if let (Ok(v_in), Some(v_expected_in)) = (g.incoming_edges(v) , expected_incoming.get(&v)){
				
				if !unordered_sublist(&v_in, v_expected_in, |g_edge, &e_v| {
					e_v == g_edge.source()
				} ){
					return false;
				}
				
			}else {
				unreachable!();
			}
		}
		//For all vertices, the expected edges were found
		return true;
	})
}

//Test runners
quickcheck!{
	fn AdjListGraph_PROP_init_correct_vertex_count(g: ArbitraryGraphDescription<u32>) -> bool {
		init_correct_vertex_count(g)
	}
	fn AdjListGraph_PROP_init_correct_edge_count(g: ArbitraryGraphDescription<u32>) -> bool {
		init_correct_edge_count(g)
	}
	fn AdjListGraph_PROP_init_expected_vertices_subsetof_graph(g: ArbitraryGraphDescription<u32>) -> bool {
		init_expected_vertices_subsetof_graph(g)
	}
	
	fn AdjListGraph_PROP_init_graph_vertices_subsetof_expected(g: ArbitraryGraphDescription<u32>) -> bool{
		init_graph_vertices_subsetof_expected(g)
	}
	
	fn AdjListGraph_PROP_init_expected_edges_subsetof_graph(g: ArbitraryGraphDescription<u32>) -> bool {
		init_expected_edges_subsetof_graph(g)
	}
	
	fn AdjListGraph_PROP_init_graph_edges_subsetof_expected(g: ArbitraryGraphDescription<u32>) -> bool {
		init_graph_edges_subsetof_expected(g)
	}
	
	fn AdjListGraph_PROP_init_expected_outgoing_edges_subsetof_graph(g: ArbitraryGraphDescription<u32>) -> bool {
		init_expected_outgoing_edges_subsetof_graph(g)
	}
	
	fn AdjListGraph_PROP_init_graph_outgoing_edges_subsetof_expected(g: ArbitraryGraphDescription<u32>) -> bool {
		init_graph_outgoing_edges_subsetof_expected(g)
	}

	fn AdjListGraph_PROP_init_expected_incoming_edges_subsetof_graph(g: ArbitraryGraphDescription<u32>) -> bool{
		init_expected_incoming_edges_subsetof_graph(g)
	}
	
	fn AdjListGraph_PROP_init_graph_incoming_edges_subsetof_expected(g: ArbitraryGraphDescription<u32>) -> bool{
		init_graph_incoming_edges_subsetof_expected(g)
	}
}































