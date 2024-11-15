use crate::mock_graph::{MockEdgeWeight, MockVertex, MockVertexWeight};
use graphene::core::{
	property::{AddEdge, NewVertex},
	Directedness, Graph,
};
use std::{borrow::Borrow, collections::HashMap};
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

/// Creates a closure that calculates the equality between two 3-tuples where
/// their elements implement partial equality.
pub fn _3_tuple_equality<A1, B1, C1, A2, B2, C2>() -> impl Fn(&(A1, B1, C1), &(A2, B2, C2)) -> bool
where
	A1: PartialEq<A2>,
	B1: PartialEq<B2>,
	C1: PartialEq<C2>,
{
	|v1: &(A1, B1, C1), v2: &(A2, B2, C2)| v1.0 == v2.0 && v1.1 == v2.1 && v1.2 == v2.2
}

/// Returns whether the first list in an unordered sublist of the second list.
///
/// One list is an unordered sublist of another if all its elements can be found
/// in the other list, without duplications. Examples ( `<` as operator):
///
/// - `[1,2,3] < [3,2,1]`
/// - `[1,2,3] < [2,3,3,1]`
/// - `[1,2,2,3] !< [1,2,3]`
pub fn unordered_sublist<B, P, F>(sublist: &Vec<B>, superlist: &Vec<P>, equal: F) -> bool
where
	F: Fn(&B, &P) -> bool,
{
	// Track whether each element in the superlist has been used
	// to match an element of the sublist
	let mut used = Vec::new();
	used.resize(superlist.len(), false);

	// For each sublist element
	'outer: for sub_e in sublist
	{
		// Look through all superelements
		for (i, super_e) in superlist.iter().enumerate()
		{
			// If the element is unused
			if !used[i]
			{
				// if they are equal
				if equal(&sub_e, super_e)
				{
					// Assign the super element as used and move to the nex subelement
					used[i] = true;
					continue 'outer;
				}
			}
		}
		// The subelement was not found
		return false;
	}
	// All subelement were found
	true
}

/// Identical to `unordered_sublist()` except for values which are `Eq`.
pub fn unordered_sublist_equal<L: PartialEq<R>, R>(sublist: &Vec<L>, superlist: &Vec<R>) -> bool
{
	unordered_sublist(sublist, superlist, |v_sub, v_super| v_sub == v_super)
}

#[allow(dead_code)]
/// Returns whether the two lists are equivalent if the order of the elements is
/// ignored.
pub fn unordered_equivalent_lists<B, P, F1, F2>(l1: &Vec<B>, l2: &Vec<P>, e1: F1, e2: F2) -> bool
where
	F1: Fn(&B, &P) -> bool,
	F2: Fn(&P, &B) -> bool,
{
	unordered_sublist(l1, l2, e1) && unordered_sublist(l2, l1, e2)
}

/// Returns whether the two lists are equivalent if the order of the elements is
/// ignored.
pub fn unordered_equivalent_lists_equal<L: PartialEq<R>, R: PartialEq<L>>(
	l1: &Vec<L>,
	l2: &Vec<R>,
) -> bool
{
	unordered_sublist_equal(l1, l2) && unordered_sublist_equal(l2, l1)
}

/// Copies the topology of the mock graph into `g`.
///
/// Each vertex in the mock gets a new vertex in `g`.
/// All edges are copied to the respective new vertices.
/// All weights are cloned.
pub fn auto_copy_from<G, M>(g: &mut G, mock: &M) -> HashMap<MockVertex, G::Vertex>
where
	G: NewVertex<VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight> + AddEdge,
	M: Graph<
		Vertex = MockVertex,
		Directedness = G::Directedness,
		VertexWeight = MockVertexWeight,
		EdgeWeight = MockEdgeWeight,
	>,
{
	// Add all the vertices, remembering which mock vertices match which real
	// vertices
	let mut vertex_map = HashMap::new();
	for v in mock.all_vertices()
	{
		let new_v = g
			.new_vertex_weighted(mock.vertex_weight(&v).unwrap().clone())
			.unwrap();
		vertex_map.insert(v, new_v.clone());

		// Insert all edge to/from the finished vertices
		for (v_done, new_v_done) in vertex_map.iter()
		{
			for e_w in mock.edges_between(&v, v_done)
			{
				g.add_edge_weighted(new_v.borrow(), new_v_done.borrow(), e_w.borrow().clone())
					.unwrap();
			}
			if G::Directedness::directed() && *v_done != v
			{
				for e_w in mock.edges_between(v_done, &v)
				{
					g.add_edge_weighted(new_v_done.borrow(), new_v.borrow(), e_w.borrow().clone())
						.unwrap();
				}
			}
		}
	}

	vertex_map
}
