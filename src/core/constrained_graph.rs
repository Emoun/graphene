
use crate::core::{
	Graph, ManualGraph, AutoGraph, EdgeWeighted
};
/*
///
/// Defines a graph which has some constraint on how it is mutated.
///
/// An example could be a graph which prohibits duplicate edges, ignoring weights, called a
/// unique graph. Such a graph must then be implemented such that adding an edge
/// checks for duplicates and rejects any such.
///
/// More specifically, to uphold the contract of this trait the following must hold:
///
/// - The implementation of `BaseGraph` on the type must uphold the specified constraint. In our example
///  `add_graph()` must reject any edge which is already in the graph.
/// - The methods of this trait must be implemented.
///
/// The following methods must be implemented for this trait:
///
/// - `invariant_holds`: checks the constraint invariant on the current state of the graph
/// and returns whether it holds. In our example, it will go though all edges, and return false
/// if any duplicate is found.
///
/// - `uncon_add_vertex`: Tries to add a vertex without upholding the invariant.
///
/// - `uncon_remove_vertex`: Tries to remove a vertex without upholding the invariant.
///
/// - `uncon_add_edge`: Tries to add an edge without upholding the invariant. In our example, it
/// will add the edge without checking for duplicates. This means that when the call terminates, the
/// graph may not uphold the invariant of no duplicates.
///
/// - `uncon_remove_edge`: Tries to remove an edge without upholding the invariant.
///
/// The `uncon_...` methods are intentionally `unsafe` as they may result in a graph state which
/// does not uphold its own invariant, and should therefore not be used lightly. The real use case
/// for them come from the `unconstrained` default method. By using it, and the `Unconstrainer`
/// it returns, the user can try executing multiple mutating operations at once, and only after
/// that ensure that the graph still upholds its invariant Example:
///
/// ```
/// use graphene::core::*;
/// use graphene::core::constraint::*;
/// use graphene::common::*;
///
/// let mut g = UniqueGraph::<AdjListGraph<u32,()>>::graph(vec![1,2], vec![]).unwrap();
/// let e = BaseEdge::new(1,2,());
///
/// assert!(g.add_edge(e).is_ok());
/// assert!(g.add_edge(e).is_err());
/// assert!(g.unconstrained().add_edge(e).constrain().is_err());
/// assert!(g.unconstrained()
/// 			.add_edge(e)
/// 			.remove_edge(e)
/// 			.constrain()
/// 			.is_ok());
/// ```
/// We can see here that the same edge cannot be added twice with `g.add_edge(e)`.
/// When using `unconstrained()` we first add the edge, and then remove it again. This
/// means the graph will in the end again only have a single edge, which upholds the invariant.
///
///
*/
pub trait UnconstrainedGraph: Graph
{
	///
	/// Checks whether the current state of the graph upholds the constraint invariant.
	///
	fn invariant_holds(&self) -> bool;
	fn unconstrained_add_edge_weighted<E>(&mut self, e: E) -> Result<(),()>
		where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>;
	fn unconstrained_remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight,()>;
	fn unconstrained_remove_edge_where<F>(&mut self, f: F)
							-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
		where
			F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool
	;
}

pub trait UnconstrainedManualGraph: UnconstrainedGraph + ManualGraph
{
	
	fn unconstrained_add_vertex_weighted(&mut self, v: Self::Vertex, w: Self::VertexWeight) -> Result<(),()>;
}

pub trait UnconstrainedAutoGraph: UnconstrainedGraph + AutoGraph
{
	
	fn unconstrained_new_vertex_weighted(&mut self, w: Self::VertexWeight) -> Result<Self::Vertex,()>;
}