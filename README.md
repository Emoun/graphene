
# graphene
[![Build Status](https://travis-ci.org/Emoun/graphene.svg?branch=master)](https://travis-ci.org/Emoun/graphene)
[![](http://meritbadge.herokuapp.com/graphene)](https://crates.io/crates/graphene)
[![](https://docs.rs/graphene/badge.svg)](https://docs.rs/graphene)

A general purpose, extensible [Graph Theory](https://en.wikipedia.org/wiki/Graph_theory)
data type and algorithm library for Rust.

This project is still in early design and development stage and is still changing a lot.
A good public API is not the focus of the project yet, documentation may be scarce, and bugs may be rampant.

## Example

```Rust
#[macro_use]
extern crate graphene;
use graphene::core::*;
use graphene::core::constraint::*;
use graphene::common::AdjListGraph;
//!
custom_graph!{
	// Name of the resulting graph type
	struct MyGraph<V,W>
	// The BaseGraph implementation to base the new graph on.
	as AdjListGraph<V,W>
	// The graph wrappers that will constrain the BaseGraph implementation so that
	// it upholds the property traits.
	use UniqueGraph,UndirectedGraph
	// The property traits the new graph implements
	impl Unique,Undirected
	// The generic bounds
	where V: Vertex, W: Weight
}

fn main(){
    let mut g = MyGraph::graph(vec![1,2,3], vec![(1,2,1),(2,3,2),(3,1,3)]).unwrap();
	assert_eq!(g.edges_between(1,2).len(), 2);

	// Cannot add an edge that is already there because the graph
	// is declared as Unique, meaning no two edges may be incident
	// on the same vertices and have the weight.
	assert!(g.add_edge(BaseEdge::new(1,2,1)).is_err());
	assert_eq!(g.edges_between(1,2).len(), 2);
}
```

### Use Cases

The following is a non-exhaustive list of use cases Graphene aims to support:

* Common, finite, in-memory graphs.
* Abstract-Syntax-Tree-like graphs and use cases. (I.e. compilers and the like) The important part here is the distictive visitor pattern often used in these cases.
* Math. This requires scientifically accurate naming and API.
* Remote graph, i.e. graphs which are hosted on different machines and interaction with the data structure is dependent on a communication channel. These graphs may also be too big to fit in-memory for the machines that are working on them.
* Infinite graphs. E.g. graphs over the possible moves and board state of games like Chess or Go.
* Embedded, no-std (and maybe no-alloc) environments.
* Compatible with existing graph crates (i.e. traits should be implementable for existing graph implementation): petgraph, graphlib
 
### Naming convensions

When choosing names for traits, struct, functions, etc.. the following guidelines should be kept in mind:

* Avoid abreviations. 

Not writing out words make it hard for the uninitiated to figure out what is meant.
Therefore, avoid using abreviations, acronyms, contractions or other forms of shortening as much as possible. 
Extemely common shortenings that can be expected to be known by almost any programmer are an exception (e.g. 'DFS' as a short form of 'Depth-First-Search' can be expected to be familiar by anyone that has taken a introductory course on algorithms.)
If shortenings are used, the complete form must be used in the initial part of the documentation, such no confusion can arise.
If two concepts have the same well-known shortening, neither of them can use it. 

* Prioritize autocompletion

IDE autocompletion is a good way to learn the API.
Therefore, naming should account for how autocompletion works.
Users should be expected to search for their needed struct, function, etc. using autocompletion, which means names should account for that.
E.g. similar concepts should include the same words (preferably is a similar order) such that autocompletion can should the best suggestsions.
An example could be all methods operating on vertices starting with the word 'vertex' followed by more specifics ('vertex_add', 'vertex_remove', 'vertex_weight', ...)

* Accuracy

Correct naming should be a priority, even in cases where they differ from the common naming. E.g. A directed acyclic graph with a edges pointing away from a 'root' is called an Arborescence and not a DAG nor simply Tree.



### License

Licensed under the MIT license.

Any file with its own license header is also licensed under the above license, at your option.
If the header specifies a different license than the above, it must be seen as an additional choice of license
and the above license or the header license can be chosen at your option.

Any intentional contribution to this repository is licensed under the above license in addition
to any contribution specific license without exception.





