use crate::analyzer::call_graph::CallGraph;
use crate::errors::AppError;
use petgraph::dot::{Dot, Config};
use std::process::Command;
use std::fs;
use which::which;


pub fn render_dot_image(graph: &CallGraph, output_path: &str) -> Result<(), AppError> {
    let dot_contents = format!("{:?}", Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]));

    let temp_dot_path = "temp_call_graph.dot";
    fs::write(temp_dot_path, &dot_contents)?;

    let dot_path = which("dot").map_err(|_| {
        AppError::Generic("Graphviz 'dot' command not found in PATH. Please install Graphviz.".into())
    })?;

    Command::new(dot_path)
        .arg("-Tpng")
        .arg(temp_dot_path)
        .arg("-o")
        .arg(output_path)
        .spawn()?
        .wait()?;

   
    let _ = fs::remove_file(temp_dot_path);

    Ok(())
}
