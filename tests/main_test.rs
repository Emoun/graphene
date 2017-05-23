extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
mod arbitraries;

use graphene::implementations::adjacency_list::{AdjListGraph, BaseEdge};
use graphene::implementations::*;
use graphene::graph::FineGrainedGraph;
use arbitraries::{ArbitraryUsizeGraph};

//Helper functions
fn create_graph(vertices: Vec<usize>) -> Option<UsizeGraph>{
	let v_count  = vertices.len();
	
	//Create all edges
	let mut edges = Vec::new();
	for i in 0..v_count{
		edges.push((i, vertices[i]%v_count));
	}
	
	//Create graph
	UsizeGraph::new(vertices, edges)
}

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
fn created_graph_has_correct_vertex_count(vertices: Vec<usize>) -> bool{
	let v_count = vertices.len();
	
	match create_graph(vertices){
		Some(g) => {
			g.vertex_count() == v_count
		}
		None => false,
	}
}

fn created_graph_has_correct_edge_count(vertices: Vec<usize>) -> bool{
	let v_count = vertices.len();
	
	match create_graph(vertices){
		Some(g) => {
			g.edge_count() == v_count
		}
		None => false,
	}
}

fn created_graph_has_correct_vertices(vertices: Vec<usize>) -> bool{
	let vertex_clones = vertices.clone();
	
	match create_graph(vertices){
		Some(g) => {
			let mut used = Vec::new();
			
			//Set all's presence to false
			let g_vertices = g.all_vertices();
			for _ in 0..g_vertices.len(){
				used.push(false);
			}
			
			//For each required vertex
			'outer:
			for i in 0..vertex_clones.len(){
				//Look through all the vertices in the graph
				for (i, &&vertex_value) in g_vertices.iter().enumerate(){
					//If you find a vertex the required value
					if !used[i] {
						//Assign it as used and move to the next required vertex
						used[i] = true;
						continue 'outer;
					}
				}
				//If the required value is not found
				return false;
			}
			//All required values found
			true
		}
		None => false,
	}
}

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

	fn prop_created_graph_has_correct_vertex_count(vertices: Vec<usize>) -> bool{
		created_graph_has_correct_vertex_count(vertices)
	}
	
	fn prop_created_graph_has_correct_edge_count(vertices: Vec<usize>) -> bool{
		created_graph_has_correct_edge_count(vertices)
	}
	
	fn prop_created_graph_has_correct_vertices(vertices: Vec<usize>) -> bool{
		created_graph_has_correct_vertices(vertices)
	}
	
	fn prop_create_adjListGraph_has_correct_vertex_count(vertices: Vec<usize>) -> bool{
		create_adjListGraph_has_correct_vertex_count(vertices)
	}
	/*
	fn prop_can_add_vertex(g: ArbitraryUsizeGraph, val: usize) -> bool{
		can_add_vertex(g,val)
	}
	*/
}

/*
fn can_add_vertex(mut g: ArbitraryUsizeGraph, val: usize) -> bool{
	let init_v = g.graph.vertex_count();
	let new_graph = g.graph.add_vertex(val).unwrap();
	init_v + 1 == new_graph.vertex_count()
}



#[test]
fn main_test() -> () {
	let mut g = UsizeGraph::new(vec![1,2]);
	assert!(g.vertex_count() == 2);
	assert!(g.edge_count() == 0);
	assert!(g.outgoing_edges(&0).unwrap().len() == 0);
	assert!(g.outgoing_edges(&1).unwrap().len() == 0);
	assert!(g.incoming_edges(&0).unwrap().len() == 0);
	assert!(g.incoming_edges(&1).unwrap().len() == 0);
	
}
*/
