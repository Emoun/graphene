use graph::*;
use implementations::adjacency_list::*;

impl<T:Copy> BaseEdge<T>{
	
	pub fn new(source: T, sink: T)-> BaseEdge<T>{
		BaseEdge{source, sink}
	}
	
}

impl<T: Copy> Sourced<T> for BaseEdge<T> {
	fn source(&self) -> T {
		self.source
	}
}

impl<T: Copy> Sinked<T> for BaseEdge<T> {
	fn sink(&self) -> T {
		return self.sink;
	}
}
