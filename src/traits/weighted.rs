use std::fmt::Debug;

use crate::{Graph, GraphError, GraphMut};

/// A trait for weighted graphs, extending the core functionality of a graph.
pub trait WeightedGraph: Graph {
    /// The type of weights associated with the edges.
    ///
    /// This type must implement `Clone` and `Debug` to allow copying and debugging.
    type Weight: Clone + Debug;

    /// Returns the weight of the edge between two vertices, if it exists.
    ///
    /// # Arguments
    ///
    /// * `u` - A reference to the source vertex.
    /// * `v` - A reference to the target vertex.
    ///
    /// # Returns
    ///
    /// * `Some(&Self::Weight)` - If an edge exists between the vertices, returns a reference to its weight.
    /// * `None` - If no edge exists between the vertices.
    fn edge_weight(&self, u: &Self::Vertex, v: &Self::Vertex) -> Option<&Self::Weight>;
}

/// A trait for mutable operations on weighted graphs.
pub trait WeightedGraphMut: WeightedGraph + GraphMut {
    /// Adds or updates the weight of an edge between two vertices.
    ///
    /// If the edge weight does not exist, it will be created with the specified weight.
    /// If the edge already exists, its weight will be updated.
    ///
    /// # Arguments
    ///
    /// * `u` - A reference to the source vertex.
    /// * `v` - A reference to the target vertex.
    /// * `weight` - The weight to assign to the edge.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation succeeds.
    /// * `Err(GraphError::VertexNotFound)` - If one or both vertices do not exist in the graph.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::VertexNotFound` if one or both of the vertices are not in the graph.
    fn set_edge_weight(
        &mut self,
        u: &Self::Vertex,
        v: &Self::Vertex,
        weight: Self::Weight,
    ) -> Result<(), GraphError>;
}
