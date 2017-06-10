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


custom_graph!{
	TestGraph
	where AdjListGraph
	impl Undirected,Unique
	use UndirectedGraph,UniqueGraph
}

//#[test]
fn testGraphTest(){
	let mut g = TestGraph{graph: UndirectedGraph{graph: UniqueGraph{graph: AdjListGraph::<u32,()>::empty_graph()}}};
	
	g.add_vertex(1).unwrap();
	g.add_vertex(2).unwrap();
	assert!(g.add_edge(BaseEdge::new(1,1,())).is_ok());
	assert!(g.add_edge(BaseEdge::new(1,1,())).is_err());
	assert!(g.unconstrained().add_edge(BaseEdge::new(1,2,())).constrain().is_err());
	
	println!("{:?}", g);
}

///
/// Custom graph that uses AdjListGraph and is Undirected and Unique
///
#[derive(Clone,Debug)]
struct UndirectedUniqueGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{
	graph:	UndirectedGraph<V,W,<AdjListGraph<V,W> as BaseGraph>::VertexIter,<AdjListGraph<V,W> as BaseGraph>::EdgeIter,
				UniqueGraph<V,W,<AdjListGraph<V,W> as BaseGraph>::VertexIter,<AdjListGraph<V,W> as BaseGraph>::EdgeIter,
					AdjListGraph<V,W>>>
}

impl<V,W> BaseGraph for UndirectedUniqueGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{
	type Vertex = V;
	type Weight = W;
	type VertexIter = <AdjListGraph<V,W> as BaseGraph>::VertexIter;
	type EdgeIter = <AdjListGraph<V,W> as BaseGraph>::EdgeIter;
	fn empty_graph() -> Self {
		UndirectedUniqueGraph { graph: UndirectedGraph{graph: UniqueGraph{graph: AdjListGraph::<V,W>::empty_graph()}}}
	}
	
	wrap!{graph.all_vertices(&self) -> Self::VertexIter}
	
	wrap!{graph.all_edges(&self) -> Self::EdgeIter}
	
	wrap!{graph.add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
	wrap!{graph.add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
	
	wrap!{graph.remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
}
impl<V,W> ConstrainedGraph for 	UndirectedUniqueGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{
	wrap!{graph.invariant_holds(&self) -> bool}
	
	wrap_uncon_methods!{graph}
}

impl<V,W> Unique for UndirectedUniqueGraph<V,W>
	where
		V: Vertex,
		W: Weight,{}
impl<V,W> Undirected for UndirectedUniqueGraph<V,W>
	where
		V: Vertex,
		W: Weight,{}

//#[test]
fn someTest(){
	let vertices = vec![1, 2, 3];
	let edges = vec![(1, 2, ()), (2, 3, ()), (3, 1, ())];
	let mut uncon_g = AdjListGraph::graph(vertices.clone(), edges.clone()).unwrap();
	
	let mut unique_g = constraint::UniqueGraph::
		<_,_,_,_,AdjListGraph<_,_>>::
			graph(vertices.clone(),edges.clone()).unwrap();
	
	let mut undir_unique_g = constraint::UndirectedGraph::
		<_,_,_,_,
			constraint::UniqueGraph<_,_,_,_,AdjListGraph<_,_>>
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


