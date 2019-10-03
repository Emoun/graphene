use std::ops::{Deref, DerefMut};

trait Graph {
	type R: PartialOrd<u32>;
	fn graph_fn<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Self::R>> ;
}
trait GraphMut: Graph {
	fn graph_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=&'a mut Self::R>>;
}
trait GraphMut2: Graph {
	fn graph_mut2(&mut self) -> &mut Self::R;
}

trait ImplGraph {
	type Graph: Graph;
	fn get_graph(&self) -> &Self::Graph;
}
trait ImplGraphMut: ImplGraph {
	fn get_graph_mut(&mut self) -> &mut Self::Graph;
}
trait BaseConstraint: Sized + ImplGraph {
	fn constrain<G>(self) -> Result<G, ()>
		where G: Constraint<Base=Self>
	{
		G::constrain(self)
	}
}
trait BaseConstraintMut: BaseConstraint + ImplGraphMut {}
trait Constraint: Sized  + ImplGraph{
	type Base:  BaseConstraint;
	type Constrained: Constraint<Base=Self::Base>;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>;
	fn unconstrain_single(self) -> Self::Constrained;
	
	fn constrain(g: Self::Base) -> Result<Self, ()>{
		println!("Default constrain()");
		Self::constrain_single(Self::Constrained::constrain(g)?)
	}
	fn unconstrain(self) -> Self::Base{
		self.unconstrain_single().unconstrain()
	}
	
}
trait ConstraintMut: Constraint<Base=<Self as ConstraintMut>::BaseMut, Constrained=<Self as ConstraintMut>::ConstrainedMut> + ImplGraphMut {
	type BaseMut:  BaseConstraintMut;
	type ConstrainedMut: ConstraintMut<BaseMut=Self::BaseMut>;
}

impl<G: Graph, D: Deref<Target=G>> ImplGraph for D {
	type Graph = G;
	fn get_graph(&self) -> &Self::Graph { println!("ImplGraph for Deref get_Graph()"); &**self}
}
impl<G: Graph, D: DerefMut<Target=G>> ImplGraphMut for D {
	fn get_graph_mut(&mut self) -> &mut Self::Graph {&mut **self}
}
impl<G: Graph, D: Deref<Target=G>> BaseConstraint for D {}
impl<G: GraphMut, D: DerefMut<Target=G>> BaseConstraintMut for D {}
impl<B: BaseConstraint> Constraint for B {
	type Base = Self;
	type Constrained = Self;

	fn unconstrain_single(self) -> Self::Constrained { self }
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> { println!("Constraint for BaseConstraint constrain_single()");Ok(g)}

	fn unconstrain(self) -> Self::Base { self }
	fn constrain(g: Self::Base) -> Result<Self, ()>{ println!("Constraint for BaseConstraint constrain()");Ok(g) }
}
impl<B: BaseConstraintMut> ConstraintMut for B {
	type BaseMut = Self;
	type ConstrainedMut = Self;
}

struct BaseGraph(u32);
impl BaseGraph {
	fn base_graph_fn(&self) -> u32
	{
		self.0
	}
}
impl Graph for BaseGraph{
	type R = u32;
	fn graph_fn<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a u32>> {
		Box::new(vec![&self.0].into_iter())
	}
}
impl GraphMut for BaseGraph {
	fn graph_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=&'a mut u32>> {
		Box::new(vec![&mut self.0].into_iter())
	}
}
impl GraphMut2 for BaseGraph {
	fn graph_mut2(&mut self) -> &mut u32 { &mut self.0 }
}
impl ImplGraph for BaseGraph {
	type Graph = Self;
	
	fn get_graph(&self) -> &Self::Graph {
		self
	}
}
impl ImplGraphMut for BaseGraph {
	fn get_graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl BaseConstraint for BaseGraph {}
impl BaseConstraintMut for BaseGraph{}

struct Connected<C: Constraint>(C);
impl<C: Constraint> ImplGraph for Connected<C> {
	type Graph = Self;
	
	fn get_graph(&self) -> &Self::Graph {
		println!("ImplGraph for Connected get_graph()");
		self
	}
}
impl<C: Constraint> ImplGraphMut for Connected<C>  {
	fn get_graph_mut(&mut self) -> &mut Self::Graph {
		self
	}
}
impl<C: Constraint> Constraint for Connected<C> {
	type Base = C::Base;
	type Constrained = C;

