use crate::core::trait_aliases::Id;
use std::ops::{Deref, DerefMut};

/// An edge in a [graph](trait.Graph.html) with vertices of type `V`.
///
/// An edge is simply a pair of vertices in a graph.
/// Every edge has a distinguished `source` and `sink`.
/// For undirected graphs, which vertex is which has no meaning.
/// For directed graphs, an edge points from the `source` to the `sink`.
///
/// This trait has a blanket implementation implementation for any pair `(V,V)`
/// or triple `(V,V,W)`. Therefore, the easiest way to create an edge is to
/// simply use a pair. The triple can be used if the edge is weighted
pub trait Edge<V>
where
	V: Id,
{
	/// The source vertex of the edge.
	fn source(&self) -> V;
	/// the sink vertex of the edge.
	fn sink(&self) -> V;

	/// Returns `true` if the source and sink are the same vertex.
	fn is_loop(&self) -> bool
	{
		self.source() == self.sink()
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

/// An edge in a [graph](trait.Graph.html) with vertices of type `V` and an
/// owned weight `W`.
///
/// These edges are usually use to either give a new weighted edge to a graph,
/// or to get it back from the graph. As such, the weight is owned by the edge
/// itself, unlike for [EdgeDeref](trait.EdgeDeref.html) which simply points to
/// a weight owned by a graph.
///
/// It has a blanket implementation for any triple `(V,V,W)`.
pub trait EdgeWeighted<V, W>: Edge<V> + Sized
where
	V: Id,
{
	/// Returns the weight of the edge.
	///
	/// This method will throw away the vertices of the edge.
	/// Use [split](trait.EdgeWeighted.html#method.split) if you need the
	/// vertices too.
	fn weight_owned(self) -> W;

	/// Returns a reference to the weight of the edge.
	fn weight_ref(&self) -> &W;

	/// returns a mutable reference to the weight of the edge.
	fn weight_ref_mut(&mut self) -> &mut W;

	/// Splits the edge's vertices from the weight.
	fn split(self) -> ((V, V), W)
	{
		((self.source(), self.sink()), self.weight_owned())
	}
}

/// A weighted edge in a [graph](trait.Graph.html) with vertices of type `V`.
///
/// `W` is a reference to the weight in the graph.
///
/// It has a blanket implementation for any triple `(V,V,W)` where `W`
/// implements [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html). `W::Target` is then
/// the type of the weight.
pub trait EdgeDeref<V, W>: Edge<V>
where
	W: Deref,
	V: Id,
{
	/// Returns a reference to the weight of the edge.
	fn weight(&self) -> &W::Target;
}

/// A weighted edge in a [graph](trait.Graph.html) with vertices of type `V`.
///
/// `W` is a mutable reference to the weight in the graph.
///
/// It has a blanket implementation for any triple `(V,V,W)` where `W`
/// implements [DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html). `W::Target` is then
/// the type of the weight.
pub trait EdgeDerefMut<V, W>: EdgeDeref<V, W>
where
	W: DerefMut,
	V: Id,
{
	/// Returns a mutable reference to the weight of the edge.
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
