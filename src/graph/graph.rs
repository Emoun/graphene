
pub trait Sourced<V> {
	fn source(&self) -> V;
}

pub trait Sinked<V> {
	fn sink(&self) -> V;
}

pub trait Weighted<W> {
	fn weight(&self) -> &W;
}

pub trait FineGrainedGraph<'a,
	Vertex,
	VertexCollector,
	EdgeCollector,
	OutgoingCollector,
	IncomingCollector,
>
where
	Vertex : Clone,
{
	
	fn vertex_count(&'a self) -> usize;
	
	fn edge_count(&'a self) -> usize;
	
	fn all_vertices(&'a self) -> VertexCollector;
	
	fn all_edges(&'a self) -> EdgeCollector;
	
	fn outgoing_edges(&'a self, v: Vertex) -> Result<OutgoingCollector, ()>;
	
	fn incoming_edges(&'a self, v: Vertex) -> Result<IncomingCollector, ()>;
	
	fn edges_between(&'a self, v1: Vertex, v2: Vertex) -> Result<EdgeCollector,()>;
	
}

pub trait Graph<'a,V,E,O,I> : FineGrainedGraph<'a,
	V,
	Vec<V>,
	Vec<E>,
	Vec<O>,
	Vec<I>,
>
	where
		V: Clone,
		E: Sourced<V> + Sinked<V>,
		O: Sinked<V>,
		I: Sourced<V>,
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
{
	fn valid_ref(&self, v: &'a V) -> bool;
	
}
