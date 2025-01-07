use std::error::Error;
use std::fmt::Debug;

/// Enum representing errors related to graph operations.
///
/// This enum includes common errors that might occur during graph manipulation,
/// such as invalid operations, missing vertices or edges, and duplicates.
///
/// Example usage:
///
/// ```rust
/// use crate::GraphError;
///
/// fn example() -> Result<(), GraphError> {
///     Err(GraphError::VertexNotFound)
/// }
///
/// fn main() {
///     match example() {
///         Ok(_) => println!("Operation succeeded."),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
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
    /// Implementation of the `Display` trait for user-friendly error messages.
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
