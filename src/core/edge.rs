
use core::trait_aliases::Id;

///
/// Edge
///
pub trait Edge<V,W>
	where V: Id
{
	fn source(&self) -> V;
	fn sink(&self) -> V;
	fn weight(&self) -> &W;
}

///
/// Weighted edge, mutable
///
pub trait EdgeMut<V,W>: Edge<V,W>
	where V: Id
{
	fn weight_mut(&mut self) -> &mut W;
}

impl<V> Edge<V,()> for (V,V)
	where V: Id
{
	fn source(&self) -> V{
		self.0
	}
	fn sink(&self) -> V{
		self.1
	}
	fn weight(&self) -> &()
	{
		&()
	}
}
/*
// For some reason this is not currently possible.
impl<V> EdgeMut<V,()> for (V,V)
	where V: Id
{
	fn weight_mut<'a>(&mut self) -> &mut ()
	{
		&mut ()
	}
}
*/
impl<'a,V,W> Edge<V,W> for (V,V,&'a W)
	where V: Id
{
	fn source(&self) -> V{
		self.0
	}
	fn sink(&self) -> V{
		self.1
	}
	fn weight(&self) -> &W
	{
		self.2
	}
}

impl<'a,V,W> Edge<V,W> for (V,V,&'a mut W)
	where V: Id
{
	fn source(&self) -> V{
		self.0
	}
	fn sink(&self) -> V{
		self.1
	}
	fn weight(&self) -> &W
	{
		self.2
	}
}

impl<'a,V,W> EdgeMut<V,W> for (V,V,&'a mut W)
	where V: Id
{
	fn weight_mut(&mut self) -> &mut W
	{
		self.2
	}
}
