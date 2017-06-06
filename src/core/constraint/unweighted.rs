use super::*;

pub trait Unweighted<V,Vi,Ei>: BaseGraph<Vertex=V,Weight=(),VertexIter=Vi,EdgeIter=Ei>
	where
		V: Vertex,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,()>
{}

