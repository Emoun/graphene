
///
/// A macro for defining a custom graph with a specific set of constraints.
///
/// The resulting graph is generic over vertices and weights (<V,W>) and implement
/// `GraphWrapper` and the constraints given.
///
/// Syntax:
///
/// ``` text
/// struct <graph name>
/// as <name of type implementing ConstrainedGraph>
/// impl (<constraint>),*
/// use (<GraphWrapper to use to uphold the constraint, i.e. will wrap the ConstrainedGraph>),*
/// ```
///
/// The 'impl' and 'use' clauses are optional.
///
///
#[macro_export]
macro_rules! custom_graph{
	{
		pub $($rest:tt)*
	} => {
		custom_graph!{
			[flag_public]
			$($rest)*
		}
	};
	{
		struct $($rest:tt)*
	} => {
		custom_graph!{
			[ ]
			struct $($rest)*
		}
	};
	
// Stack builders
	{
		[ $($stack:tt)* ]
		struct $graph_name:ident
		$($rest:tt)*
	} => {
		custom_graph!{
			[$graph_name $($stack)*] $($rest)*
		}
	};
	{
		[ $($stack:tt)* ]
		as $base_graph:ident $($rest:tt)*
	} => {
		custom_graph!{
			[ $base_graph $($stack)* ] $($rest)*
		}
	};
	{
		[ $($stack:tt)* ]
		<$V:ident,$W:ident> $($rest:tt)*
	} => {
		custom_graph!{
			[<$V,$W> $($stack)* ] $($rest)*
		}
	};
	{
		[ $($stack:tt)* ]
		<$V:ty,$W:ty> $($rest:tt)*
	} => {
		custom_graph!{
			[<$V,$W> $($stack)* ] $($rest)*
		}
	};
	{
		[ $($stack:tt)* ]
		impl $($rest:tt)*
	} => {
		custom_graph!{
			[; $($stack)* ] $($rest)*
		}
	};
	{
		//Can only specify wrappers to use after the definition of the backing struct
		//the '<' ensures that the last thing to parse is a generic group
		[  < $($stack:tt)* ]
		use $($rest:tt)*
	} => {
		custom_graph!{
			[ < $($stack)* ] $($rest)*
		}
	};
	{
		// If the previous token was an identifier, require a ','
		[ $w:ident $($stack:tt)* ]
		, $v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[$v $w $($stack)* ] $($rest)*
		}
	};
	{
		// Previous token wasn't an identifier, no ','
		[ $($stack:tt)* ]
		$v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[$v $($stack)* ] $($rest)*
		}
	};
	
	

