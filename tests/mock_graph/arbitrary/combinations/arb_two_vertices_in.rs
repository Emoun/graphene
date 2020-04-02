use crate::mock_graph::{
	arbitrary::{ArbVerticesIn, GuidedArbGraph, Limit},
	MockEdgeWeight, MockVertex, MockVertexWeight,
};
use graphene::{
	core::{Ensure, Graph, GraphDeref, GraphDerefMut},
	impl_ensurer,
};
use quickcheck::{Arbitrary, Gen};
use rand::Rng;
use static_assertions::_core::marker::PhantomData;
use std::{collections::HashSet, iter::FromIterator, ops::RangeBounds};

/// Used with `ArbTwoVerticesIn` to choose whether the two vertices must be
/// unique (`Unique`),
/// or can be duplicates (`NonUnique`, is the default).
pub trait Uniqueness: Clone + Send
{
	fn unique() -> bool;
}

/// Signals `ArbTwoVerticesIn` that the two vertices can't be the same.
#[derive(Clone, Debug, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unique();
/// Signals `ArbTwoVerticesIn` that the two vertices are allowed to be the same.
/// This is the default.
#[derive(Clone, Debug, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct NonUnique();

impl Uniqueness for Unique
{
	fn unique() -> bool
	{
		true
	}
}

impl Uniqueness for NonUnique
{
	fn unique() -> bool
	{
		false
	}
}

/// An arbitrary graph and two vertices in it.
///
/// Depending on `U`, the two vertices are either allowed to be the same
/// (`NonUnique`, default), or they must be unique (`Unique`).
///
/// Note: All graphs will have at least 1 vertex for non-unique and 2 vertices
/// for unique, meaning this type never includes the empty graph.
#[derive(Clone, Debug)]
pub struct ArbTwoVerticesIn<G, U = NonUnique>(
	pub G,
	pub MockVertex,
	pub MockVertex,
	pub PhantomData<U>,
)
where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: Uniqueness;

impl<G, U> ArbTwoVerticesIn<G, U>
where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: Uniqueness,
{
	pub fn new(g: G, v1: MockVertex, v2: MockVertex) -> Self
	{
		Self(g, v1, v2, PhantomData)
	}
}

impl<Gr, U> Arbitrary for ArbTwoVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: 'static + Uniqueness,
{
	fn arbitrary<G: Gen>(g: &mut G) -> Self
	{
		Self::arbitrary_guided(g, .., ..)
	}

	fn shrink(&self) -> Box<dyn Iterator<Item = Self>>
	{
		self.shrink_guided(HashSet::new())
	}
}
impl<Gr, U> GuidedArbGraph for ArbTwoVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph + GraphDerefMut,
	Gr::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: 'static + Uniqueness,
{
	fn arbitrary_guided<G: Gen>(
		g: &mut G,
		v_range: impl RangeBounds<usize>,
		e_range: impl RangeBounds<usize>,
	) -> Self
	{
		let (v_min, v_max, e_min, e_max) = Self::validate_ranges(g, v_range, e_range);

		// Create a graph with at least 1 or 2 vertices (1 for non-unique, 2 for Unique)
		let v_min_min = 1 + (U::unique() as usize);
		let v_min_max = if v_min_min < v_min { v_min } else { v_min_min };
		let graph = Gr::arbitrary_guided(g, v_min_max..v_max, e_min..e_max);
		let verts: Vec<_> = graph.graph().all_vertices().collect();
		let v1 = verts[g.gen_range(0, verts.len())];
		let v2 = loop
		{
			let candidate = verts[g.gen_range(0, verts.len())];
			if !U::unique()
			{
				break candidate;
			}
			if candidate != v1
			{
				break candidate;
			}
		};

		Self::new(graph, v1, v2)
	}

	fn shrink_guided(&self, mut limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		// Don't let it shrink to less than 1/2 vertices, can happen if self.1 and
		// self.2 are equal
		limits.insert(Limit::VertexMin(1 + (U::unique() as usize)));
		Box::new(
			ArbVerticesIn(
				self.0.clone(),
				HashSet::from_iter([self.1, self.2].iter().cloned()),
			)
			.shrink_guided(limits)
			.map(|g| {
				if !U::unique()
				{
					// we cycle, such that when the set only contains 1 vertex, we can use the same
					// one for both positions.
					let mut set = g.1.iter().cycle();
					Self::new(g.0, *set.next().unwrap(), *set.next().unwrap())
				}
				else
				{
					let mut set = g.1.iter();
					Self::new(g.0, *set.next().unwrap(), *set.next().unwrap())
				}
			}),
		)
	}
}

impl<G, U> Ensure for ArbTwoVerticesIn<G, U>
where
	G: Arbitrary + Ensure,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: Uniqueness,
{
	fn ensure_unvalidated(c: Self::Ensured) -> Self
	{
		let (v1, v2) = {
			let mut verts = c.graph().all_vertices();
			if !U::unique()
			{
				let v = verts.next().unwrap();
				(v, verts.next().unwrap_or(v))
			}
			else
			{
				(verts.next().unwrap(), verts.next().unwrap())
			}
		};
		Self::new(c, v1, v2)
	}

	fn validate(c: &Self::Ensured) -> bool
	{
		c.graph().all_vertices().count() >= (1 + (U::unique() as usize))
	}
}

impl_ensurer! {
	use<G,U> ArbTwoVerticesIn<G,U>: Ensure
	as (self.0): G
	where
	G: Arbitrary + GraphDeref,
	G::Graph:
		Graph<Vertex = MockVertex, VertexWeight = MockVertexWeight, EdgeWeight = MockEdgeWeight>,
	U: Uniqueness
}
