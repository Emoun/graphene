use crate::core::{
	property::{RemoveVertex, Rooted},
	Ensure, Graph, GraphDerefMut,
};
use std::{
	borrow::Borrow,
	fmt::{Debug, Error, Formatter},
};

/// A marker trait for graphs with at least 1 vertex.
///
/// `V` is the number of vertices that are guaranteed to be in the graph.
/// If `Unique` is true (default) then indexed vertices methods will never return duplicate
/// vertices.
pub trait HasVertex<const V: usize = 1>: Graph
{
	/// Ensures this trait cannot be implemented with V=0.
	///
	/// Add a call to this associated type in the implementing type's
	/// constructor to ensure if that type ever gets v=0, compilation will
	/// fail.
	///
	/// Example:
	/// ```compile_fail, E0080
	/// # use std::borrow::Borrow;
	/// # use graphene::{
	/// # 	common::AdjListGraph,
	/// # 	core::{Directed, Graph, property::HasVertex}
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
	/// struct Struct<const V: usize>(usize);
	///
	/// impl<const V: usize> HasVertex<V> for Struct<V> {
	/// 	fn get_vertex_idx(&self, idx: usize) -> Self::Vertex {
	/// 		()
	/// 	}
	/// }
	///
	/// impl<const V: usize> Struct<V> {
	/// 	fn new() -> Self {
	/// 		_ = Self::ASSERT_NOT_0; // This ensures errors are thrown if V = 0
	/// 		Struct(V)
	/// 	}
	/// }
	///
	/// let _ = Struct::<0>::new(); // Will cause a compile error
	/// let _ = Struct::<1>::new(); // Will compile successfully
	/// ```
	const ASSERT_NOT_0: () = assert!(V > 0, "Found type implementing HasVertex<0>");

	/// Returns a vertex in the graph.
	///
	/// Successive calls do not guarantee to return the same vertex,
	/// even though the graph hasn't changed.
	fn get_vertex(&self) -> Self::Vertex
	{
		_ = Self::ASSERT_NOT_0;
		self.get_vertex_at::<0>()
	}
	
	/// Returns the I'th vertex guaranteed to be in the vertex.
	///
	/// The vertex ordering (i.e. the I) is significant and does not change unless
	/// the underlying graph type changes it.
	fn get_vertex_at<const I: usize>(&self) -> Self::Vertex {
		_ = Self::ASSERT_NOT_0;
		const {
			assert!(I < V)
		}
		assert!(I < V, "I out of bounds");
		self.get_vertex_idx(I)
	}
	
	/// Returns vertex guaranteed to be in the vertex at the given index in the ordering.
	///
	/// The vertex ordering (i.e. the idx) is significant and does not change unless
	/// the underlying graph type changes it.
	fn get_vertex_idx(&self, idx: usize) -> Self::Vertex;
}

/// Ensures the underlying graph has at least 1 vertex.
///
/// Gives no guarantees on which vertex is returned by any given call to
/// `get_vertex` if the graph has multiple vertices.
#[derive(Clone)]
pub struct HasVertexGraph<C: Ensure>(C);

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

impl<C: Ensure, const V: usize> HasVertex<V> for HasVertexGraph<C>
{
	fn get_vertex_idx(&self, idx: usize) -> Self::Vertex
	{
		assert!(idx < V);
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
pub struct VertexInGraph<C: Ensure, const V: usize = 1>(C, [<C::Graph as Graph>::Vertex; V]);

impl<C: Ensure, const V: usize> VertexInGraph<C, V>
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
		_ = Self::ASSERT_NOT_0;
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

impl<C: Ensure, const V: usize> Debug for VertexInGraph<C, V>
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

impl<C: Ensure, const V: usize> Ensure for VertexInGraph<C, V>
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

impl<C: Ensure + GraphDerefMut, const V: usize> RemoveVertex for VertexInGraph<C, V>
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

impl<C: Ensure, const V: usize> HasVertex<V> for VertexInGraph<C, V>
{
	fn get_vertex_idx(&self, idx: usize) -> Self::Vertex
	{
		assert!(idx < V);
		self.1[idx]
	}
}

impl<C: Ensure> Rooted for VertexInGraph<C>
where
	C::Graph: Rooted,
{
	fn root(&self) -> Self::Vertex
	{
		self.get_vertex()
	}

	fn set_root(&mut self, v: impl Borrow<Self::Vertex>) -> Result<(), ()>
	{
		self.set_vertex([*v.borrow()])
	}
}

impl_ensurer! {
	use<C ; const V: usize> VertexInGraph<C,V>: Ensure, HasVertex, RemoveVertex, Rooted
	as (self.0) : C
	as (self.1) : [<C::Graph as Graph>::Vertex;V]
}
