extern crate petgraph;

use petgraph::{Undirected, Directed, Graph};
use petgraph::dot::{Dot, Config};
use petgraph::algo;
use std::collections::{HashSet, HashMap};

#[derive(Eq, PartialEq, Clone, Debug, Hash, Default, PartialOrd)]
pub struct FotGNode {
    id: String,
    label: Option<String>,
    shape: Option<String>,
}

impl FotGNode {
    pub fn new(id: &str, label: Option<&str>) -> FotGNode {
        FotGNode { id: id.to_string(), label: label.map(|s| s.to_string()), shape: None }
    }

    pub fn matched(&self, other: &FotGNode) -> bool {
        self.id == other.id
    }
}

pub type FotEdge = (FotGNode, FotGNode);
pub type FotDiGraph = Graph<FotGNode, FotGNode, Directed>;
pub type FotUnGraph = Graph<FotGNode, FotGNode, Undirected>;

#[derive(Debug)]
pub struct RawGraph {
    directed: bool,
    label: String,
    nodes: Vec<FotGNode>,
    edges: Vec<FotEdge>,
}

fn foo() {
    let mut og: Graph<FotGNode, FotGNode> = Graph::new();
    let raw_edges: Vec<(String, String)> = [("fn0", "fn1"), ("fn2", "fn3"), ("fn0", "fn2")].iter().map(|&(s1, s2)| (s1.to_string(), s2.to_string())).collect();
    let mut node_map = HashMap::new();
    for i in 0..4 {
        let id = format!("fn{}", i);
        let node = FotGNode::new(&id, Some(&id));
        let idx = og.add_node(node);
        node_map.insert(id, idx);
    }
    let mut edges = Vec::new();
    for (id1, id2) in raw_edges {
        let idx1 = node_map[&id1];
        let idx2 = node_map[&id2];
        edges.push((idx1, idx2));
    }
    og.extend_with_edges(&edges);
    let dot = Dot::with_config(&og, &[Config::EdgeNoLabel]);
    println!("{:?}", dot);
    let node_indices: Vec<_> = node_map.values().collect();
    println!("indices: {:?}", node_indices);
    for idx in node_indices {
        println!("{:?}", og[*idx]);
    }

//    for &i in node_indices.iter() {
//        let scores: Vec<_> = algo::dijkstra(&og, *i, None, |e| *e.weight()).into_iter().collect();
//        println!("i: {:?}, scores: {:?}", i, scores);
//    }
}


fn main() {
    foo();
}

