mod cli;
pub mod errors;
pub mod parser;
pub mod analyzer;
pub mod visualizer;


use crate::cli::CliArgs;
use crate::errors::AppError;
use crate::parser::parse_source_files;
use crate::analyzer::build_call_graph;
use crate::visualizer::generate_graphviz_image;
use clap::Parser;

fn main() -> Result<(), AppError> {

    let args = CliArgs::parse();
    
    let file_asts = parse_source_files(&args.input)?;

    let graph = build_call_graph(&file_asts);

    generate_graphviz_image(&graph, &args.output)?;

    println!("Call graph successfully generated at: {}", args.output);
    Ok(())
}
