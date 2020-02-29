use crate::core::{Graph, GraphDeref};
use std::ops::Deref;

/// A marker trait that specifies that the type is a base implementation of a
/// graph with fixed properties that cannot be removed.
///
/// This can be further insured, but cannot be released.
/// `Insure` is automatically implemented for any graph that implements
/// this trait, however `Insure`'s methods do nothing, returning the same
/// object. Conceptually, a base graph is its own insurer.
///
/// NOTE: When specialization is supported, Insure should not be
/// implemented for any type implementing `BaseGraph`. (currently not possible,
/// since it will result in multiple implementations. Specialization will make
/// it possible.)
pub trait BaseGraph: Sized + GraphDeref
{
	fn insure_all<G>(self) -> Result<G, ()>
	where
		G: Insure<Base = Self>,
	{
		G::insure_all(self)
	}
}
/// An implementing type insures a base graph implementation.
///
/// Multiple levels of insurers are supported.
pub trait Insure: Release
{
	fn insure_unvalidated(c: Self::Insured) -> Self;

	fn validate(c: &Self::Insured) -> bool;

	/// insures a graph that is also already insured.
	fn insure(c: Self::Insured) -> Result<Self, ()>
	{
		if Self::validate(&c)
		{
			Ok(Self::insure_unvalidated(c))
		}
		else
		{
			Err(())
		}
	}

	/// Fully insures a base graph implementation type with all
	/// levels of properties.
	fn insure_all(g: Self::Base) -> Result<Self, ()>
	{
		Self::insure(Self::Insured::insure_all(g)?)
	}
}

pub trait Release: Sized + GraphDeref
{
	/// The base graph implementation being insured
	type Base: BaseGraph;

	/// The next level of properties.
	type Insured: Insure<Base = Self::Base>;

	/// Release only this level's properties, maintaining
	/// the next level's properties
	fn release(self) -> Self::Insured;

	/// Fully release this type, returning the base graph implementation
	/// type
	fn release_all(self) -> Self::Base
	{
		self.release().release_all()
	}
}

impl<G: Graph, D: Deref<Target = G>> BaseGraph for D {}
impl<B: BaseGraph> Insure for B
{
	fn insure_unvalidated(c: Self::Insured) -> Self
	{
		c
	}

	fn validate(_c: &Self::Insured) -> bool
	{
		true
	}

	fn insure_all(g: Self::Base) -> Result<Self, ()>
	{
		Self::insure(g)
	}
}
impl<B: BaseGraph> Release for B
{
	type Base = Self;
	type Insured = Self;

	fn release(self) -> Self::Insured
	{
		self
	}

	fn release_all(self) -> Self::Base
	{
		self
	}
}
