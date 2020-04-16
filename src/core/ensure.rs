use crate::core::{Graph, GraphDeref};
use std::ops::Deref;

pub trait Payload<B>
{
	type Item;
	fn split(self) -> (Self::Item, B);

	fn new(a: Self::Item, b: B) -> Self;
}
impl<A> Payload<A> for A
{
	type Item = ();

	fn split(self) -> ((), A)
	{
		((), self)
	}

	fn new(_: (), a: A) -> Self
	{
		a
	}
}
impl<A, B> Payload<B> for (A, B)
{
	type Item = A;

	fn split(self) -> (A, B)
	{
		self
	}

	fn new(a: A, b: B) -> Self
	{
		(a, b)
	}
}

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
	fn ensure_all<G>(self, p: G::Payload) -> Result<G, ()>
	where
		G: Ensure<Base = Self>,
	{
		G::ensure_all(self, p)
	}
}
pub trait BaseGraphUnloaded: BaseGraph
{
	fn ensure_all<G>(self) -> Result<G, ()>
	where
		G: Ensure<Base = Self>,
		G::Payload: Payload<(), Item = ()>,
	{
		G::ensure_all(self, <G::Payload>::new((), ()))
	}
}

/// An implementing type ensures a base graph implementation.
///
/// Multiple levels of ensurers are supported.
pub trait Ensure: Release
{
	fn ensure_unvalidated(
		c: Self::Ensured,
		p: <Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	) -> Self;

	fn validate(
		c: &Self::Ensured,
		p: &<Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	) -> bool;

	fn ensure(
		c: Self::Ensured,
		p: <Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	) -> Result<Self, ()>
	{
		if Self::validate(&c, &p)
		{
			Ok(Self::ensure_unvalidated(c, p))
		}
		else
		{
			Err(())
		}
	}

	fn ensure_all(g: Self::Base, p: Self::Payload) -> Result<Self, ()>
	{
		let (p, rest) = p.split();
		Self::ensure(Self::Ensured::ensure_all(g, rest)?, p)
	}
}
pub trait EnsureUnloaded: Ensure
where
	<Self as Release>::Payload:
		Payload<<<Self as Release>::Ensured as Release>::Payload, Item = ()>,
{
	fn ensure_unvalidated(c: Self::Ensured) -> Self
	{
		<Self as Ensure>::ensure_unvalidated(c, ())
	}
	fn validate(c: &Self::Ensured) -> bool
	{
		<Self as Ensure>::validate(c, &())
	}
	fn ensure(c: Self::Ensured) -> Result<Self, ()>
	{
		<Self as Ensure>::ensure(c, ())
	}
	fn ensure_all(g: Self::Base) -> Result<Self, ()>
	where
		<Self as Release>::Payload: Payload<(), Item = ()>,
	{
		Ensure::ensure_all(g, <<Self as Release>::Payload>::new((), ()))
	}
}

pub trait Release: Sized + GraphDeref
{
	/// The base graph implementation being ensured
	type Base: BaseGraph;

	/// The next level of properties.
	type Ensured: Ensure<Base = Self::Base>;
	type Payload: Payload<<Self::Ensured as Release>::Payload>;

	/// Release only this level's properties, maintaining
	/// the next level's properties
	fn release(
		self,
	) -> (
		Self::Ensured,
		<Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	);

	/// Fully release this type, returning the base graph implementation
	/// type
	fn release_all(self) -> (Self::Base, Self::Payload)
	{
		let (ensured, payload) = self.release();
		let (base, payload_rest) = Release::release_all(ensured);
		(base, Payload::new(payload, payload_rest))
	}
}
pub trait ReleaseUnloaded: Release
{
	/// Release only this level's properties, maintaining
	/// the next level's properties
	fn release(self) -> Self::Ensured
	{
		Release::release(self).0
	}

	/// Fully release this type, returning the base graph implementation
	/// type
	fn release_all(self) -> Self::Base
	{
		Release::release_all(self).0
	}
}

impl<G: Graph, D: Deref<Target = G>> BaseGraph for D {}
impl<B: BaseGraph> Ensure for B
{
	fn ensure_unvalidated(
		c: Self::Ensured,
		_: <Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	) -> Self
	{
		c
	}

	fn validate(
		_: &Self::Ensured,
		_: &<Self::Payload as Payload<<Self::Ensured as Release>::Payload>>::Item,
	) -> bool
	{
		true
	}

	fn ensure_all(g: Self::Base, _: Self::Payload) -> Result<Self, ()>
	{
		Ensure::ensure(g, ())
	}
}
impl<B: BaseGraph> Release for B
{
	type Base = Self;
	type Ensured = Self;
	type Payload = ();

	fn release(self) -> (Self::Ensured, ())
	{
		(self, ())
	}

	fn release_all(self) -> (Self::Base, Self::Payload)
	{
		(self, ())
	}
}
impl<B: BaseGraph> BaseGraphUnloaded for B {}
impl<E: Ensure> EnsureUnloaded for E where
	E::Payload: Payload<<E::Ensured as Release>::Payload, Item = ()>
{
}
impl<E: Release> ReleaseUnloaded for E {}
