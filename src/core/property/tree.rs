use crate::core::{
	property::{
		Acyclic, AcyclicGraph, AddEdge, ConnectedGraph, DirectedGraph, HasVertex, HasVertexGraph,
		NewVertex, NoLoops, RemoveVertex, Unique, UniqueGraph, Weak, WeakGraph,
	},
	Directed, Ensure, Graph, GraphDerefMut, Guard,
};
use duplicate::duplicate_item;
use std::borrow::Borrow;

/// A marker trait for [weighted tree graphs](https://mathworld.wolfram.com/WeightedTree.html).
///
/// Trees are simple, connected, acyclic graphs.
///
/// Wolfram Alpha defines 'Trees' as being unweighted and undirected and
/// 'Weighted Trees' as allowing weights. Because weights and directions are
/// generally useful, we use the simpler name for the general case and 'Strict
/// Tree' for the unweighted, undirected case.
///
/// TODO: add trait for [strict trees](https://mathworld.wolfram.com/Tree.html)
/// TODO: add trait for [arborescence](https://mathworld.wolfram.com/Arborescence.html)
///
/// Properties of interest:
/// https://mathworld.wolfram.com/PerfectlyWeightedTree.html
pub trait Tree: HasVertex + Unique + Weak + Acyclic
{
	/// Returns whether the vertex is a [leaf](https://mathworld.wolfram.com/TreeLeaf.html) in the graph.
	///
	/// Leaves have exactly one incident edge.
	/// Notice that the tree with only one vertex has no leaves, while the tree
	/// with two vertices has two leaves.
	///
	/// Unlike Wolfram Alpha's description, this method does not account for
	/// roots of rooted trees. TODO: add link to relevant method that does
	/// account.
	fn is_leaf(&self, v: impl Borrow<Self::Vertex>) -> bool
	{
		self.edges_incident_on(v).count() == 1
	}
}

/// Methods for adding leaves to a tree without specifying direction.
///
/// If the tree is directed, the direction of the new edge to the leaf is
/// dictated by the implementer.
pub trait NewLeafUndirected: Tree
{
	/// Adds a new leaf to the tree with the given parent and weights.
	///
	/// If the tree is directed, the weight is implicit.
	fn new_leaf_weighted(
		&mut self,
		parent: impl Borrow<Self::Vertex>,
		w: Self::VertexWeight,
		e: Self::EdgeWeight,
	) -> Result<Self::Vertex, ()>;

	/// Adds a new leaf to the tree with the given parent and default weights.
	///
	/// If the tree is directed, the weight is implicit.
	fn new_leaf(&mut self, parent: impl Borrow<Self::Vertex>) -> Result<Self::Vertex, ()>
	where
		Self::VertexWeight: Default,
		Self::EdgeWeight: Default,
	{
		self.new_leaf_weighted(parent, Default::default(), Default::default())
	}
}

/// Methods for adding leaves to a tree with a direction.
pub trait NewLeafDirected: Tree<Directedness = Directed>
{
	/// Adds a new leaf to the tree with the given parent, direction, and
	/// weights.
	fn new_leaf_weighted(
		&mut self,
		parent: impl Borrow<Self::Vertex>,
		to_new: bool,
		w: Self::VertexWeight,
		e: Self::EdgeWeight,
	) -> Result<Self::Vertex, ()>;

	/// Adds a new leaf to the tree with the given parent, direction, and
	/// default weights.
	fn new_leaf(
		&mut self,
		parent: impl Borrow<Self::Vertex>,
		to_new: bool,
	) -> Result<Self::Vertex, ()>
	where
		Self::VertexWeight: Default,
		Self::EdgeWeight: Default,
	{
		self.new_leaf_weighted(parent, to_new, Default::default(), Default::default())
	}
}

#[derive(Clone, Debug)]
pub struct TreeGraph<C: Ensure>(C);

impl<C: Ensure> Ensure for TreeGraph<C>
{
	fn ensure_unchecked(c: Self::Ensured, _: ()) -> Self
	{
		Self(c)
	}

	fn can_ensure(c: &Self::Ensured, _: &()) -> bool
	{
		UniqueGraph::can_guard(c)
			&& AcyclicGraph::can_guard(c)
			&& HasVertexGraph::can_guard(c)
			&& if let Ok(g) = DirectedGraph::guard(c.graph())
			{
				WeakGraph::can_guard(&g)
			}
			else
			{
				ConnectedGraph::can_guard(c)
			}
	}
}

impl<C: Ensure + GraphDerefMut> RemoveVertex for TreeGraph<C>
where
	C::Graph: RemoveVertex,
{
	fn remove_vertex(&mut self, v: impl Borrow<Self::Vertex>) -> Result<Self::VertexWeight, ()>
	{
		if !self.is_leaf(v.borrow())
		{
			return Err(());
		}
		self.0.graph_mut().remove_vertex(v)
	}
}

impl<C: Ensure + GraphDerefMut> NewLeafUndirected for TreeGraph<C>
where
	C::Graph: NewVertex + AddEdge,
{
	fn new_leaf_weighted(
		&mut self,
		parent: impl Borrow<Self::Vertex>,
		w: Self::VertexWeight,
		e: Self::EdgeWeight,
	) -> Result<Self::Vertex, ()>
	{
		if !self.0.graph().contains_vertex(parent.borrow())
		{
			return Err(());
		}
		let v = self.0.graph_mut().new_vertex_weighted(w)?;
		self.0.graph_mut().add_edge_weighted(parent, v, e)?;
		Ok(v)
	}
}

impl<C: Ensure + GraphDerefMut> NewLeafDirected for TreeGraph<C>
where
	C::Graph: NewVertex<Directedness = Directed> + AddEdge,
{
	fn new_leaf_weighted(
		&mut self,
		parent: impl Borrow<Self::Vertex>,
		to_new: bool,
		w: Self::VertexWeight,
		e: Self::EdgeWeight,
	) -> Result<Self::Vertex, ()>
	{
		if !self.0.graph().contains_vertex(parent.borrow())
		{
			return Err(());
		}
		let v = self.0.graph_mut().new_vertex_weighted(w)?;

		if to_new
		{
			self.0.graph_mut().add_edge_weighted(parent, v, e)?;
		}
		else
		{
			self.0.graph_mut().add_edge_weighted(v, parent, e)?;
		}
		Ok(v)
	}
}

#[duplicate_item(
	Prop; [Tree]; [HasVertex]; [Weak]; [Acyclic]; [NoLoops]; [Unique];
)]
impl<C: Ensure> Prop for TreeGraph<C> {}

impl_ensurer! {
	use<C> TreeGraph<C>: Ensure, Tree, HasVertex, Acyclic, NoLoops, Weak, Unique, RemoveVertex,
		NewLeafUndirected, NewLeafDirected,
		// cannot add vertices without edges or add or remove edges alone
		NewVertex, AddEdge, RemoveEdge
	as (self.0) : C
}
