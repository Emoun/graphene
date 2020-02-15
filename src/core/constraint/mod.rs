#[macro_use]
mod impl_constraints;
mod base_props;
mod connected;
mod directed_constraint;
mod no_loops;
mod reflexive;
mod subgraph;
mod undirected_constraint;
mod unilateral;
mod unique;
mod weak;

pub use self::{
	base_props::*, connected::*, directed_constraint::*, impl_constraints::*, no_loops::*,
	reflexive::*, subgraph::*, undirected_constraint::*, unilateral::*, unique::*, weak::*,
};
use crate::core::{
	proxy::{EdgeProxyGraph, ProxyVertex, VertexProxyGraph},
	Constrainer, Edge,
};

/// Will try to remove an edge from the graph that holds for the given function.
///
/// If after removing the edge, the given Constrainer ('C') doesn't hold, then
/// the edge isn't removed in the first place.
///
/// Will always need a type annotation for the Constrainer 'C'.
pub fn proxy_remove_edge_where<'a, C, G, F>(
	g: &'a mut G,
	f: F,
) -> Result<(G::Vertex, G::Vertex, G::EdgeWeight), ()>
where
	G: RemoveEdge,
	F: Fn((G::Vertex, G::Vertex, &G::EdgeWeight)) -> bool,
	C: Constrainer<Constrained = EdgeProxyGraph<&'a G>, Base = EdgeProxyGraph<&'a G>>,
{
	let to_remove = g
		.all_edges()
		.find(|&e| f(e))
		.map(|e| (e.source(), e.sink()));
	let proxy = if let Some(e) = to_remove
	{
		// 	We use the unsafe block here to allow us to use 'g' again later.
		// Currently, the compiler can't see when 'proxy' is no longer used,
		// and therefore 'g' is free to be used again.
		// I think this is caused by: https://github.com/rust-lang/rust/issues/53528
		// but not sure.
		let g2: &G = unsafe { (g as *mut G).as_ref().unwrap() };

		let mut proxy = EdgeProxyGraph::new(g2);
		proxy.remove_edge((e.source(), e.sink()))?;
		proxy
	}
	else
	{
		return Err(());
	};

	if C::constrain_single(proxy).is_ok()
	{
		// 	Here we use 'g' again since 'proxy' is no longer used.
		// The compiler doesn't recognize that 'proxy' isn't used in this blocks,
		// and therefore, this wouldn't work when giving 'proxy' 'g' directly.
		g.remove_edge_where(f)
	}
	else
	{
		Err(())
	}
}

/// Will try to remove the given vertex from the graph.
///
/// If after removing the vertex, the given Constrainer ('C') doesn't hold, then
/// the vertex isn't removed in the first place.
///
/// Will always need a type annotation for the Constrainer 'C'.
pub fn proxy_remove_vertex<'a, C, G>(g: &'a mut G, v: G::Vertex) -> Result<G::VertexWeight, ()>
where
	G: RemoveVertex,
	C: Constrainer<Constrained = VertexProxyGraph<&'a G>, Base = VertexProxyGraph<&'a G>>,
{
	// 	We use the unsafe block here to allow us to use 'g' again later.
	// Currently, the compiler can't see when 'proxy' is no longer used,
	// and therefore 'g' is free to be used again.
	// I think this is caused by: https://github.com/rust-lang/rust/issues/53528
	// but not sure.
	let g2: &G = unsafe { (g as *mut G).as_ref().unwrap() };
	let mut proxy = VertexProxyGraph::new(g2);

	proxy
		.remove_vertex(ProxyVertex::Underlying(v))
		.expect("Couldn't remove a vertex from the proxy");

	if C::constrain_single(proxy).is_ok()
	{
		g.remove_vertex(v)
	}
	else
	{
		Err(())
	}
}
