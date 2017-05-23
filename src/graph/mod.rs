
pub trait Sourced<V> {
	fn source(&self) -> V;
}

pub trait Sinked<V> {
	fn sink(&self) -> V;
}

pub trait Weighted<W> {
	fn weight(&self) -> &W;
}


pub trait FineGrainedGraph<'a>{
	type Vertex: Clone;
	type VertexCollector;
	type EdgeCollector;
	type OutgoingCollector;
	type IncomingCollector;
	
	fn vertex_count(&'a self) -> usize;
	
	fn edge_count(&'a self) -> usize;
	
	fn all_vertices(&'a self) -> Self::VertexCollector;
	
	fn all_edges(&'a self) -> Self::EdgeCollector;
	
	fn outgoing_edges(&'a self, v: Self::Vertex) -> Result<Self::OutgoingCollector, ()>;
	
	fn incoming_edges(&'a self, v: Self::Vertex) -> Result<Self::IncomingCollector, ()>;
	
	fn edges_between(&'a self, v1: Self::Vertex, v2: Self::Vertex) -> Result<Self::EdgeCollector,()>;
	
}

pub trait Graph<'a,V,E,O,I> : FineGrainedGraph<'a,
	Vertex= 			V,
	VertexCollector=	Vec<V>,
	EdgeCollector =  	Vec<E>,
	OutgoingCollector = Vec<O>,
	IncomingCollector = Vec<I>,
>
where
	V: Clone,
	E: Sourced<Self::Vertex> + Sinked<Self::Vertex>,
	O: Sinked<Self::Vertex>,
	I: Sourced<Self::Vertex>,
{
}

pub trait StableGraph<'a,V,E,O,I> : Graph<'a,
	&'a V,
	E,
	O,
	I,
>
where
	V: 'a,
	E: Sourced<&'a V> + Sinked<&'a V>,
	O: Sinked<&'a V>,
	I: Sourced<&'a V>,
{}
/*
pub trait Mutable<'a,V,E,O,I>: StableGraph<'a,V,E,O,I>
where
	V:,
	E:,
	O:,
	E:,
{



}
*/