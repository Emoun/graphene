
///
/// Implements constraints for the given struct.
///
/// Doesn't implement the constraint given after the struct. That constraint must use the original
/// name of the trait.
///
/// Supported constraint traits: Unique, NoLoops, Reflexive
///
#[macro_export]
macro_rules! impl_constraints {

	{
		$struct:ident<$generic_graph:ident>: $trait:ident
		$(where $($bounds:tt)*)?
	} => {
		//Directed
		impl_constraints!{
			@inner
			$struct<$generic_graph>: $trait
			$([$($bounds)*])? DirectedConstraint
		}
		
		//Undirected
		impl_constraints!{
			@inner
			$struct<$generic_graph>: $trait
			$([$($bounds)*])? UndirectedConstraint
		}
		
		// Unique
		impl_constraints!{
			@inner
			$struct<$generic_graph>: $trait
			$([$($bounds)*])? Unique
		}

		// NoLoops
		impl_constraints!{
			@inner
			$struct<$generic_graph>: $trait
			$([$($bounds)*])? NoLoops
		}

		// Reflexive
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait Reflexive}]
			true = [{}]
			false = [{
				impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::Reflexive
					for $struct<$generic_graph>
					where
						$generic_graph: $crate::core::constraint::Reflexive,
						$generic_graph::EdgeWeight: Default,
						<$generic_graph::Graph as Graph>::EdgeWeight: Default
						$($($bounds)*)?
				{}
			}]
		}

		// Connected
		impl_constraints!{
			@inner
			$struct<$generic_graph>: $trait
			$([$($bounds)*])? Connected
		}
	};
	
	{
		@inner
		$struct:ident<$generic_graph:ident>: $trait:ident
		$([$($bounds:tt)*])? $constraint:ident
	} => {
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait $constraint}]
			true = [{}]
			false = [{
				impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::$constraint
					for $struct<$generic_graph>
					where
						$generic_graph: $crate::core::constraint::$constraint,
						$($($bounds)*)?
				{}
			}]
		}
	}
}
