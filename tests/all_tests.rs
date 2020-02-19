//#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit = "8192"]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

/// Makes multiple versions of the given code.
///
/// # Example
///
/// ```
/// duplicate_for! {
/// 	$directedness [directed [Directed] undirected [Undirected]]
///
/// struct SomeStruct<directedness>;
/// 			}
/// ```
///
/// The above code after the first line of the macro will be duplicated twice.
/// In each duplicate, every `directedness` is substituted by either `Directed`
/// or `Undirected`. The result of the expansion is therefore:
///
/// ```
/// mod directed
/// 	{
/// 	use super::*;
/// 	struct SomeStruct<Directed>;
/// 	}
/// mod undirected
/// 	{
/// 	use super::*;
/// 	struct SomeStruct<Undirected>;
/// 	}
/// ```
///
/// The input must start with a '$' followed by a identifier which is used as a
/// placeholder to substitute.
/// Then comes a `[]`, which contains a list of identifier + substitution pairs.
/// Everthing after the list will then be duplicated in modules with the name of
/// the pair identifier, and every instance of the placeholder being changed for
/// the substitution.
macro_rules! duplicate_for {
	{
		$dollar:tt $placeholder:ident
		[$($substitute: tt)*]
		$($rest:tt)*
	} => {
		macro_rules! duplicate_for_predicate {
			{
				$dollar caller:tt
				input = [{ $dollar body:tt }]
			} => {
				tt_equal::tt_equal! {
					$dollar caller
					input = [{ $placeholder $dollar body }]
				}
			}
		}

		duplicate_for!{
			@inner@
			[$($substitute)*]
			$($rest)*
		}
	};

	{
		@inner@
		[ $id:ident [$($substitute: tt)*]]
		$($rest:tt)*
	} => {
		mod $id {
			use super::*;
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ duplicate_for_predicate }]
				replace_with = [{ $($substitute)* }]
				input = [{ $($rest)* }]
			}
		}
	};

	{
		@inner@
		[ $id:ident [$($substitute: tt)*] $($id_rest:ident [$($subs_rest: tt)*])*]
		$($rest:tt)*
	} => {
		duplicate_for!{
			@inner@
			[$id [$($substitute)*]]
			$($rest)*
		}
		duplicate_for!{
			@inner@
			[$($id_rest [$($subs_rest)*])*]
			$($rest)*
		}
	}
}

/// Makes two versions of the given code for each directedness.
///
/// The input must start with a '$' followed by a identifier which is used as a
/// placeholder for directedness.
/// Everthing after the identifier will then be duplicated, with the first
/// version using 'graphene::core::Directed' anywhere the placeholder is found,
/// and the second version using 'graphene::core::Undirected'.
///
/// The two version are put in modules called 'directed' and 'undirected', so
/// shouldn't interfere with each other or any code surrounding the macro call.
/// The modules use the super-module's imports.
macro_rules! duplicate_for_directedness{

	{
		$dollar:tt $placeholder:ident
		$($rest:tt)*
	} => {
		duplicate_for!{
			$dollar $placeholder
			[directed [graphene::core::Directed] undirected [graphene::core::Undirected]]
			$($rest)*
		}
	};
}

#[macro_use]
mod mock_graph;
mod algo;
mod common;
mod core;
