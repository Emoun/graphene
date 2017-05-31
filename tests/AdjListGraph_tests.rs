
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
			if val > 0  && !self.vertex_values.contains(&(val-1)){
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
					if e.0 == self.vertex_values.len()-1 {
						t_edge.0 = i;
					}
					if e.1 == self.vertex_values.len()-1 {
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
fn edges_by_value<V>(desc: &ArbitraryGraphDescription<V>)
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

fn after_graph_init<F>(desc: &ArbitraryGraphDescription<u32>, holds: F) -> bool
where 	F: Fn(AdjListGraph<u32>) -> bool,
{
	if let Some(g) = AdjListGraph::new(
		desc.vertex_values.clone(), desc.edges.clone())
	{
		return holds(g);
	}
	false
}

fn find_addable_value(g:&AdjListGraph<u32>, v:u32)-> u32{
	let mut new_v = v;
	while g.all_vertices().contains(&new_v){
		new_v = if new_v == std::u32::MAX {0}else { new_v + 1 };
	}
	new_v
}

fn add_appropriate_value(g: &mut AdjListGraph<u32>, v: u32) -> u32{
	let new_v = find_addable_value(&g, v);
	
	g.add_vertex(new_v).unwrap();
	new_v
}

fn edges_subsetof_graph(edges: &Vec<(u32,u32)>, g: &AdjListGraph<u32>) -> bool{
	unordered_sublist(edges, &g.all_edges(), |&expected, ref g_edge|{
		expected.0 == g_edge.source() &&
			expected.1 == g_edge.sink()
	})
}

fn remove_appropriate_vertex(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,  index:usize)
-> (usize,u32){
	let removed_i = appropriate_index(index,desc);
	let removed_v = desc.vertex_values[removed_i];
	
	g.remove_vertex(removed_v).unwrap();
	(removed_i, removed_v)
}

fn edges_independent_of_vertex(desc:&ArbitraryGraphDescription<u32>, v: u32) -> Vec<(u32,u32)>{
	
	let value_edges = edges_by_value(desc);
	let mut result = Vec::new();
	for &e in value_edges.iter().filter(|&&(source,sink)| source != v && sink != v){
		result.push(e);
	}
	result
}

fn appropriate_index(i: usize, desc:&ArbitraryGraphDescription<u32>)-> usize{
	i % desc.vertex_values.len()
}

fn add_appropriate_edge(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,
						   source_i_cand: usize, sink_i_cand: usize)
-> BaseEdge<u32>
{
	let source_i = appropriate_index(source_i_cand, desc);
	let sink_i = appropriate_index(sink_i_cand, desc);
	
	let source_v = desc.vertex_values[source_i];
	let sink_v = desc.vertex_values[sink_i];
	let added_edge = BaseEdge::new(source_v, sink_v);
	g.add_edge(added_edge).unwrap();
	added_edge
}

fn remove_appropriate_edge(desc:&ArbitraryGraphDescription<u32>, g: &mut AdjListGraph<u32>,
							edge_index_cand: usize)
-> (usize, BaseEdge<u32>)
{
	let edge_index = edge_index_cand % desc.edges.len();
	let v_source_i = desc.edges[edge_index].0;
	let v_sink_i = desc.edges[edge_index].1;
	
	let edge = BaseEdge::new(desc.vertex_values[v_source_i], desc.vertex_values[v_sink_i]);
	
	g.remove_edge(edge).unwrap();
	(edge_index, edge)
}

fn original_edges_maintained_subsetof_graph_after<F>(desc: ArbitraryGraphDescription<u32>, action: F)
 -> bool
where
	F: Fn(&ArbitraryGraphDescription<u32>, &mut AdjListGraph<u32>) -> ()
{
	after_graph_init(&desc, | mut g|{
		action(&desc, &mut g);
		edges_subsetof_graph(&edges_by_value(&desc), &g)
	})
	
}

fn graph_subsetof_edges(g: &AdjListGraph<u32>,edges: &Vec<(u32,u32)>) -> bool{
	unordered_sublist(&g.all_edges(), edges, |ref g_edge, &expected|{
		expected.0 == g_edge.source() &&
			expected.1 == g_edge.sink()
	})
}

fn unordered_sublist_equal<T>(sublist:&Vec<T>, superlist:&Vec<T>) -> bool
where
	T: Eq
{
	unordered_sublist(sublist, superlist, |v_sub, v_super|{
		v_sub == v_super
	})
}

fn after_init_and_add_edge<F>(desc: &ArbitraryGraphDescription<u32>,
						   source_i_cand: usize, sink_i_cand:usize, holds: F)
-> bool
where
	F: Fn(AdjListGraph<u32>, BaseEdge<u32>) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = add_appropriate_edge(desc, &mut g, source_i_cand, sink_i_cand);
		holds(g, edge)
	})
}

