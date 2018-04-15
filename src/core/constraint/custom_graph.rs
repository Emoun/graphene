
///
///Declares a custom graph with a specific set of constraints:
///
///```
///#[macro_use]
///extern crate graphene;
///
///use graphene::core::{
///    Vertex,Weight,GraphWrapper,
///    constraint::{Unique,Undirected,UniqueGraph,UndirectedGraph}
///};
///use graphene::common::{AdjListGraph};
///
///custom_graph!{
///    pub struct MyGraph<V,W>
///    as AdjListGraph<V,W>
///    use UndirectedGraph, UniqueGraph
///    impl Undirected, Unique
///    where V: Vertex, W: Weight
///}
///fn main(){}
///```
///
///### Syntax
///
///* __`(pub) struct`__ : Defines the name of the graph struct to be created.
/// Also defines the visibility of it in the usual syntax.
///
///* __`as`__ : The graph implementation to back up the created struct.
///The struct is therefore just a wrapper around the backing implmentation.
///
///* __`use`__ : Graph wrappers to wrap around the backing graph to ensure
///the chosen constraints are supported. The struct is then wrapped around
///these wrappers.
///
///* __`impl`__ : Which constraints the struct implements. It is the users
///responsibility to ensure that the struct, its backing graph and the wrappers
///together maintain the required invariants of there constraints.
///
///* __`where`__ : Trait bounds on any generic type.
///
///The __`struct`__ and __`as`__ clauses are mandatory. __`use`__ is optional,
///but, if present, must be followed by the __`impl`__ clause, while __`impl`__ may
///appear without the __`use`__ clause. The __`where`__ is optional as
///needed by the type system like any other __`where`__ clause.
///
///The above invocation of the macro can be read as follows:
///
/// Declare the public struct `MyGraph` as an `AdjListGraph` using
/// `UndirectedGraph` and `UniqueGraph` to implement the constraints
/// `Undirected` and `Unique`.
///
/// ### Produces
///
/// The resulting graph is a wrapper around the types given in the __`as`__
/// and __`use`__ clauses. The macro guarantees that the struct implements
/// the basic graph traits (most notably `BaseGraph` and `ConstrainedGraph`.
/// Additionally, the traits in the __`impl`__ clause (and they should be constraint
/// traits only), are implemented regardless of whether their invariants
/// are maintained by the resulting struct. Therefore, the user should make
/// sure that the specified backing graph and wrappers actually implement
/// the constraint type that are specified.
///
/// Other than that, the struct is ready to use as a full blown graph
/// without any need to implement anything else (excepting task specific
/// functionality).
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
		$(,)*impl $($rest:tt)*
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
		[ @constraint_wrappers $($stack:tt)*] $(,)*where $($rest:tt)*
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
		[ @constraints $($stack:tt)*] $(,)* where $($rest:tt)*
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
	
