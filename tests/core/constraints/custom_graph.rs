use super::*;

// Private unconstrained
custom_graph!{
	struct G1 where AdjListGraph
}
// Public unconstrained
custom_graph!{
	pub struct G2 where AdjListGraph
}