mod ast;
mod parser;
mod engine;

use pest::Parser;
use std::fs;
use std::path::Path;

fn compile_spec_file(path_str: &str) {
    let spec_path = Path::new(path_str);
    if !spec_path.exists() {
        println!("Skipping: File not found at {:?}", spec_path);
        return;
    }

    let file_content = fs::read_to_string(spec_path).expect("Unable to read spec file");
    println!("\nLoading Target Specification Document: {:?}", spec_path);

    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&file_content) {
        match parser::SVEParser::parse(parser::Rule::program, &code_block) {
            Ok(parsed_tree) => {
                let mut context = ast::CompilerContext::new();
                parser::build_ast(parsed_tree, &mut context);
                
                println!("--- Compiler Verification Context Allocation ---");
                println!("Allocated Symbol Primitives: {:?}", context.symbol_table.keys());
                println!("Generated Node Count: {}", context.ast_nodes.len());
            }
            Err(e) => eprintln!("Parser Compilation Exception: {:?}", e),
        }
    }
}

fn main() {
    println!("=== HERACLITUS MULTI-MODULE COMPILER ARCHITECTURE ===");
    
    // Test compilation path scaling loops across our baseline files
    compile_spec_file("../public-logoslib/specs/LOGOS_001_accident.md");
    compile_spec_file("../public-logoslib/specs/LOGOS_002_adhoc.md");
}
