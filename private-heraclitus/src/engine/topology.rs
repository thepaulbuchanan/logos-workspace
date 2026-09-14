use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

pub fn verify_dependency_topology(dependencies: Vec<(String, String)>) -> Result<Vec<String>, Vec<String>> {
    let mut graph = DiGraph::<String, ()>::new();
    let mut node_indices = HashMap::new();

    for (parent, child) in &dependencies {
        node_indices.entry(parent.clone()).or_insert_with(|| graph.add_node(parent.clone()));
        node_indices.entry(child.clone()).or_insert_with(|| graph.add_node(child.clone()));
    }

    for (parent, child) in &dependencies {
        let parent_idx = node_indices.get(parent).unwrap();
        let child_idx = node_indices.get(child).unwrap();
        graph.add_edge(*parent_idx, *child_idx, ());
    }

    match toposort(&graph, None) {
        Ok(sorted_indices) => {
            let sorted_nodes = sorted_indices.into_iter().map(|idx| graph[idx].clone()).collect();
            Ok(sorted_nodes)
        }
        Err(cycle) => {
            let loop_node = graph[cycle.node_id()].clone();
            Err(vec![loop_node, "Cyclical Loop Closed".to_string()])
        }
    }
}
