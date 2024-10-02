use std::fmt::{Display, Formatter};
use crate::networking::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub edges: Vec<Edge>,
    pub bandwidth: f64,

}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NodeId(pub char);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}