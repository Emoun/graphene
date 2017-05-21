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
	type Edge: Sourced<Self::Vertex> + Sinked<Self::Vertex>;
	type Outgoing: Sinked<Self::Vertex>;
	type Incoming: Sourced<Self::Vertex>;
	
	
	fn number_of_vertices(&self) -> usize;
	
	fn number_of_edges(&self) -> usize;
	
	fn all_vertices(&self) -> HashSet<Self::Vertex>;
	
	fn all_edges(&self) -> Vec<Self::Edge>;
	
	fn outgoing_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Outgoing>, ()>;
	
	fn incoming_edges(&self, v: &Self::Vertex) -> Result<Vec<Self::Incoming>, ()>;
	
	fn edges_between(&self, v1: &Self::Vertex, v2: &Self::Vertex) -> Result<Vec<Self::Edge>,()>;
	
}

pub trait Mutating<G>{
	type Vertex: Clone;
	type Edge: Sourced<Self::Vertex> + Sinked<Self::Vertex>;
	
	fn add_vertex(self, v: Self::Vertex) -> (G,bool);
	
	fn remove_vertex(self, v: Self::Vertex) -> (G,bool);
	
	fn add_edge(self, e: Self::Edge) -> (G, bool);
	
	fn remove_edge(self, e: Self::Edge) -> (G, bool);
	
}

