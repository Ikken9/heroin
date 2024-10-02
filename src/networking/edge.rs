use crate::networking::node::NodeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub to: NodeId,
    pub weight: f64
}

pub fn negotiate_bandwidth(bandwidth_first_node: f64, bandwidth_second_node: f64) -> f64 {
    if bandwidth_first_node < bandwidth_second_node {
        return bandwidth_first_node
    }

    bandwidth_second_node
}

pub fn calculate_edge_weight(delay: u32, bandwidth: f64, pkt_size: f64) -> f64 {
    (pkt_size / bandwidth) + delay as f64
}

