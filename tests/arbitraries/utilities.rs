
use graphene::common::*;
use graphene::core::*;
use quickcheck::*;
use arbitraries::*;
use std;
use std::iter::FromIterator;


#[macro_export]
macro_rules! holds_if{
	{
		$e:expr
	}=> {
		if $e {
			return true;
		}
	}
}

///
/// Returns whether the first list in an unordered sublist of the second list.
///
/// One list os an unordered sublist of another if all its elements can be found in the
/// other list, without duplications. Examples ( `<` as operator):
///
/// - `[1,2,3] < [3,2,1]`
/// - `[1,2,3] < [2,3,3,1]`
/// - `[1,2,2,3] !< [1,2,3]`
///
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

///
/// Identical to `unordered_sublist()` except for values which are `Eq`.
///
pub fn unordered_sublist_equal<T>(sublist:&Vec<T>, superlist:&Vec<T>) -> bool
	where
		T: Eq
{
	unordered_sublist(sublist, superlist, |v_sub, v_super|{
		v_sub == v_super
	})
}

///
/// Initialises a graph ( using its `graph()` function) based on the given description and passes it to
/// the given function, returning the value the function returns, except
/// if the graph initialization fails, in which case false is always returned
/// (without running the function)
///
pub fn graph_init<G,F>(desc: &GraphDescription<
										<G as BaseGraph>::Vertex,
										<G as BaseGraph>::Weight>,
					   holds: F)
					   -> bool
	where
		G: BaseGraph,
		<G as BaseGraph>::Vertex: ArbVertex,
		<G as BaseGraph>::Weight: ArbWeight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		F: Fn(G) -> bool
{
	if let Ok(g) = G::graph(
		desc.values.clone(), desc.edges_by_value())
	{
		return holds(g);
	}
	false
}

///
/// Initialises a graph based on the given description (see `graph_init()`),
/// an appropriate edge is added to it (see `add_appropriate_edge()`),
/// after which the graph and the added edge are passed to the given function
/// who's return value is then returned by this function, except if the graph
/// initialization or the edge addition fails, in which case either false is returned or it panics.
///
pub fn graph_init_and_add_edge<G,F>(
	desc: &GraphDescription<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>,
	source_i_cand: usize,
	sink_i_cand:usize,
	weight: <G as BaseGraph>::Weight,
	holds: F)
	-> bool
	where
		G: BaseGraph,
		<G as BaseGraph>::Vertex: ArbVertex,
		<G as BaseGraph>::Weight: ArbWeight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		F: Fn(G, BaseEdge<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>) -> bool,
{
	graph_init(desc, |mut g| {
		let edge = add_appropriate_edge(desc, &mut g, source_i_cand, sink_i_cand, weight);
		holds(g, edge)
	})
}

///
/// Initialises a graph based on the given description (see `graph_init()`),
/// an appropriate edge removed from it (see `remove_appropriate_edge()`),
/// after which the graph and the removed edge and its index in the description are passed to t
/// he given function who's return value is then returned by this function, except if the graph
/// initialization or the edge removal fails, in which case either false is returned or it panics.
///
pub fn graph_init_and_remove_edge<G,F>(
	desc: &GraphDescription<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>,
	edge_index: usize, holds: F)
	-> bool
	where
		G: BaseGraph,
		<G as BaseGraph>::Vertex: ArbVertex,
		<G as BaseGraph>::Weight: ArbWeight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		F: Fn(G, (usize, BaseEdge<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>)) -> bool,
{
	graph_init(desc, |mut g| {
		let edge = remove_appropriate_edge(desc, &mut g, edge_index);
		holds(g, edge)
	})
}

