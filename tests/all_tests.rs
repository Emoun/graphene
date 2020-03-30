//#![feature(trace_macros)] //trace_macros!(true);
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
mod mock_graph;
mod algo;
mod common;
mod core;
