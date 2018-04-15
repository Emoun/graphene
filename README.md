
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
	// it upholds the constraint traits.
	use UniqueGraph,UndirectedGraph
	// The constraint traits the new graph implements
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

### License

Licensed under the MIT license.

Any file with its own license header is also licensed under the above license, at your option.
If the header specifies a different license than the above, it must be seen as an additional choice of license
and the above license or the header license can be chosen at your option.

Any intentional contribution to this repository is licensed under the above license in addition
to any contribution specific license without exception.





