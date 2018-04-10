use super::*;

// Private unconstrained
custom_graph!{
	struct G1 where AdjListGraph
}
#[test]
fn g1_test(){
	let mut g = G1::<u32,()>::empty_graph();
	g.add_vertex(4).unwrap();
}

// Public unconstrained
custom_graph!{
	pub struct G2 where AdjListGraph
}
#[test]
fn g2_test(){
	let mut g = G2::<u32,()>::empty_graph();
	g.add_vertex(4).unwrap();
}

// Private single-constrained unwrapped
custom_graph!{
	struct G3 where AdjListGraph impl Undirected
}
#[test]
fn g3_test(){
	let mut g = G3::<u32,()>::empty_graph();
	g.add_vertex(1).unwrap();
	g.add_vertex(2).unwrap();
	g.add_edge(BaseEdge::new(1,2,())).unwrap();
	let w_g = UndirectedGraph::wrap(g);
	assert!(!w_g.invariant_holds());
}

// Public single-constrained unwrapped
custom_graph!{
	pub struct G4 where AdjListGraph impl Undirected
}
#[test]
fn g4_test(){
	let mut g = G4::<u32,()>::empty_graph();
	g.add_vertex(1).unwrap();
	g.add_vertex(2).unwrap();
	g.add_edge(BaseEdge::new(1,2,())).unwrap();
	let w_g = UndirectedGraph::wrap(g);
	assert!(!w_g.invariant_holds());
}