use std::ops::{Deref, DerefMut};

trait Graph {
	fn graph_fn(&self) -> u32 ;
}
trait GraphMut: Graph {
	fn graph_mut(&mut self) -> &mut u32;
}

trait BaseConstraint: Sized {
	type Graph: Graph;
	fn get_graph(&self) -> &Self::Graph;
}
trait BaseConstraintMut: BaseConstraint
{
	type GraphMut: GraphMut;
	fn get_graph_mut(&mut self) -> &mut Self::GraphMut;
}
trait Constraint: Sized
{
	type Base:  BaseConstraint;
	type Constrained: Constraint<Base=Self::Base>;
	
	fn base_single(&self) -> &Self::Constrained;
	fn unconstrain_single(self) -> Self::Constrained;
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>;
	
	fn unconstrain(self) -> Self::Base{
		self.unconstrain_single().unconstrain()
	}
	fn constrain(g: Self::Base) -> Result<Self, ()>{
		Self::constrain_single(Self::Constrained::constrain(g)?)
	}
	fn base(&self) -> &<Self::Base as BaseConstraint>::Graph {
		self.base_single().base()
	}
	
}
trait ConstraintMut: Constraint
{
	type BaseMut:  BaseConstraintMut;
	type ConstrainedMut: ConstraintMut<BaseMut=Self::BaseMut>;
	fn base_single_mut(&mut self) -> &mut Self::ConstrainedMut;
	fn base_mut(&mut self) -> &mut <Self::BaseMut as BaseConstraintMut>::GraphMut {
		self.base_single_mut().base_mut()
	}
}

impl<B: BaseConstraint> Constraint for B{
	type Base = Self;
	type Constrained = Self;
	
	fn base_single(&self) -> &Self::Constrained { &self }
	fn unconstrain_single(self) -> Self::Constrained { self }
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> { Ok(g)}
	
	fn unconstrain(self) -> Self::Base { self }
	fn constrain(g: Self::Base) -> Result<Self, ()>{ Ok(g) }
	fn base(&self) -> &<Self::Base as BaseConstraint>::Graph { self.get_graph() }
}
impl<B: BaseConstraintMut> ConstraintMut for B{
	type BaseMut = Self;
	type ConstrainedMut = Self;
	fn base_single_mut(&mut self) -> &mut Self::Constrained { self }
	fn base_mut(&mut self) -> &mut <Self::BaseMut as BaseConstraintMut>::GraphMut {
		self.get_graph_mut()
	}
}

impl<G: Graph, D: Deref<Target=G>> BaseConstraint for D
{
	type Graph = G;
	fn get_graph(&self) -> &Self::Graph {&**self}
}
impl<G: GraphMut, D: DerefMut<Target=G>> BaseConstraintMut for D
{
	type GraphMut = G;
	fn get_graph_mut(&mut self) -> &mut Self::GraphMut {&mut **self}
}

struct BaseGraph(u32);
impl Graph for BaseGraph{
	fn graph_fn(&self) -> u32 {
		self.0
	}
}
impl GraphMut for BaseGraph {
	fn graph_mut(&mut self) -> &mut u32 { &mut self.0 }
}
impl BaseConstraint for BaseGraph {
	type Graph = Self;
	fn get_graph(&self) -> &Self::Graph {self}
}
impl BaseConstraintMut for BaseGraph{
	type GraphMut = Self;
	fn get_graph_mut(&mut self) -> &mut Self::GraphMut {self}
}

struct Connected<C: Constraint>(C);
impl<C: Constraint> Graph for Connected<C>{
	fn graph_fn(&self) -> u32 {
		self.base().graph_fn()
	}
}
impl<C: ConstraintMut> GraphMut for Connected<C>
{
	fn graph_mut(&mut self) -> &mut u32 { self.0.base_mut().graph_mut() }
}
impl<C: Constraint> Constraint for Connected<C>
{
	type Base = C::Base;
	type Constrained = C;
	
	fn base_single(&self) -> &Self::Constrained { &self.0 }
	fn unconstrain_single(self) -> Self::Constrained { self.0 }
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> {
		if g.base().graph_fn() < 32 {
			Ok(Self(g))
		} else {
			Err(())
		}
		
	}
}
impl<C: ConstraintMut> ConstraintMut for Connected<C>
{
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
	
	fn base_single_mut(&mut self) -> &mut Self::Constrained { &mut self.0 }
}

#[test]
fn test(){
	let mut g = BaseGraph(16);
	assert_eq!(g.graph_fn(), 16);
	
	let c_ref = Connected::constrain_single(&g).unwrap();
	assert_eq!(c_ref.graph_fn(), 16);
	
	let mut c_ref_mut = Connected::constrain_single(&mut g).unwrap();
	*c_ref_mut.graph_mut() = 30;
	assert_eq!(c_ref_mut.graph_fn(), 30);
	
	let c_owned = Connected::constrain_single(g).unwrap();
	assert_eq!(c_owned.graph_fn(), 30);
	
}