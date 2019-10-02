use crate::core::{Graph, GraphMut};
use std::ops::{Deref, DerefMut};

///
/// A marker trait that specifies that the type is a base implementation of a graph
/// with fixed constraints that cannot be removed.
///
/// This can be further constrained, but cannot be unconstrained.
/// `Constrainer` is automatically implemented for any graph that implments this trait,
/// however `Constrainer`'s methods do nothing, returning the same object.
/// Conceptually, a base graph is its own constrainer.
///
/// NOTE: When specialization is supported, Constrainer should not be implemented for
/// any type implementing `BaseGraph`. (currently not possible, since it will result in multiple
/// implementations. Specialization will make it possible.)
///
pub trait BaseGraph: Sized
{
	type Graph: Graph;
	
	fn graph(&self) -> &Self::Graph;
	fn constrain<G>(self) -> Result<G, ()>
		where G: Constrainer<Base=Self>
	{
		G::constrain(self)
	}
}
pub trait BaseGraphMut: BaseGraph {
	fn graph_mut(&mut self) -> &mut Self::Graph;
}

///
/// An implementing type constrains a base graph implementation.
///
/// Multiple levels of constrainers are supported.
///
pub trait Constrainer: Sized
{
	///
	/// The base graph implementation being constrained
	///
	type Base: BaseGraph;
	
	///
	/// The next level of constraints.
	///
	type Constrained: Constrainer<Base=Self::Base>;
	
	///
	/// Constrains a graph that is also already constrained.
	///
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>;
	
	fn constrained(&self) -> &Self::Constrained;
	
	///
	/// Unconstrain only this level's constraints, maintaining
	/// the next level's constraints
	///
	fn unconstrain_single(self) -> Self::Constrained;
	
	
	///
	/// Fully constrains a base graph implementation type with all
	/// levels of constraints.
	///
	fn constrain(g: Self::Base) -> Result<Self, ()>
	{
		Self::constrain_single(Self::Constrained::constrain(g)?)
	}
	
	fn base(&self) -> &<Self::Base as BaseGraph>::Graph {
		self.constrained().base()
	}
	
	///
	/// Fully unconstrains this type, returning the base graph implementation type
	///
	fn unconstrain(self) -> Self::Base {
		self.unconstrain_single().unconstrain()
	}
}
pub trait ConstrainerMut: Constrainer<Base=<Self as ConstrainerMut>::BaseMut,
	Constrained=<Self as ConstrainerMut>::ConstrainedMut>
{
	type BaseMut:  BaseGraphMut;
	type ConstrainedMut: ConstrainerMut<BaseMut=Self::BaseMut>;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut;
	fn base_mut(&mut self) -> &mut <Self::BaseMut as BaseGraph>::Graph
	{
		self.constrained_mut().base_mut()
	}
}

impl<G: Graph, D: Deref<Target=G>> BaseGraph for D {
	type Graph = G;
	
	fn graph(&self) -> &Self::Graph {
		&**self
	}
}
impl<G: GraphMut, D: DerefMut<Target=G>> BaseGraphMut for D
{
	fn graph_mut(&mut self) -> &mut Self::Graph {
		&mut **self
	}
}
impl<B: BaseGraph> Constrainer for B
{
	type Base = Self;
	type Constrained = Self;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()> {
		Ok(g)
	}
	
	fn constrained(&self) -> &Self::Constrained {
		&self
	}
	
	fn unconstrain_single(self) -> Self::Constrained {
		self
	}
	
	fn constrain(g: Self::Base) -> Result<Self, ()> {
		Ok(g)
	}
	
	fn base(&self) -> &<Self::Base as BaseGraph>::Graph {
		self.graph()
	}
	
	fn unconstrain(self) -> Self::Base
	{
		self
	}
}
impl<B: BaseGraphMut> ConstrainerMut for B
{
	type BaseMut = Self;
	type ConstrainedMut = Self;
	
	fn constrained_mut(&mut self) -> &mut Self::ConstrainedMut {
		self
	}
	fn base_mut(&mut self) -> &mut <Self::BaseMut as BaseGraph>::Graph {
		self.graph_mut()
	}
}

