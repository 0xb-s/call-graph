# Call Graph Visualizer

This project is a CLI tool to generate function call graphs for Rust codebases.  

## How to use ? 
Provide the path to a directory containing `.rs` files, and the tool produces a schematic image of function calls.  

## Requirement
Requires [Graphviz](https://graphviz.org/) for visualizing the call graph.  

## How to use ? 
Run the tool with `cargo run -- --input <path_to_codebase> --output <output_image>`.  

## Note
This tool can be  useful for auditors.
