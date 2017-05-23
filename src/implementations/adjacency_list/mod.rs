
mod base_graph;
mod basic_impl;
mod base_edge;

pub use self::base_graph::*;
pub use self::base_edge::*;
pub use self::basic_impl::*;


pub struct BaseEdge<T>
	where
		T:Copy,
{
	source: T,
	sink:T,
}

#[derive(Clone, Debug)]
pub struct AdjListGraph<T> {
	edges: Vec<Vec<usize>>,
	values:Vec<T>,
}