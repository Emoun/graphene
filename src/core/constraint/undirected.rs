use super::*;

pub trait Undirected<V,W,Vi,Ei>: BaseGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
{}