///
/// Finds an integer value that is not already present in the
/// given graph starting from the given value.
///
/// A suitable value is found by incrementing the given value
/// untill one is found that is no in the graph.
///
pub fn find_addable_value<G>(g:&G, v:u32)-> u32
where
	G: BaseGraph<Vertex=u32>,
	<G as BaseGraph>::Weight: ArbWeight,
	<G as BaseGraph>::VertexIter: FromIterator<u32>,
	<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<u32,<G as BaseGraph>::Weight>>,
	<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
	<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let mut new_v = v;
	while let Some(_) = g.all_vertices().into_iter().position( |t_v| t_v == new_v){
		new_v = if new_v == std::u32::MAX {0}else { new_v + 1 };
	}
	new_v
}

///
/// Adds a new vertex to the graph with a suitable value based on the given value.
///
/// See ``find_addable_value` as to how the value is found.
///
pub fn add_appropriate_value<W>(g: &mut AdjListGraph<u32,W>, v: u32) -> u32
where
	W: ArbWeight,
{
	let new_v = find_addable_value(g, v);
	
	g.add_vertex(new_v).unwrap();
	new_v
}

///
/// Returns whether the given edge descriptions are an unordered sublist of
/// the edges in the given graph.
///
/// For the definition of an unordered sublist see `unordered_sublist()`.
///
pub fn edges_sublistof_graph<V,W,G>(edges: &Vec<(V, V, W)>, g: &G) -> bool
where
	V: ArbVertex,
	W: ArbWeight,
	G: BaseGraph<Vertex=V,Weight=W>,
	<G as BaseGraph>::VertexIter: FromIterator<V>,
	<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
	<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
	<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let g_edges = g.all_edges().into_iter().collect();
	unordered_sublist(edges, &g_edges, |&expected, ref g_edge|{
		expected.0 == g_edge.source &&
			expected.1 == g_edge.sink &&
			expected.2 == g_edge.weight
	})
}

///
/// Returns whether the edges in the given graph are an unordered sublist of
/// the edges in the given description.
///
/// For the definition of an unordered sublist see `unordered_sublist()`.
///
pub fn graph_sublistof_edges<V,W,G>(g: &G, edges: &Vec<(V, V, W)>) -> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let g_edges = g.all_edges().into_iter().collect();
	unordered_sublist(&g_edges, edges, |ref g_edge, &expected|{
		expected.0 == g_edge.source &&
			expected.1 == g_edge.sink &&
			expected.2 == g_edge.weight
	})
}

///
/// Removes a vertex from the graph, returning the index of the removed vertex in the description
/// and the value of the vertex. Assumes the graph and the description are equal and uses the given
/// index to choose the vertex to remove.
///
/// #Panics:
/// May panic if the graph and the description are not identical and the vertex chosen for removal
/// is not present in the grpah.
///
pub fn remove_appropriate_vertex <V,W,G>(
	desc:&GraphDescription<V,W>,
	g: &mut G,
	index:usize)
	-> (usize,V)
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let removed_i = appropriate_vertex_index(index, desc);
	let removed_v = desc.values[removed_i];
	
	g.remove_vertex(removed_v).unwrap();
	(removed_i, removed_v)
}

///
/// Returns all the edge in the description, by value, which are not incident on
/// the given vertex.
///
pub fn edges_not_incident_on_vertex<V,W>(
	desc:&GraphDescription<V,W>,
	v: V)
	-> Vec<(V, V, W)>
	where
		V: ArbVertex,
		W: ArbWeight,
{
	let edges_by_value = desc.edges_by_value();
	
	//Collect all edges not incident of the given vertex
	edges_by_value.into_iter().filter(|&(source,sink,_)| source != v && sink != v).collect()
}

///
/// Returns a valid index into the vertex values of the description
/// base on the given index.
///
pub fn appropriate_vertex_index<V,W>(i: usize, desc:&GraphDescription<V,W>) -> usize
	where
		V: ArbVertex,
		W: ArbWeight,
{
	i % desc.values.len()
}

///
/// Returns a vertex value present in the desc based on the given index.
///
/// The given index does not have to be valid in the description, it will be converted to
/// a valid one. See `appropriate_vertex_index()`.
///
pub fn appropriate_vertex_value_from_index<V,W>(desc:&GraphDescription<V,W>, i_cand: usize) -> V
	where
		V: ArbVertex,
		W: ArbWeight,
{
	let i = appropriate_vertex_index(i_cand, &desc);
	desc.values[i]
}