	fn unconstrain_single(self) -> Self::Constrained { self.0 }
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> {
		println!("Constraint for Connected constrain_single()");
		if g.get_graph().graph_fn().next().unwrap() < &32 {
			Ok(Self(g))
		} else {
			Err(())
		}

	}
}
impl<C: ConstraintMut> ConstraintMut for Connected<C> {
	type BaseMut = C::BaseMut;
	type ConstrainedMut = C;
}

impl<C: Constraint> Connected<C> {
	fn connected_fn(&self) -> &<C::Graph as Graph>::R
	{
		self.get_graph().graph_fn().next().unwrap()
	}
}
impl<C: Constraint> Graph for Connected<C>{
	type R = <C::Graph as Graph>::R;
	fn graph_fn<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Self::R>> {
		println!("Graph for Connected graph_fn()");
		self.0.get_graph().graph_fn()
	}
}
impl<C: ConstraintMut> GraphMut for Connected<C>
	where C::Graph: GraphMut {
	fn graph_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
		&'a mut <C::Graph as Graph>::R>>
	{
		self.0.get_graph_mut().graph_mut()
	}
}
impl<C: ConstraintMut> GraphMut2 for Connected<C>
	where C::Graph: GraphMut2 {
	fn graph_mut2(&mut self) -> &mut <C::Graph as Graph>::R {
		self.0.get_graph_mut().graph_mut2()
	}
}

#[test]
fn constrainer_constraining_base(){
	type ConstrainedGraphRef<'a> =
	Connected<
		Connected<
			&'a BaseGraph
		>>;

	let mut g = BaseGraph(16);
	assert_eq!(g.graph_fn().next().unwrap(), &16);

	let c_ref = ConstrainedGraphRef::constrain(&g).unwrap();
	assert_eq!(c_ref.connected_fn(), &16);

	type ConstrainedGraphMut<'a> =
	Connected<
		Connected<
			&'a mut BaseGraph
		>>;

	let mut c_ref_mut = ConstrainedGraphMut::constrain(&mut g).unwrap();
	*(c_ref_mut.graph_mut().next().unwrap()) = 30;
	*c_ref_mut.graph_mut2() = 30;
	assert_eq!(c_ref_mut.connected_fn(), &30);

	type ConstrainedGraph<'a> =
	Connected<
		Connected<
			BaseGraph
		>>;

	let c_owned = ConstrainedGraph::constrain(g).unwrap();
	assert_eq!(c_owned.connected_fn(), &30);

}

#[test]
fn inline_constrainer_constraining_base(){

	let mut g = BaseGraph(16);
	assert_eq!(g.graph_fn().next().unwrap(), &16);

	let c_ref = <Connected<Connected<&BaseGraph>>>::constrain(&g).unwrap();
	assert_eq!(c_ref.connected_fn(), &16);

	let mut c_ref_mut = <Connected<Connected<&mut BaseGraph>>>::constrain(&mut g).unwrap();
	*c_ref_mut.graph_mut().next().unwrap() = 30;
	assert_eq!(c_ref_mut.connected_fn(), &30);

	let c_owned = <Connected<Connected<BaseGraph>>>::constrain(g).unwrap();
	assert_eq!(c_owned.connected_fn(), &30);

}

#[test]
fn base_constrains_self_by_constraint_inference(){
	type ConstrainedGraph<G> = Connected< Connected< Connected<G> > >;

	let mut g = BaseGraph(16);
	assert_eq!(g.graph_fn().next().unwrap(), &16);

	let c_ref: ConstrainedGraph<&BaseGraph> = (&g).constrain().unwrap();
	assert_eq!(c_ref.connected_fn(), &16);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_eq!(c_ref_unc.connected_fn(), &16);

	let mut c_ref_mut: ConstrainedGraph<&mut BaseGraph> = (&mut g).constrain().unwrap();
	*c_ref_mut.graph_mut().next().unwrap() = 30;
	assert_eq!(c_ref_mut.connected_fn(), &30);
	let mut c_ref_mut_unc = c_ref_mut.unconstrain_single();
	assert_eq!(c_ref_mut_unc.connected_fn(), &30);
	*c_ref_mut_unc.graph_mut().next().unwrap() = 31;
	assert_eq!(c_ref_mut_unc.connected_fn(), &31);

	let c_owned: ConstrainedGraph<BaseGraph> = g.constrain().unwrap();
	assert_eq!(c_owned.connected_fn(), &31);
	let c_owned_unc = c_owned.unconstrain_single();
	assert_eq!(c_owned_unc.connected_fn(), &31);
	let g2 = c_owned_unc.unconstrain();
	assert_eq!(g2.base_graph_fn(), 31);
}

#[test]
fn base_constrains_self_by_inline_constraint_inference(){

	let mut g = BaseGraph(16);
	assert_eq!(g.graph_fn().next().unwrap(), &16);

	let c_ref: Connected<Connected<Connected<&BaseGraph>>> = (&g).constrain().unwrap();
	assert_eq!(c_ref.connected_fn(), &16);
	let c_ref_unc = c_ref.unconstrain_single();
	assert_eq!(c_ref_unc.connected_fn(), &16);

	let mut c_ref_mut: Connected<Connected<Connected<&mut BaseGraph>>> = (&mut g).constrain().unwrap();
	*c_ref_mut.graph_mut().next().unwrap() = 30;
	assert_eq!(c_ref_mut.connected_fn(), &30);
	let mut c_ref_mut_unc = c_ref_mut.unconstrain_single();
	assert_eq!(c_ref_mut_unc.connected_fn(), &30);
	*c_ref_mut_unc.graph_mut().next().unwrap() = 31;
	assert_eq!(c_ref_mut_unc.connected_fn(), &31);

	let c_owned: Connected<Connected<Connected<BaseGraph>>> = g.constrain().unwrap();
	assert_eq!(c_owned.connected_fn(), &31);
	let c_owned_unc = c_owned.unconstrain_single();
	assert_eq!(c_owned_unc.connected_fn(), &31);
	let g2 = c_owned_unc.unconstrain();
	assert_eq!(g2.base_graph_fn(), 31);
}