fn after_init_and_remove_edge<F>(desc: &ArbitraryGraphDescription<u32>,
							  edge_index: usize, holds: F)
							  -> bool
	where
		F: Fn(AdjListGraph<u32>, (usize, BaseEdge<u32>)) -> bool,
{
	after_graph_init(desc, |mut g| {
		let edge = remove_appropriate_edge(desc, &mut g, edge_index);
		holds(g, edge)
	})
}


fn invalidate_vertice(mut v: u32, desc: &ArbitraryGraphDescription<u32>) -> u32{
	
	while desc.vertex_values.contains(&v){
		v =
			if v == std::u32::MAX {0}
				else { v + 1 };
	}
	v
}

fn equal_description_and_graph_vertices(desc: &ArbitraryGraphDescription<u32>, g: &AdjListGraph<u32> )
										-> bool
{
	unordered_sublist_equal(&desc.vertex_values, &g.all_vertices()) &&
		unordered_sublist_equal(&g.all_vertices(), &desc.vertex_values)
}

fn equal_description_and_graph_edges(desc: &ArbitraryGraphDescription<u32>, g: &AdjListGraph<u32> )
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


fn init_correct_vertex_count(desc:ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(&desc, |g|{
		g.all_vertices().len() == desc.vertex_values.len()
	})
}

fn init_correct_edge_count(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(&desc, |g|{
		g.all_edges().len() == desc.edges.len()
	})
}

fn init_expected_vertices_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(&desc, |g|{
		unordered_sublist_equal(&desc.vertex_values, &g.all_vertices())
	})
}

fn init_graph_vertices_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(&desc, |g|{
		unordered_sublist_equal(&g.all_vertices(), &desc.vertex_values)
	})
}

fn init_expected_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>) -> bool{
	original_edges_maintained_subsetof_graph_after(desc, |_,_|())
}

fn init_graph_edges_subsetof_expected(desc: ArbitraryGraphDescription<u32>) -> bool{
	after_graph_init(&desc, |g|{
		graph_subsetof_edges(&g, &edges_by_value(&desc))
	})
}

fn add_vertex_increases_vertex_count(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		(desc.vertex_values.len() + 1) == g.all_vertices().len()
	})
}

fn add_vertex_maintains_original_vertices(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		unordered_sublist_equal(&desc.vertex_values, &g.all_vertices())
	})
}

fn add_vertex_contains_added_value(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		let new_v = add_appropriate_value(&mut g,v);
		g.all_vertices().contains(&new_v)
	})
}

fn add_vertex_rejects_existing_value(desc: ArbitraryGraphDescription<u32>, v: usize) -> bool{
	after_graph_init(&desc, | mut g|{
		holds_if!(g.all_vertices().len() == 0);
		
		let verts = g.all_vertices();
		let i =  v % verts.len();
		
		g.add_vertex(verts[i]).is_err()
	})
}

fn add_vertex_maintains_edge_count(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
	after_graph_init(&desc, | mut g|{
		add_appropriate_value(&mut g,v);
		desc.edges.len() == g.all_edges().len()
	})
}

fn add_vertex_maintains_original_edges(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
	
	original_edges_maintained_subsetof_graph_after(desc, |_, g|{
		add_appropriate_value(g,v);
	})
}

