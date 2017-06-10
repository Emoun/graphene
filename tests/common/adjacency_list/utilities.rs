use super::*;

pub fn AdjListGraph_init<V,W,F>(desc: &GraphDescription<V,W>, holds: F) -> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		F: Fn(AdjListGraph<V,W>) -> bool
{
	graph_init::<AdjListGraph<_,_>,_>(desc, holds)
}

pub fn AdjListGraph_init_and_add_edge<V,W,F>(
	desc: &GraphDescription<V,W>,
	source_i_cand: usize,
	sink_i_cand: usize,
	weight: W,
	holds: F)
	-> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		F: Fn(AdjListGraph<V,W>,BaseEdge<V,W>) -> bool,
{
	graph_init_and_add_edge::<AdjListGraph<_,_>,_>(desc, source_i_cand, sink_i_cand, weight, holds)
}

pub fn AdjListGraph_init_and_remove_edge<V,W,F>(
	desc: &GraphDescription<V,W>,
	edge_index: usize, holds: F)
	-> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		F: Fn(AdjListGraph<V,W>,(usize,BaseEdge<V,W>)) -> bool,
{
	graph_init_and_remove_edge::<AdjListGraph<_,_>,_>(desc, edge_index, holds)
}


