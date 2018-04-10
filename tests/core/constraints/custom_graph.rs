use super::*;

// Private unconstrained
custom_graph!{
	struct G1<V,W> where AdjListGraph<V,W>
}
// Public unconstrained
custom_graph!{
	pub struct G2<S,P> where AdjListGraph<S,P>
}
// Private single-constrained unwrapped
custom_graph!{
	struct G3<V,W> where AdjListGraph<V,W> impl Undirected
}
// Public single-constrained unwrapped
custom_graph!{
	pub struct G4<S,P> where AdjListGraph<S,P> impl Undirected
}
// Private doubly-constrained unwrapped
custom_graph!{
	struct G5<V,W> where AdjListGraph<V,W> impl Undirected, Unique
}
// Public doubly-constrained unwrapped
custom_graph!{
	pub struct G6<S,P> where AdjListGraph<S,P> impl Undirected, Unique
}
// Private doubly-constrained single-wrapped
custom_graph!{
	struct G7<V,W> where AdjListGraph<V,W> impl Undirected, Unique use UndirectedGraph
}
// Public doubly-constrained single-wrapped
custom_graph!{
	pub struct G8<S,P> where AdjListGraph<S,P> impl Undirected, Unique use UndirectedGraph
}
// Private doubly-constrained doubly-wrapped
custom_graph!{
	struct G9<S,P> where AdjListGraph<S,P> impl Undirected, Unique use UndirectedGraph, UniqueGraph
}
// Public doubly-constrained doubly-wrapped
custom_graph!{
	pub struct G10<V,W> where AdjListGraph<V,W> impl Undirected, Unique use UndirectedGraph, UniqueGraph
}
// Private unconstrained unwrapped

// The following tests show that the structs have been implemented correctly
#[test]
fn g1_test(){
	let g = G1::<u32,()>::empty_graph();
	type_check_graph(&g);
}
#[test]
fn g2_test(){
	let g = G2::<u32,()>::empty_graph();
	type_check_graph(&g);
}
#[test]
fn g3_test(){
	let g = G3::<u32,()>::empty_graph();
	type_check_undirected(&g);
}
#[test]
fn g4_test(){
	let g = G4::<u32,()>::empty_graph();
	type_check_undirected(&g);
}
#[test]
fn g5_test(){
	let g = G5::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
}
#[test]
fn g6_test(){
	let g = G6::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
}
#[test]
fn g7_test(){
	let g = G7::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<AdjListGraph<_,_>> = g.wrapped();
	
}
#[test]
fn g8_test(){
	let g = G8::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<AdjListGraph<_,_>> = g.wrapped();
	
}
#[test]
fn g9_test(){
	let g = G9::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<UniqueGraph<AdjListGraph<_,_>>> = g.wrapped();
	
}
#[test]
fn g10_test(){
	let g = G10::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<UniqueGraph<AdjListGraph<_,_>>>  = g.wrapped();
	
}


// Functions to typecheck the generated structs
macro_rules! typecheck_functions{
	{
		$(fn $name:ident : $($constraint:ident),*);*
	} => {
		$(
			#[allow(unused_variables)]
			fn $name<T>(g: &T)
				where
					T: BaseGraph $(+ $constraint)*,
					<<T as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
					<<T as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
			{}
		)*
	};
}

typecheck_functions!{
	fn type_check_graph:;
	fn type_check_undirected: Undirected;
	fn type_check_undirected_unique: Undirected, Unique
}
