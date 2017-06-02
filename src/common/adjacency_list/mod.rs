
mod impl_base_graph;

pub use self::impl_base_graph::*;

use core::*;

#[derive(Clone, Debug)]
pub struct AdjListGraph<T> {
	edges: Vec<Vec<usize>>,
	values:Vec<T>,
}

impl<T> AdjListGraph<T>
	where
		T: Eq + Copy
{
	
	pub fn new(values: Vec<T>, edges: Vec<(usize, usize)>) -> Option<AdjListGraph<T>> {
		let mut g = AdjListGraph{ edges: Vec::new(), values: values };
		
		//Validate all edges point to vertices
		for &(source, sink) in &edges {
			if source >= g.values.len() || sink >= g.values.len(){
				return None;
			}
		}
		
		//Initialise adjacency list
		for _ in 0..g.values.len(){
			g.edges.push(Vec::new());
		}
		
		//Insert each edge
		for &(source, sink) in &edges {
			g.edges[source].push(sink);
		}
		Some(g)
	}
	
	fn get_index(&self, v: T) -> Option<usize>{
		self.values.iter().position(|ref value| **value == v)
	}
	
	fn get_value(&self, i: usize) -> Option<T>{
		if i < self.values.len() {
			Some(self.values[i].clone())
		}else {
			None
		}
	}
	
	fn if_valid_edge<F>(&mut self, e:BaseEdge<T,()>, cont: F) -> Result<(), ()>
		where F: Fn(&mut Self,usize, usize)-> Result<(),()>
	{
		if let (Some(source_i), Some(sink_i))
		= (self.get_index(e.source()), self.get_index(e.sink()))
			{
				return cont(self, source_i, sink_i);
			}
		Err(())
	}
}

