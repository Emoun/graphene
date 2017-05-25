#![allow(non_snake_case)]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use graphene::implementations::adjacency_list::*;
use graphene::graph::*;
use quickcheck::{Arbitrary,Gen};
use std::collections::{HashMap};

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

//Property functions
fn created_adjListGraph_has_correct_vertex_count(g: ArbitraryGraphDescription<u32>) -> bool{
	let v_count = g.vertex_values.len();
	
	match AdjListGraph::new(g.vertex_values, g.edges){
		Some(g) => {
			g.vertex_count() == v_count
		}
		None => false,
	}
}

fn created_adjListGraph_has_correct_edge_count(g: ArbitraryGraphDescription<u32>) -> bool{
	let e_count = g.edges.len();
	
	match AdjListGraph::new(g.vertex_values, g.edges){
		Some(g) => {
			g.edge_count() == e_count
		}
		None => false,
	}
}

fn created_adjListGraph_has_correct_vertex_values(desc: ArbitraryGraphDescription<u32>) -> bool{
	let vertex_clones = desc.vertex_values.clone();
	
	if let Some(g) = AdjListGraph::new(desc.vertex_values, desc.edges){
		
		//Track whether each vertex of the graph has been used
		// to match an original vertex
		let mut used = Vec::new();
		used.resize(g.vertex_count(), false);
		
		//For each required vertex
		'outer:
		for &r_v in vertex_clones.iter(){
			//Look through all the vertices in the graph
			for (i, &v) in g.all_vertices().iter().enumerate(){
				//If you find a vertex with the required value
				if !used[i] && r_v == v{
					//Assign it as used and move to the next required vertex
					used[i] = true;
					continue 'outer;
				}
			}
			//If the required value is not found
			return false;
		}
		return true;
	}
	false
}

fn created_adjListGraph_has_correct_edges(desc: ArbitraryGraphDescription<u32>) -> bool{
	//Construct all expected edges
	let (desc, expected_edges) = edges_by_value(desc);
	
	
	if let Some(g) = AdjListGraph::new(desc.vertex_values, desc.edges){
		//Track whether each edge in the graph has been used
		// to match an expected edge
		let mut used = Vec::new();
		used.resize(g.edge_count(), false);
		
		//For each expected edge
		'outer:
		for (e_source, e_sink) in expected_edges {
			//Look through all the edges in the graph
			for (i, edge) in g.all_edges().iter().enumerate(){
				//If you find an edge with the required value
				//that has not been used before
				if !used[i] &&
					e_source == edge.source() &&
					e_sink == edge.sink()
				{
					//Assign it as used and move to the next required vertex
					used[i] = true;
					continue 'outer;
				}
			}
			//If the expected edge is not found
			return false;
		}
		//All expected edges were found
		return true;
	}
	false
}

fn created_adjListGraph_has_correct_outgoing_edges(desc: ArbitraryGraphDescription<u32>) -> bool{
	
	let (desc, edges_by_value) = edges_by_value(desc);
	
	//Construct expected outgoing edges
	let mut expected_outgoing = HashMap::new();
	for v in desc.vertex_values.iter().cloned(){
		expected_outgoing.insert(v, Vec::new());
	}
	for e in edges_by_value{
		expected_outgoing.get_mut(&e.0).unwrap().push(e.1);
	}
	
	if let Some(g) = AdjListGraph::new(desc.vertex_values, desc.edges){
		//For each vertex in the graph
		for v in g.all_vertices() {
			if let Ok(v_out) = g.outgoing_edges(v) {
				//Track whether the outgoing edges have been used
				//to match to an expected edge
				let mut used = Vec::new();
				used.resize(v_out.len(), false);
				let expected_v_out = expected_outgoing.get(&v).unwrap();
				//For each expected outgoing edge
				'outer:
				for &e_edge in expected_v_out{
					//Look through all the outgoing edges in the graph
					for (g_i, ref g_edge) in v_out.iter().enumerate(){
						//If you find an unused matching edge
						//mark it as used
						if !used[g_i] &&
							e_edge == g_edge.sink()
						{
							used[g_i] = true;
							continue 'outer;
						}
					}
					//No matching edge was found in the graph
					return false;
				}
				//All expected edges matched an edge in the graph
			}else {
				unreachable!();
			}
		}
		//For all vertices, the expected edges were found
		return true;
	}
	false
}

//Test runners
quickcheck!{

	fn prop_created_adjListGraph_has_correct_vertex_count(g: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_vertex_count(g)
	}
	
	fn prop_created_adjListGraph_has_correct_edge_count(g: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_edge_count(g)
	}
	
	fn prop_created_adjListGraph_has_correct_vertex_values(g: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_vertex_values(g)
	}
	
	fn prop_created_adjListGraph_has_correct_edges(desc: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_edges(desc)
	}
	
	fn prop_created_adjListGraph_has_correct_outgoing_edges(desc: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_outgoing_edges(desc)
	}
}