use crate::core::{
	property::{AddEdge, NoLoops, NoLoopsGraph, Unique, UniqueGraph},
	Ensure, Graph, GraphDerefMut, Undirected,
};
use duplicate::duplicate_item;
use std::borrow::Borrow;

/// A marker trait for [simple graphs](https://mathworld.wolfram.com/SimpleGraph.html)
pub trait Simple: NoLoops + Unique {}

#[derive(Clone, Debug)]
pub struct SimpleGraph<C: Ensure>(C);

impl<C: Ensure> SimpleGraph<C>
where
	C::Graph: Graph<EdgeWeight = (), Directedness = Undirected>,
{
	/// Constrains the given graph.
	///
	/// The given graph must be simple. This is not checked by this function.
	pub fn unchecked(c: C) -> Self
	{
		Self(c)
	}
}

impl<C: Ensure> Ensure for SimpleGraph<C>
where
	C::Graph: Graph<EdgeWeight = (), Directedness = Undirected>,
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		NoLoopsGraph::<C>::can_ensure(c, &()) && UniqueGraph::can_ensure(c, &())
	}
}

impl<C: Ensure + GraphDerefMut> AddEdge for SimpleGraph<C>
where
	C::Graph: AddEdge<EdgeWeight = (), Directedness = Undirected>,
{
	fn add_edge_weighted(
		&mut self,
		source: impl Borrow<Self::Vertex>,
		sink: impl Borrow<Self::Vertex>,
		weight: Self::EdgeWeight,
	) -> Result<(), ()>
	{
		if !(source.borrow() != sink.borrow()
			&& Self::can_add_edge(self, source.borrow(), sink.borrow()))
		{
			Err(())
		}
		else
		{
			self.0.graph_mut().add_edge_weighted(source, sink, weight)
		}
	}
}

#[duplicate_item(
	Prop; [Unique]; [NoLoops]; [Simple];
)]
impl<C: Ensure> Prop for SimpleGraph<C> where
	C::Graph: Graph<EdgeWeight = (), Directedness = Undirected>
{
}

impl_ensurer! {
	use<C> SimpleGraph<C>: Ensure, Unique, NoLoops, Simple, AddEdge
	as (self.0) : C
	where C::Graph: Graph<EdgeWeight=(), Directedness=Undirected>
}
