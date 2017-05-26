
use graph::BaseGraph;

pub trait GraphBinding<G>{
	
	fn graph(self) -> G;
}

pub trait CollectorBinding<V,G> : GraphBinding<G>{
	
	fn iter(&self) -> Iterator<Item=(V)>;
	
}

pub trait Mutable<'a,G,V,E,Vc,Ec,>: BaseGraph<'a,
	V,
	Vc,
	Ec,
>
	where
		G:	Mutable<'a,G,V,E,Vc,Ec,>,
		V: 	Clone + Eq,
		Vc:	CollectorBinding<V,G>,
		Ec:	CollectorBinding<E,G>,
{
	fn get_mutable(self) -> G;
	
	fn get_immutable(self) -> Self;
	
	fn add_vertex(self, v: V) -> G;
	
}