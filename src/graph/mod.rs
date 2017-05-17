use std::collections::HashSet;


pub trait Sourced<V> {
	fn source(&self) -> V;
}

pub trait Sinked<V> {
	fn sink(&self) -> V;
}

pub trait Weighted<W> {
	fn weight(&self) -> W;
}


pub trait Graph {
	type Vertex: Clone;
	type Weight: Clone;
	type Edge: Sourced<Self::Vertex> + Sinked<Self::Vertex> + Weighted<Self::Weight>;
	
	fn number_of_vertices(&self) -> usize;
	
	fn number_of_edges(&self) -> usize;
	
	fn all_vertices(&self) -> HashSet<Self::Vertex>;
	
	fn all_edges(&self) -> Vec<Self::Edge>;
	
	fn outgoing_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Vertex>, ()>;
	
	fn incoming_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Vertex>, ()>;
	
	fn edges_between(&self, source: &Self::Vertex, sink: &Self::Vertex) -> Result<Vec<Self::Weight>, ()>;
}

