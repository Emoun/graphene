use crate::core::Graph;
use std::ops::{Deref, DerefMut};

pub trait GraphDeref
{
	type Graph: Graph;
	fn graph(&self) -> &Self::Graph;
}
pub trait GraphDerefMut: GraphDeref
{
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
