use super::*;

pub trait Unweighted<V,Vi,Ei>: BaseGraph<Vertex=V,Weight=(),VertexIter=Vi,EdgeIter=Ei>
	where
		V: Copy + Eq,
		Vi: IntoIterator<Item=V>,
		Ei: IntoIterator<Item=BaseEdge<V,()>>
{}