fn remove_vertex_decreases_vertex_count(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
	holds_if!{desc.vertex_values.len() == 0};
	
	after_graph_init(&desc, | mut g|{
		remove_appropriate_vertex(&desc,&mut g,i);
		(desc.vertex_values.len() - 1) == g.all_vertices().len()
	})
}

fn remove_vertex_maintains_unremoved_vertices(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
	holds_if!{desc.vertex_values.len() == 0};
	
	after_graph_init(&desc, | mut g|{
		let (rem_i, _) = remove_appropriate_vertex(&desc,&mut g,i);
		let mut vertex_clones = desc.vertex_values.clone();
		vertex_clones.remove(rem_i);
		unordered_sublist_equal(&vertex_clones, &g.all_vertices())
	})
}

fn remove_vertex_removes_vertex_from_graph(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
	holds_if!{desc.vertex_values.len() == 0};
	
	after_graph_init(&desc, | mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		
		!g.all_vertices().contains(&removed_v)
	})
}

fn remove_vertex_after_independent_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
	holds_if!{desc.vertex_values.len() == 0};
	
	after_graph_init(&desc, | mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		let indy_edges = edges_independent_of_vertex(&desc, removed_v);
		
		unordered_sublist(&indy_edges, &g.all_edges(), |&(e_source, e_sink), g_edge|{
			e_source == g_edge.source() &&
				e_sink == g_edge.sink()
		})
	})
}

fn remove_vertex_after_graph_subsetof_independent_edges(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
	holds_if!{desc.vertex_values.len() == 0};
	
	after_graph_init(&desc, | mut g|{
		let (_, removed_v) = remove_appropriate_vertex(&desc,&mut g,i);
		
		let indy_edges = edges_independent_of_vertex(&desc, removed_v);
		
		unordered_sublist(&g.all_edges(), &indy_edges, |g_edge, &(e_source, e_sink)|{
			e_source == g_edge.source() &&
				e_sink == g_edge.sink()
		})
	})
}

fn remove_vertex_rejects_absent_vertex(desc: ArbitraryGraphDescription<u32>, v:u32) -> bool{
	
	after_graph_init(&desc, | mut g|{
		let mut value = v;
		while g.all_vertices().contains(&value){
			value += 1;
		}
		
		g.remove_vertex(value).is_err()
	})
}

fn add_edge_increases_edge_count(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
	-> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	
	after_init_and_add_edge(&desc, source_i_cand, sink_i_cand, |g,_|{
		g.all_edges().len() == (desc.edges.len() + 1)
	})
}

fn add_edge_maintain_original_edges(desc: ArbitraryGraphDescription<u32>,
									source_i_cand: usize, sink_i_cand:usize)
									-> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	
	original_edges_maintained_subsetof_graph_after(desc, |d, g|{
		add_appropriate_edge(d, g, source_i_cand,sink_i_cand);
	})
}

fn add_edge_graph_subsetof_original_edges_and_added_edge(desc: ArbitraryGraphDescription<u32>,
							   source_i_cand: usize, sink_i_cand:usize)
							   -> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_graph_init(&desc, |mut g|{
		let edge = add_appropriate_edge(&desc,&mut g, source_i_cand, sink_i_cand);
		let mut original_edges_v = edges_by_value(&desc);
		original_edges_v.push((edge.source(), edge.sink()));
		graph_subsetof_edges(&g, &original_edges_v)
	})
}

fn add_edge_maintains_vertices(desc: ArbitraryGraphDescription<u32>,
							   source_i_cand: usize, sink_i_cand:usize)
							   -> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_init_and_add_edge(&desc, source_i_cand, sink_i_cand, |g, _|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

fn add_edge_reject_invalid_source(desc: ArbitraryGraphDescription<u32>,
								  source: u32, sink: u32) -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_source = invalidate_vertice(source, &desc);
		
		g.add_edge(BaseEdge::new(invalid_source, sink)).is_err()
	})
}

