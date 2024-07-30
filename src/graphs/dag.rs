use std::{collections::{HashMap, HashSet}, hash::Hash};

#[derive(Debug)]
pub struct Neighbors<T> {
    parents: HashSet<T>,
    children: HashSet<T>,
}

impl<T: Eq + Hash> Neighbors<T> {
    pub fn new() -> Neighbors<T> {
        Neighbors {
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }

    pub fn add_parent(&mut self, parent: T) {
        self.parents.insert(parent);
    }

    pub fn add_child(&mut self, child: T) {
        self.children.insert(child);
    }

    pub fn remove_parent(&mut self, parent: T) {
        self.parents.remove(&parent);
    }

    pub fn is_root(&self) -> bool {
        self.parents.is_empty()
    }

    pub fn get_children(&self) -> &HashSet<T> {
        &self.children
    }
}

impl<T: Eq + Hash> Default for Neighbors<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct DAG<T> {
    nodes: HashMap<T, Neighbors<T>>,
}

impl<T: Eq + Hash + Copy + Ord> DAG<T> {
    pub fn get_children(&self, node: T) -> Option<&HashSet<T>> {
        self.nodes.get(&node).map(Neighbors::get_children)
    }

    pub fn delete_root_node(&mut self, node: T) -> Result<(), &str> {
        if !self.nodes[&node].is_root() {
            return Err("Node is not a root node");
        }
        for child in self.nodes[&node].get_children().clone() {
            self.nodes.get_mut(&child).unwrap().remove_parent(node);
        }
        self.nodes.remove(&node);
        Ok(())
    }

    fn get_root_nodes(&self) -> HashSet<T> {
        self.nodes
            .iter()
            .filter(|(_, neigh)| neigh.is_root())
            .map(|(node, _)| *node)
            .collect()
    }

    pub fn root_nodes_exist(&self) -> bool {
        !self.get_root_nodes().is_empty()
    }

    pub fn get_min_root_node(&mut self, except: &HashSet<T>) -> Option<T> {
        self.get_root_nodes()
            .iter()
            .filter(|node| !except.contains(node))
            .min()
            .copied()
    }

    pub fn new() -> DAG<T> {
        DAG {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.nodes.entry(from).or_default().add_child(to);
        self.nodes.entry(to).or_default().add_parent(from);
    }
}

impl<T: Eq + Hash + Copy + Ord> Default for DAG<T> {
    fn default() -> Self {
        Self::new()
    }
}
