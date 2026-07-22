use std::ops::Deref;

/// Represents a value that is either owned or borrowed.
///
/// Requires [`Deref`] so both options can be used the same way in most cases.
/// If a reference is needed that doesn't borrow the [`MaybeOwned`] itself, use
/// [`into_borrow`](MaybeOwned::into_borrowed).
///
/// To create an owned value, use [`Owned`].
pub trait MaybeOwned<'a>: Deref
{
	/// Returns the borrowed value, if any.
	fn into_borrowed(self) -> Option<&'a Self::Target>
	where
		Self: Sized,
		Self::Target: Sized;
}

impl<'a, W: ?Sized> MaybeOwned<'a> for &'a W
{
	fn into_borrowed(self) -> Option<&'a W>
	{
		Some(self)
	}
}

/// A zero-cost wrapper marking an edge weight as owned by value rather than
/// borrowed from the graph.
///
/// Used as the [`EdgeWeightRef`](crate::core::Graph::EdgeWeightRef) of graphs
/// such as [`EdgeWeightMap`](crate::core::proxy::EdgeWeightMap) that compute
/// edge weights on the fly. Dereferences to the wrapped weight, so it can be
/// used just like a reference.
#[derive(Clone, Debug)]
pub struct Owned<W>(pub W);

impl<W> Deref for Owned<W>
{
	type Target = W;

	fn deref(&self) -> &W
	{
		&self.0
	}
}

impl<'a, W> MaybeOwned<'a> for Owned<W>
{
	fn into_borrowed(self) -> Option<&'a W>
	{
		None
	}
}