fn add_edge_reject_invalid_sink(desc: ArbitraryGraphDescription<u32>,
								  source: u32, sink: u32) -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_sink = invalidate_vertice(sink, &desc);
		
		g.add_edge(BaseEdge::new(source ,invalid_sink)).is_err()
	})
}

fn remove_edge_decreases_edge_count(desc: ArbitraryGraphDescription<u32>,
								edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	after_init_and_remove_edge(&desc, edge_index, |g, _|{
		(desc.edges.len() -1) == g.all_edges().len()
	})
}

fn remove_edge_maintains_vertices(desc: ArbitraryGraphDescription<u32>,
								  edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	after_init_and_remove_edge(&desc, edge_index, |g, _|{
		equal_description_and_graph_vertices(&desc, &g)
	})
}

fn remove_edge_after_graph_is_equals_to_desc_minus_edge(desc: ArbitraryGraphDescription<u32>,
														edge_index: usize) -> bool
{
	holds_if!(desc.edges.len() == 0);
	
	after_init_and_remove_edge(&desc, edge_index, |g, (i,_)|{
		let mut desc_clone = desc.clone();
		desc_clone.edges.remove(i);
		equal_description_and_graph_edges(&desc_clone, &g)
	})
}

fn remove_edge_rejects_non_edge(	desc: ArbitraryGraphDescription<u32>,
									source_i_cand:usize, sink_i_cand: usize)
	-> bool
{
	holds_if!(desc.vertex_values.len() == 0);
	after_graph_init(&desc, |mut g|{
		let v_nr = desc.vertex_values.len();
		let mut source_i = source_i_cand % v_nr;
		let mut sink_i = sink_i_cand % v_nr;
		
		let mut i = 0;
		while desc.edges.contains(&(source_i, sink_i)) && i<v_nr {
			source_i += 1;
			source_i %= v_nr;
			let mut j = 0;
			while desc.edges.contains(&(source_i,sink_i)) && j < v_nr{
				sink_i += 1;
				sink_i %= v_nr;
				j += 1;
			}
			i += 1;
		}
		if desc.edges.contains(&(source_i, sink_i)) {
			//The graph contains all edge possibilities.
			//Since we cannot find an edge that is not present,
			//the property must hold
			return true;
		}
		let edge = BaseEdge::new(desc.vertex_values[source_i], desc.vertex_values[sink_i]);
		g.remove_edge(edge).is_err()
	})
}

fn remove_edge_rejects_invalid_source(	desc: ArbitraryGraphDescription<u32>,
									source:u32, sink: u32)
									-> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_source = invalidate_vertice(source, &desc);
		
		g.remove_edge(BaseEdge::new(invalid_source, sink)).is_err()
	})
}

