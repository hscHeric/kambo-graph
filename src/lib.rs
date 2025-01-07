//! Kambo Graph is a powerful and flexible graph theory library for Rust.
//!
//! This library provides a collection of traits and implementations for working
//! with different types of graphs and graph algorithms.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Module for handling errors related to graph operations.
///
/// This module provides the `GraphError` enum, which defines a variety of
/// errors that might occur during graph manipulations, such as missing
/// vertices, duplicate edges, or invalid operations.
pub mod error;

/// Module defining traits for graph structures.
pub mod traits;

pub use error::GraphError;
pub use traits::graph::Graph;

/// The current version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
