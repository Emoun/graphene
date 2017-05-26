
pub trait Sourced<V> {
	fn source(&self) -> V;
}

pub trait Sinked<V> {
	fn sink(&self) -> V;
}

pub trait Weighted<W> {
	fn weight(&self) -> &W;
}

pub trait BaseGraph<'a,
	Vertex,
	VertexCollector,
	EdgeCollector,
>
where
	Vertex : Clone + Eq
{
	fn all_vertices(&'a self) -> VertexCollector;
	
	fn all_edges(&'a self) -> EdgeCollector;
}

