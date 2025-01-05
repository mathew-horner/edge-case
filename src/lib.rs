#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

/// A weighted directional graph.
pub struct WeightedDigraph<K: Eq + Hash, V, E> {
    vertex_map: HashMap<K, V>,
    repr: GraphRepr<K, E>,
}

impl<K: Eq + Hash, V, E> Default for WeightedDigraph<K, V, E> {
    fn default() -> Self {
        Self {
            vertex_map: HashMap::new(),
            repr: GraphRepr::default(),
        }
    }
}

impl<K: Eq + Hash, V, E> WeightedDigraph<K, V, E> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.vertex_map.get(key)
    }

    pub fn connect(&mut self, u: &K, v: K, weight: E) {
        self.repr.connect(u, v, weight);
    }

    pub fn get_edge(&self, u: &K, v: &K) -> Option<&E> {
        self.repr.get_edge(u, v)
    }
}

impl<K: Clone + Eq + Hash, V, E> WeightedDigraph<K, V, E> {
    pub fn insert(&mut self, key: K, value: V) {
        self.vertex_map.insert(key.clone(), value);
        self.repr.insert(key);
    }
}

/// An unweighted directional graph.
pub struct Digraph<K: Eq + Hash, V> {
    vertex_map: HashMap<K, V>,
    repr: GraphRepr<K, ()>,
}

/// A weighted undirectional graph.
pub struct WeightedGraph<K: Eq + Hash, V, E> {
    vertex_map: HashMap<K, V>,
    repr: GraphRepr<K, E>,
}

/// An unweighted undirectional graph.
pub struct Graph<K: Eq + Hash, V> {
    vertex_map: HashMap<K, V>,
    repr: GraphRepr<K, ()>,
}

// TODO: This should probably be a trait?

enum GraphRepr<K: Eq + Hash, E> {
    AdjacencyList(AdjacencyList<K, E>),
    AdjacencyMatrix(AdjacencyMatrix<K, E>),
}

impl<K: Eq + Hash, E> Default for GraphRepr<K, E> {
    fn default() -> Self {
        Self::AdjacencyList(AdjacencyList::new())
    }
}

impl<K: Eq + Hash, E> GraphRepr<K, E> {
    fn insert(&mut self, key: K) {
        match self {
            Self::AdjacencyList(list) => list.insert(key),
            Self::AdjacencyMatrix(matrix) => matrix.insert(key),
        }
    }

    fn connect(&mut self, u: &K, v: K, weight: E) {
        match self {
            Self::AdjacencyList(list) => list.connect(u, v, weight),
            Self::AdjacencyMatrix(matrix) => matrix.connect(u, v, weight),
        }
    }

    fn get_edge(&self, u: &K, v: &K) -> Option<&E> {
        match self {
            Self::AdjacencyList(list) => list.get_edge(u, v),
            Self::AdjacencyMatrix(matrix) => matrix.get_edge(u, v),
        }
    }
}

struct AdjacencyList<K: Eq + Hash, E> {
    list: HashMap<K, HashMap<K, E>>,
}

impl<K: Eq + Hash, E> AdjacencyList<K, E> {
    fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K) {
        self.list.insert(key, HashMap::new());
    }

    fn connect(&mut self, u: &K, v: K, weight: E) {
        let Some(edge_list) = self.list.get_mut(u) else {
            return;
        };
        edge_list.insert(v, weight);
    }

    fn get_edge(&self, u: &K, v: &K) -> Option<&E> {
        self.list.get(u)?.get(v)
    }
}

struct AdjacencyMatrix<K: Hash, E> {
    idx_map: HashMap<K, usize>,
    matrix: Vec<Vec<Option<E>>>,
}

impl<K: Eq + Hash, E> AdjacencyMatrix<K, E> {
    fn new() -> Self {
        Self {
            idx_map: HashMap::new(),
            matrix: Vec::new(),
        }
    }

    fn insert(&mut self, key: K) {
        for row in self.matrix.iter_mut() {
            row.push(None);
        }
        let size = self.matrix.len();
        self.matrix.push((0..size + 1).map(|_| None).collect());
        self.idx_map.insert(key, size);
    }

    fn connect(&mut self, u: &K, v: K, weight: E) {
        let Some((u_idx, v_idx)) = self.idxs(u, &v) else {
            return;
        };
        self.matrix[u_idx][v_idx] = Some(weight);
    }

    fn get_edge(&self, u: &K, v: &K) -> Option<&E> {
        let (u_idx, v_idx) = self.idxs(u, v)?;
        self.matrix[u_idx][v_idx].as_ref()
    }

    fn idxs(&self, u: &K, v: &K) -> Option<(usize, usize)> {
        Some((*self.idx_map.get(u)?, *self.idx_map.get(v)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_write_vertex() {
        let mut graph = WeightedDigraph::<_, _, ()>::new();
        graph.insert(1, "hi");
        graph.insert(2, "bye");
        assert_eq!(graph.get(&1), Some(&"hi"));
        assert_eq!(graph.get(&2), Some(&"bye"));
        assert_eq!(graph.get(&3), None);
    }

    #[test]
    fn connect_vertices() {
        let mut graph = WeightedDigraph::new();
        graph.insert(1, "hi");
        graph.insert(2, "bye");
        graph.connect(&1, 2, 10_000);
        assert_eq!(graph.get_edge(&1, &2), Some(&10_000));
        assert_eq!(graph.get_edge(&2, &1), None);
    }
}
