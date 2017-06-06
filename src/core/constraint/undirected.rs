use super::*;

pub trait Undirected<V,W,Vi,Ei>: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
	where
		V: Copy + Eq,
		W: Copy + Eq,
		Vi: IntoIterator<Item=V>,
		Ei: IntoIterator<Item=BaseEdge<V,W>>
{}