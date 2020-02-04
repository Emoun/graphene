//#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit = "8192"]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

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
		macro_rules! duplicate_for_directedness_predicate {
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
		mod directed{
			use super::*;
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ duplicate_for_directedness_predicate }]
				replace_with = [{ graphene::core::Directed }]
				input = [{ $($rest)* }]
			}
        }
        mod undirected{
        	use super::*;
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ duplicate_for_directedness_predicate }]
				replace_with = [{ graphene::core::Undirected }]
				input = [{ $($rest)* }]
			}
		}
	};
}

#[macro_use]
mod mock_graph;
mod algo;
mod common;
mod core;
