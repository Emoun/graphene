//! A collection of graph algorithm implementations.

mod bfs;
mod dijkstra_shortest_paths;
mod retain;
pub mod search;
mod tarjan_scc;

pub use self::{bfs::*, dijkstra_shortest_paths::*, retain::*, tarjan_scc::*};
use crate::{
	algo::search::new_search_retained,
	core::{property::VertexInGraph, Ensure, Graph},
};
use std::borrow::Borrow;

pub fn path_exists<G: Graph>(
	g: &G,
	source: impl Borrow<G::Vertex>,
	sink: impl Borrow<G::Vertex>,
) -> bool
{
	if let Ok(g) = VertexInGraph::ensure(g, [source.borrow().clone()])
	{
		if g.contains_vertex(sink.borrow())
		{
			return new_search_retained(&g)
				.find(|v| v == sink.borrow())
				.is_some();
		}
	}
	false
}
