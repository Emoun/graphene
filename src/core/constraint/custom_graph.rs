
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
	
// Stack builders
	{
		[ @privacy $($stack:tt)* ]
		struct $struct_name:ident
		$($rest:tt)*
	} => {
		custom_graph!{
			[@struct_name[$struct_name] @privacy $($stack)*] $($rest)*
		}
	};
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
//------------------------------------------------------- Generic handling
	/*{ //Start generics
		@generics
		[$($stack:tt)*]
		< $($rest:tt)*
	}=>{
		custom_graph!{
			@generics [[<]$($stack)* ] $($rest)*
		}
	};
	{ //Stop generics after struct name
		@generics
		[[$($generics:tt)*] @struct_name $($stack:tt)*]
		> $($rest:tt)*
	}=>{
		custom_graph!{
			[@generics[$($generics)* >] @struct_name  $($stack)* ] $($rest)*
		}
	};
	{ //Stop generics after base graph definition
		@generics
		[[$($generics:tt)*] @base_graph_name $($stack:tt)*]
		> $($rest:tt)*
	}=>{
		custom_graph!{
			[@generics[$($generics)* >] @@base_graph_name  $($stack)* ] $($rest)*
		}
	};
	{ //Continue generics
		@generics
		[[$($generics:tt)*] $($stack:tt)*]
		$next:tt $($rest:tt)*
	}=>{
		custom_graph!{
			@generics [[$($generics)* $next] $($stack)* ] $($rest)*
		}
	};*/
	{
		[ $($stack:tt)* ]
		<$V:tt,$W:tt> $($rest:tt)*
	} => {
		custom_graph!{
			[@generics[<$V,$W>] $($stack)* ] $($rest)*
		}
	};
//--------------------------------------------------------
	{
		//Can only specify wrappers to use after the definition of the backing struct
		//the '@generics' ensures that the last thing to parse is a generic group
		[  @generics $($stack:tt)* ]
		use $($rest:tt)*
	} => {
		custom_graph!{
			[[] @generics $($stack)* ] $($rest)*
		}
	};
	{	//Can specify constraints after the wrappers are done
		[ [$($wraps:tt)*] $($stack:tt)* ]
		impl $($rest:tt)*
	} => {
		custom_graph!{
			[[] @constraint_wrappers[$($wraps)*] $($stack)* ] $($rest)*
		}
	};
	{	//Can specify constraints after the generics of the base graph
		[ @generics $base_graph_generics:tt @base_graph_name $($stack:tt)* ]
		impl $($rest:tt)*
	} => {
		custom_graph!{
			[[] @constraint_wrappers[] @generics $base_graph_generics @base_graph_name $($stack)* ] $($rest)*
		}
	};
	{
		// If the previous token was an identifier, require a ','
		[ [$w:ident $($prev:tt)*] $($stack:tt)* ]
		, $v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[[$v $w $($prev)*] $($stack)* ] $($rest)*
		}
	};
	{
		// Previous token wasn't an identifier, no ','
		[[$($prev:tt)*] $($stack:tt)* ]
		$v:ident $($rest:tt)*
	} => {
		custom_graph!{
			[[$v $($prev)*] $($stack)* ] $($rest)*
		}
	};
	{	//If the last thing to be decoded is generics, there must not be any
		// wrappers or constraints defined.
		// Therefore, define empty wrappers
		[ @generics $($stack:tt)*]
	} => {
		custom_graph!{ [@constraint_wrappers[] @generics $($stack)*]}
	};
	{	//If the last thing to be decoded are wrappers,
		// there must not be constraints defined.
		// Therefore, define empty constraints
		[ @constraint_wrappers $($stack:tt)*]
	} => {
		custom_graph!{ [@constraints[] @constraint_wrappers $($stack)*]}
	};
	{	//If the last block does not have a name, it must be the constraints block.
		// Therefore, flag it.
		[ [$($constraints:tt)*] $($stack:tt)*]
	} => {
		custom_graph!{ [@constraints[$($constraints)*] $($stack)*]}
	};
	
//expand functions
	{	//If the last thing to be decoded is constraints, we are done
		[ @constraints $($stack:tt)*]
	} => {
		custom_graph!{@declare_struct_and_impl_minimum [@constraints $($stack)*]}
		custom_graph!{@impl_constraint_traits [@constraints $($stack)*]}
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
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy[]
		]
	}=>{
		// Define graph struct
		struct $struct_name $(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight)*
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
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy[pub]
		]
	}=>{
		// Define graph struct
		pub struct $struct_name $(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight)*
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
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		// Impl GraphWrapper
		impl$(<$V1,$W1>)* GraphWrapper for $struct_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
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
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		// Impl BaseGraph
		impl$(<$V1,$W1>)* BaseGraph for $struct_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
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
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		//Impl ConstrainedGraph
		impl$(<$V1,$W1>)* ConstrainedGraph for $struct_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			wrapped_method!{invariant_holds(&self) -> bool}
			
			wrapped_uncon_methods!{}
		}
	};
	{
		@derive_debug
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		// Derive Debug
		impl$(<$V1,$W1>)* std::fmt::Debug for $struct_name$(<$V1,$W1>)*
			where
				$(
					$V1: Vertex + std::fmt::Debug,
					$W1: Weight + std::fmt::Debug,
				)*
		{
			fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result{
				write!(f, "{} {{ wraps: {:?} }}", stringify!($struct_name$(<$V1,$W1>)*), self.wraps)
			}
		}
	};
	{
		@derive_clone
		[	@constraints[$($constraints:tt)*] @constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		//Derive Clone
		impl$(<$V1,$W1>)* Clone for $struct_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{
			fn clone(&self) -> $struct_name$(<$V1,$W1>)*{
				$struct_name::wrap(self.wraps.clone())
			}
		}
	};
	{
		@impl_constraint_traits
		[	@constraints[$first_con_trait:ident $($constraints:tt)*]
			@constraint_wrappers[$($constraint_wrappers:tt)*]
			@generics[<$T1:ty,$T2:ty>] @base_graph_name[$base_graph_name:ident]
			@generics[$(<$V1:ident,$W1:ident>)*] @struct_name[$struct_name:ident]
			@privacy $privacy:tt
		]
	}=>{
		// Impl the constraint traits
		impl$(<$V1,$W1>)* $first_con_trait for $struct_name$(<$V1,$W1>)*
			where $($V1: Vertex,$W1: Weight,)*
		{}
		custom_graph!{@impl_constraint_traits
			[
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
		[ @constraints[] $($rest:tt)*]
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









