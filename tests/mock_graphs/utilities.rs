
use graphene::{
	core::{
		Graph,
		trait_aliases::{
			Id,IntoFromIter
		}
	},
};

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
/// Returns a valid index into the vertex values of the graph
/// based on the given index.
///
pub fn appropriate_vertex_index<'a,G>(graph:&G, idx_cand: usize) -> usize
	where
		G: Graph<'a>,
{
	idx_cand % graph.all_vertices().into_iter().count()
}

///
/// Returns a vertex value present in the desc based on the given index.
///
/// The given index does not have to be valid in the description, it will be converted to
/// a valid one. See `appropriate_vertex_index()`.
///
pub fn appropriate_vertex_value_from_index<'a,G,V>(graph:&G, idx_cand: usize) -> V
	where
		G: Graph<'a, Vertex=V>,
		V: Id,
		G::VertexIter : IntoFromIter<V>,
		G::EdgeIter : IntoFromIter<(V, V, &'a G::EdgeWeight)>,
		G::EdgeMutIter : IntoFromIter<(V, V, &'a mut G::EdgeWeight)>,
{
	let i = appropriate_vertex_index(graph, idx_cand);
	graph.all_vertices().into_iter().nth(i).unwrap()
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
		T: PartialEq
{
	unordered_sublist(sublist, superlist, |v_sub, v_super|{
		v_sub == v_super
	})
}
