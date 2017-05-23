
pub trait Sourced<V> {
	fn source(&self) -> &V;
}

pub trait Sinked<V> {
	fn sink(&self) -> &V;
}

pub trait Weighted<W> {
	fn weight(&self) -> &W;
}


pub trait Graph<'a> {
	type Vertex: Clone;
	type Edge: Sourced<Self::Vertex> + Sinked<Self::Vertex>;
	type Outgoing: Sinked<Self::Vertex>;
	type Incoming: Sourced<Self::Vertex>;
	
	fn vertex_count(&'a self) -> usize;
	
	fn edge_count(&'a self) -> usize;
	
	fn all_vertices(&'a self) -> Vec<&Self::Vertex>;
	
	fn all_edges(&'a self) -> Vec<Self::Edge>;
	
	fn outgoing_edges(&'a self, v: &'a Self::Vertex) -> Result<Vec<Self::Outgoing>, ()>;
	
	fn incoming_edges(&'a self, v: &'a Self::Vertex) -> Result<Vec<Self::Incoming>, ()>;
	
	fn edges_between(&'a self, v1: &'a Self::Vertex, v2: &'a Self::Vertex) -> Result<Vec<Self::Edge>,()>;
	
}

pub trait Mutating<'a, G> where G: 'a{
	type Vertex: Clone;
	type Edge: Sourced<Self::Vertex> + Sinked<Self::Vertex>;
	
	fn add_vertex(self, v: Self::Vertex) -> Result<G,(G, Self::Vertex)>;
	
	fn remove_vertex(self, v: &'a Self::Vertex)
		-> Result<(G, Self::Vertex), (G, &'a Self::Vertex)>;
	
	fn add_edge(self, e: Self::Edge) -> Result<(G, Self::Edge),(G, Self::Edge)>;
	
	fn remove_edge(self, e: Self::Edge) -> Result<G,(G, Self::Edge)>;
	
}
