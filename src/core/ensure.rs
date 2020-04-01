use crate::core::{Graph, GraphDeref};
use std::ops::Deref;

/// A marker trait that specifies that the type is a base implementation of a
/// graph with fixed properties that cannot be removed.
///
/// This can be further ensured, but cannot be released.
/// `Ensure` is automatically implemented for any graph that implements
/// this trait, however `Ensure`'s methods do nothing, returning the same
/// object. Conceptually, a base graph is its own ensurer.
///
/// NOTE: When specialization is supported, Ensure should not be
/// implemented for any type implementing `BaseGraph`. (currently not possible,
/// since it will result in multiple implementations. Specialization will make
/// it possible.)
pub trait BaseGraph: Sized + GraphDeref
{
	fn ensure_all<G>(self) -> Result<G, ()>
	where
		G: Ensure<Base = Self>,
	{
		G::ensure_all(self)
	}
}
/// An implementing type ensures a base graph implementation.
///
/// Multiple levels of ensurers are supported.
pub trait Ensure: Release
{
	fn ensure_unvalidated(c: Self::Ensured) -> Self;

	fn validate(c: &Self::Ensured) -> bool;

	/// ensures a graph that is also already ensured.
	fn ensure(c: Self::Ensured) -> Result<Self, ()>
	{
		if Self::validate(&c)
		{
			Ok(Self::ensure_unvalidated(c))
		}
		else
		{
			Err(())
		}
	}

	/// Fully ensures a base graph implementation type with all
	/// levels of properties.
	fn ensure_all(g: Self::Base) -> Result<Self, ()>
	{
		Self::ensure(Self::Ensured::ensure_all(g)?)
	}
}

pub trait Release: Sized + GraphDeref
{
	/// The base graph implementation being ensured
	type Base: BaseGraph;

	/// The next level of properties.
	type Ensured: Ensure<Base = Self::Base>;

	/// Release only this level's properties, maintaining
	/// the next level's properties
	fn release(self) -> Self::Ensured;

	/// Fully release this type, returning the base graph implementation
	/// type
	fn release_all(self) -> Self::Base
	{
		self.release().release_all()
	}
}

impl<G: Graph, D: Deref<Target = G>> BaseGraph for D {}
impl<B: BaseGraph> Ensure for B
{
	fn ensure_unvalidated(c: Self::Ensured) -> Self
	{
		c
	}

	fn validate(_c: &Self::Ensured) -> bool
	{
		true
	}

	fn ensure_all(g: Self::Base) -> Result<Self, ()>
	{
		Self::ensure(g)
	}
}
impl<B: BaseGraph> Release for B
{
	type Base = Self;
	type Ensured = Self;

	fn release(self) -> Self::Ensured
	{
		self
	}

	fn release_all(self) -> Self::Base
	{
		self
	}
}
