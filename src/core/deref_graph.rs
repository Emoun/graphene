use crate::core::Graph;
use std::ops::{Deref, DerefMut};

/// Trait for types that reference a graph type.
///
/// This is part of the ensurance system and should be implemented
/// by any type that is itself a graph, or any type that references one.
///
/// This trait has a blanket implementation for any type that implements
/// [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) where `Target`
/// implements [Graph](trait.Graph.html).
pub trait GraphDeref
{
	/// The type of the graph referenced.
	type Graph: Graph;

	/// Returns a reference to the underlying graph.
	fn graph(&self) -> &Self::Graph;
}
/// Trait for types that reference a graph type mutably.
///
/// This is part of the ensurance system and should be implemented
/// by any type that is itself a graph, or any type that references one mutably.
///
/// This trait has a blanket implementation for any type that implements
/// [DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) where `Target`
/// implements [Graph](trait.Graph.html).
pub trait GraphDerefMut: GraphDeref
{
	/// Returns a mutable reference to the underlying graph.
	fn graph_mut(&mut self) -> &mut Self::Graph;
}

impl<G: Graph, D: Deref<Target = G>> GraphDeref for D
{
	type Graph = G;

	fn graph(&self) -> &Self::Graph
	{
		&**self
	}
}
impl<G: Graph, D: DerefMut<Target = G>> GraphDerefMut for D
{
	fn graph_mut(&mut self) -> &mut Self::Graph
	{
		&mut **self
	}
}
