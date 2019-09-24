
///
/// Helps with the delegation of the implementation of the Graph trait
/// for graph wrappers.
/// Assumes the wrappers target is self.0 and that it is generic on
/// a Graph.
///
/// Instead of manually implementing Graph for the wrapper (and possibly using
/// the delegate crate for delegating Graph's methods), this macro will delegate
/// the implementation to the wrapped type (self.0), except for the methods
/// specified.
/// To work, the manually implemented methods must use the exact same tokens
/// as their definition in Graph. Otherwise, this macro wont recognize them
/// and will delegate them to the graph, meaning the method will be duplicated
/// and an error will be thrown by the compiler.
///
/// Supports where clauses.
///
macro_rules! delegate_graph {

	{
		$graph:ident<$generic:ident>
		where
		$($rest:tt)*
	} => {
		delegate_graph!{
			@internal_parse_where
			$graph $generic []
			$($rest)*
		}
	} ;
	
	{
		$graph:ident<$generic:ident>
		{$($rest:tt)*}
	} => {
		delegate_graph!{
			@internal_parse
			$graph $generic []
			@no_delegate[[]]
			@no_del_bods[]
			$($rest)*
		}
	} ;
	
	{
		@internal_parse_where
		$graph:ident $generic:ident $where_clause:tt
		{$($no_dels:tt)*}
	} => {
		delegate_graph!{
			@internal_parse
			$graph $generic $where_clause
			@no_delegate[[]]
			@no_del_bods[]
			$($no_dels)*
		}
	};

	{
		@internal_parse_where
		$graph:ident $generic:ident [$($where_clause:tt)*]
		$next:tt $($rest:tt)*
	} => {
		delegate_graph!{
			@internal_parse_where
			$graph $generic [$($where_clause)* $next]
			$($rest)*
		}
	};

	{
		@internal_parse
		$graph:ident $generic:ident $where_clause:tt
		@no_delegate[[$($cur_no_del:tt)*] $($rest_no_del:tt)*]
		@no_del_bods [$($rest_bods:tt)*]
		{$($body:tt)*} $($rest:tt)*
	} => {
		delegate_graph!{
			@internal_parse
			$graph $generic $where_clause
			@no_delegate[ [] [$($cur_no_del)*] $($rest_no_del)*]
			@no_del_bods[ {$($body)*} $($rest_bods)*]
			$($rest)*
		}
	};

	{
		@internal_parse
		$graph:ident $generic:ident $where_clause:tt
		@no_delegate[[$($cur_no_del:tt)*] $($rest_no_del:tt)*]
		@no_del_bods $bods:tt
		$next:tt $($rest:tt)*
	} => {
		delegate_graph!{
			@internal_parse
			$graph $generic $where_clause
			@no_delegate[ [$($cur_no_del)* $next] $($rest_no_del)*]
			@no_del_bods $bods
			$($rest)*
		}
	};

	{
		@internal_parse
		$graph:ident $generic:ident $where_clause:tt
		@no_delegate[[] $($no_del:tt)*]
		@no_del_bods $bods:tt
	} => {
		delegate_graph!{
			@internal_sort_init
			$graph $generic $where_clause
			@candidates[
				[fn all_vertices_weighted<'a>(&'a self)
						-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a Self::VertexWeight)>>]
						
				[fn all_vertices_weighted_mut<'a>(&'a mut self)
					-> Box<dyn 'a + Iterator<Item=(Self::Vertex, &'a mut Self::VertexWeight)>>]
				
				[fn remove_vertex(&mut self, v: Self::Vertex) -> Result<Self::VertexWeight, ()>]
				
				[fn all_edges<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=
					(Self::Vertex, Self::Vertex, &'a Self::EdgeWeight)>>]
				
				[fn all_edges_mut<'a>(&'a mut self) -> Box<dyn 'a + Iterator<Item=
					(Self::Vertex, Self::Vertex, &'a mut Self::EdgeWeight)>>]
				
				[fn remove_edge_where<F>(&mut self, f: F)
					-> Result<(Self::Vertex, Self::Vertex, Self::EdgeWeight), ()>
					where F: Fn((Self::Vertex, Self::Vertex, &Self::EdgeWeight)) -> bool]
			
				[fn add_edge_weighted<E>(&mut self, e: E) -> Result<(), ()>
					where E: EdgeWeighted<Self::Vertex, Self::EdgeWeight>]
			]
			@no_delegate [$($no_del)*]
			@no_del_bods $bods
			@to_delegate[]
		}
	};

	{
		@internal_sort_init
		$graph:ident $generic:ident $where_clause:tt
		@candidates[ $next_cand:tt $($rest_cands:tt)* ]
		@no_delegate $funcs:tt
		@no_del_bods $bods:tt
		@to_delegate $to_dels:tt
	} => {
		delegate_graph!{
			@internal_sort_step
			$graph $generic $where_clause
			@candidates[ $($rest_cands)* ]
			@no_delegate $funcs
			@no_del_bods $bods
			@to_delegate $to_dels
			@cur_cand $next_cand
			@no_del_rest $funcs
		}
	};
	
	{
		@internal_sort_init
		$graph:ident $generic:ident $where_clause:tt
		@candidates[] // no more candidates
		@no_delegate $funcs:tt
		@no_del_bods $bods:tt
		@to_delegate $to_dels:tt
	} => {
		delegate_graph!{
			@internal_finish
			$graph $generic $where_clause
			@no_delegate $funcs
			@no_del_bods $bods
			@to_delegate $to_dels
		}
	};
	
	{
		@internal_sort_step
		$graph:ident $generic:ident $where_clause:tt
		@candidates $cands:tt
		@no_delegate $no_del:tt
		@no_del_bods $bods:tt
		@to_delegate [$($to_del:tt)*]
		@cur_cand $cand:tt
		@no_del_rest[$next_no_del:tt $($rest_no_del:tt)*]
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
            input = [{ $cand $next_no_del }]
            true = [{
                delegate_graph!{
					@internal_sort_init
					$graph $generic $where_clause
					@candidates $cands
					@no_delegate $no_del
					@no_del_bods $bods
					@to_delegate [$($to_del)*]
					// Throw current candidate out, since it shouldn't be delegated
				}
            }]
            false = [{
                delegate_graph!{
					@internal_sort_step
					$graph $generic $where_clause
					@candidates $cands
					@no_delegate $no_del
					@no_del_bods $bods
					@to_delegate [$($to_del)*]
					@cur_cand $cand
					@no_del_rest[$($rest_no_del)*] // Throw away the no_del, checking the rest
				}
            }]
		}
	};

	{
		@internal_sort_step
		$graph:ident $generic:ident $where_clause:tt
		@candidates $cands:tt
		@no_delegate $no_del:tt
		@no_del_bods $bods:tt
		@to_delegate [$($to_del:tt)*]
		@cur_cand $cand:tt
		@no_del_rest[] // no more no_dels, means candidate must be delegated
	} => {
		delegate_graph!{
			@internal_sort_init
			$graph $generic $where_clause
			@candidates $cands
			@no_delegate $no_del
			@no_del_bods $bods
			@to_delegate [$($to_del)* $cand]
		}
	};
	
	{
		@internal_finish
		$graph:ident $generic:ident [$($where_clause:tt)*]
		@no_delegate [$([$($no_del:tt)*])*]
		@no_del_bods [$($no_del_bod:tt)*]
		@to_delegate [$([$($to_del:tt)*])*]
	} => {
		impl<$generic: crate::core::Graph> crate::core::Graph for $graph<$generic>
			where $($where_clause)*
		{
			type Vertex = G::Vertex;
			type VertexWeight = G::VertexWeight;
			type EdgeWeight = G::EdgeWeight;
			type Directedness = G::Directedness;
			
			delegate::delegate! {
				target self.0 {
					$( $($to_del)*; )*
				}
			}
			$( #[allow(unused_variables)] $($no_del)* $no_del_bod)*
		}
	};
}

#[macro_use]
mod impl_constraints;
mod unique;
mod no_loops;
mod reflexive;
mod connected;

pub use self::{
	unique::*,
	no_loops::*,
	reflexive::*,
	impl_constraints::*,
	connected::*,
};

