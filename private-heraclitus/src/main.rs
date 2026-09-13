mod ast;
mod parser;
mod engine;
mod auth;
mod dashboard;
mod storage;
mod api;

use pest::Parser;
use std::fs;
use std::path::Path;

fn compile_spec_file(path: &Path, engine: &mut engine::VerificationEngine) {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    let mut lemma_id = String::new();
    for line in file_content.lines() {
        let clean_line = line.trim().to_lowercase();
        if clean_line.starts_with("lemma_id") || clean_line.starts_with("id") {
            if let Some(val_part) = line.split(':').nth(1) {
                lemma_id = val_part.replace('"', "").replace('\'', "").trim().to_string();
                break;
            }
        }
    }
    if lemma_id.is_empty() {
        if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
            lemma_id = file_stem.split('_').take(2).collect::<Vec<&str>>().join("_");
        }
    }
    
    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&file_content) {
        if let Ok(parsed_tree) = parser::SVEParser::parse(parser::Rule::program, &code_block) {
            let mut context = ast::CompilerContext::new();
            parser::build_ast(parsed_tree, &mut context);
            engine.register_lemma(lemma_id, context);
        }
    }
}

fn main() {
    println!("==================================================");
    println!("=== HERACLITUS GRAPH TOPOLOGY RUNTIME ===========");
    println!("==================================================");

    let mut v_engine = engine::VerificationEngine::new();
    let specs_dir = Path::new("../public-logoslib/specs");
    
    if specs_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(specs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    compile_spec_file(&path, &mut v_engine);
                }
            }
        }
    }

    println!("Operational Core Library Ingested. Total Lemmas: {}", v_engine.compile_dictionary.len());

    // --- TEST 1: Clean Hierarchical Document Pipeline ---
    println!("\n[Test 1: Running Topological Sort on Valid Argument Dependency Tree...]");
    let valid_dependencies = vec![
        ("LOGOS_003_adhominem".to_string(), "LOGOS_002_adhoc".to_string()),
        ("LOGOS_002_adhoc".to_string(), "LOGOS_001_accident".to_string()),
    ];

    match v_engine.verify_dependency_topology(valid_dependencies) {
        Ok(sorted_sequence) => println!("  ↳ STATUS: SUCCESS. Optimized Compilation Order: {:?}", sorted_sequence),
        Err(_) => println!("  ↳ STATUS: FAILED. Unexpected sorting exception."),
    }

    // --- TEST 2: Circular Argument Intercept Loop ---
    println!("\n[Test 2: Running Topological Sort on Malformed Circular Dependency Loop...]");
    let circular_dependencies = vec![
        ("LOGOS_006_baserate".to_string(), "LOGOS_007_beggingquestion".to_string()),
        ("LOGOS_007_beggingquestion".to_string(), "LOGOS_006_baserate".to_string()), // The loop-closing edge
    ];

    match v_engine.verify_dependency_topology(circular_dependencies) {
        Ok(_) => println!("  ↳ STATUS: PASSED. (Error: Engine failed to catch loop)"),
        Err(loop_path) => {
            println!("  ↳ STATUS: COMPILE BLOCKED (Cyclical Reference Conflict Detected!)");
            println!("    Identified Trap Core Node: {:?}", loop_path);
        }
    }
    println!("--------------------------------------------------");
}