//helpers
	{
		[$($stack:tt)*]
	} => {
		custom_graph!{@check_for_constaint_traits
			[$($stack)*]}
	};
	{
		// Has constraint traits to implement
		@check_for_constaint_traits
		[$($constraints:ident)* ; $($rest:tt)*]
	}=>{
		custom_graph!{@declare_struct_and_impl_minimum [$($rest)*]}
		custom_graph!{@impl_constraint_traits [$($constraints)* ; $($rest)*]}
	};
	{
		// Doesn't have constraint traits to implement
		@check_for_constaint_traits
		[$($rest:tt)*]
	}=>{
		custom_graph!{@declare_struct_and_impl_minimum [$($rest)*]}
	};
	{
		@declare_struct_and_impl_minimum
		[$($stack:tt)*]
	}=>{
		custom_graph!{@declare_struct [$($stack)*]}
		custom_graph!{@impl_graph_wrapper [$($stack)*]}
		custom_graph!{@impl_base_graph [$($stack)*]}
		custom_graph!{@impl_contained_graph [$($stack)*]}
		custom_graph!{@derive_debug [$($stack)*]}
		custom_graph!{@derive_clone [$($stack)*]}
	};
	{
		@declare_struct
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
		]
	}=>{
		// Define graph struct
		struct $graph_name $(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($con_graph,$base_graph>>)*
				$base_graph<$T1,$T2>
			}
		}
	};
	{
		@declare_struct
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			flag_public
		]
	}=>{
		// Define graph struct
		pub struct $graph_name $(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($con_graph,$base_graph>>)*
				$base_graph <$T1,$T2>
			}
		}
	};
	{
		@impl_graph_wrapper
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		// Impl GraphWrapper
		impl$(<$V1,$W1>)* GraphWrapper for $graph_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			custom_graph!{
				@as_associated
				custom_graph!{
					@in_struct
					$($con_graph,$base_graph >>)*
					$base_graph<$T1,$T2>
				}
			}
			
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
	{
		@impl_base_graph
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		// Impl BaseGraph
		impl$(<$V1,$W1>)* BaseGraph for $graph_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			type Vertex = $T1;
			type Weight = $T2;
			type VertexIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::VertexIter;
			type EdgeIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::EdgeIter;
		
			fn empty_graph() -> Self{
				$graph_name::wrap(
					<Self as GraphWrapper>::Wrapped::empty_graph()
				)
			}
			wrapped_method!{all_vertices(&self) -> Self::VertexIter}
	
			wrapped_method!{all_edges(&self) -> Self::EdgeIter}
			
			wrapped_method!{add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{add_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
			
			wrapped_method!{remove_edge(&mut self, e: BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
		}
	};
	{
		@impl_contained_graph
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		//Impl ConstrainedGraph
		impl$(<$V1,$W1>)* ConstrainedGraph for $graph_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			wrapped_method!{invariant_holds(&self) -> bool}
			
			wrapped_uncon_methods!{}
		}
	};
	{
		@derive_debug
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		// Derive Debug
		impl$(<$V1,$W1>)* std::fmt::Debug for $graph_name$(<$V1,$W1>)*
			where
				$(
					$V1: Vertex + std::fmt::Debug,
					$W1: Weight + std::fmt::Debug,
				)*
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ wraps: {:?} }}", stringify!($graph_name$(<$V1,$W1>)*), self.wraps)
			}
		}
	};
	{
		@derive_clone
		[	$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		//Derive Clone
		impl$(<$V1,$W1>)* Clone for $graph_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			fn clone(&self) -> $graph_name$(<$V1,$W1>)*{
				$graph_name::wrap(self.wraps.clone())
			}
		}
	};
	{
		@impl_constraint_traits
		[	$first_con_trait:ident $($rest_traits:ident)*;
			$($con_graph:ident)*
			<$T1:ty,$T2:ty> $base_graph:ident
			$(<$V1:ident,$W1:ident>)* $graph_name:ident
			$($rest:tt)*
		]
	}=>{
		// Impl the constraint traits
		impl$(<$V1,$W1>)* $first_con_trait for $graph_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{}
		custom_graph!{@impl_constraint_traits
			[
				$($rest_traits)*;
				$($con_graph)*
				<$T1,$T2> $base_graph
				$(<$V1,$W1>)* $graph_name
				$($rest)*
			]
		}
	};
	{
		@impl_constraint_traits
		// When all constraints have been implemented, accept the last ';'
		// but do nothing, as we are done.
		[;$($rest:tt)*]
	}=>{};
	{
		@as_associated
		$($rest:tt)*
	}=>{
		type Wrapped = $($rest)* ;
	};
	{
		@in_struct
		$base_graph:ident<$V:ident, $W:ident>
	}=>{
		$base_graph<$V,$W>
	};
	{
		@in_struct
		$base_graph:ident<$T1:ty,$T2:ty>
	}=>{
		$base_graph<$T1,$T2>
	};
	{
		@in_struct
		$con_graph:ident,$base_graph:ident >>
		$($rest:tt)*
	} => {
		$con_graph<
			custom_graph!{
				@in_struct
				$($rest)*
			}
		>
	};
}









