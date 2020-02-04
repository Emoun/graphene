use crate::core::trait_aliases::Id;
use std::ops::{Deref, DerefMut};

/// Edge
pub trait Edge<V>
where
	V: Id,
{
	fn source(&self) -> V;
	fn sink(&self) -> V;

	fn is_loop(&self) -> bool
	{
		self.source() == self.sink()
	}
}

pub trait EdgeWeighted<V, W>: Edge<V> + Sized
where
	V: Id,
{
	fn weight_owned(self) -> W;

	fn weight_ref(&self) -> &W;

	fn weight_ref_mut(&mut self) -> &mut W;

	fn split(self) -> ((V, V), W)
	{
		((self.source(), self.sink()), self.weight_owned())
	}

	/// Returns the other vertex than the one given.
	///
	/// If the one given is not in this edge, the sink is returned.
	fn other(&self, v: V) -> V
	{
		if self.sink() == v
		{
			self.source()
		}
		else
		{
			self.sink()
		}
	}
}

pub trait EdgeDeref<V, W>: Edge<V>
where
	W: Deref,
	V: Id,
{
	fn weight(&self) -> &W::Target;
}

pub trait EdgeDerefMut<V, W>: EdgeDeref<V, W>
where
	W: DerefMut,
	V: Id,
{
	fn weight_mut(&mut self) -> &mut W::Target;
}

impl<V> Edge<V> for (V, V)
where
	V: Id,
{
	fn source(&self) -> V
	{
		self.0
	}

	fn sink(&self) -> V
	{
		self.1
	}
}
impl<V> EdgeWeighted<V, ()> for (V, V)
where
	V: Id,
{
	fn weight_owned(self) {}

	fn weight_ref(&self) -> &()
	{
		&()
	}

	fn weight_ref_mut(&mut self) -> &mut ()
	{
		unimplemented!() // TODO: what to do about this?
	}
}

impl<V, W> Edge<V> for (V, V, W)
where
	V: Id,
{
	fn source(&self) -> V
	{
		self.0
	}

	fn sink(&self) -> V
	{
		self.1
	}
}
impl<V, W> EdgeWeighted<V, W> for (V, V, W)
where
	V: Id,
{
	fn weight_owned(self) -> W
	{
		self.2
	}

	fn weight_ref(&self) -> &W
	{
		&self.2
	}

	fn weight_ref_mut(&mut self) -> &mut W
	{
		&mut self.2
	}
}

impl<V, W> EdgeDeref<V, W> for (V, V, W)
where
	W: Deref,
	V: Id,
{
	fn weight(&self) -> &W::Target
	{
		&self.2
	}
}

impl<V, W> EdgeDerefMut<V, W> for (V, V, W)
where
	W: DerefMut,
	V: Id,
{
	fn weight_mut(&mut self) -> &mut W::Target
	{
		&mut self.2
	}
}
