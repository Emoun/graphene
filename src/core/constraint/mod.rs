use super::*;

///
/// Implements a method for the wrapped value:
///
/// Syntax:
///
/// ``` text
/// unsafe? <wrapped member>.<method name>( <type of self> self
/// (, <args>)* -> <return type>
/// ```
/// expands to:
///
/// ``` text
/// unsafe? fn <method name>(<type of self> self <, <args>>*) -> <return type>{
/// 	self.<wrapped member>.<method name>( (<args>,)*)
/// }
/// ```
///
#[macro_export]
macro_rules! wrap {
	{
		$member:ident.$fn_name:ident( &self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		fn $fn_name(&self, $($arg_name : $arg_type),*) -> $ret {
			self.$member.$fn_name($($arg_name),*)
		}
	};
	{
		$member:ident.$fn_name:ident( & mut self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		fn $fn_name(& mut self, $($arg_name : $arg_type),*) -> $ret {
			self.$member.$fn_name($($arg_name),*)
		}
	};
	{
		unsafe $member:ident.$fn_name:ident( &self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		unsafe fn $fn_name(&self, $($arg_name : $arg_type),*) -> $ret {
			self.$member.$fn_name($($arg_name),*)
		}
	};
	{
		unsafe $member:ident.$fn_name:ident( & mut self
			$(, $arg_name:ident : $arg_type:ty)*) -> $ret:ty
	} => {
		unsafe fn $fn_name(& mut self, $($arg_name : $arg_type),*) -> $ret {
			self.$member.$fn_name($($arg_name),*)
		}
	};
}

///
/// Wrappes the four uncon_* methods from `ConstrainedGraph` using `wrap!`.
/// Must be called inside an impl of `ConstrainedGraph`.
///
#[macro_export]
macro_rules! wrap_uncon_methods{
	{
		$wrapped_member:ident
	} => {
		wrap!{unsafe $wrapped_member.uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
		wrap!{unsafe $wrapped_member.uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
	
		wrap!{unsafe $wrapped_member.uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
	
		wrap!{unsafe $wrapped_member.uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
	}
}

mod undirected;
mod unique;

pub use self::undirected::*;
pub use self::unique::*;



