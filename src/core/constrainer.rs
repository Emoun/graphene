use crate::core::Graph;

///
/// An implementing type constrains a base graph implementation.
///
/// Multiple levels of constrainers are supported.
///
pub trait Constrainer: Sized + Graph
{
	///
	/// The base graph implementation being constrained
	///
	type BaseGraph: BaseGraph;
	
	///
	/// The next level of constraints.
	///
	type Constrained: Constrainer<BaseGraph=Self::BaseGraph>;
	
	///
	/// Unconstrain only this level's constraints, maintaining
	/// the next level's constraints
	///
	fn unconstrain_single(self) -> Self::Constrained;
	
	///
	/// Constrains a graph that is also already constrained.
	///
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>;
	
	///
	/// Fully unconstrains this type, returning the base graph implementation type
	///
	fn unconstrain(self) -> Self::BaseGraph{
		self.unconstrain_single().unconstrain()
	}
	
	///
	/// Fully constrains a base graph implementation type with all
	/// levels of constraints.
	///
	fn constrain(g: Self::BaseGraph) -> Result<Self, ()>
	{
		Self::constrain_single(Self::Constrained::constrain(g)?)
	}
}

///
/// A marker trait that specifies that the type is a base implementation of a graph
/// with fixed constraints that cannot be removed.
///
/// This can be further constrained, but cannot be unconstrained.
/// `Constrainer` is automatically implemented for any graph that implments this trait,
/// however `Constrainer`'s methods do nothing, returning the same object.
/// Conceptually, a base graph is its own constrainer.
///
/// NOTE: When specialization is supported, Constrainer should not be implemented for
/// any type implementing `BaseGraph`. (currently not possible, since it will result in multiple
/// implementations. Specialization will make it possible.)
///
pub trait BaseGraph: Sized + Graph
{
	fn constrain<G>(self) -> Result<G, ()>
		where G: Constrainer<BaseGraph=Self>
	{
		G::constrain(self)
	}
}

impl<G> Constrainer for G
	where G: BaseGraph
{
	type BaseGraph = Self;
	type Constrained = Self;
	
	fn constrain_single(g: Self::Constrained) -> Result<Self, ()>{
		Ok(g)
	}
	fn unconstrain_single(self) -> Self::Constrained{
		self
	}
	fn unconstrain(self) -> Self::BaseGraph{
		self
	}
	fn constrain(g: Self::BaseGraph) -> Result<Self, ()>
	{
		Ok(g)
	}
}