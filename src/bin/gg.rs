extern crate petgraph;

use petgraph::graphmap::{GraphMap, UnGraphMap, DiGraphMap};
use petgraph::graph::{Graph, UnGraph, DiGraph};
use petgraph::dot::{Dot, Config};
use petgraph::algo;
use std::collections::{HashSet, HashMap};

type FotGNode = String;
type FotEdge = u16;

pub type FDiGraph = Graph<FotGNode, FotEdge>;

pub fn main() {
    let raw_edges: Vec<(String, String)> = [("fn0", "fn1"), ("fn2", "fn3"), ("fn0", "fn2")].iter().map(|&(s1, s2)| (s1.to_string(), s2.to_string())).collect();
    let raw_nodes: Vec<String> = vec!["fn0".to_string(), "fn1".to_string(), "fn2".to_string(), "fn3".to_string()];
    let ids: Vec<&str> = Vec::new();
    {
        let mut og: FDiGraph = DiGraph::new();
        let mut idx_map = HashMap::new();
        for n in raw_nodes {
            let nn = n.clone();
            let idx = og.add_node(n);
            idx_map.insert(nn, idx);
        }
        for (n1, n2) in raw_edges {
            let idx1 = idx_map[&n1];
            let idx2 = idx_map[&n2];
            og.add_edge(idx1, idx2, 1);
        }
        let dot = Dot::with_config(&og, &[Config::EdgeNoLabel]);
        println!("{:?}\n", dot);

        for n in og.node_indices() {
            let scores = algo::dijkstra(&og, n, None, |e| *e.weight());
            println!("n={:?}, scores={:?}", n, scores);
        }
    }
}