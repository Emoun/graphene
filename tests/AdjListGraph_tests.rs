#![allow(non_snake_case)]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use graphene::implementations::adjacency_list::*;
use graphene::graph::*;
use quickcheck::{Arbitrary,Gen};

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
//Helper functions
fn create_adjListGraph(vertices: Vec<usize>) -> Option<AdjListGraph<usize>>{
	let v_count  = vertices.len();
	
	//Create all edges
	let mut edges = Vec::new();
	for i in 0..v_count{
		edges.push((i, vertices[i]%v_count));
	}
	
	//Create graph
	AdjListGraph::new(vertices, edges)
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


//Test runners
quickcheck!{

	fn prop_created_adjListGraph_has_correct_vertex_count(g: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_vertex_count(g)
	}
	
	fn prop_created_adjListGraph_has_correct_edge_count(g: ArbitraryGraphDescription<u32>) -> bool{
		created_adjListGraph_has_correct_edge_count(g)
	}
}