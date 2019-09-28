//#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit="4096"]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

macro_rules! tt_curry {
	{
		$curried_macro:ident $dollar:tt $macro_to_curry:ident {
			$($body_to_curry:tt)*
		}
	} => {
		macro_rules! $curried_macro {
			{
				$dollar caller:tt
				$dollar input:ident = [{$dollar($dollar body:tt)*}]
			} => {
				$macro_to_curry! {
					$dollar caller
					$dollar input = [{ $($body_to_curry)* $dollar($dollar body)* }]
				}
			};
		}
	}
}

macro_rules! duplicate_for_directedness{

	{
		$dollar:tt $placeholder:ident
		$($rest:tt)*
	} => {
		tt_curry!{is_placeholder $dollar tt_equal{$placeholder}}
		
		mod directed{
			use tt_equal::tt_equal;
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ is_placeholder }]
				replace_with = [{ graphene::core::Directed }]
				input = [{ $($rest)* }]
			}
        }
        mod undirected{
        	use tt_equal::tt_equal;
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ is_placeholder }]
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
