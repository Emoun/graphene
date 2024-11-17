#[macro_use]
mod impl_ensurer;
mod acyclic;
mod base_props;
mod connected;
mod directedness_ensurers;
mod has_vertex;
mod no_loops;
mod reflexive;
mod rooted;
mod simple;
mod subgraph;
mod unilateral;
mod unique;
mod weak;

pub use self::{
	acyclic::*, base_props::*, connected::*, directedness_ensurers::*, has_vertex::*, no_loops::*,
	reflexive::*, rooted::*, simple::*, subgraph::*, unilateral::*, unique::*, weak::*,
};
use crate::core::{
	proxy::{EdgeProxyGraph, ProxyVertex, VertexProxyGraph},
	Ensure,
};
use std::borrow::Borrow;

/// Will try to remove an edge from the graph that holds for the given function.
///
/// If after removing the edge, the given Ensure ('C') doesn't hold, then
/// the edge isn't removed in the first place.
///
/// Will always need a type annotation for the Ensure 'C'.
pub fn proxy_remove_edge_where_weight<'a, C, G, F>(
	g: &'a mut G,
	source: &G::Vertex,
	sink: &G::Vertex,
	f: F,
) -> Result<G::EdgeWeight, ()>
where
	G: RemoveEdge,
	F: Fn(&G::EdgeWeight) -> bool,
	C: Ensure<Ensured = EdgeProxyGraph<&'a G>, Base = EdgeProxyGraph<&'a G>, Payload = ()>,
{
	// We use the unsafe block here to allow us to use 'g' again later.
	// Currently, the compiler can't see when 'proxy' is no longer used,
	// and therefore 'g' is free to be used again.
	// I think this is caused by: https://github.com/rust-lang/rust/issues/53528
	// but not sure.
	let g2: &G = unsafe { (g as *mut G).as_ref().unwrap() };

	let mut proxy = EdgeProxyGraph::new(g2);
	proxy.remove_edge(source, sink)?;

	if C::can_ensure(&proxy, &())
	{
		// 	Here we use 'g' again since 'proxy' is no longer used.
		// The compiler doesn't recognize that 'proxy' isn't used in this blocks,
		// and therefore, this wouldn't work when giving 'proxy' 'g' directly.
		g.remove_edge_where_weight(source, sink, f)
	}
	else
	{
		Err(())
	}
}

/// Will try to remove the given vertex from the graph.
///
/// If after removing the vertex, the given Ensure ('C') doesn't hold, then
/// the vertex isn't removed in the first place.
///
/// Will always need a type annotation for the Ensure 'C'.
pub fn proxy_remove_vertex<'a, C, G>(g: &'a mut G, v: &G::Vertex) -> Result<G::VertexWeight, ()>
where
	G: RemoveVertex,
	C: Ensure<Ensured = VertexProxyGraph<&'a G>, Base = VertexProxyGraph<&'a G>, Payload = ()>,
{
	// 	We use the unsafe block here to allow us to use 'g' again later.
	// Currently, the compiler can't see when 'proxy' is no longer used,
	// and therefore 'g' is free to be used again.
	// I think this is caused by: https://github.com/rust-lang/rust/issues/53528
	// but not sure.
	let g2: &G = unsafe { (g as *mut G).as_ref().unwrap() };
	let mut proxy = VertexProxyGraph::new(g2);

	proxy
		.remove_vertex(&ProxyVertex::Underlying(v.borrow().clone()))
		.expect("Couldn't remove a vertex from the proxy");

	if C::can_ensure(&proxy, &())
	{
		g.remove_vertex(v)
	}
	else
	{
		Err(())
	}
}
