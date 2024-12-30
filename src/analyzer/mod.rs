
pub mod call_graph;

use crate::parser::ast_extractor::FileAst;
use crate::analyzer::call_graph::{CallGraph, NodeIndexMap};


pub fn build_call_graph(file_asts: &[FileAst]) -> CallGraph {
    let mut call_graph = CallGraph::new();

 
    let mut node_map = NodeIndexMap::default();


    for file_ast in file_asts {
        for func in &file_ast.functions {
            call_graph.add_function_node(&mut node_map, &func.name);
        }
    }

    for file_ast in file_asts {
        for func in &file_ast.functions {
            for called_func in &func.calls {
                call_graph.add_call_edge(&mut node_map, &func.name, called_func);
            }
        }
    }

    call_graph
}
