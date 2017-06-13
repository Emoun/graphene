use super::*;
use self::Operations::*;

///
/// Defines mutating operations that can be executed on a graph.
///
pub enum Operations<V,W>
	where
		V: Vertex,
		W: Weight,
{
	AddVertex(V),
	AddEdge(BaseEdge<V,W>),
	RemoveVertex(V),
	RemoveEdge(BaseEdge<V,W>),
}

///
/// Handles execution of a set of mutating operations on a given graph
/// 'atomically', in the sense that constraints are only verified after
/// all the operation have been executed.
///
/// The operations are executed lazily, i.e. only when `constrain()` is called.
///
///
pub struct Unconstrainer<'a,V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: 'a + ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
{
	graph: &'a mut G,
	operations: Vec<Operations<V,W>>,
}

impl<'a,V,W,Vi,Ei,G> Unconstrainer<'a,V,W,Vi,Ei,G>
	where
		V: Vertex,
		W: Weight,
		Vi: VertexIter<V>,
		Ei: EdgeIter<V,W>,
		<Vi as IntoIterator>::IntoIter: ExactSizeIterator,
		<Ei as IntoIterator>::IntoIter: ExactSizeIterator,
		G: ConstrainedGraph<Vertex=V,Weight=W,VertexIter=Vi,EdgeIter=Ei>
{
	
	pub fn new(g: &'a mut G) -> Self{
		Unconstrainer{graph:g, operations: Vec::new()}
	}
	
	///
	/// Enqueues an addition of the given vertex to the list of operations.
	///
	pub fn add_vertex(mut self, v: V) -> Self{
		self.operations.push(Operations::AddVertex(v));
		self
	}
	
	///
	/// Enqueues a removal of the given vertex to the list of operations.
	///
	pub fn remove_vertex(mut self, v:V) -> Self{
		self.operations.push(Operations::RemoveVertex(v));
		self
	}
	
	///
	/// Enqueues an addition of the given edge to the list of operations.
	///
	pub fn add_edge(mut self, e: BaseEdge<V,W>) -> Self{
		self.operations.push(Operations::AddEdge(e));
		self
	}
	
	///
	/// Enqueues a removal of the given edge to the list of operations.
	///
	pub fn remove_edge(mut self, e: BaseEdge<V,W>) -> Self{
		self.operations.push(Operations::RemoveEdge(e));
		self
	}
	
	///
	/// Executes all operations in the list, ensuring that each one isn't rejected
	/// and then checks the invariant of the graph.
	///
	/// If any operation is rejected, or the invariant of the graph does not hold after
	/// all operations are executed, any change made is rolled back, and `Err` is returned.
	///
	/// Guarantees that if `Ok` is returned, then the graph upholds its constraint invariant.
	///
	///
	pub fn constrain(mut self) -> Result<(),()> {
		match self.execute_unconstrained_operations(){
			Err(ops) =>{
				// One of the operations failed, therefore roll back changes
				self.rollback_operations(ops);
				Err(())
			}
			Ok(()) =>{
				// All operations accepted, test invariant
				if self.graph.invariant_holds() {
					Ok(())
				}else{
					let op_count = self.operations.len();
					self.rollback_operations(op_count);
					Err(())
				}
			}
		}
	}
	
	fn rollback_operations(&mut self, rollback_count:usize) {
		let ref operations = self.operations;
		let ref mut graph = self.graph;
		
		for j in (0..(rollback_count)).rev(){
			unsafe{
				match operations[j] {
					AddVertex(v) => graph.uncon_remove_vertex(v),
					AddEdge(e) => graph.uncon_remove_edge(e),
					RemoveVertex(v) => graph.uncon_add_vertex(v),
					RemoveEdge(e) => graph.uncon_add_edge(e),
				}.unwrap()
			}
		}
	}
	
	fn execute_unconstrained_operations(&mut self) -> Result<(),usize>{
		let ref operations = self.operations;
		let ref mut graph = self.graph;
		
		let mut i = 0;
		while i < operations.len() {
			match unsafe {
				match operations[i] {
					AddVertex(v) => graph.uncon_add_vertex(v),
					AddEdge(e) => graph.uncon_add_edge(e),
					RemoveVertex(v) => graph.uncon_remove_vertex(v),
					RemoveEdge(e) => graph.uncon_remove_edge(e),
				}
			}{
				Err(()) =>{
					/* Operation i failed
					 */
					// Rollback all operations that executed before the (i+1)'th
					return Err(i);
				}
				Ok(())	=> i += 1,
			}
		}
		Ok(())
	}
}

