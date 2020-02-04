mod impl_graph;

pub use self::impl_graph::*;
use crate::core::{Directed, Directedness};
use std::marker::PhantomData;

/// A graph using the adjacency list representation.
#[derive(Clone, Debug)]
pub struct AdjListGraph<Vw, Ew, D = Directed>
where
	D: Directedness,
{
	/// Adjacency list representation of the graph.
	/// Each index in vec is a vertex.
	vertices: Vec<(
		// The weight of the vertex
		Vw,
		// A list outgoing edges from this vertex
		Vec<(
			// The index of the sink vertex
			usize,
			// The weight of the edge
			Ew,
		)>,
	)>,
	phantom: PhantomData<D>,
}

impl<Vw, Ew, D> AdjListGraph<Vw, Ew, D>
where
	D: Directedness,
{
	pub fn new() -> Self
	{
		Self {
			vertices: Vec::new(),
			phantom: PhantomData,
		}
	}
}
