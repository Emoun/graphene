
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

// Decode struct
	{
		pub $($rest:tt)*
	} => {
		custom_graph!{
			[@privacy[pub]]
			$($rest)*
		}
	};
	{
		struct $($rest:tt)*
	} => {
		custom_graph!{
			[ @privacy[] ]
			struct $($rest)*
		}
	};
	{
		[ @privacy $($stack:tt)* ]
		struct $struct_name:ident
		$($rest:tt)*
	} => {
		custom_graph!{
			[@struct_name[$struct_name] @privacy $($stack)*] $($rest)*
		}
	};
	
// Decode Base Graph
	{	// There might not be any generics for the structs so accept
		// backing graph immediately and put empty generics on the stack
		[ @struct_name $($stack:tt)* ]
		as $base_graph_name:ident $($rest:tt)*
	} => {
		custom_graph!{
			[ @base_graph_name[$base_graph_name] @generics[] @struct_name $($stack)* ] $($rest)*
		}
	};
	{	// If the generics of the struct are done, accept the base graph name
		[ @generics $($stack:tt)* ]
		as $base_graph_name:ident $($rest:tt)*
	} => {
		custom_graph!{
			[ @base_graph_name[$base_graph_name] @generics $($stack)* ] $($rest)*
		}
	};

// Decode constraint wrappers
	{
		//Can only specify wrappers to use after the definition of the backing struct
		//the '@generics' ensures that the last thing to parse is a generic group
		[  @generics $($stack:tt)* ]
		use $($rest:tt)*
	} => {
		custom_graph!{
			[@constraint_wrappers[] @generics $($stack)* ] $($rest)*
		}
	};

// Decode constraints
	{	//Can specify constraints after the wrappers are done
		[ @constraint_wrappers[$($wraps:tt)*] $($stack:tt)* ]
		impl $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[] @constraint_wrappers[$($wraps)*] $($stack)* ] $($rest)*
		}
	};
	{	//Can specify constraints after the generics of the base graph
		[ @generics $base_graph_generics:tt @base_graph_name $($stack:tt)* ]
		impl $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[] @constraint_wrappers[] @generics $base_graph_generics @base_graph_name $($stack)* ] $($rest)*
		}
	};
	
//Decode 'where' clause
	{	//If the last thing to be decoded is the base graphs generics
		// Add empty blocks for wrappers
		[ @generics $base_g:tt @base_graph_name $($stack:tt)*] where $($rest:tt)*
	}=>{
		custom_graph!{
			[ 	@constraint_wrappers[]
				@generics $base_g
				@base_graph_name
				$($stack)*
			]
			where
			$($rest)*
		}
	};
	{	//If the last thing to be decoded is the constaint wrappers
		// Add empty blocks for wrappers
		[ @constraint_wrappers $($stack:tt)*] where $($rest:tt)*
	}=>{
		custom_graph!{
			[ 	@constraints[]
				@constraint_wrappers
				$($stack)*
			]
			where
			$($rest)*
		}
	};
	{	//If the last thing to be decoded is the constraints
		// then we are ready to decode the rest of the input
		// as being in the 'where clause'
		[ @constraints $($stack:tt)*] where $($rest:tt)*
	}=>{
		custom_graph!{
			[ 	@where_clause[[]]
				@constraints
				$($stack)*
			]
			$($rest)*
		}
	};
	{	// While grouping 'where' groups if you encounter a ',' but there is more
		// input, it means the group is done, and there is a new one.
		// so ready a new group.
		[ @where_clause[[$($current_group:tt)*] $($rest_groups:tt)*] $($stack:tt)*]
		$next:tt , $($rest:tt)+
	}=>{
		custom_graph!{
			[ 	@where_clause[[][$($current_group)* $next] $($rest_groups)*]
				$($stack)*
			]
			$($rest)+
		}
	};
	{	// While grouping 'where' groups if you encounter a ',' and there is no more
		// input, the where clause is done.
		[ @where_clause[[$($current_group:tt)*] $($rest_groups:tt)*] $($stack:tt)*]
		$next:tt ,
	}=>{
		custom_graph!{
			[ 	@where_clause[[$($current_group)* $next] $($rest_groups)*]
				$($stack)*
			]
		}
	};
	{	// While grouping 'where' groups if you dont encounter a ','
		// you add the token to the current group
		[ @where_clause[[$($current_group:tt)*] $($rest_groups:tt)*] $($stack:tt)*]
		$next:tt $($rest:tt)*
	}=>{
		custom_graph!{
			[ 	@where_clause[[$($current_group)* $next] $($rest_groups)*]
				$($stack)*
			]
			$($rest)*
		}
	};