///
/// Defines a graph which has some constraint on how it is mutated.
///
/// An example could be a graph which prohibits duplicate edges, ignoring wrights, called a
/// unique graph. Such a graph must then be implemented such that adding an edge
/// checks for duplicates and rejects any such.
///
/// More specifically, to uphold the contract of this trait the following must hold:
///
/// - The implementation of `BaseGraph` on the type must uphold the specified constraint. In our example
///  `add_graph()` must reject any edge which is already in the graph.
/// - The methods of this trait must be implemented.
///
/// The following methods must be implemented for this trait:
///
/// - `invariant_holds`: checks the constraint invariant on the current state of the graph
/// and returns whether it holds. In our example, it will go though all edges, and return false
/// if any duplicate is found.
///
/// - `uncon_add_vertex`: Tries to add a vertex without upholding the invariant.
///
/// - `uncon_remove_vertex`: Tries to remove a vertex without upholding the invariant.
///
/// - `uncon_add_edge`: Tries to add an edge without upholding the invariant. In our example, it
/// will add the edge without checking for duplicates. This means that when the call terminates, the
/// graph may not uphold the invariant of no duplicates.
///
/// - `uncon_remove_edge`: Tries to remove an edge without upholding the invariant.
///
/// The `uncon_...` methods are intentionally `unsafe` as they may result in a graph state which
/// does not uphold its own invariant, and should therefore not be used lightly. The real use case
/// for them come from the `unconstrained` default method. By using it, and the `Unconstrainer`
/// it returns, the user can try executing multiple mutating operations at once, and only after
/// that ensure that the graph still upholds its invariant Example:
///
/// ```
/// use graphene::core::*;
/// use graphene::core::constraint::*;
/// use graphene::common::*;
///
/// let mut g = UniqueGraph::<AdjListGraph<u32,()>>::graph(vec![1,2], vec![]).unwrap();
/// let e = BaseEdge::new(1,2,());
///
/// assert!(g.add_edge(e).is_ok());
/// assert!(g.add_edge(e).is_err());
/// assert!(g.unconstrained().add_edge(e).constrain().is_err());
/// assert!(g.unconstrained()
/// 			.add_edge(e)
/// 			.remove_edge(e)
/// 			.constrain()
/// 			.is_ok());
/// ```
/// We can see here that the same edge cannot be added twice with `g.add_edge(e)`.
/// When using `unconstrained()` we first add the edge, and then remove it again. This
/// means the graph will in the end again only have a single edge, which upholds the invariant.
///
///
pub trait ConstrainedGraph: BaseGraph
where
	Self: Sized,
	<Self::VertexIter as IntoIterator>::IntoIter: ExactSizeIterator,
	<Self::EdgeIter as IntoIterator>::IntoIter: ExactSizeIterator,
{
	///
	/// Checks whether the current state of the graph upholds the constraint invariant.
	///
	fn invariant_holds(&self) -> bool;
	
	///
	/// Adds the given vertex to the graph without upholding the constraint invariant.
	///
	/// The only constraint upheld is that of the `BaseGraph` which states no vertex
	/// value duplicates.
	///
	unsafe fn uncon_add_vertex(&mut self, v: Self::Vertex) -> Result<(),()>;
	
	///
	/// Removes the given vertex from the graph without upholding the constraint invariant.
	///
	/// The only constraint upheld is that of the `BaseGraph` which states all edges
	/// must be incident on valid vertices.
	///
	unsafe fn uncon_remove_vertex(&mut self, v: Self::Vertex) -> Result<(),()>;
	
	///
	/// Adds the given edge to the graph without upholding the constraint invariant.
	///
	/// The only constraint upheld is that of the `BaseGraph` which states all edges
	/// must be incident on valid vertices.
	///
	unsafe fn uncon_add_edge(&mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
	///
	/// Removes the given edge from the graph without upholding the constraint invariant.
	///
	///
	unsafe fn uncon_remove_edge(&mut self, e: BaseEdge<Self::Vertex,Self::Weight>) -> Result<(),()>;
	
	///
	/// Returns an `Unconstrainer` connected to the graph.
	///
	fn unconstrained<'a>(&'a mut self) -> Unconstrainer<
		Self::Vertex, Self::Weight, Self::VertexIter, Self::EdgeIter, Self>{
		Unconstrainer::new(self)
	}
	
	
}

