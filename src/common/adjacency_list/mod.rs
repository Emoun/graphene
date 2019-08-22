
mod impl_graph;
//mod impl_weights;

pub use self::impl_graph::*;
use crate::core::{trait_aliases::{
	Id,
}, Directed, Directedness};
use std::marker::PhantomData;


///
/// A graph using the adjacency list representation.
///
#[derive(Clone, Debug)]
pub struct AdjListGraph<V,Vw,Ew,D=Directed>
	where
		V: Id,
		D: Directedness
{
	///
	/// Adjacency list representation of the graph.
	/// Each index in vec is a vertex.
	///
	vertices: Vec<(
		//The ID of the vertex
		V,
		//The weight of the vertex
		Vw,
		//A list outgoing edges from this vertex
		Vec<(
			//The index of the sink vertex
			usize,
			//The weight of the edge
			Ew
		)>
	)>,
	phantom: PhantomData<D>
}

impl<V,Vw,Ew,D> AdjListGraph<V,Vw,Ew,D>
	where V:Id, D: Directedness
{
	pub fn new() -> Self
	{
		Self{vertices: Vec::new(), phantom: PhantomData}
	}
}
/*
impl<V,W> AdjListGraph<V,W>
	where
		V: Id,
{
	
	pub fn new(values: Vec<V>, edges: Vec<(usize, usize,usize)>) -> Option<AdjListGraph<V,W>> {
		
		//Validate no duplicate vertices
		let mut seen = Vec::new();
		for v in values.iter() {
			if seen.contains(v) {
				return None;
			}else{
				seen.push(*v);
			}
		}
		let mut g = AdjListGraph{ edges: Vec::new(), values: values, edge_weights: Vec::new() };
		
		//Validate all edges point to vertices
		for &(source, sink, _) in &edges {
			if source >= g.values.len() || sink >= g.values.len(){
				return None;
			}
		}
		
		//Initialise adjacency list
		for _ in 0..g.values.len(){
			g.edges.push(Vec::new());
		}
		
		//Insert each edge
		for &(source, sink,weight) in &edges {
			g.edges[source].push((sink,weight));
		}
		Some(g)
	}
	
	fn get_index(&self, v: V) -> Option<usize>{
		self.values.iter().position(|ref value| **value == v)
	}
	
	#[allow(dead_code)]
	fn get_value(&self, i: usize) -> Option<V>{
		if i < self.values.len() {
			Some(self.values[i].clone())
		}else {
			None
		}
	}
	
	fn valid_adjacency<E>(&self, e: &E) -> bool
		where E: Edge<V, ()>,
	{
		if let (Some(_), Some(_))
			= (self.get_index(*e.source()), self.get_index(*e.sink()))
		{
			true
		}else{
			false
		}
	}
	
	fn if_valid_edge<E,F>(&mut self, e: E, cont: F) -> Result<(), ()>
		where
			E: Edge<V, usize>,
			F: Fn(&mut Self,usize, usize, usize)-> Result<(),()>
	{
		if let (Some(source_i), Some(sink_i))
		= (self.get_index(*e.source()), self.get_index(*e.sink()))
			{
				if *e.id() < self.edge_weights.len() {
					return cont(self, source_i, sink_i, *e.id());
				}
			}
		Err(())
	}
}
*/
