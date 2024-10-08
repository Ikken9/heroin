use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::networking::edge::Edge;
use crate::networking::node::{Node, NodeId};

pub struct Graph {
    pub nodes: HashMap<NodeId, Node>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new()
        }
    }

    fn add_node(&mut self, id: NodeId, bandwidth: f64) {
        self.nodes.insert(id, Node::new(id, bandwidth));
    }

    fn add_edge(&mut self, source: NodeId, target: NodeId, latency: f64) {
        if let (Some(source_node), Some(target_node)) = (self.nodes.get(&source), self.nodes.get(&target)) {
            let negotiated_bandwidth = source_node.bandwidth.min(target_node.bandwidth);

            if let Some(node) = self.nodes.get_mut(&source) {
                node.edges.push(Edge { target, latency, bandwidth: negotiated_bandwidth });
            }
        }
    }

    fn find_all_paths(&self, source: usize, target: usize, max_hops: usize) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = Vec::new();
        self.dfs(source, target, max_hops, &mut visited, &mut current_path, &mut paths);
        paths
    }

    fn dfs(
        &self,
        current: usize,
        target: usize,
        max_hops: usize,
        visited: &mut HashSet<usize>,
        current_path: &mut Vec<usize>,
        paths: &mut Vec<Vec<usize>>,
    ) {
        if visited.contains(&current) || max_hops == 0 {
            return;
        }

        visited.insert(current);
        current_path.push(current);

        if current == target {
            paths.push(current_path.clone());
        } else {
            for edge in &self.nodes[&current].edges {
                self.dfs(edge.target, target, max_hops - 1, visited, current_path, paths);
            }
        }

        current_path.pop();
        visited.remove(&current);
    }

    // Calculate the cost of a path based on latency and bandwidth
    fn calculate_cost(&self, path: &[usize]) -> (f64, f64) {
        let mut total_latency = 0.0;
        let mut min_bandwidth = f64::MAX;

        for i in 0..path.len() - 1 {
            let current_node = &self.nodes[&path[i]];
            for edge in &current_node.edges {
                if edge.target == path[i + 1] {
                    total_latency += edge.latency;
                    min_bandwidth = min_bandwidth.min(edge.bandwidth);
                    break;
                }
            }
        }

        (total_latency, min_bandwidth)
    }

    // Select the best paths based on cost
    fn select_best_paths(&self, all_paths: Vec<Vec<usize>>, num_paths: usize) -> Vec<Vec<usize>> {
        let mut path_costs: Vec<(Vec<usize>, f64)> = all_paths
            .iter()
            .map(|path| {
                let (latency, bandwidth) = self.calculate_cost(path);
                let cost = latency / bandwidth; // Example cost function
                (path.clone(), cost)
            })
            .collect();

        path_costs.sort_by_key(|(_, cost)| *cost);
        path_costs.into_iter().take(num_paths).map(|(path, _)| path).collect()
    }

    // Balance the load across the selected paths
    fn balance_traffic(&self, selected_paths: Vec<Vec<usize>>, total_traffic: u32) {
        let traffic_per_path = total_traffic / selected_paths.len() as u32;

        for path in selected_paths {
            println!("Sending {} units of traffic through path: {:?}", traffic_per_path, path);
        }
    }
}

#[derive(PartialEq)]
struct State {
    node: NodeId,
    cost: f64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}