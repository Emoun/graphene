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

mod unweighted;
mod undirected;
mod unique;

pub use self::unweighted::*;
pub use self::undirected::*;
pub use self::unique::*;



