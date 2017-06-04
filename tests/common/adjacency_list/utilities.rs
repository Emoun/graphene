
use graphene::common::*;
use graphene::core::*;
use quickcheck::*;
use arbitraries::*;
use std;

//Helper functions

///
/// Returns all the edges in the given description
/// by the value of the vertices they point to and from
///
pub fn edges_by_value<V,W>(desc: &GraphDescription<V,W>)
	-> Vec<(V, V,W)>
where
	V: Arbitrary + Copy + Eq,
	W: Arbitrary + Copy + Eq,
{
	let mut edges = Vec::new();
	
	for e in &desc.edges{
		let t_source = desc.values[e.0];
		let t_sink = desc.values[e.1];
		edges.push((t_source, t_sink, e.2));
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

pub fn after_graph_init<V,W,F>(desc: &GraphDescription<V,W>, holds: F) -> bool
where
	V: Arbitrary + Copy + Eq,
	W: Arbitrary + Copy + Eq,
	F: Fn(AdjListGraph<V,W>) -> bool,
{
	if let Some(g) = AdjListGraph::new(
		desc.values.clone(), desc.edges.clone())
	{
		return holds(g);
	}
	false
}

pub fn find_addable_value<W>(g:&AdjListGraph<u32,W>, v:u32)-> u32
where
	W: Arbitrary + Copy + Eq,
{
	let mut new_v = v;
	while g.all_vertices().contains(&new_v){
		new_v = if new_v == std::u32::MAX {0}else { new_v + 1 };
	}
	new_v
}

pub fn add_appropriate_value<W>(g: &mut AdjListGraph<u32,W>, v: u32) -> u32
where
	W: Arbitrary + Copy + Eq,
{
	let new_v = find_addable_value(&g, v);
	
	g.add_vertex(new_v).unwrap();
	new_v
}

pub fn edges_subsetof_graph<V,W>(edges: &Vec<(V,V,W)>, g: &AdjListGraph<V,W>) -> bool
where
	V: Arbitrary + Copy + Eq,
	W: Arbitrary + Copy + Eq,
{
	unordered_sublist(edges, &g.all_edges(), |&expected, ref g_edge|{
		expected.0 == g_edge.source &&
			expected.1 == g_edge.sink &&
			expected.2 == g_edge.weight
	})
}

pub fn remove_appropriate_vertex <V,W>(
	desc:&GraphDescription<V,W>,
	g: &mut AdjListGraph<V,W>,
	index:usize)
	-> (usize,V)
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	let removed_i = appropriate_index(index,desc);
	let removed_v = desc.values[removed_i];
	
	g.remove_vertex(removed_v).unwrap();
	(removed_i, removed_v)
}

pub fn edges_independent_of_vertex<V,W>(
	desc:&GraphDescription<V,W>,
	v: V)
	-> Vec<(V, V, W)>
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	
	let value_edges = edges_by_value(desc);
	let mut result = Vec::new();
	for &e in value_edges.iter().filter(|&&(source,sink,_)| source != v && sink != v){
		result.push(e);
	}
	result
}

pub fn appropriate_index<V,W>(i: usize, desc:&GraphDescription<V,W>) -> usize
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	i % desc.values.len()
}

pub fn add_appropriate_edge<V,W>(	desc:&GraphDescription<V,W>, g: &mut AdjListGraph<V,W>,
									source_i_cand: usize, sink_i_cand: usize, weight: W)
	-> BaseEdge<V,W>
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	let source_i = appropriate_index(source_i_cand, desc);
	let sink_i = appropriate_index(sink_i_cand, desc);
	
	let source_v = desc.values[source_i];
	let sink_v = desc.values[sink_i];
	let added_edge = BaseEdge::new(source_v, sink_v, weight);
	g.add_edge(added_edge).unwrap();
	added_edge
}

pub fn remove_appropriate_edge<V,W>(	desc:&GraphDescription<V,W>,
										g: &mut AdjListGraph<V,W>,
										edge_index_cand: usize)
	-> (usize, BaseEdge<V,W>)
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	let edge_index = edge_index_cand % desc.edges.len();
	let desc_edge = desc.edges[edge_index];
	let v_source_i = desc_edge.0;
	let v_sink_i = desc_edge.1;
	
	let edge = BaseEdge::new(desc.values[v_source_i], desc.values[v_sink_i], desc_edge.2);
	
	g.remove_edge(edge).unwrap();
	(edge_index, edge)
}

pub fn original_edges_maintained_subsetof_graph_after<V,W,F>(
	desc: GraphDescription<V,W>,
	action: F)
	-> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
		F: Fn(&GraphDescription<V,W>, &mut AdjListGraph<V,W>) -> ()
{
	after_graph_init(&desc, | mut g|{
		action(&desc, &mut g);
		edges_subsetof_graph(&edges_by_value(&desc), &g)
	})
}

pub fn graph_subsetof_edges<V,W>(g: &AdjListGraph<V,W>,edges: &Vec<(V,V,W)>) -> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	unordered_sublist(&g.all_edges(), edges, |ref g_edge, &expected|{
		expected.0 == g_edge.source &&
			expected.1 == g_edge.sink &&
			expected.2 == g_edge.weight
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

pub fn after_init_and_add_edge<V,W,F>(	desc: &GraphDescription<V,W>, source_i_cand: usize,
										sink_i_cand:usize, weight: W, holds: F)
	-> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
		F: Fn(AdjListGraph<V,W>, BaseEdge<V,W>) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = add_appropriate_edge(desc, &mut g, source_i_cand, sink_i_cand, weight);
		holds(g, edge)
	})
}

pub fn after_init_and_remove_edge<V,W,F>(desc: &GraphDescription<V,W>, edge_index: usize, holds: F)
	-> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
		F: Fn(AdjListGraph<V,W>, (usize, BaseEdge<V,W>)) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = remove_appropriate_edge(desc, &mut g, edge_index);
		holds(g, edge)
	})
}


pub fn invalidate_vertice<W>(mut v: u32, desc: &GraphDescription<u32,W>) -> u32
	where
		W: Arbitrary + Copy + Eq,
{
	
	while desc.values.contains(&v){
		v =
			if v == std::u32::MAX {0}
				else { v + 1 };
	}
	v
}

pub fn equal_description_and_graph_vertices<V,W>(
	desc: &GraphDescription<V,W>, g: &AdjListGraph<V,W> )
	-> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
{
	unordered_sublist_equal(&desc.values, &g.all_vertices()) &&
		unordered_sublist_equal(&g.all_vertices(), &desc.values)
}

pub fn equal_description_and_graph_edges<V,W>(
	desc: &GraphDescription<V,W>, g: &AdjListGraph<V,W> )
	-> bool
	where
		V: Arbitrary + Copy + Eq,
		W: Arbitrary + Copy + Eq,
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

