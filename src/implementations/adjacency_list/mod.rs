
mod impl_fine_grained_graph;
mod base_edge;

pub use self::base_edge::*;
pub use self::impl_fine_grained_graph::*;

#[derive(Clone,Debug)]
pub struct BaseEdge<T>
	where
		T:Copy,
{
	source: T,
	sink:T,
}

#[derive(Clone, Debug)]
pub struct AdjListGraph<T> {
	edges: Vec<Vec<usize>>,
	values:Vec<T>,
}

impl<T> AdjListGraph<T>
	where
		T: Eq + Clone
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
	
}

