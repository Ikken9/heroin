use crate::networking::edge::Edge;

pub type NodeId = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub bandwidth: f64,
    pub edges: Vec<Edge>
}

impl Node {
    pub fn new(id: NodeId, bandwidth: f64) -> Self {
        Node {
            id,
            bandwidth,
            edges: Vec::new(),
        }
    }
}