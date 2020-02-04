use crate::core::Graph;
use std::ops::{Deref, DerefMut};

pub trait ImplGraph
{
	type Graph: Graph;
	fn graph(&self) -> &Self::Graph;
}
pub trait ImplGraphMut: ImplGraph
{
	fn graph_mut(&mut self) -> &mut Self::Graph;
}
/// A marker trait that specifies that the type is a base implementation of a
/// graph with fixed constraints that cannot be removed.
///
/// This can be further constrained, but cannot be unconstrained.
/// `Constrainer` is automatically implemented for any graph that implements
/// this trait, however `Constrainer`'s methods do nothing, returning the same
/// object. Conceptually, a base graph is its own constrainer.
///
/// NOTE: When specialization is supported, Constrainer should not be
/// implemented for any type implementing `BaseGraph`. (currently not possible,
/// since it will result in multiple implementations. Specialization will make
/// it possible.)
pub trait BaseGraph: Sized + ImplGraph
{
	fn constrain<G>(self) -> Result<G, ()>
	where
		G: Constrainer<Base = Self>,
	{
		G::constrain(self)
	}
}
/// An implementing type constrains a base graph implementation.
///
/// Multiple levels of constrainers are supported.
pub trait Constrainer: Sized + ImplGraph
{
	/// The base graph implementation being constrained
	type Base: BaseGraph;

	/// The next level of constraints.
	type Constrained: Constrainer<Base = Self::Base>;

	/// Constrains a graph that is also already constrained.
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>;

	/// Unconstrain only this level's constraints, maintaining
	/// the next level's constraints
	fn unconstrain_single(self) -> Self::Constrained;

	/// Fully constrains a base graph implementation type with all
	/// levels of constraints.
	fn constrain(g: Self::Base) -> Result<Self, ()>
	{
		Self::constrain_single(Self::Constrained::constrain(g)?)
	}

	/// Fully unconstrains this type, returning the base graph implementation
	/// type
	fn unconstrain(self) -> Self::Base
	{
		self.unconstrain_single().unconstrain()
	}
}

impl<G: Graph, D: Deref<Target = G>> ImplGraph for D
{
	type Graph = G;

	fn graph(&self) -> &Self::Graph
	{
		&**self
	}
}
impl<G: Graph, D: DerefMut<Target = G>> ImplGraphMut for D
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		&mut **self
	}
}
impl<G: Graph, D: Deref<Target = G>> BaseGraph for D {}
impl<B: BaseGraph> Constrainer for B
{
	type Base = Self;
	type Constrained = Self;

	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>
	{
		Ok(g)
	}

	fn unconstrain_single(self) -> Self::Constrained
	{
		self
	}

	fn constrain(g: Self::Base) -> Result<Self, ()>
	{
		Ok(g)
	}

	fn unconstrain(self) -> Self::Base
	{
		self
	}
}
