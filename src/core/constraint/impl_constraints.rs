
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
		
		// Unique
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait Unique}]
			true = [{}]
			false = [{
				impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::Unique
					for $struct<$generic_graph>
					where
						$generic_graph: $crate::core::constraint::Unique,
						$($($bounds)*)?
				{}
			}]
		}

		// NoLoops
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait NoLoops}]
			true = [{}]
			false = [{
				impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::NoLoops
					for $struct<$generic_graph>
					where
						$generic_graph: $crate::core::constraint::NoLoops,
						$($($bounds)*)?
				{}
			}]
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
		tt_call::tt_if!{
			condition = [{tt_equal::tt_equal}]
			input = [{$trait Connected}]
			true = [{}]
			false = [{
				impl<$generic_graph: $crate::core::Constrainer> $crate::core::constraint::Connected
					for $struct<$generic_graph>
					where
						$generic_graph: $crate::core::constraint::Connected,
						$($($bounds)*)?
				{}
			}]
		}
	}
}
