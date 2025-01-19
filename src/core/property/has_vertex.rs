use crate::core::{property::RemoveVertex, Ensure, Graph, GraphDerefMut, Release};
use std::{
	borrow::Borrow,
	fmt::{Debug, Error, Formatter},
};

/// A marker trait for graphs with at least 1 vertex.
pub trait HasVertex: Graph
{
	/// Return a vertex from the graph.
	///
	/// May return different vertices on successive calls to the same unmodified
	/// graph.
	fn any_vertex(&self) -> Self::Vertex;

	// fn as_vertex_in(self: impl Borrow<Self>) -> VertexInGraph<Self, 1, true>
	// 	where
	// 		Self: Ensure,
	// 		<Self as GraphDeref>::Graph: Graph<Vertex=Self::Vertex>
	// {
	// 	let v = self.borrow().any_vertex();
	// 	VertexInGraph::ensure_unchecked(self, [v])
	// }
}

/// For specifying specific vertices in a graph.
///
/// This is primarily used as input to various algorithms. E.g. a search
/// algorithm will require a starting vertex and so might take `VertexIn<1>` as
/// input. Finding a path between two vertices could likewise take `VertexIn<2,
/// false>` as input.
///
/// The specified vertices are ordered and indexed.
///
/// `N` is the number of vertices specified.
/// `UNIQUE` signifies whether there are any duplicates in the vertices.
/// If true, there can be no duplicate vertices.
pub trait VertexIn<const N: usize = 1, const UNIQUE: bool = true>: HasVertex
{
	/// Ensures this trait gets valid parameters.
	///
	/// This trait does not accept N=0, or N=1 and !UNIQUE.
	///
	/// Add a call to this associated type in the implementing type's
	/// constructor to ensure if that type ever get invalid parameters,
	/// compilation will fail.
	///
	/// Example:
	/// ```compile_fail, E0080
	/// # use std::borrow::Borrow;
	/// # use graphene::{
	/// # 	common::AdjListGraph,
	/// # 	core::{Directed, Graph, property::{VertexIn, HasVertex}}
	/// # };
	/// # impl<const V: usize> Graph for Struct<V> {
	/// # 	type Vertex = ();
	/// # 	type VertexWeight = ();
	/// # 	type EdgeWeight = ();
	/// # 	type EdgeWeightRef<'a> = () where Self: 'a;
	/// # 	type Directedness = Directed;
	/// #
	/// # 	fn all_vertices_weighted(&self) -> impl Iterator<Item=(Self::Vertex, &Self::VertexWeight)>
	/// # 	{
	/// #         std::iter::empty()
	/// # 	}
	/// #
	/// # 	fn edges_between(&self, source: impl Borrow<Self::Vertex>, sink: impl Borrow<Self::Vertex>)
	/// # 		-> impl Iterator<Item=Self::EdgeWeightRef<'_>>
	/// # 	{
	/// # 		std::iter::empty()
	/// # 	}
	/// #
	/// # }
	/// # impl<const N: usize> HasVertex for Struct<N> {
	/// # 	fn any_vertex(&self) -> Self::Vertex{
	/// # 		()
	/// # 	}
	/// # }
	/// struct Struct<const N: usize>(usize);
	///
	/// impl<const N: usize> VertexIn<N> for Struct<N> {
	/// 	fn vertex_at_idx(&self, idx: usize) -> Self::Vertex {
	/// 		()
	/// 	}
	/// }
	///
	/// impl<const V: usize> Struct<V> {
	/// 	fn new() -> Self {
	/// 		_ = Self::ASSERT_VALID_PARAMS; // This ensures errors are thrown if V = 0
	/// 		Struct(V)
	/// 	}
	/// }
	///
	/// let _ = Struct::<0>::new(); // Will cause a compile error
	/// let _ = Struct::<1>::new(); // Will compile successfully
	/// ```
	const ASSERT_VALID_PARAMS: () = {
		assert!(N > 0, "Found type implementing VertexIn<0>");
		assert!(
			N != 1 || UNIQUE,
			"Found type implementing VertexIn<1, false>"
		);
	};

	/// Returns the I'th vertex specified in the graph.
	fn vertex_at<const I: usize>(&self) -> Self::Vertex
	{
		_ = Self::ASSERT_VALID_PARAMS;
		const { assert!(I < N) }
		self.vertex_at_idx(I)
	}

