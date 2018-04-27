
use core::{BaseGraph, Edge};

pub trait AutoEdgeGraph: BaseGraph
{
	
	fn add_edge<E>(&mut self, e: E) -> Result<(),()>
		where E: Edge<Self::Vertex, ()>;
	
	
}