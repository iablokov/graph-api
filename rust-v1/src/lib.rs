use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// I should be hashable
pub struct Node<I,T> where I: Hash + Clone
{
    id   : u32, // unique id for private use
    pub name : I,   // unique name for public use
    pub data : T
}

pub struct Graph<I,T> where I: Hash + Clone + Eq + PartialEq
{
    nodes   : HashMap<u32, Node<I,T>>,
    edges   : HashMap<u32, HashSet<u32>>,
    names   : HashMap<I, u32>,
    next_id : u32
}

impl<I,T> Graph<I,T> where I: Hash + Clone + Eq + PartialEq
{
    pub fn new() -> Graph<I,T>
    {
        Graph { nodes : HashMap::new(), edges : HashMap::new(), names : HashMap::new(), next_id : 0 }
    }

    pub fn add_node(&mut self, name: I, data: T) -> u32
    {
        let id = self.next_id;
        let node = Node { id : id, name : name.clone(), data : data };
        self.nodes.insert(id, node);
        self.names.insert(name, id);
        self.next_id += 1;
        id
    }

    pub fn find_node_by_id(&self, id: u32) -> Option<&Node<I,T>>
    {
        self.nodes.get(&id)
    }

    pub fn find_node_by_name(&self, name: &I) -> Option<&Node<I,T>>
    {
        match self.names.get(name)
        {
            Some(id) => self.nodes.get(id),
            None     => None
        }
    }
}

