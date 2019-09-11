//#![feature(trace_macros)] //trace_macros!(true);
#![recursion_limit="256"]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
mod mock_graph;
mod core;
mod common;
mod algo;
