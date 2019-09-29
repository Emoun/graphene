//#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit="4096"]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

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
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ duplicate_for_directedness_predicate }]
				replace_with = [{ graphene::core::Directed }]
				input = [{ $($rest)* }]
			}
        }
        mod undirected{
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
mod core;
mod common;
mod algo;
