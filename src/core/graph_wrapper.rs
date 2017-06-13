use super::*;

///
/// Implements a method for the wrapped graph:
///
/// Syntax:
///
/// ``` text
/// unsafe? <method name>( <type of self> self
/// (, <args>)* -> <return type>
/// ```
/// expands to:
///
/// ``` text
/// unsafe? fn (<type of self> self <, <args>>*) -> <return type>{
/// 	self.wrapped().<method name>( (<args>,)*)
/// }
/// ```
///
#[macro_export]
macro_rules! wrapped_method {
	{
		$fn_name:ident( &self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		fn $fn_name(&self, $($arg_name : $arg_type),*) -> $ret {
			self.wrapped().$fn_name($($arg_name),*)
		}
	};
	{
		$fn_name:ident( & mut self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		fn $fn_name(& mut self, $($arg_name : $arg_type),*) -> $ret {
			self.wrapped_mut().$fn_name($($arg_name),*)
		}
	};
	{
		unsafe $fn_name:ident( &self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		unsafe fn $fn_name(&self, $($arg_name : $arg_type),*) -> $ret {
			self.wrapped().$fn_name($($arg_name),*)
		}
	};
	{
		unsafe $fn_name:ident( & mut self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		unsafe fn $fn_name(& mut self, $($arg_name : $arg_type),*) -> $ret {
			self.wrapped_mut().$fn_name($($arg_name),*)
		}
	};
}

///
/// Implements the four uncon_* methods from `ConstrainedGraph` for a graph using `wrapped_method!`.
/// Must be called inside an impl of `ConstrainedGraph`.
///
#[macro_export]
macro_rules! wrapped_uncon_methods{
	{
	} => {
		wrapped_method!{unsafe uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
		wrapped_method!{unsafe uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
		wrapped_method!{unsafe uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
	
		wrapped_method!{unsafe uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
	}
}

///
/// Implements `BaseGraph` for a `GraphWrapper` by having all methods
/// call the corresponding wrapped graph methods.
///
#[macro_export]
macro_rules! impl_BaseGraph_for_wrapper{
	{
		$graph_name:ident
	} => {
		impl<G> BaseGraph for $graph_name<G>
			where
				G: ConstrainedGraph,
				<G as BaseGraph>::Vertex: Vertex,
				<G as BaseGraph>::Weight: Weight,
				<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
				<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		{
			type Vertex = <G as BaseGraph>::Vertex;
			type Weight = <G as BaseGraph>::Weight;
			type VertexIter = <G as BaseGraph>::VertexIter;
			type EdgeIter = <G as BaseGraph>::EdgeIter;
			
			fn empty_graph() -> Self{
				$graph_name::wrap(G::empty_graph())
			}
		
			wrapped_method!{all_vertices(&self) -> Self::VertexIter}
			
			wrapped_method!{all_edges(&self) -> Self::EdgeIter}
			
			wrapped_method!{add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
			
			wrapped_method!{remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
		}
	}
}
#[macro_export]
macro_rules! impl_ConstrainedGraph_for_wrapper{
	{
	$graph_name:ident
	} => {
		impl<G> ConstrainedGraph for $graph_name<G>
			where
				G: ConstrainedGraph,
				<G as BaseGraph>::Vertex: Vertex,
				<G as BaseGraph>::Weight: Weight,
				<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
				<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		{
			wrapped_method!{invariant_holds(&self) -> bool}
		
			wrapped_uncon_methods!{}
		}
	}
}

///
/// Implements `ConstrainedGraph` for a `GraphWrapper` by having all methods
/// call the corresponding wrapped graph methods.
///
#[macro_export]
macro_rules! impl_constraints_for_wrapper{
	{
		// Name of the resulting graph.
		$graph_name:ident
		
		// Name of the constraint implementations
		: $($con_trait:ident),+
	} => {
		$(
			impl<G> $con_trait for $graph_name<G>
				where
					G: ConstrainedGraph,
					<G as BaseGraph>::Vertex: Vertex,
					<G as BaseGraph>::Weight: Weight,
					<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
					<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
			{}
		)*
	}
}

///
/// Defines a new public struct with implements `GraphWrapper`.
/// The struct is generic over any `ConstrainedGraph` G.
///
#[macro_export]
macro_rules! graph_wrapper{
	{
		$(#[$attr:meta])*
		struct $graph_name:ident
	} =>{
		$(#[$attr])*
		#[derive(Debug,Clone)]
		pub struct $graph_name<G>
			where
				G: ConstrainedGraph,
				<G as BaseGraph>::Vertex: Vertex,
				<G as BaseGraph>::Weight: Weight,
				<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
				<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		{
			wraps: G
		}
		
		impl<G> GraphWrapper for $graph_name<G>
			where
				G: ConstrainedGraph,
				<G as BaseGraph>::Vertex: Vertex,
				<G as BaseGraph>::Weight: Weight,
				<<G as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
				<<G as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
		{
			
			type Wrapped = G;
			
			fn wrap(g: Self::Wrapped) -> Self{
				$graph_name{wraps: g}
			}
			
			fn wrapped(&self) -> &Self::Wrapped{
				&self.wraps
			}
			
			fn wrapped_mut(&mut self) -> &mut Self::Wrapped{
				&mut self.wraps
			}
			
			fn unwrap(self) -> Self::Wrapped{
				self.wraps
			}
		}
	};
}

///
/// Defines a type that wraps a `ConstrainedGraph`.
///
pub trait GraphWrapper
	where
		<Self::Wrapped as BaseGraph>::Vertex: Vertex,
		<Self::Wrapped as BaseGraph>::Weight: Weight,
		<<Self::Wrapped as BaseGraph>::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
		<<Self::Wrapped as BaseGraph>::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	type Wrapped: 	ConstrainedGraph;
	
	fn wrap(g: Self::Wrapped) -> Self;
	
	fn wrapped(&self) -> &Self::Wrapped;
	
	fn wrapped_mut(&mut self) -> &mut Self::Wrapped;
	
	fn unwrap(self) -> Self::Wrapped;
}





