// Decode list of constraints
	{
		// The first contraint does have to have ',' before it
		[ @constraints[] $($stack:tt)* ]
		$v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[[$v]] $($stack)* ] $($rest)*
		}
	};
	{
		// next constraint needs a comma
		[@constraints[ $($other_constraints:tt)+ ] $($stack:tt)* ]
		, $next:ident $($rest:tt)*
	} => {
		custom_graph!{
			[@constraints[[$next]$($other_constraints)+] $($stack)* ] $($rest)*
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
		[ @constraint_wrappers $($stack:tt)*] $(,)*
	} => {
		custom_graph!{ [@constraints[] @constraint_wrappers $($stack)*]}
	};
	{	//If the last thing to be decoded are constraints,
		// there must not be a 'where' defined.
		// Therefore, define an empty 'where' block
		[ @constraints $($stack:tt)*] $(,)*
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
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy[]
		]
	}=>{
		// Define graph struct
		struct $struct_name $($struct_generics)*
			where $($($where_clause)* ,)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($constraint_wrappers,$base_graph_name>>)*
				$base_graph_name $($base_generics)*
			}
		}
	};
	{
		@declare_struct
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy[pub]
		]
	}=>{
		// Define graph struct
		pub struct $struct_name $($struct_generics)*
			where $($($where_clause)* ,)*
		{
			wraps:
			custom_graph!{
				@in_struct
				$($constraint_wrappers,$base_graph_name>>)*
				$base_graph_name $($base_generics)*
			}
		}
	};
	{
		@impl_graph_wrapper
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*]@struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl GraphWrapper
		impl$($struct_generics)* $crate::core::GraphWrapper for $struct_name $($struct_generics)*
			where $($($where_clause)* ,)*
		{
			custom_graph!{
				@as_associated
				custom_graph!{
					@in_struct
					$($constraint_wrappers,$base_graph_name >>)*
					$base_graph_name $($base_generics)*
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
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl BaseGraph
		impl$($struct_generics)* $crate::core::BaseGraph for $struct_name$($struct_generics)*
			where $($($where_clause)* ,)*
		{
			type Vertex = $T1;
			type Weight = $T2;
			type VertexIter = <<Self as $crate::core::GraphWrapper>::Wrapped as $crate::core::BaseGraph>::VertexIter;
			type EdgeIter = <<Self as $crate::core::GraphWrapper>::Wrapped as $crate::core::BaseGraph>::EdgeIter;
		
			fn empty_graph() -> Self{
				$struct_name::wrap(
					<Self as $crate::core::GraphWrapper>::Wrapped::empty_graph()
				)
			}
			wrapped_method!{all_vertices(&self) -> Self::VertexIter}
	
			wrapped_method!{all_edges(&self) -> Self::EdgeIter}
			
			wrapped_method!{add_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{remove_vertex(&mut self, v: Self::Vertex) -> Result<(), ()>}
			
			wrapped_method!{add_edge(&mut self, e: $crate::core::BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
			
			wrapped_method!{remove_edge(&mut self, e: $crate::core::BaseEdge<Self::Vertex, Self::Weight>) -> Result<(), ()>}
		}
	};
	{
		@impl_contained_graph
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		//Impl ConstrainedGraph
		impl$($struct_generics)* $crate::core::ConstrainedGraph for $struct_name$($struct_generics)*
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
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Derive Debug
		impl$($struct_generics)* std::fmt::Debug for $struct_name$($struct_generics)*
			where $($($where_clause)* + std::fmt::Debug,)*
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ wraps: {:?} }}", stringify!($struct_name$($struct_generics)*), self.wraps)
}
		}
	};
	{
		@derive_clone
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		//Derive Clone
		impl$($struct_generics)* Clone for $struct_name$($struct_generics)*
			where $($($where_clause)* ,)*
		{
			fn clone(&self) -> $struct_name$($struct_generics)*{
				$struct_name::wrap(self.wraps.clone())
			}
		}
	};
	{
		@impl_constraint_traits
		[	@where_clause[$([$($where_clause:tt)*])*]
			@constraints[[$first_con_trait:ident] $($constraints:tt)*]
			@constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[$($base_generics:tt)*] @base_graph_name[$base_graph_name:ident]
			@generics[$($struct_generics:tt)*] @struct_name[$struct_name:ident]
			@privacy$privacy:tt
		]
	}=>{
		// Impl the constraint traits
		impl$($struct_generics)* $first_con_trait for $struct_name$($struct_generics)*
			where $($($where_clause)* ,)*
		{}
		custom_graph!{@impl_constraint_traits
			[	@where_clause[$([$($where_clause)*])*]
				@constraints[$($constraints)*]
				@constraint_wrappers[$($constraint_wrappers)*]
				@generics[$($struct_generics)*] @base_graph_name[$base_graph_name]
				@generics[$($struct_generics)*] @struct_name[$struct_name]
				@privacy $privacy
			]
		}
	};
	{
		@impl_constraint_traits
		// When all but all constraints have been implemented, as we are done.
		[	@where_clause $where_clause:tt
			@constraints[] $($rest_stack:tt)*]
	}=>{};
	{
		@as_associated
		$($rest:tt)*
	}=>{
		type Wrapped = $($rest)* ;
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
	{
		@in_struct
		$base_graph_name:ident $($base_generics:tt)*
	}=>{
		$base_graph_name$($base_generics)*
	};
}
