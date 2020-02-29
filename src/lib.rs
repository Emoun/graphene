// `graphene` is a general purpose, extensible [Graph Theory](https://en.wikipedia.org/wiki/Graph_theory)
// data type and algorithm library.
//
// ### Quick Examples:
//
// Using `common::AdjListGraph`, we define a graph with 3 vertices
// **1** , **2** , **3** and 3 weighted edges ( **1** -(1)-> **2** -(2)-> **3**
// -(3)-> **1**):
//
// ```
// # #![allow(unused_variables)]
// use graphene::{
// 	common::AdjListGraph,
// 	core::{BaseEdge, BaseGraph},
// 	};
//
// // `graph()` takes a Vec of vertex values and a Vec of edges
// // where an edge is a tuple, (v1,v2,w), of the source v1, the sink v2, and the weight w.
// let mut g = AdjListGraph::graph(vec![1, 2, 3], vec![(1, 2, 1), (2, 3, 2), (3, 1, 3)]).unwrap();
// // Adding a vertex
// assert!(g.add_vertex(4).is_ok());
// // Adding an edge
// assert!(g.add_edge(BaseEdge::new(3, 4, 2)).is_ok());
// ```
// ---
//
// We can declare a different graph that does not allow edge duplicates (a
// unique graph) and is undirected (the previous graph wasn't Unique and was
// directed):
//
// ```
// #[macro_use]
// extern crate graphene;
// use graphene::{
// 	common::AdjListGraph,
// 	core::{property::*, *},
// 	};
//
// custom_graph! {
// // Name of the resulting graph type
// struct MyGraph<V,W>
// // The BaseGraph implementation to base the new graph on.
// as AdjListGraph<V,W>
// // The graph wrappers that will constrain the BaseGraph implementation so that
// // it upholds the property traits.
// use UniqueGraph,UndirectedGraph
// // The property traits the new graph implements
// impl Unique,Undirected
// // The generic bounds
// where V: Vertex, W: Weight
// 			}
//
// fn main()
// 	{
// 	let mut g = MyGraph::graph(vec![1, 2, 3], vec![(1, 2, 1), (2, 3, 2), (3, 1, 3)]).unwrap();
// 	assert_eq!(g.edges_between(1, 2).len(), 2);
//
// 	// Cannot add an edge that is already there because the graph
// 	// is declared as Unique, meaning no two edges may be incident
// 	// on the same vertices and have the weight.
// 	assert!(g.add_edge(BaseEdge::new(1, 2, 1)).is_err());
// 	assert_eq!(g.edges_between(1, 2).len(), 2);
//
// 	// When a graph is undirected, the direction of the given edge
// 	// (whether 1 -> 2 or 2 -> 1) makes no difference.
// 	// Since the graph is unique, adding an edge in the opposite
// 	// direction as an existing edge is therefore illegal.
// 	assert!(g.add_edge(BaseEdge::new(2, 1, 1)).is_err());
// 	assert_eq!(g.edges_between(1, 2).len(), 2);
// 	}
// ```
//
// # About
//
// `graphene` aims at providing general purpose traits, structs, and macros for
// defining graphs, and functions that implement various algorithms that will
// run on any implementer of the library. Additionally, general purpose graph
// implementations will be provided.
//
// Because of the promise of generality, `graphene` is designed in a way that
// will allow users to implement their own graph types for their specific needs
// that can easily be integrated with the library's algorithms. Additionally,
// `graphene` aims at avoiding graph type bloat, i.e. defining many similar
// graph types:
//
// - `SimpleWeightedDirectedGraph`
// - `SimpleUnweightedDirectedGraph`
// - `SimpleWeightedUndirectedGraph`
// - `WeightedUndirectedGraph`
// - ...
//
// Instead, the user can semi-dynamically specify constraints on basic graph
// implementation to suit his needs. More on that later.
//
// `graphene` will use the terminology used by
// [Wolfram MathWorld](http://mathworld.wolfram.com/topics/GraphTheory.html)
// where possible. More specifically, 'vertex' and 'edge' will be used. We
// define that given a directed edge **v1** -> **v2**, then the `source` vertex
// is **v1** and the `sink` vertex is **v2**. Likewise, the edge is *sourced* in
// **v1** and *sinked* (the misspelling of 'sunk' is intentional) in **v2**. For
// both directed and undirected graphs the edge is *incident* on **v1** and
// **v2**.
//
// The crate is divided in three modules:
//
// - `core`: Contains the general purpose traits, structs, functions, and macros
//   for graph implementation.
// - `algo` (**TODO**) : Contains graph algorithms that accept any graph that
//   implements `core`.
// - `common`: Contains general purpose and commonly used graph implementations
//   of `core` for quick usage.
//
// # Vertices and Edges
//
// In `graphene` vertices have a value, which is use to identify each vertex.
// Therefore, these values must be unique for every vertex in the graph.
// Internally, a graph implementation may manage its vertices as it wishes, but
// it must communicate with  the outside world using the vertices' values.
//
// Edges are identified by the tuple `(source,sink,weight)`. Edges do not have
// to be unique in the graph, which means two edges with the same source, sink
// and weight are practically indistinguishable. This is by design, as if the
// information in the tuple is not enough to distinguish two edges,
// then choosing either one should not make a difference.
//
// # Directionality
//
// By default, edges in any graph are directed. If an undirected graph is
// needed, the `core` provides an `Undirected` property trait which can be
// implemented for a given graph. However, behind the scenes, any `Undirected`
// graph is a directed graph with edges in both directions. Therefore, given
// that any BaseGraph may be `Undirected`, an additional property trait,
// `Directed` is provided (**TODO**) which defines a graph as being specifically
// directed.
//
// This may seem confusing, so here is a deeper explanation:
//
// A BaseGraph uses directed edges. An `Undirected` BaseGraph will still use
// directed edges, but it will treat all edges it receives from the user as
// undirected. This means, if the user wants to add an undirected edge **1** -
// **2**, the graph will actually add **1** -> **2** and **1** <- **2**.
// On the other hand, when the graph outputs edges to the user, they are given
// as their directed pairs, e.g. **1** -> **2** and **1** <- **2** for the
// undirected edge **1** - **2**. This means that the user must handle these two
// directed edges as one undirected one.
//
// Consider then a function which takes a BaseGraph as its input. Since the
// input is not bounded by `Directed` it may be `Undirected`. If the function
// cannot handle undirected graphs it will fail if given any such graph.
// Therefore it should bound its input with `Directed`.
//
// So, when to actually bound a function with `Directed` instead of just
// `BaseGraph`?:
//
// - If it cannot handle there are an edge in each directed between two
//   vertices.
//
// - If it cannot handle that two edges between two vertices, one in each
//   direction, have the same weight.
//
// - If it needs to keep track of specific edges. Consider an algorithm that
//   tracks whether an edge has been used,
// if the graph is 'Directed`, then an edge in each direction between two
// vertices must be tracked independently, but if its `Undirected`, then a pair
// in each direction must be tracked as one edge.
//
// The formal requirements and definitions have only been presented in a way
// sufficient for understanding the idea. Therefore, the formal definition
// should be consulted for complete accuracy.
//
// ### FAQ on directionality
//
// - Why does an `Undirected` graph output edges in directed pairs?
//
// In short, to allow functions to be directionality-agnostic. Consider
// implementing an algorithm that works on both directed and undirected graphs.
// If `Undirected` did not return pairs, then the function would have to know
// that the outputted edge was undirected, which means it would have to bound
// its input with `Undirected` forcing you to implement another, differently
// named function for the `Directed` case. But if `Undirected` outputs
// directed pairs, the function will be able to use them as if they were one
// undirected edge, since they tell the function that both directions are
// available between the two vertices. Therefore, the function can just bound
// its input with `BaseGraph` and handle both directionalities at once.
//
// - Why does an `Undirected` graph treat input edges as undirected and not
//   require a directed pair instead?
//
// For similar reasons as the above answer. If a function is
// directionality-agnostic, but does mutate the graph, then it wouldn't know
// that a graph is `Undirected` and therefore wouldn't know to add a pair
// for each edge. By having the graph itself handle the splitting of an
// undirected edge into a directed pair, the function can bound its input by
// just `BaseGraph` and handle both directionalities at the same time.
//
// ### Directionality example
//
// Consider the property trait `Unique`. A graph is unique if for all edges,
// no two edges are incident on the same two vertices in the same direction.
// Consequently, for undirected graphs this means that only one undirected edge
// is allowed between any two vertices, while an edge in each direction is
// allowed for directed graphs.
//
// The `graphene::core::constaint` module is able to simply implement this
// property by iterating over all the directed edges a graph has, and reject
// it if any two edges have the same source and sink. This implementation is
// directionality agnostic since undirected graphs return a directed pair, which
// is allowed of a directed unique graph. Had `Undirected` graphs returned only
// one undirected edge, two property traits `UniqueDirection` and
// `UniqueUndirected` would have been needed. This is because the following two
// edges, (1,2) and (2,1), would be invalid for undirected graphs, as the edges
// are identical when disregarding direction (which undirected graphs do). For
// directed graphs the two edges are acceptable, as they have different
// directions.
//
// Therefore, the `graphene`'s directionality design allows for a single
// implementation for the `Unique` property that works for both directed and
// undirected graphs.
//
//
// # FAQ
//
// - How do i initialize an unweighted graph, it seems then all require a
//   weight, e.g. AdjListGraph<V,W>?
//
// By convention, `()` is treated as the lack of a weight. Therefore,
// AdjListGraph<V,()> is an unweighted graph.
//
//
#![recursion_limit = "256"]
#[macro_use]
pub mod core;
pub mod algo;
pub mod common;
