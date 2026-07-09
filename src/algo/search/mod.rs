//! This module contains various ways of traversing a graph from a starting
//! vertex.
//!
//! Even though *search* is used as the main term for traversing a graph, this
//! module only handle the traversal, delegating the searching to the user.
//!
//! [`Search`] is the main trait implemented by the various search algorithms.
//! The algorithms do not themselves own or borrow the graph they are searching,
//! allowing for mutable access to the graphs while searching it.
//! The [`Search::next`] borrows the graph being searched every time, returning
//! the next vertex in the search.
//!
//! If mutable access to the searched graph is not needed during a searching, the [`Search::retain`] method is provided by all search algorithms, returning an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) which retains ownership/borrowing of the graph during the search.
//! This is likely the most straightforward and often used way of searching.
//!
//! # Searching
//!
//! All searches must have a starting vertex, which is designated by
//! [`VertexIn::vertex_at::<0>`]. Calling [`Search::next`] or [`Iterator::next`] then iteratively returns vertices in the [connected component](https://mathworld.wolfram.com/ConnectedComponent.html) of the starting vertex.
//! The starting vertex itself will never be returned by these methods.
//! [`None`] is returned when all vertices in the connected component have been
//! returned.
//!
//! The order of returned vertices is dependent on the specific search algorithm
//! used. Functions are provided for when the order is not important in addition
//! to functions that examine properties of graphs based on searching through
//! them.
//!
//! The following example shows how to initiate a retained search using an
//! unspecified algorithm:
//! ```
//! use graphene::{
//! 	algo::search::{new_search_retained, Search},
//! 	common::AdjListGraph,
//! 	core::{
//! 		Ensure,
//! 		property::{
//! 			NewVertex, AddEdge, VertexInGraph
//! 		}
//! 	},
//! };
//! use graphene::core::Graph;
//!
//! // Initialize the graph
//! let mut graph = AdjListGraph::<usize,()>::new();
//!
//! let v0 = graph.new_vertex_weighted(0).unwrap();
//! let v1 = graph.new_vertex_weighted(1).unwrap();
//! let v2 = graph.new_vertex_weighted(2).unwrap();
//!
//! graph.add_edge(&v0,&v1).unwrap();
//! graph.add_edge(&v1,&v2).unwrap();
//!
//! // We use `VertexInGraph` to ensure traversal starts at v0.
//! let graph = VertexInGraph::ensure(graph, [v0]).unwrap();
//!
//! // Initialize the search
//! let mut search = new_search_retained(&graph);
//!
//! // We search for the first vertex with weight == 1.
//! let found_vertex = search.find(|&v| graph.vertex_weight(&v).unwrap() == &1).unwrap();
//! assert_eq!(v1, found_vertex)
//! ```
mod dfs;
mod search;

pub use dfs::*;
pub use search::*;

use crate::core::{property::VertexIn, GraphDeref};
/// Initializes a new search using an unspecified algorithm.
pub fn new_search<G>(graph: G) -> impl Search<G::Graph>
where
	G: GraphDeref,
	G::Graph: VertexIn<1>,
{
	Dfs::new_simple(graph.graph())
}

/// Initializes a new retained search using an unspecified algorithm.
pub fn new_search_retained<G>(graph: G) -> Retained<G, impl Search<G::Graph>>
where
	G: GraphDeref,
	G::Graph: VertexIn<1>,
{
	Dfs::new_simple(graph.graph()).retain(graph)
}
