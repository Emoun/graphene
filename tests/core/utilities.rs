use super::*;

pub fn GraphMock_init<V,W,F>(desc: &GraphDescription<V,W>, holds: F) -> bool
	where
		V: ArbVertex,
		W: ArbWeight,
		F: Fn(GraphMock<V,W>) -> bool,
{
	graph_init(desc,holds)
}