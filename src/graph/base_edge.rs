

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct BaseEdge<V,W>
	where
		V: Copy+Eq,
		W: Copy+Eq,
{
	pub source: V,
	pub sink:V,
	pub weight: W,
}

impl<V:Copy+Eq, W:Copy+Eq> BaseEdge<V,W>{
	
	pub fn new(source: V, sink: V, weight: W)-> BaseEdge<V,W>{
		BaseEdge{source, sink, weight}
	}
	
	pub fn source(&self) -> V {
		self.source
	}
	
	pub fn sink(&self) -> V {
		self.sink
	}
	
	pub fn weight(&self) -> W{
		self.weight
	}
}

