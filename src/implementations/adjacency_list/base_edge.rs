use graph::*;
use implementations::adjacency_list::*;


impl<T:Copy+Eq> BaseEdge<T>{
	
	pub fn new(source: T, sink: T)-> BaseEdge<T>{
		BaseEdge{source, sink}
	}
	
}

impl<T: Copy+Eq> Sourced<T> for BaseEdge<T> {
	fn source(&self) -> T {
		self.source
	}
}

impl<T: Copy+Eq> Sinked<T> for BaseEdge<T> {
	fn sink(&self) -> T {
		return self.sink;
	}
}