///
/// Adds an appropriate edge (I.e. incident on valid vertices) to the graph returning the
/// edge added.
///
/// Assumes the graph is equals to the given description.
///
/// #Panics:
/// If the graph and description are not equal causing the addition of an invalid edge.
///
pub fn add_appropriate_edge<V,W,G>(	desc:&GraphDescription<V,W>, g: &mut G,
									source_i_cand: usize, sink_i_cand: usize, weight: W)
	-> BaseEdge<V,W>
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let source_v = appropriate_vertex_value_from_index(desc, source_i_cand);
	let sink_v = appropriate_vertex_value_from_index(desc,sink_i_cand);
	let added_edge = BaseEdge::new(source_v, sink_v, weight);
	g.add_edge(added_edge).unwrap();
	added_edge
}

///
/// Removes an edge from the graph, returning the edge removed and its index in the description.
/// The given index is used to find an edge in the given description
/// which is then removed from the graph.
///
/// Assumes the graph is equals to the description.
///
/// #Panics:
/// If the graph is not equal to the description.
///
pub fn remove_appropriate_edge<V,W,G>(	desc:&GraphDescription<V,W>,
										g: &mut G,
										edge_index_cand: usize)
	-> (usize, BaseEdge<V,W>)
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let edge_index = edge_index_cand % desc.edges.len();
	let desc_edge = desc.edges[edge_index];
	let v_source_i = desc_edge.0;
	let v_sink_i = desc_edge.1;
	
	let edge = BaseEdge::new(desc.values[v_source_i], desc.values[v_sink_i], desc_edge.2);
	
	g.remove_edge(edge).unwrap();
	(edge_index, edge)
}

///
/// Initializes a graph based on the given description and returns whether the edges in the description
/// are still in the graph after the given action is executed.
///
pub fn original_edges_maintained_sublistof_graph_after<G,F>(
	desc: GraphDescription<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>,
	action: F)
	-> bool
	where
		G: BaseGraph,
		<G as BaseGraph>::Vertex: ArbVertex,
		<G as BaseGraph>::Weight: ArbWeight,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		F: Fn(&GraphDescription<<G as BaseGraph>::Vertex,<G as BaseGraph>::Weight>, &mut G) -> ()
{
	graph_init(&desc, |mut g|{
		action(&desc, &mut g);
		edges_sublistof_graph(&desc.edges_by_value(), &g)
	})
}

///
/// Returns a vertex value that is not in the description.
///
/// This is done by incrementing the given value until one which
/// is not used in the description is found.
///
pub fn invalidate_vertice<W>(mut v: u32, desc: &GraphDescription<u32,W>) -> u32
	where
		W: ArbWeight,
{
	while desc.values.contains(&v){
		v =
			if v == std::u32::MAX {0}
				else { v + 1 };
	}
	v
}

///
/// Returns whether the given description and graph have equal vertices.
///
pub fn equal_description_and_graph_vertices<V,W,G>(
	desc: &GraphDescription<V,W>, g: &G )
	-> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	let graph_v = &g.all_vertices().into_iter().collect();
	let desc_v = &desc.values;
	
	unordered_sublist_equal(desc_v, graph_v) &&
		unordered_sublist_equal(graph_v, desc_v)
}

///
/// Returns whether the given description and graph have equal edges.
///
pub fn equal_description_and_graph_edges<V,W,G>(
	desc: &GraphDescription<V,W>, g: &G )
	-> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		G: BaseGraph<Vertex=V,Weight=W>,
		<G as BaseGraph>::VertexIter: FromIterator<V>,
		<G as BaseGraph>::EdgeIter: FromIterator<BaseEdge<V,W>>,
		<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	edges_sublistof_graph(&desc.edges_by_value(), g) &&
		graph_sublistof_edges(g, &desc.edges_by_value())
}

