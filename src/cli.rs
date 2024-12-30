
use clap::{Parser};


#[derive(Parser, Debug)]
#[command(author = "0xb-s", version = "0.1.0", about = "Generates a function call graph for Rust codebases.")]
pub struct CliArgs {

    #[arg(short, long)]
    pub input: String,


    #[arg(short, long, default_value = "call_graph.png")]
    pub output: String,
}
