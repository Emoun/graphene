use crate::core::{Graph, Edge, Directedness};

pub struct DFS<'a, G>
	where
		G:'a + Graph,
{
	graph: &'a G,
	visited: Vec<G::Vertex>,
	
	///
	/// The vertex on the stack, and whether on_exit should be called upon popping.
	///
	stack: Vec<(G::Vertex, bool)>,
	on_exit: &'a mut dyn FnMut(G::Vertex),
}

impl<'a, G> DFS<'a,G>
	where
		G:'a + Graph,
{
	pub fn new(g: &'a G, v: G::Vertex, on_exit: &'a mut dyn FnMut(G::Vertex)) -> Self
	{
		Self{graph: g, visited: Vec::new(), stack: vec![(v, true)], on_exit }
	}
	
	pub fn visited(&self, v: G::Vertex) -> bool
	{
		self.visited.contains(&v)
	}
	
}

impl<'a, G> Iterator for DFS<'a,G>
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
				(self.on_exit)(last.0);
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


