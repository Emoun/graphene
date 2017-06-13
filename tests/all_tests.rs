#![allow(non_snake_case)]
#![allow(unused_imports)]
#[macro_use]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
mod arbitraries;
mod common;
mod core;

use graphene::core::*;
use graphene::core::constraint::*;
use graphene::common::*;

///
/// Custom graph that uses AdjListGraph and is Undirected and Unique
///
custom_graph!{
	struct UndirectedUniqueGraph
	where AdjListGraph
	impl Undirected,Unique
	use UndirectedGraph,UniqueGraph
}

//#[test]
fn testGraphTest(){
	let mut g = UndirectedUniqueGraph::<u32,()>::empty_graph();
	
	g.add_vertex(1).unwrap();
	g.add_vertex(2).unwrap();
	assert!(g.add_edge(BaseEdge::new(1,1,())).is_ok());
	assert!(g.add_edge(BaseEdge::new(1,1,())).is_err());
	assert!(g.unconstrained().add_edge(BaseEdge::new(1,2,())).constrain().is_err());
	
	println!("{:?}", g);
	println!("Wrappen: {:?}", g.wrapped());
}

//#[test]
fn someTest(){
	let vertices = vec![1, 2, 3];
	let edges = vec![(1, 2, ()), (2, 3, ()), (3, 1, ())];
	let mut uncon_g = AdjListGraph::graph(vertices.clone(), edges.clone()).unwrap();
	
	let mut unique_g = constraint::UniqueGraph::
		<AdjListGraph<_,_>>::
			graph(vertices.clone(),edges.clone()).unwrap();
	
	let mut undir_unique_g = constraint::UndirectedGraph::
		<constraint::UniqueGraph<AdjListGraph<_,_>>
			>::
			graph(vertices.clone(),edges.clone()).unwrap();
	
	let mut custom_g = UndirectedUniqueGraph::graph(vertices.clone(),edges.clone()).unwrap();
	
	println!("Unconstrained Graph:\t {:?}", uncon_g);
	println!("Unique Graph:\t\t\t {:?}", unique_g);
	println!("Undir unique Graph:\t\t {:?}", undir_unique_g);
	println!("UndirectedUniqueGraph:\t {:?}", custom_g);
	
	assert!(uncon_g.add_edge(BaseEdge::new(1,2,())).is_ok());
	assert!(unique_g.add_edge(BaseEdge::new(1,2,())).is_err());
	assert!(undir_unique_g.add_edge(BaseEdge::new(1,2,())).is_err());
	assert!(custom_g.add_edge(BaseEdge::new(1,2,())).is_err());
	
	assert!(unique_g.unconstrained().add_vertex(4).constrain().is_ok());
	assert!(undir_unique_g.unconstrained().add_vertex(4).constrain().is_ok());
	assert!(custom_g.unconstrained().add_vertex(4).constrain().is_ok());
	
	assert!(undir_unique_g	.unconstrained()
							.remove_edge(BaseEdge::new(1,2,()))
							.constrain().is_err());
	assert!(custom_g		.unconstrained()
							.remove_edge(BaseEdge::new(1,2,()))
							.constrain().is_err());
	
	assert!(unique_g.unconstrained()
					.add_edge(BaseEdge::new(1,2,()))
					.add_vertex(5)
					.constrain().is_err());
	
	assert!(unique_g.unconstrained()
					.add_edge(BaseEdge::new(1,2,()))
					.add_vertex(5)
					.remove_edge(BaseEdge::new(1,2,()))
					.constrain().is_ok());
	
	assert!(unique_g.unconstrained()
					.add_edge(BaseEdge::new(1,6,()))
					.add_vertex(6)
					.constrain().is_err());
	
	assert!(unique_g.unconstrained()
					.add_vertex(6)
					.add_edge(BaseEdge::new(1,6,()))
					.constrain().is_ok());
	
	assert!(unique_g.add_edge(BaseEdge::new(1,6,())).is_err());
	assert!(unique_g.add_edge(BaseEdge::new(6,1,())).is_ok());
	
	
	println!("Unique Graph: {:?}", unique_g);
	
}
