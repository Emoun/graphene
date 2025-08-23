mod acyclic_graph;
mod combinations;
mod connected_graph;
mod guided_arb_graph;
mod mock_graph;
mod simple;
mod tree_graph;
mod unique_graph;
mod vertex_in_graph;

pub use self::{
	acyclic_graph::*, combinations::*, connected_graph::*, guided_arb_graph::*, tree_graph::*,
	unique_graph::*,
};
use quickcheck::Gen;
use rand::Rng;

/// Chooses number of vertices and edges based on a function that calculates the
/// edge range based on a chosen vertex count
///
/// `edge_range` returns the min and max allowed number of edges (both
/// inclusive) based on the given vertex count
fn choose_size_static_edges<G: Gen>(
	g: &mut G,
	v_min: usize,
	v_max: usize,
	e_min: usize,
	e_max: usize,
	edge_range: impl Fn(usize) -> (usize, usize),
) -> (usize, usize)
{
	assert!(
		e_min <= edge_range(v_max).1,
		"Minimum number of edges higher than theoretically possible: e_min: {}, Max possible: {}",
		e_min,
		edge_range(v_max).1
	);
	assert!(
		e_max >= edge_range(v_max).0,
		"Maximum number of edges lower than theoretically possible: e_max: {}, Min possible: {}",
		e_max,
		edge_range(v_max).0
	);

	let v_count = g.gen_range(v_min, v_max);
	let e_count = g.gen_range(
		std::cmp::max(e_min, edge_range(v_count).0),
		std::cmp::min(e_max, edge_range(v_count).1 + 1),
	);

	(v_count, e_count)
}
