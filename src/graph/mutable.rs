
use graph::FineGrainedGraph;

pub trait GraphBinding<G>{
	
	fn graph(self) -> G;
}

pub trait CollectorBinding<V,G> : GraphBinding<G>{
	
	fn iter(&self) -> Iterator<Item=(V)>;
	
}

pub trait Mutable<'a,G,V,E,O,I,Vc,Ec,Oc,Ic>: FineGrainedGraph<'a,
	V,
	Vc,
	Ec,
	Oc,
	Ic,
>
	where
		G:	Mutable<'a,G,V,E,O,I,Vc,Ec,Oc,Ic>,
		V: 	Clone + Eq,
		Vc:	CollectorBinding<V,G>,
		Ec:	CollectorBinding<E,G>,
		Oc:	CollectorBinding<O,G>,
		Ec:	CollectorBinding<I,G>,
{
	fn get_mutable(self) -> G;
	
	fn get_immutable(self) -> Self;
	
	fn add_vertex(self, v: V) -> G;
	
}