#![allow(non_snake_case)]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use graphene::implementations::adjacency_list::*;
use graphene::graph::*;

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
fn create_adjListGraph_has_correct_vertex_count(vertices: Vec<usize>) -> bool{
	let v_count = vertices.len();
	
	match create_adjListGraph(vertices){
		Some(g) => {
			g.vertex_count() == v_count
		}
		None => false,
	}
}

//Test runners
quickcheck!{
	fn prop_create_adjListGraph_has_correct_vertex_count(vertices: Vec<usize>) -> bool{
		create_adjListGraph_has_correct_vertex_count(vertices)
	}
}