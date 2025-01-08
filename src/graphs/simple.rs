use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use crate::{
    traits::weighted::{WeightedGraph, WeightedGraphMut},
    Graph, GraphMut,
};

/// Represents a simple graph using an adjacency list (no self-loops or multiple edges)
#[derive(Clone, Debug)]
pub struct SimpleGraph<V, W = ()>
where
    V: Eq + Hash + Clone + Debug,
    W: Clone + Debug,
{
    vertices: HashMap<V, HashSet<V>>,
    edges: HashMap<(V, V), W>,
    directed: bool,
}

impl<V> Graph for SimpleGraph<V, ()>
where
    V: Eq + Hash + Clone + Debug,
{
    type Vertex = V;

    fn vertices(&self) -> Box<dyn Iterator<Item = &Self::Vertex> + '_> {
        Box::new(self.vertices.keys())
    }

    fn neighbors(&self, v: &Self::Vertex) -> Option<Box<dyn Iterator<Item = &Self::Vertex> + '_>> {
        self.vertices.get(v).map(|neighbors| {
            Box::new(neighbors.iter()) as Box<dyn Iterator<Item = &Self::Vertex> + '_>
        })
    }

    fn contains_vertex(&self, v: &Self::Vertex) -> bool {
        self.vertices.contains_key(v)
    }

    fn contains_edge(&self, u: &Self::Vertex, v: &Self::Vertex) -> bool {
        self.edges.contains_key(&(u.clone(), v.clone()))
    }

    fn is_directed(&self) -> bool {
        self.directed
    }
}

impl<V> GraphMut for SimpleGraph<V, ()>
where
    V: Eq + Hash + Clone + Debug,
{
    fn add_vertex(&mut self, vertex: Self::Vertex) -> Result<(), crate::GraphError> {
        if self.contains_vertex(&vertex) {
            Err(crate::GraphError::EdgeAlreadyExists)
        } else {
            self.vertices.insert(vertex, HashSet::new());
            Ok(())
        }
    }

    fn remove_vertex(&mut self, vertex: &Self::Vertex) -> Result<(), crate::GraphError> {
        if !self.contains_vertex(vertex) {
            return Err(crate::GraphError::VertexNotFound);
        }

        let vertex_clone = vertex.clone();
        self.vertices.remove(vertex);

        self.edges.retain(|(u, v), _| u != vertex && v != vertex);

        for neighbors in self.vertices.values_mut() {
            neighbors.remove(&vertex_clone);
        }

        Ok(())
    }

    fn add_edge(&mut self, u: &Self::Vertex, v: &Self::Vertex) -> Result<(), crate::GraphError> {
        if !self.contains_vertex(u) || !self.contains_vertex(v) {
            return Err(crate::GraphError::VertexNotFound);
        }

        if self.contains_edge(u, v) {
            return Err(crate::GraphError::EdgeAlreadyExists);
        }

        // Se o grafo não for dirigido ele adiciona a aresta u em v
        self.vertices.get_mut(u).unwrap().insert(v.clone());
        if !self.directed {
            self.vertices.get_mut(v).unwrap().insert(u.clone());
        }

        Ok(())
    }

    fn remove_edge(&mut self, u: &Self::Vertex, v: &Self::Vertex) -> Result<(), crate::GraphError> {
        if !self.contains_edge(u, v) {
            return Err(crate::GraphError::EdgeNotFound);
        }

        self.edges.remove(&(u.clone(), v.clone()));

        self.vertices.get_mut(u).unwrap().remove(v);
        if !self.directed {
            self.vertices.get_mut(v).unwrap().remove(u);
        }

        Ok(())
    }
}

impl<V, W> WeightedGraph for SimpleGraph<V, W>
where
    V: Clone + Debug + Hash + Eq,
    W: Clone + Debug,
    SimpleGraph<V, W>: Graph<Vertex = V>,
{
    type Weight = W;

    fn edge_weight(&self, u: &Self::Vertex, v: &Self::Vertex) -> Option<&Self::Weight> {
        self.edges.get(&(u.clone(), v.clone()))
    }
}

impl<V, W> WeightedGraphMut for SimpleGraph<V, W>
where
    V: Clone + Debug + Hash + Eq,
    W: Clone + Debug,
    SimpleGraph<V, W>: GraphMut<Vertex = V>,
{
    fn set_edge_weight(
        &mut self,
        u: &Self::Vertex,
        v: &Self::Vertex,
        weight: Self::Weight,
    ) -> Result<(), crate::GraphError> {
        if !self.contains_vertex(u) || !self.contains_vertex(v) {
            return Err(crate::GraphError::VertexNotFound);
        }

        self.vertices.get_mut(u).unwrap().insert(v.clone());
        if !self.directed {
            self.vertices.get_mut(v).unwrap().insert(u.clone());
        }

        self.edges.insert((u.clone(), v.clone()), weight.clone());
        if !self.directed {
            self.edges.insert((v.clone(), u.clone()), weight.clone());
        }

        Ok(())
    }
}

impl<V, W> Default for SimpleGraph<V, W>
where
    V: Eq + Hash + Clone + Debug,
    W: Clone + Debug,
{
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            directed: false,
        }
    }
}
