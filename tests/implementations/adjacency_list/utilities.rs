
use graphene::implementations::adjacency_list::*;
use graphene::graph::*;
use quickcheck::*;
use arbitraries::*;
use std;

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
pub fn edges_by_value<V>(desc: &ArbitraryGraphDescription<V>)
	-> Vec<(V, V)>
where
	V: Arbitrary
{
	let mut edges = Vec::new();
	
	for e in &desc.edges{
		let t_source = desc.vertex_values[e.0].clone();
		let t_sink = desc.vertex_values[e.1].clone();
		edges.push((t_source, t_sink));
	}
	edges
}

pub fn unordered_sublist<B,P,F>(sublist:&Vec<B>, superlist:&Vec<P>, equal: F) -> bool
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

pub fn after_graph_init<F>(desc: &ArbitraryGraphDescription<u32>, holds: F) -> bool
where 	F: Fn(AdjListGraph<u32>) -> bool,
{
	if let Some(g) = AdjListGraph::new(
		desc.vertex_values.clone(), desc.edges.clone())
	{
		return holds(g);
	}
	false
}

pub fn find_addable_value(g:&AdjListGraph<u32>, v:u32)-> u32{
	let mut new_v = v;
	while g.all_vertices().contains(&new_v){
		new_v = if new_v == std::u32::MAX {0}else { new_v + 1 };
	}
	new_v
}

pub fn add_appropriate_value(g: &mut AdjListGraph<u32>, v: u32) -> u32{
	let new_v = find_addable_value(&g, v);
	
	g.add_vertex(new_v).unwrap();
	new_v
}

pub fn edges_subsetof_graph(edges: &Vec<(u32,u32)>, g: &AdjListGraph<u32>) -> bool{
	unordered_sublist(edges, &g.all_edges(), |&expected, ref g_edge|{
		expected.0 == g_edge.source() &&
			expected.1 == g_edge.sink()
	})
}

pub fn remove_appropriate_vertex(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,  index:usize)
-> (usize,u32){
	let removed_i = appropriate_index(index,desc);
	let removed_v = desc.vertex_values[removed_i];
	
	g.remove_vertex(removed_v).unwrap();
	(removed_i, removed_v)
}

pub fn edges_independent_of_vertex(desc:&ArbitraryGraphDescription<u32>, v: u32) -> Vec<(u32,u32)>{
	
	let value_edges = edges_by_value(desc);
	let mut result = Vec::new();
	for &e in value_edges.iter().filter(|&&(source,sink)| source != v && sink != v){
		result.push(e);
	}
	result
}

pub fn appropriate_index(i: usize, desc:&ArbitraryGraphDescription<u32>)-> usize{
	i % desc.vertex_values.len()
}

pub fn add_appropriate_edge(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,
						   source_i_cand: usize, sink_i_cand: usize)
-> BaseEdge<u32,()>
{
	let source_i = appropriate_index(source_i_cand, desc);
	let sink_i = appropriate_index(sink_i_cand, desc);
	
	let source_v = desc.vertex_values[source_i];
	let sink_v = desc.vertex_values[sink_i];
	let added_edge = BaseEdge::new(source_v, sink_v,());
	g.add_edge(added_edge).unwrap();
	added_edge
}

pub fn remove_appropriate_edge(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,
							edge_index_cand: usize)
-> (usize, BaseEdge<u32,()>)
{
	let edge_index = edge_index_cand % desc.edges.len();
	let v_source_i = desc.edges[edge_index].0;
	let v_sink_i = desc.edges[edge_index].1;
	
	let edge = BaseEdge::new(desc.vertex_values[v_source_i], desc.vertex_values[v_sink_i],());
	
	g.remove_edge(edge).unwrap();
	(edge_index, edge)
}

pub fn original_edges_maintained_subsetof_graph_after<F>(desc: ArbitraryGraphDescription<u32>, action: F)
 -> bool
where
	F: Fn(&ArbitraryGraphDescription<u32>, &mut AdjListGraph<u32>) -> ()
{
	after_graph_init(&desc, | mut g|{
		action(&desc, &mut g);
		edges_subsetof_graph(&edges_by_value(&desc), &g)
	})
	
}

pub fn graph_subsetof_edges(g: &AdjListGraph<u32>,edges: &Vec<(u32,u32)>) -> bool{
	unordered_sublist(&g.all_edges(), edges, |ref g_edge, &expected|{
		expected.0 == g_edge.source() &&
			expected.1 == g_edge.sink()
	})
}

pub fn unordered_sublist_equal<T>(sublist:&Vec<T>, superlist:&Vec<T>) -> bool
where
	T: Eq
{
	unordered_sublist(sublist, superlist, |v_sub, v_super|{
		v_sub == v_super
	})
}

pub fn after_init_and_add_edge<F>(desc: &ArbitraryGraphDescription<u32>,
						   source_i_cand: usize, sink_i_cand:usize, holds: F)
-> bool
where
	F: Fn(AdjListGraph<u32>, BaseEdge<u32,()>) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = add_appropriate_edge(desc, &mut g, source_i_cand, sink_i_cand);
		holds(g, edge)
	})
}

pub fn after_init_and_remove_edge<F>(desc: &ArbitraryGraphDescription<u32>,
							  edge_index: usize, holds: F)
							  -> bool
	where
		F: Fn(AdjListGraph<u32>, (usize, BaseEdge<u32,()>)) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = remove_appropriate_edge(desc, &mut g, edge_index);
		holds(g, edge)
	})
}


pub fn invalidate_vertice(mut v: u32, desc: &ArbitraryGraphDescription<u32>) -> u32{
	
	while desc.vertex_values.contains(&v){
		v =
			if v == std::u32::MAX {0}
				else { v + 1 };
	}
	v
}

pub fn equal_description_and_graph_vertices(desc: &ArbitraryGraphDescription<u32>, g: &AdjListGraph<u32> )
										-> bool
{
	unordered_sublist_equal(&desc.vertex_values, &g.all_vertices()) &&
		unordered_sublist_equal(&g.all_vertices(), &desc.vertex_values)
}

pub fn equal_description_and_graph_edges(desc: &ArbitraryGraphDescription<u32>, g: &AdjListGraph<u32> )
										-> bool
{
	edges_subsetof_graph(&edges_by_value(&desc), &g) &&
		graph_subsetof_edges(&g, &edges_by_value(&desc))
}

//Property functions

macro_rules! holds_if{
	{
		$e:expr
	}=> {
		if $e {
			return true;
		}
	}
}

