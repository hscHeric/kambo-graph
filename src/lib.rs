//! Kambo Graph is a powerful and flexible graph theory library for Rust.
//!
//! This library provides a collection of traits and implementations for working
//! with different types of graphs and graph algorithms.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::perf)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// Module for handling errors related to graph operations.
///
/// This module provides the `GraphError` enum, which defines a variety of
/// errors that might occur during graph manipulations, such as missing
/// vertices, duplicate edges, or invalid operations.
pub mod error;

/// Graph struct implementations
pub mod graphs;

/// Module defining traits for graph structures.
pub mod traits;

/// Module defining utilities functions
pub mod utils;

pub use error::GraphError;
pub use graphs::simple::SimpleGraph;
pub use traits::graph::Graph;
pub use traits::graph::GraphMut;

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
