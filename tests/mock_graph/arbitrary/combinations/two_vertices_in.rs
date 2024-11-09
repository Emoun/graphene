use crate::mock_graph::{
	arbitrary::{GuidedArbGraph, Limit},
	MockType, MockVertex, TestGraph,
};
use graphene::{
	core::{
		property::{HasVertex, VertexInGraph},
		Ensure, Graph, GraphDeref, ReleaseUnloaded,
	},
	impl_ensurer,
};
use quickcheck::Gen;
use rand::Rng;
use std::{collections::HashSet, fmt::Debug, marker::PhantomData};

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
pub struct TwoVerticesIn<G, U = NonUnique>(pub VertexInGraph<G>, pub MockVertex, PhantomData<U>)
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
	U: Uniqueness;

impl<G, U> TwoVerticesIn<G, U>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
	U: Uniqueness,
{
	pub fn new(g: G, v1: MockVertex, v2: MockVertex) -> Self
	{
		if U::unique() && v1 == v2
		{
			panic!("Unique vertices aren't allowed: '{:?}', '{:?}'", v1, v2);
		}
		if !g.graph().contains_vertex(v2)
		{
			panic!("Vertex not in graph: '{:?}'", v2);
		}
		Self(VertexInGraph::ensure(g, v1).unwrap(), v2, PhantomData)
	}

	pub fn get_both(&self) -> (MockVertex, MockVertex)
	{
		(self.0.get_vertex(), self.1)
	}

	pub fn get_two_vertices<Ge: Gen>(g: &mut Ge, graph: &G) -> (MockVertex, MockVertex)
	{
		let verts: Vec<_> = graph.graph().all_vertices().collect();
		assert!(verts.len() >= (1 + (U::unique() as usize)));
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
		(v1, v2)
	}
}

impl<G, U> Ensure for TwoVerticesIn<G, U>
where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
	U: Uniqueness,
{
	fn ensure_unvalidated(c: Self::Ensured, _: ()) -> Self
	{
		let v2 = {
			let mut verts = c.all_vertices();
			if !U::unique()
			{
				verts.next().unwrap()
			}
			else
			{
				verts.filter(|&v| v != c.get_vertex()).next().unwrap()
			}
		};
		Self(c, v2, PhantomData)
	}

	fn validate(c: &Self::Ensured, _: &()) -> bool
	{
		!U::unique() || c.graph().all_vertices().count() >= 2
	}
}

impl_ensurer! {
	use<G,U> TwoVerticesIn<G,U>: Ensure
	as (self.0): VertexInGraph<G>
	where
	G: GuidedArbGraph,
	G::Graph: TestGraph,
	<G::Graph as Graph>::EdgeWeight: MockType,
	U: Uniqueness
}

impl<Gr, U> GuidedArbGraph for TwoVerticesIn<Gr, U>
where
	Gr: GuidedArbGraph,
	Gr::Graph: TestGraph,
	<Gr::Graph as Graph>::EdgeWeight: MockType,
	U: 'static + Uniqueness,
{
	fn choose_size<G: Gen>(
		g: &mut G,
		v_min: usize,
		v_max: usize,
		e_min: usize,
		e_max: usize,
	) -> (usize, usize)
	{
		let v_min = std::cmp::max(v_min, 1 + (U::unique() as usize));
		assert!(v_max > v_min);
		Gr::choose_size(g, std::cmp::max(v_min, 1), v_max, e_min, e_max)
	}

	fn arbitrary_fixed<G: Gen>(g: &mut G, v_count: usize, e_count: usize) -> Self
	{
		assert!(v_count >= 1 + (U::unique() as usize));

		let graph = Gr::arbitrary_fixed(g, v_count, e_count);
		let (v1, v2) = Self::get_two_vertices(g, &graph);

		Self::new(graph, v1, v2)
	}

	fn shrink_guided(&self, limits: HashSet<Limit>) -> Box<dyn Iterator<Item = Self>>
	{
		let mut result = Vec::new();

		// Shrink without removing the referenced vertices
		let mut lims = limits.clone();
		// We only need to limits the second vertex, as VertexInGraph
		// will manage the first one isn't removed
		lims.insert(Limit::VertexKeep(self.1));

		result.extend(
			self.0
				.shrink_guided(lims)
				.map(|g| Self(g, self.1, PhantomData)),
		);

		if !U::unique() && self.get_vertex() != self.1
		{
			// Shrink by making both vertices the same
			result.push(Self(self.0.clone(), self.get_vertex(), PhantomData));
			result.push(Self(
				VertexInGraph::ensure(self.0.clone().release(), self.1).unwrap(),
				self.1,
				PhantomData,
			));
		}

		Box::new(result.into_iter())
	}
}
