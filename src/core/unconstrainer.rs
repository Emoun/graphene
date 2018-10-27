

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
