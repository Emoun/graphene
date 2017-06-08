

#![allow(non_snake_case)]
extern crate graphene;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod common;
mod arbitraries;



use graphene::core::*;
use graphene::common::*;

#[test]
fn someTest(){
	let vertices = vec![1, 2, 3];
	let edges = vec![(1, 2, ()), (2, 3, ()), (3, 1, ())];
	let mut uncon_g = AdjListGraph::graph(vertices.clone(), edges.clone()).unwrap();
	
	let mut unique_g = constraint::UniqueGraph::
		<u32,(),Vec<u32>,Vec<BaseEdge<u32,()>>,AdjListGraph<u32,()>>::
			graph(vertices.clone(),edges.clone()).unwrap();
	
	let mut undir_unique_g = constraint::UndirectedGraph::
		<u32,(),Vec<u32>,Vec<BaseEdge<u32,()>>,
			constraint::UniqueGraph<u32,(),Vec<u32>,Vec<BaseEdge<u32,()>>,AdjListGraph<u32,()>>
			>::
			graph(vertices.clone(),edges.clone()).unwrap();
	
	println!("Unconstrained Graph:\t {:?}", uncon_g);
	println!("Unique Graph:\t\t\t {:?}", unique_g);
	println!("Undir unique Graph:\t\t {:?}", undir_unique_g);
	
	assert!(uncon_g.add_edge(BaseEdge::new(1,2,())).is_ok());
	assert!(unique_g.add_edge(BaseEdge::new(1,2,())).is_err());
	assert!(undir_unique_g.add_edge(BaseEdge::new(1,2,())).is_err());
	
	assert!(unique_g.unconstrained().add_vertex(4).constrain().is_ok());
	assert!(undir_unique_g.unconstrained().add_vertex(4).constrain().is_ok());
	
	assert!(undir_unique_g	.unconstrained()
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