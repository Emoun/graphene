
use crate::core::trait_aliases::Id;

///
/// Edge
///
pub trait Edge<V>
	where V: Id
{
	fn source(&self) -> V;
	fn sink(&self) -> V;
}

pub trait EdgeWeighted<V,W>: Edge<V>
	where V: Id
{
	fn get_weight(self) -> W;
}

pub trait WeightRef<V,W>: Edge<V>
	where V: Id
{
	fn weight(&self) -> &W;
}

///
/// Weighted edge, mutable
///
pub trait WeightRefMut<V,W>: WeightRef<V,W>
	where V: Id
{
	fn weight_mut(&mut self) -> &mut W;
}



impl<V> Edge<V> for (V,V)
	where V: Id
{
	fn source(&self) -> V{
		self.0
	}
	fn sink(&self) -> V{
		self.1
	}
}
impl<V> EdgeWeighted<V,()> for (V,V)
	where V: Id
{
	fn get_weight(self){}
}
impl<V> WeightRef<V,()> for (V, V)
	where V: Id
{
	fn weight(&self) -> &()
	{
		&()
	}
}

impl<V,W> Edge<V> for (V,V,W)
	where V: Id
{
	fn source(&self) -> V{
		self.0
	}
	fn sink(&self) -> V{
		self.1
	}
}
impl<V,W> EdgeWeighted<V,W> for (V,V,W)
	where V: Id
{
	fn get_weight(self) -> W
	{
		self.2
	}
}
impl<'a,V,W> WeightRef<V,W> for (V,V,W)
	where V: Id
{
	fn weight(&self) -> &W
	{
		&self.2
	}
}
impl<'a,V,W> WeightRefMut<V,W> for (V,V,W)
	where V: Id
{
	fn weight_mut(&mut self) -> &mut W
	{
		&mut self.2
	}
}

impl<'a,V,W> WeightRef<V,W> for (V,V,&'a W)
	where V: Id
{
	fn weight(&self) -> &W
	{
		self.2
	}
}

impl<'a,V,W> WeightRef<V,W> for (V,V,&'a mut W)
	where V: Id
{
	fn weight(&self) -> &W
	{
		self.2
	}
}
impl<'a,V,W> WeightRefMut<V,W> for (V,V,&'a mut W)
	where V: Id
{
	fn weight_mut(&mut self) -> &mut W
	{
		self.2
	}
}