//Decode generics
	{ //Start generics after struct declaration
		[ @struct_name $($stack:tt)*]
		< $($rest:tt)*
	}=>{
		custom_graph!{
			 [@generics[<] @struct_name $($stack)* ] $($rest)*
		}
	};
	{ //Start generics after base graph declaration
		[ @base_graph_name $($stack:tt)*]
		< $($rest:tt)*
	}=>{
		custom_graph!{
			 [@generics[<] @base_graph_name $($stack)* ] $($rest)*
		}
	};
	{ //Stop generics
		[@generics[$($generics:tt)*] $($stack:tt)*]
		> $($rest:tt)*
	}=>{
		custom_graph!{
			[@generics[$($generics)* >]  $($stack)* ] $($rest)*
		}
	};
	{ //Continue generics
		[@generics[$($generics:tt)*] $($stack:tt)*]
		$next:tt $($rest:tt)*
	}=>{
		custom_graph!{
			 [@generics[$($generics)* $next] $($stack)* ] $($rest)*
		}
	};
// Decode list if wrappers
	{
		// If the previous token was an identifier, require a ','
		[ @constraint_wrappers[$w:ident $($prev:tt)*] $($stack:tt)* ]
		, $v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraint_wrappers[$v $w $($prev)*] $($stack)* ] $($rest)*
		}
	};
	{
		// Previous token wasn't an identifier, no ','
		[@constraint_wrappers[$($prev:tt)*] $($stack:tt)* ]
		$v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraint_wrappers[$v $($prev)*] $($stack)* ] $($rest)*
		}
	};
	
// Decode list if constraints
	{
		// If the previous token was an identifier, require a ','
		[ @constraints[$w:ident $($prev:tt)*] $($stack:tt)* ]
		, $v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[$v $w $($prev)*] $($stack)* ] $($rest)*
		}
	};
	{
		// Previous token wasn't an identifier, no ','
		[@constraints[$($prev:tt)*] $($stack:tt)* ]
		$v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[$v $($prev)*] $($stack)* ] $($rest)*
		}
	};
	
// Utility decoders
	{
		@add_until
		[[$($into:tt)*] $($stack:tt)*]
		[$next:tt > $($rest:tt)*]
		>
	}=>{
	
	};
	
// No more input to decode
	{	//If the last thing to be decoded is generics, there must not be any
		// wrappers, constraints or 'where' defined.
		// Therefore, define empty wrappers
		[ @generics $($stack:tt)*]
	} => {
		custom_graph!{ [@constraint_wrappers[] @generics $($stack)*]}
	};
	{	//If the last thing to be decoded are wrappers,
		// there must not be constraints or 'where' defined.
		// Therefore, define empty constraints
		[ @constraint_wrappers $($stack:tt)*]
	} => {
		custom_graph!{ [@constraints[] @constraint_wrappers $($stack)*]}
	};
	{	//If the last thing to be decoded are constraints,
		// there must not be a 'where' defined.
		// Therefore, define an empty 'where' block
		[ @constraints $($stack:tt)*]
	} => {
		custom_graph!{ [@where_clause[] @constraints $($stack)*]}
	};
	{	// If the last block is 'where' we are done decoding
		[ @where_clause $($stack:tt)*]
	} => {
		custom_graph!{@declare_struct_and_impl_minimum [@where_clause $($stack)*]}
		custom_graph!{@impl_constraint_traits [@where_clause $($stack)*]}
	};
	
