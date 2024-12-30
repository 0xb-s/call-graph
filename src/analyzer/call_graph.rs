use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;


pub type InternalGraph = Graph<String, (), Directed>;

#[derive(Debug)]
pub struct CallGraph {
    pub graph: InternalGraph,
}

#[derive(Default)]
pub struct NodeIndexMap {

    pub map: HashMap<String, NodeIndex>,
}

impl CallGraph {

    pub fn new() -> Self {
        Self {
            graph: InternalGraph::new(),
        }
    }


    pub fn add_function_node(&mut self, node_map: &mut NodeIndexMap, function_name: &str) {
        if !node_map.map.contains_key(function_name) {
            let node_idx = self.graph.add_node(function_name.to_string());
            node_map.map.insert(function_name.to_string(), node_idx);
        }
    }


    pub fn add_call_edge(&mut self, node_map: &mut NodeIndexMap, caller: &str, callee: &str) {
        self.add_function_node(node_map, caller);
        self.add_function_node(node_map, callee);

        let caller_idx = *node_map.map.get(caller).unwrap();
        let callee_idx = *node_map.map.get(callee).unwrap();
        self.graph.add_edge(caller_idx, callee_idx, ());
    }
}
