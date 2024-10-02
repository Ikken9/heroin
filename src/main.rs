use crate::networking::edge::Edge;
use crate::networking::graph::Graph;
use crate::networking::node::{Node, NodeId};

mod networking;

fn main() {
    let edge_a_b = Edge { to: NodeId('b'), weight: 2.0 };
    let edge_b_a = Edge { to: NodeId('a'), weight: 2.0 };
    let edge_a_c = Edge { to: NodeId('c'), weight: 6.0 };
    let edge_c_a = Edge { to: NodeId('a'), weight: 6.0 };
    let edge_b_d = Edge { to: NodeId('d'), weight: 5.0 };
    let edge_d_b = Edge { to: NodeId('b'), weight: 5.0 };
    let edge_c_d = Edge { to: NodeId('d'), weight: 8.0 };
    let edge_d_c = Edge { to: NodeId('c'), weight: 8.0 };
    let edge_d_e = Edge { to: NodeId('e'), weight: 1.0 };
    let edge_d_g = Edge { to: NodeId('g'), weight: 4.0 };
    let edge_e_g = Edge { to: NodeId('g'), weight: 2.0 };
    let edge_e_f = Edge { to: NodeId('f'), weight: 6.0 };
    let edge_g_f = Edge { to: NodeId('f'), weight: 3.0 };

    let node_a = Node { id: NodeId('a'), edges: vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a] };
    let node_b = Node { id: NodeId('b'), edges: vec![edge_b_a, edge_a_b, edge_b_d, edge_d_b] };
    let node_c = Node { id: NodeId('c'), edges: vec![edge_c_a, edge_a_c, edge_c_d, edge_d_c] };
    let node_d = Node { id: NodeId('d'), edges: vec![edge_d_b, edge_b_d, edge_d_c, edge_c_d, edge_d_e, edge_d_g] };
    let node_e = Node { id: NodeId('e'), edges: vec![edge_e_f, edge_e_g] };
    let node_f = Node { id: NodeId('f'), edges: vec![] };
    let node_g = Node { id: NodeId('g'), edges: vec![edge_g_f] };

    let mut network = Graph::new();
    network.add_vertex(node_a.clone());
    network.add_vertex(node_b);
    network.add_vertex(node_c);
    network.add_vertex(node_d);
    network.add_vertex(node_e);
    network.add_vertex(node_f);
    network.add_vertex(node_g);

    network.connect_nodes(node_a);
}

