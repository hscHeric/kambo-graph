use std::error::Error;
use std::fmt::Debug;

/// Enum representing errors related to graph operations.
///
/// This enum includes common errors that might occur during graph manipulation,
/// such as invalid operations, missing vertices or edges, and duplicates.
///
#[derive(Debug)]
pub enum GraphError {
    /// Vertex not found in the graph.
    VertexNotFound,
    /// Vertex already exists in the graph.
    VertexAlreadyExists,
    /// Edge already exists in the graph.
    EdgeAlreadyExists,
    /// Edge not found in the graph.
    EdgeNotFound,
    /// An invalid operation was requested for the graph.
    ///
    /// A detailed reason can be provided as a `String`.
    InvalidOperation(String),
}

impl std::fmt::Display for GraphError {
    /// Formats the error for user-friendly display.
    ///
    /// This implementation converts each variant of `GraphError` into a
    /// readable string message. It is primarily used to print or log errors.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::VertexNotFound => write!(f, "Vertex not found."),
            GraphError::VertexAlreadyExists => write!(f, "Vertex already exists."),
            GraphError::EdgeAlreadyExists => write!(f, "Edge already exists."),
            GraphError::EdgeNotFound => write!(f, "Edge not found."),
            GraphError::InvalidOperation(msg) => {
                write!(f, "Invalid operation: {}", msg)
            }
        }
    }
}

impl Error for GraphError {}