	/// Returns the I'th vertex specified in the graph.
	fn vertex_at_idx(&self, idx: usize) -> Self::Vertex;
}

/// Ensures the underlying graph has at least 1 vertex.
#[derive(Clone)]
pub struct HasVertexGraph<C: Ensure>(C);

impl<C: Ensure> HasVertexGraph<C>
{
	pub fn as_vertex_in(self) -> VertexInGraph<C>
	{
		let v = self.any_vertex();
		VertexInGraph::ensure_unchecked(self.release(), [v])
	}
}

impl<C: Ensure> Ensure for HasVertexGraph<C>
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		c.graph().all_vertices().next().is_some()
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for HasVertexGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if self.all_vertices().nth(1).is_some()
		{
			self.0.graph_mut().remove_vertex(v)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure> HasVertex for HasVertexGraph<C>
{
	fn any_vertex(&self) -> Self::Vertex
	{
		self.all_vertices()
			.next()
			.expect("HasVertexGraph has no vertices.")
	}
}

impl_ensurer! {
	use<C> HasVertexGraph<C>: Ensure, HasVertex, RemoveVertex
	as (self.0) : C
}

/// Ensures a specific vertex or vertices is in the underlying graph.
///
/// The designated vertices cannot be removed from the graph.
#[derive(Clone)]
pub struct VertexInGraph<C: Ensure, const V: usize = 1, const UNIQUE: bool = true>(
	C,
	[<C::Graph as Graph>::Vertex; V],
);

impl<C: Ensure, const V: usize, const U: bool> VertexInGraph<C, V, U>
{
	/// ```compile_fail, E0080
	/// use graphene::common::AdjListGraph;
	/// use graphene::core::property::VertexInGraph;
	/// use graphene::core::Ensure;
	///
	/// 	let _ = VertexInGraph::<_, 0>::ensure_unchecked(AdjListGraph::<(), ()>::new(), []);
	/// ```
	fn new(c: C, vs: [<C::Graph as Graph>::Vertex; V]) -> Self
	{
		_ = Self::ASSERT_VALID_PARAMS;
		Self(c, vs)
	}

	pub fn set_vertex(
		&mut self,
		replacements: impl Borrow<[<C::Graph as Graph>::Vertex; V]>,
	) -> Result<(), ()>
	{
		if replacements
			.borrow()
			.iter()
			.all(|v| self.0.graph().contains_vertex(v))
		{
			self.1 = replacements.borrow().clone();
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure, const V: usize, const U: bool> Debug for VertexInGraph<C, V, U>
where
	C: Debug,
	<C::Graph as Graph>::Vertex: Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
	{
		f.debug_tuple("VertexInGraph")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<C: Ensure, const V: usize, const U: bool> Ensure for VertexInGraph<C, V, U>
{
	fn ensure_unchecked(c: Self::Ensured, v: [<C::Graph as Graph>::Vertex; V]) -> Self
	{
		Self::new(c, v)
	}

	fn can_ensure(c: &Self::Ensured, p: &[<C::Graph as Graph>::Vertex; V]) -> bool
	{
		p.iter().all(|v| c.graph().contains_vertex(v))
	}
}

impl<C: Ensure + GraphDerefMut, const V: usize, const U: bool> RemoveVertex
	for VertexInGraph<C, V, U>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if self.1.iter().all(|v2| v2 != v.borrow())
		{
			self.0.graph_mut().remove_vertex(v)
		}
		else
		{
			Err(())
		}
	}
}

impl<C: Ensure, const V: usize, const U: bool> HasVertex for VertexInGraph<C, V, U>
{
	fn any_vertex(&self) -> Self::Vertex
	{
		self.vertex_at::<0>()
	}
}

impl<C: Ensure, const V: usize, const U: bool> VertexIn<V, U> for VertexInGraph<C, V, U>
{
	fn vertex_at_idx(&self, idx: usize) -> Self::Vertex
	{
		self.1[idx]
	}
}

impl_ensurer! {
	use<C ; const V: usize, const U: bool> VertexInGraph<C,V,U>: Ensure, HasVertex, VertexIn, RemoveVertex
	as (self.0) : C
	as (self.1) : [<C::Graph as Graph>::Vertex;V]
}
