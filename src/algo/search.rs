use crate::core::{Graph, Edge, Directedness};

///
/// DFS
///
///
/// ### Notes
///
/// _Why isn't `on_exit` a closure?_
///
/// There are Three possibilities for an API using closures:
///
/// 1. Direct closure : This requires DFS to be generic on the closures type.
/// this is possible but means DFS cannot be used in places where its explicit type is needed.
/// I.e. it cannot be a field in a different struct nor be returned by a function.
///
/// 2. Referenced closure: If DFS takes a reference to a closure, it no longer needs to be generic
/// on the closures type. However, it limits where DFS can be used, since its now bound by the
/// lifetime of the reference. It also doesn't solve the issue with other struct using DFS,
/// because you can't have the closure anywhere when not using the DFS.
///
/// 3. Boxed closure: Technically possible, but requires `std` and imposes allocations.
///
/// This solution is as flexible as nr. 1, but solves the issue with naming the closures type.
/// In essence, we are simulating a closure by have `on_exit` be a function and taking `on_exit_args`,
/// thats basically what a closure is.
///
pub struct DFS<'a, G, F>
	where
		G:'a + Graph,
{
	graph: &'a G,
	visited: Vec<G::Vertex>,
	
	///
	/// The vertex on the stack, and whether on_exit should be called upon popping.
	///
	stack: Vec<(G::Vertex, bool)>,
	on_exit: fn(G::Vertex, &mut F),
	on_exit_arg: F,
}

impl<'a, G, F> DFS<'a, G, F>
	where
		G:'a + Graph,
{
	pub fn new(g: &'a G, v: G::Vertex, on_exit: fn(G::Vertex, &mut F), on_exit_arg: F) -> Self
	{
		Self{graph: g, visited: Vec::new(), stack: vec![(v, true)], on_exit, on_exit_arg }
	}
	
	pub fn visited(&self, v: G::Vertex) -> bool
	{
		self.visited.contains(&v)
	}
	
}

impl<'a, G> DFS<'a, G, ()>
	where
		G:'a + Graph,
{
	pub fn new_simple(g: &'a G, v: G::Vertex) -> Self
	{
		fn do_nothing<T>(_: T, _: &mut ()){}
		Self::new(g, v, do_nothing, ())
	}
}

impl<'a, G, F> Iterator for DFS<'a, G, F>
	where
		G:'a + Graph,
{
	type Item = G::Vertex;
	
	fn next(&mut self) -> Option<Self::Item> {
		/*	The meaning of markers:
			If its on the stack it means we are still visiting it or its children.
			
			If its exit marked, it means when we are finished visiting it and its children,
			we will cal the 'on_exit' closure on it, and then pop it.
			If its not exit marked, it means this instance of it on the stack was never used for
			visiting this vertex's children and we just pop it, without calling the closure.
			
			If it is marked visited it means it means we are either visiting its children, or we
			are finished doing so. Either way, it shouldn't go on the stack again at any point.
		 */
		
		// Pop any vertices that we are done visiting (and since its on the top of the stack,
		// we must be done visiting its children).
		while self.visited(self.stack.last()?.0.clone()) {
			let last = self.stack.pop()?;
			
			// If its exit marked, call the closure on it.
			if last.1 {
				(self.on_exit)(last.0, &mut self.on_exit_arg);
			}
		}
		
		// Get the top of the stack. This is necessarily a non-visited vertex.
		// If the stack is empty, then this will return none
		let to_return = self.stack.last()?.0.clone();
		
		// Mark visited
		self.visited.push(to_return);
		// Exit mark, since we will use it for exploring its children
		self.stack.last_mut()?.1 = true;
		
		// Explore children
		for e in self.graph.edges_incident_on(to_return.clone())
		{
			let child =
				if to_return == e.source() { e.sink() }
				// In a directed graph, we have to skip incoming edges
				else if G::Directedness::directed() { continue }
				else { e.source() };
			
			if !self.visited(child.clone()) {
				// Push to stack without exit mark
				self.stack.push((child, false));
			}
		}
		Some(to_return)
	}
}


