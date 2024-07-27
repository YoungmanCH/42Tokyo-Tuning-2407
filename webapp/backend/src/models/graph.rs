use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());
        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(from_node_id, 0);
        heap.push(Reverse((0, from_node_id)));

        while let Some(Reverse((dist, node_id))) = heap.pop() {
            if node_id == to_node_id {
                return dist;
            }

            if dist > *distances.get(&node_id).unwrap_or(&i32::MAX) {
                continue;
            }

            if let Some(edges) = self.edges.get(&node_id) {
                for edge in edges {
                    let new_dist = dist.saturating_add(edge.weight);
                    if new_dist < *distances.get(&edge.node_b_id).unwrap_or(&i32::MAX) {
                        distances.insert(edge.node_b_id, new_dist);
                        heap.push(Reverse((new_dist, edge.node_b_id)));
                    }
                }
            }
        }

        *distances.get(&to_node_id).unwrap_or(&i32::MAX)
    }
}