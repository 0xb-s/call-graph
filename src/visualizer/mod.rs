pub mod graphviz;

use crate::analyzer::call_graph::CallGraph;
use crate::errors::AppError;
use self::graphviz::render_dot_image;


pub fn generate_graphviz_image(graph: &CallGraph, output_path: &str) -> Result<(), AppError> {
    render_dot_image(graph, output_path)?;
    Ok(())
}
