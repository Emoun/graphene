
mod impl_base_graph;

pub use self::impl_base_graph::*;
use core::*;

#[derive(Clone, Debug)]
pub struct AdjListGraph<V,W> {
	edges: Vec<Vec<(usize,W)>>,
	values:Vec<V>,
}

impl<V,W> AdjListGraph<V,W>
	where
		V: Vertex,
		W: Weight,
{
	
	pub fn new(values: Vec<V>, edges: Vec<(usize, usize,W)>) -> Option<AdjListGraph<V,W>> {
		
		//Validate no duplicate vertices
		let mut seen = Vec::new();
		for v in values.iter() {
			if seen.contains(v) {
				return None;
			}else{
				seen.push(*v);
			}
		}
		let mut g = AdjListGraph{ edges: Vec::new(), values: values };
		
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
	
	fn get_value(&self, i: usize) -> Option<V>{
		if i < self.values.len() {
			Some(self.values[i].clone())
		}else {
			None
		}
	}
	
	fn if_valid_edge<F>(&mut self, e:BaseEdge<V,W>, cont: F) -> Result<(), ()>
		where F: Fn(&mut Self,usize, usize, W)-> Result<(),()>
	{
		if let (Some(source_i), Some(sink_i))
		= (self.get_index(e.source), self.get_index(e.sink))
			{
				return cont(self, source_i, sink_i, e.weight);
			}
		Err(())
	}
}

