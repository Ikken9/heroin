use crate::networking::node::NodeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub target: NodeId,
    pub latency: f64,
    pub bandwidth: f64
}

pub fn calculate_edge_weight(delay: u32, bandwidth: f64, pkt_size: f64) -> f64 {
    (pkt_size / bandwidth) + delay as f64
}

pub fn edge_cost(latency: f64, bandwidth: f64) -> f64 {
    latency / bandwidth
}