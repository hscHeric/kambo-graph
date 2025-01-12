use std::{fmt::Debug, hash::Hash};

use crate::GraphError;

/// A trait defining the core functionality of a graph.
///
/// This trait provides methods to interact with the graph structure,
/// such as retrieving vertices, neighbors, and checking for the existence
/// of vertices and edges. It also includes utility methods to determine
/// graph properties like its directedness, order (number of vertices), and edge count.
///
/// # Associated Types
/// - `Vertex`: Represents the type of the vertices in the graph. It must implement
///   `Eq`, `Hash`, `Clone`, and `Debug` traits.
pub trait Graph {
    /// The type of vertices in the graph.
    ///
    /// Vertices must implement `Eq`, `Hash`, `Clone`, and `Debug`.
    type Vertex: Eq + Hash + Clone + Debug;

    /// Returns an iterator over all vertices in the graph.
    ///
    /// # Returns
    /// A boxed iterator that yields references to the vertices in the graph.
    fn vertices(&self) -> Box<dyn Iterator<Item = &Self::Vertex> + '_>;

    /// Returns an iterator over the neighbors of a given vertex.
    ///
    /// # Parameters
    /// - `v`: A reference to the vertex whose neighbors are to be retrieved.
    ///
    /// # Returns
    /// - `Some(Box<dyn Iterator<Item = &Self::Vertex> + '_>)`: If the vertex exists,
    ///   an iterator over its neighbors.
    /// - `None`: If the vertex does not exist in the graph.
    fn neighbors(&self, v: &Self::Vertex) -> Option<Box<dyn Iterator<Item = &Self::Vertex> + '_>>;

    /// Checks if the graph contains a specific vertex.
    ///
    /// # Parameters
    /// - `v`: A reference to the vertex to check.
    ///
    /// # Returns
    /// - `true` if the vertex exists in the graph.
    /// - `false` otherwise.
    fn contains_vertex(&self, v: &Self::Vertex) -> bool;

    /// Checks if the graph contains an edge between two vertices.
    ///
    /// # Parameters
    /// - `u`: A reference to the source vertex.
    /// - `v`: A reference to the destination vertex.
    ///
    /// # Returns
    /// - `true` if the edge exists in the graph.
    /// - `false` otherwise.
    fn contains_edge(&self, u: &Self::Vertex, v: &Self::Vertex) -> bool;

    /// Determines if the graph is directed.
    ///
    /// # Returns
    /// - `true` if the graph is directed.
    /// - `false` otherwise.
    fn is_directed(&self) -> bool;

    /// Calculates the number of vertices (order) in the graph.
    ///
    /// # Returns
    /// The total number of vertices in the graph.
    fn order(&self) -> usize {
        self.vertices().count()
    }

    /// Calculates the number of edges in the graph.
    ///
    /// # Returns
    /// The total number of edges in the graph.
    ///
    /// # Notes
    /// - For directed graphs, each directed edge is counted once.
    /// - For undirected graphs, each edge is counted once (even though it
    ///   appears as two neighbors).
    fn edge_count(&self) -> usize {
        if self.is_directed() {
            self.vertices()
                .map(|v| self.neighbors(v).map_or(0, |n| n.count()))
                .sum()
        } else {
            self.vertices()
                .map(|v| self.neighbors(v).map_or(0, |n| n.count()))
                .sum::<usize>()
                / 2
        }
    }

    /// Returns a list of isolated vertices in the graph.
    ///
    /// An isolated vertex is a vertex with no neighbors.
    ///
    /// # Returns
    ///
    /// A `Vec` containing all the isolated vertices (cloned) in the graph.

    fn get_isolated_vertices(&self) -> Vec<Self::Vertex> {
        self.vertices()
            .filter(|&v| {
                match self.neighbors(v) {
                    Some(neighbors) => neighbors.count() == 0,
                    None => true, // No neighbors
                }
            })
            .cloned()
            .collect()
    }

    /// Checks if there is any isolated vertex in the graph.
    ///
    /// An isolated vertex is a vertex with no neighbors.
    ///
    /// # Returns
    /// - `true` if there is at least one isolated vertex in the graph.
    /// - `false` otherwise.
    fn has_isolated_vertex(&self) -> bool {
        self.vertices().any(|v| {
            match self.neighbors(v) {
                Some(neighbors) => neighbors.count() == 0,
                None => true, // No neighbors
            }
        })
    }

    // Returns the degree of the vertex in an undirected graph,
    /// or the out-degree in a directed graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex to calculate the degree
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - The degree of the vertex if it exists in the graph, None otherwise
    fn degree(&self, v: &Self::Vertex) -> Option<usize> {
        if !self.contains_vertex(v) {
            None
        } else {
            self.neighbors(v).map(|neighbors| neighbors.count())
        }
    }
}

/// A trait defining the core mutable functionality of a graph.
pub trait GraphMut: Graph {
    /// Adds a vertex to the graph.
    ///
    /// Returns an error if the vertex already exists.
    ///
    /// # Arguments
    ///
    /// * `vertex` - The vertex to add.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::VertexAlreadyExists` if the vertex is already in the graph.
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Result<(), GraphError>;

    /// Removes a vertex from the graph.
    ///
    /// Returns an error if the vertex does not exist.
    ///
    /// # Arguments
    ///
    /// * `vertex` - A reference to the vertex to be removed.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::VertexNotFound` if the vertex does not exist.
    fn remove_vertex(&mut self, vertex: &Self::Vertex) -> Result<(), GraphError>;

    /// Adds an edge to the graph.
    ///
    /// Both vertices must already exist in the graph.
    ///
    /// Returns an error if the edge already exists.
    ///
    /// # Arguments
    ///
    /// * `u` - The source vertex.
    /// * `v` - The target vertex.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::EdgeAlreadyExists` if the edge already exists.
    fn add_edge(&mut self, u: &Self::Vertex, v: &Self::Vertex) -> Result<(), GraphError>;

    /// Removes an edge from the graph.
    ///
    /// Both vertices and the edge must already exist in the graph.
    ///
    /// Returns an error if the edge does not exist.
    ///
    /// # Arguments
    ///
    /// * `u` - The source vertex.
    /// * `v` - The target vertex.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::EdgeNotFound` if the edge does not exist.
    fn remove_edge(&mut self, u: &Self::Vertex, v: &Self::Vertex) -> Result<(), GraphError>;

    /// Remove all isolated vertices from the graph.
    ///
    /// # Returns
    ///
    /// A vector of the removed vertices.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::VertexNotFound` if no isolated vertices are found.
    fn remove_isolated_vertices(&mut self) -> Result<(), GraphError>;
}