fn remove_edge_rejects_invalid_sink(	desc: ArbitraryGraphDescription<u32>,
										  source:u32, sink: u32)
										  -> bool
{
	after_graph_init(&desc, | mut g|{
		let invalid_sink = invalidate_vertice(sink, &desc);
		
		g.remove_edge(BaseEdge::new(source, invalid_sink)).is_err()
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
	
	fn AdjListGraph_PROP_add_vertex_increases_vertex_count(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
		add_vertex_increases_vertex_count(desc, v)
	}
	
	fn AdjListGraph_PROP_add_vertex_maintains_original_vertices(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
		add_vertex_maintains_original_vertices(desc, v)
	}

	fn AdjListGraph_PROP_add_vertex_contains_added_value(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
		add_vertex_contains_added_value(desc,v)
	}
	
	fn AdjListGraph_PROP_add_vertex_rejects_existing_value(desc: ArbitraryGraphDescription<u32>, v: usize) -> bool{
		add_vertex_rejects_existing_value(desc, v)
	}
	
	fn AdjListGraph_PROP_add_vertex_maintains_edge_count(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
		add_vertex_maintains_edge_count(desc, v)
	}
	
	fn AdjListGraph_PROP_add_vertex_maintains_original_edges(desc: ArbitraryGraphDescription<u32>, v: u32) -> bool{
		add_vertex_maintains_original_edges(desc, v)
	}
	
	fn AdjListGraph_PROP_remove_vertex_decreases_vertex_count(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
		remove_vertex_decreases_vertex_count(desc,i)
	}
	
	fn AdjListGraph_PROP_remove_vertex_maintains_unremoved_vertices(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
		remove_vertex_maintains_unremoved_vertices(desc, i)
	}
	
	fn AdjListGraph_PROP_remove_vertex_removes_vertex_from_graph(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
		remove_vertex_removes_vertex_from_graph(desc, i)
	}
	
	fn AdjListGraph_PROP_remove_vertex_after_independent_edges_subsetof_graph(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
		remove_vertex_after_independent_edges_subsetof_graph(desc, i)
	}
	
	fn AdjListGraph_PROP_remove_vertex_after_graph_subsetof_independent_edges(desc: ArbitraryGraphDescription<u32>, i: usize) -> bool{
		remove_vertex_after_graph_subsetof_independent_edges(desc, i)
	}
	
	fn AdjListGraph_PROP_remove_vertex_rejects_absent_vertex(desc: ArbitraryGraphDescription<u32>, v:u32) -> bool{
		remove_vertex_rejects_absent_vertex(desc, v)
	}
	
	fn AdjListGraph_PROP_add_edge_increases_edge_count(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		add_edge_increases_edge_count(desc, source_i_cand, sink_i_cand)
	}
	
	fn AdjListGraph_PROP_add_edge_maintain_original_edges(desc: ArbitraryGraphDescription<u32>,
								 source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		add_edge_maintain_original_edges(desc, source_i_cand, sink_i_cand)
	}
	
	fn AdjListGraph_PROP_add_edge_graph_subsetof_original_edges_and_added_edge
	(desc: ArbitraryGraphDescription<u32>,source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		add_edge_graph_subsetof_original_edges_and_added_edge(desc, source_i_cand, sink_i_cand)
	}
	
	fn AdjListGraph_PROP_add_edge_maintains_vertices
	(desc: ArbitraryGraphDescription<u32>,source_i_cand: usize, sink_i_cand:usize)
	-> bool{
		add_edge_maintains_vertices(desc, source_i_cand, sink_i_cand)
	}
	
	fn AdjListGraph_PROP_add_edge_reject_invalid_source
	(desc: ArbitraryGraphDescription<u32>,source: u32, sink:u32)
	-> bool{
		add_edge_reject_invalid_source(desc, source, sink)
	}
	
	fn AdjListGraph_PROP_add_edge_reject_invalid_sink
	(desc: ArbitraryGraphDescription<u32>,source: u32, sink:u32)
	-> bool{
		add_edge_reject_invalid_sink(desc, source, sink)
	}
	
	fn AdjListGraph_PROP_remove_edge_decreases_edge_count
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		remove_edge_decreases_edge_count(desc, edge_index)
	}
	
	fn AdjListGraph_PROP_remove_edge_maintains_vertices
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		remove_edge_maintains_vertices(desc, edge_index)
	}
	
	fn AdjListGraph_PROP_remove_edge_after_graph_is_equals_to_desc_minus_edge
	(desc: ArbitraryGraphDescription<u32>, edge_index: usize)
	-> bool{
		remove_edge_after_graph_is_equals_to_desc_minus_edge(desc, edge_index)
	}

	fn AdjListGraph_PROP_remove_edge_rejects_non_edge
	(desc: ArbitraryGraphDescription<u32>, source: usize, sink:usize)
	-> bool{
		remove_edge_rejects_non_edge(desc, source, sink)
	}
	
	fn AdjListGraph_PROP_remove_edge_rejects_invalid_source
	(desc: ArbitraryGraphDescription<u32>,source:u32, sink: u32)
	-> bool
	{
		remove_edge_rejects_invalid_source(desc, source, sink)
	}
	
	fn AdjListGraph_PROP_remove_edge_rejects_invalid_sink
	(desc: ArbitraryGraphDescription<u32>,source:u32, sink: u32)
	-> bool
	{
		remove_edge_rejects_invalid_sink(desc, source, sink)
	}
}































