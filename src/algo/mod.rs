//! A collection of graph algorithm implementations.

mod bfs;
mod dfs;
mod dijkstra_shortest_paths;
mod tarjan_scc;

pub use self::{bfs::*, dfs::*, dijkstra_shortest_paths::*, tarjan_scc::*};
use crate::core::{property::VertexInGraph, Ensure, Graph};
use std::borrow::Borrow;

pub fn path_exists<G: Graph>(
	g: &G,
	source: impl Borrow<G::Vertex>,
	sink: impl Borrow<G::Vertex>,
) -> bool
{
	if let Ok(g) = VertexInGraph::ensure(g, source.borrow().clone())
	{
		if g.contains_vertex(sink.borrow())
		{
			return Dfs::new_simple(&g).find(|v| v == sink.borrow()).is_some();
		}
	}
	false
}
