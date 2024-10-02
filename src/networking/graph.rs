use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::networking::node::{Node, NodeId};

pub struct Graph {
    pub vertices: HashMap<NodeId, Node>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self, node: Node) {
        let copy = node.clone();
        let id = copy.id;
        self.vertices.insert(id, copy);
    }

    pub fn connect_nodes(&mut self, start: Node) {
        let mut distances: HashMap<NodeId, f64> = HashMap::new();
        let mut visited: HashSet<NodeId> = HashSet::new();

        let mut priority_queue = BinaryHeap::new();

        distances.insert(start.id.clone(), 0.0);
        priority_queue.push(State { vertex: start.id, cost: 0.0 });

        while let Some(State { vertex: current_vertex, cost: current_distance }) = priority_queue.pop() {
            if !visited.insert(current_vertex) {
                continue;
            }

            if let Some(v) = self.vertices.get(&current_vertex) {
                for neighbor in &v.edges {
                    if let Some(next) = self.vertices.get(&neighbor.to) {
                        let distance = current_distance + neighbor.weight;

                        if distance < *distances.get(&neighbor.to).unwrap_or(&f64::MAX) {
                            distances.insert(neighbor.to.clone(), distance);
                            priority_queue.push(State { vertex: next.id, cost: distance });
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
struct State {
    vertex: NodeId,
    cost: f64
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}