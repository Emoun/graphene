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
pub trait BaseGraphGuard: BaseGraph
{
	fn guard_all<G>(self) -> Result<G, ()>
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
pub trait Ensure: ReleasePayload
{
	fn ensure_unchecked(
		c: Self::Ensured,
		p: <Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	) -> Self;

	fn can_ensure(
		c: &Self::Ensured,
		p: &<Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	) -> bool;

	fn ensure(
		c: Self::Ensured,
		p: <Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	) -> Result<Self, ()>
	{
		if Self::can_ensure(&c, &p)
		{
			Ok(Self::ensure_unchecked(c, p))
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
pub trait Guard: Ensure
where
	<Self as ReleasePayload>::Payload:
		Payload<<<Self as ReleasePayload>::Ensured as ReleasePayload>::Payload, Item = ()>,
{
	fn guard_unchecked(c: Self::Ensured) -> Self
	{
		<Self as Ensure>::ensure_unchecked(c, ())
	}
	fn can_guard(c: &Self::Ensured) -> bool
	{
		<Self as Ensure>::can_ensure(c, &())
	}
	fn guard(c: Self::Ensured) -> Result<Self, ()>
	{
		<Self as Ensure>::ensure(c, ())
	}
	fn guard_all(g: Self::Base) -> Result<Self, ()>
	where
		<Self as ReleasePayload>::Payload: Payload<(), Item = ()>,
	{
		Ensure::ensure_all(g, <<Self as ReleasePayload>::Payload>::new((), ()))
	}
}

/// Trait for remove one or more layers of ensurers, aka. releasing the
/// properties.
///
/// A _layer_ is an ensurer that ensures some property holds.
/// The base graph does not count as a layer, the ensurer wrapping the base
/// graph is therefore the first layer. Each layer may need a payload. For
/// example, an ensurer guaranteeing that a given vertex exists may have the
/// vertex as a payload.
///
pub trait ReleasePayload: Sized + GraphDeref
{
	/// The base graph implementation being ensured
	type Base: BaseGraph;

	/// The inner ensurer being further ensured.
	type Ensured: Ensure<Base = Self::Base>;

	/// The payload used to ensure this property holds.
	type Payload: Payload<<Self::Ensured as ReleasePayload>::Payload>;

	/// ReleasePayload only this level's properties, maintaining
	/// the next level's properties and returning the payload released
	fn release(
		self,
	) -> (
		Self::Ensured,
		<Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	);

	/// Fully release all ensurers, returning the base graph and the payload for
	/// all levels
	fn release_all(self) -> (Self::Base, Self::Payload)
	{
		let (ensured, payload) = self.release();
		let (base, payload_rest) = ReleasePayload::release_all(ensured);
		(base, Payload::new(payload, payload_rest))
	}
}

/// Equivalent to `ReleasePayload` except does not return any payload.
pub trait Release: ReleasePayload
{
	/// ReleasePayload only this level's properties, maintaining
	/// the next level's properties and returning the payload released
	///
	/// Like [ReleasePayload::release], but does not return the payload
	/// released.
	fn release(self) -> Self::Ensured
	{
		ReleasePayload::release(self).0
	}

	/// Fully release all ensurers, returning the base graph and the payload for
	/// all levels
	///
	/// Like [ReleasePayload::release_all], but does not return the payloads
	/// released.
	fn release_all(self) -> Self::Base
	{
		ReleasePayload::release_all(self).0
	}
}

impl<G: Graph, D: Deref<Target = G>> BaseGraph for D {}
impl<B: BaseGraph> Ensure for B
{
	fn ensure_unchecked(
		c: Self::Ensured,
		_: <Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	) -> Self
	{
		c
	}

	fn can_ensure(
		_: &Self::Ensured,
		_: &<Self::Payload as Payload<<Self::Ensured as ReleasePayload>::Payload>>::Item,
	) -> bool
	{
		true
	}

	fn ensure_all(g: Self::Base, _: Self::Payload) -> Result<Self, ()>
	{
		Ensure::ensure(g, ())
	}
}
impl<B: BaseGraph> ReleasePayload for B
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
impl<B: BaseGraph> BaseGraphGuard for B {}
impl<E: Ensure> Guard for E where
	E::Payload: Payload<<E::Ensured as ReleasePayload>::Payload, Item = ()>
{
}
impl<E: ReleasePayload> Release for E {}
