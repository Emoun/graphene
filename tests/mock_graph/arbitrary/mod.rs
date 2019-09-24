
mod combinations;
mod arb_connected_graph;
mod mock_graph;
mod arb_unique_graph;

pub  use self::{
	combinations::*,
	arb_connected_graph::*,
	mock_graph::*,
	arb_unique_graph::*,
};
use quickcheck::Gen;

fn max_vertex_count<G: Gen>(g: &G) -> usize
{
	g.size() / 5
}