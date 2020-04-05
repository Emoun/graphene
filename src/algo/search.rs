use crate::core::{property::NonNull, Directedness, Edge, Graph};

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
/// this is possible but means DFS cannot be used in places where its explicit
/// type is needed. I.e. it cannot be a field in a different struct nor be
/// returned by a function.
///
/// 2. Referenced closure: If DFS takes a reference to a closure, it no longer
/// needs to be generic on the closures type. However, it limits where DFS can
/// be used, since its now bound by the lifetime of the reference. It also
/// doesn't solve the issue with other struct using DFS, because you can't have
/// the closure anywhere when not using the DFS.
///
/// 3. Boxed closure: Technically possible, but requires `std` and imposes
/// allocations.
///
/// This solution is as flexible as nr. 1, but solves the issue with naming the
/// closures type. In essence, we are simulating a closure by have `on_exit` be
/// a function and taking `on_exit_args`, that's basically what a closure is.
pub struct DFS<'a, G, F>
where
	G: 'a + Graph,
{
	/// The graph being traversed.
	pub graph: &'a G,

	/// A custom payload, available to the function called upon a vertex exit.
	pub payload: F,
	visited: Vec<G::Vertex>,

	/// The vertex on the stack, and whether on_exit should be called upon
	/// popping.
	stack: Vec<(G::Vertex, bool)>,

	/// Function to call when exiting a vertex.
	///
	/// Provices a reference to the graph, the vertex that is exiting,
	/// and a mutable reference to the payload given to the Dfs.
	on_exit: fn(&G, G::Vertex, &mut F),
}

impl<'a, G, F> DFS<'a, G, F>
where
	G: 'a + Graph,
{
	pub fn new(g: &'a G, on_exit: fn(&G, G::Vertex, &mut F), payload: F) -> Self
	where
		G: NonNull,
	{
		Self {
			graph: g,
			visited: Vec::new(),
			stack: vec![(g.get_vertex(), true)],
			on_exit,
			payload,
		}
	}

	pub fn visited(&self, v: G::Vertex) -> bool
	{
		self.visited.contains(&v)
	}

	/// Pops the next vertex that it is finished visiting off the stack, calling
	/// `on_exit` on it.
	///
	///  If there was nothing to pop and call `on_exit` on, return false,
	/// otherwise returns true.
	pub fn advance_next_exit(&mut self) -> Option<G::Vertex>
	{
		while let Some(last) = self.stack.last()
		{
			if self.visited(last.0.clone())
			{
				let last = self.stack.pop().unwrap();

				// If its exit marked, call the closure on it.
				if last.1
				{
					(self.on_exit)(self.graph, last.0, &mut self.payload);
					return Some(last.0);
				}
			}
			else
			{
				return None;
			}
		}
		None
	}

	pub fn continue_from(&mut self, v: G::Vertex) -> bool
	{
		if !self.visited(v.clone())
		{
			self.stack.push((v, true));
			true
		}
		else
		{
			false
		}
	}
}

impl<'a, G> DFS<'a, G, ()>
where
	G: 'a + NonNull,
{
	pub fn new_simple(g: &'a G) -> Self
	{
		fn do_nothing<G, T>(_: &G, _: T, _: &mut ()) {}
		Self::new(g, do_nothing, ())
	}
}

impl<'a, G, F> Iterator for DFS<'a, G, F>
where
	G: 'a + Graph,
{
	type Item = G::Vertex;

	fn next(&mut self) -> Option<Self::Item>
	{
		// The meaning of markers:
		//
		// If its on the stack it means we are still visiting it or its children.
		//
		// If its exit marked, it means when we are finished visiting it and its
		// children, we will call the 'on_exit' closure on it, and then pop it.
		// If its not exit marked, it means this instance of it on the stack was
		// never used for visiting this vertex's children and we just pop it, without
		// calling the closure.
		//
		// If it is marked visited it means we are either visiting its children, or
		// we are finished doing so. Either way, it shouldn't go on the stack again
		// at any point.

		// Pop any vertices that we are done visiting (and since its on the top of the
		// stack, we must be done visiting its children).
		while self.advance_next_exit().is_some()
		{}

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
			let child = if to_return == e.source()
			{
				e.sink()
			}
			// In a directed graph, we have to skip incoming edges
			else if G::Directedness::directed()
			{
				continue;
			}
			else
			{
				e.source()
			};

			if !self.visited(child.clone())
			{
				// Push to stack without exit mark
				self.stack.push((child, false));
			}
		}
		Some(to_return)
	}
}
