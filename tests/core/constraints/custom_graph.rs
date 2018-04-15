
use super::*;

// Private unconstrained unwrapped
custom_graph!{
	struct G1<V,W> as AdjListGraph<V,W>
	where V: Vertex, W: Weight
}
// Public unconstrained unwrapped
custom_graph!{
	pub struct G2<S,P> as AdjListGraph<S,P>
	where S: Vertex, P: Weight
}
// Private single-constrained unwrapped
custom_graph!{
	struct G3<V,W> as AdjListGraph<V,W>
	impl Undirected
	where V: Vertex, W: Weight
}
// Public single-constrained unwrapped
custom_graph!{
	pub struct G4<S,P> as AdjListGraph<S,P>
	impl Undirected
	where S: Vertex, P: Weight
}
// Private doubly-constrained unwrapped
custom_graph!{
	struct G5<V,W> as AdjListGraph<V,W>
	impl Undirected, Unique
	where V: Vertex, W: Weight
}
// Public doubly-constrained unwrapped
custom_graph!{
	pub struct G6<S,P> as AdjListGraph<S,P>
	impl Undirected, Unique
	where S: Vertex, P: Weight
}
// Private doubly-constrained single-wrapped
custom_graph!{
	struct G7<V,W> as AdjListGraph<V,W>
	use UndirectedGraph impl Undirected, Unique
	where V: Vertex, W: Weight
}
// Public doubly-constrained single-wrapped
custom_graph!{
	pub struct G8<S,P> as AdjListGraph<S,P>
	use UndirectedGraph impl Undirected, Unique
	where S: Vertex, P: Weight
}
// Private doubly-constrained doubly-wrapped
custom_graph!{
	struct G9<S,P> as AdjListGraph<S,P>
	use UndirectedGraph, UniqueGraph impl Undirected, Unique
	where S: Vertex, P: Weight
}
// Public doubly-constrained doubly-wrapped
custom_graph!{
	pub struct G10<V,W> as AdjListGraph<V,W>
	use UndirectedGraph, UniqueGraph impl Undirected, Unique
	where V: Vertex, W: Weight
}
// Private unconstrained unwrapped non-generic
custom_graph!{
	struct G11 as AdjListGraph<i32,i32>
}
// Public unconstrained unwrapped non-generic
custom_graph!{
	pub struct G12 as AdjListGraph<i32,i32>
}
// Private doubly-constrained unwrapped non-generic
custom_graph!{
	struct G13 as AdjListGraph<i32,i32>
	impl Undirected, Unique
}
// Public doubly-constrained unwrapped non-generic
custom_graph!{
	pub struct G14 as AdjListGraph<i32,i32>
	impl Undirected, Unique
}
// Private doubly-constrained doubly-wrapped non-generic
custom_graph!{
	struct G15 as AdjListGraph<i32,u32>
	use UniqueGraph, UndirectedGraph
	impl Undirected, Unique
}
// Public doubly-constrained doubly-wrapped non-generic
custom_graph!{
	pub struct G16 as AdjListGraph<i32,u32>
	use UndirectedGraph, UniqueGraph impl Undirected, Unique
}
// Private doubly-constrained doubly-wrapped weight-generic
custom_graph!{
	struct G17<V> as AdjListGraph<V,u32>
	use UniqueGraph, UndirectedGraph
	impl Undirected, Unique
	where V:Vertex
}
// Public doubly-constrained doubly-wrapped weight-generic
custom_graph!{
	pub struct G18<V> as AdjListGraph<V,u32>
	use UndirectedGraph, UniqueGraph impl Undirected, Unique
	where V:Vertex
}
// Private doubly-constrained doubly-wrapped vertex-generic
custom_graph!{
	struct G19<V> as AdjListGraph<V,u32>
	use UniqueGraph, UndirectedGraph
	impl Undirected, Unique
	where V:Vertex
}
// Public doubly-constrained doubly-wrapped vertex-generic
custom_graph!{
	pub struct G20<W> as AdjListGraph<u32,W>
	use UndirectedGraph, UniqueGraph impl Undirected, Unique
	where W: Weight
}





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
	let _: &UniqueGraph<UndirectedGraph<AdjListGraph<_,_>>> = g.wrapped();
	
}
#[test]
fn g10_test(){
	let g = G10::<u32,()>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UniqueGraph<UndirectedGraph<AdjListGraph<_,_>>> = g.wrapped();
	
}
#[test]
fn g11_test(){
	let g = G11::empty_graph();
	type_check_graph(&g);
}
#[test]
fn g12_test(){
	let g = G12::empty_graph();
	type_check_graph(&g);
}
#[test]
fn g13_test(){
	let g = G13::empty_graph();
	type_check_undirected_unique(&g);
}
#[test]
fn g14_test(){
	let g = G13::empty_graph();
	type_check_undirected_unique(&g);
}
#[test]
fn g15_test(){
	let g = G15::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<UniqueGraph<AdjListGraph<_,_>>> = g.wrapped();
}
#[test]
fn g16_test(){
	let g = G16::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UniqueGraph<UndirectedGraph<AdjListGraph<_,_>>> = g.wrapped();
}
#[test]
fn g17_test(){
	let g = G17::<i32>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<UniqueGraph<AdjListGraph<_,_>>> = g.wrapped();
}
#[test]
fn g18_test(){
	let g = G18::<i32>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UniqueGraph<UndirectedGraph<AdjListGraph<_,_>>> = g.wrapped();
}
#[test]
fn g19_test(){
	let g = G19::<i32>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UndirectedGraph<UniqueGraph<AdjListGraph<_,_>>> = g.wrapped();
}
#[test]
fn g20_test(){
	let g = G20::<i32>::empty_graph();
	type_check_undirected_unique(&g);
	let _: &UniqueGraph<UndirectedGraph<AdjListGraph<_,_>>> = g.wrapped();
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