//expand functions
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
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy[]
		]
	}=>{
		// Define graph struct
		struct $struct_name $(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($constraint_wrappers,$base_graph_name>>)*
				$base_graph_name<$T1,$T2>
			}
		}
	};
	{
		@declare_struct
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy[pub]
		]
	}=>{
		// Define graph struct
		pub struct $struct_name $(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($constraint_wrappers,$base_graph_name>>)*
				$base_graph_name <$T1,$T2>
			}
		}
	};
	{
		@impl_graph_wrapper
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl GraphWrapper
		impl$(<$V1,$W1>)* GraphWrapper for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			custom_graph!{
				@as_associated
				custom_graph!{
					@in_struct
					$($constraint_wrappers,$base_graph_name >>)*
					$base_graph_name<$T1,$T2>
				}
			}
			
			fn wrap(g: Self::Wrapped) -> Self{
				$struct_name{wraps: g}
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
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl BaseGraph
		impl$(<$V1,$W1>)* BaseGraph for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			type Vertex = $T1;
			type Weight = $T2;
			type VertexIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::VertexIter;
			type EdgeIter = <<Self as GraphWrapper>::Wrapped as BaseGraph>::EdgeIter;
		
			fn empty_graph() -> Self{
				$struct_name::wrap(
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
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		//Impl ConstrainedGraph
		impl$(<$V1,$W1>)* ConstrainedGraph for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			wrapped_method!{invariant_holds(&self) -> bool}
			
			wrapped_uncon_methods!{}
		}
	};
	{
		@derive_debug
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Derive Debug
		impl$(<$V1,$W1>)* std::fmt::Debug for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* + std::fmt::Debug,)*
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ wraps: {:?} }}", stringify!($struct_name$(<$V1,$W1>)*), self.wraps)
			}
		}
	};
	{
		@derive_clone
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		//Derive Clone
		impl$(<$V1,$W1>)* Clone for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{
			fn clone(&self) -> $struct_name$(<$V1,$W1>)*{
				$struct_name::wrap(self.wraps.clone())
			}
		}
	};
	{
		@impl_constraint_traits
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$first_con_trait:ident $($constraints:tt)*]
			@constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl the constraint traits
		impl$(<$V1,$W1>)* $first_con_trait for $struct_name$(<$V1,$W1>)*
			where $($($where_clause)* ,)*
		{}
		custom_graph!{@impl_constraint_traits
			[	@where_clause[$([$($where_clause)*])*]
				@constraints[$($constraints)*]
				@constraint_wrappers[$($constraint_wrappers:tt)*]
				@generics[<$T1,$T2>] @base_graph_name[$base_graph_name]
				@generics[$(<$V1,$W1>)*] @struct_name[$struct_name]
				@privacy $privacy
			]
		}
	};
	{
		@impl_constraint_traits
		// When all constraints have been implemented, accept the empty constraints
		// but do nothing, as we are done.
		[ @where_clause[$($where_clause:tt)*] @constraints[] $($rest:tt)*]
	}=>{};
	{
		@as_associated
		$($rest:tt)*
	}=>{
		type Wrapped = $($rest)* ;
	};
	{
		@in_struct
		$base_graph_name:ident<$V:ident, $W:ident>
	}=>{
		$base_graph_name<$V,$W>
	};
	{
		@in_struct
		$base_graph_name:ident<$T1:ty,$T2:ty>
	}=>{
		$base_graph_name<$T1,$T2>
	};
	{
		@in_struct
		$con_graph:ident,$base_graph_name:ident >>
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







