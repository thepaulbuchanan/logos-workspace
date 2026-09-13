mod ast;
mod parser;
mod engine;

use pest::Parser;
use std::fs;
use std::path::Path;

fn compile_spec_file(path_str: &str, engine: &mut engine::VerificationEngine, id: &str) -> Option<ast::CompilerContext> {
    let spec_path = Path::new(path_str);
    if !spec_path.exists() {
        println!("Skipping: File not found at {:?}", spec_path);
        return None;
    }

    let file_content = fs::read_to_string(spec_path).expect("Unable to read spec file");
    
    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&file_content) {
        match parser::SVEParser::parse(parser::Rule::program, &code_block) {
            Ok(parsed_tree) => {
                let mut context = ast::CompilerContext::new();
                parser::build_ast(parsed_tree, &mut context);
                engine.register_lemma(id.to_string(), context.clone());
                Some(context)
            }
            Err(e) => {
                eprintln!("Parser Compilation Exception for {}: {:?}", id, e);
                None
            }
        }
    } else {
        None
    }
}

fn main() {
    println!("=== HERACLITUS MULTI-MODULE COMPILER ARCHITECTURE ===");
    let mut v_engine = engine::VerificationEngine::new();

    // Ingest and register the evolving seed cohort
    compile_spec_file("../public-logoslib/specs/LOGOS_001_accident.md", &mut v_engine, "LOGOS_001");
    compile_spec_file("../public-logoslib/specs/LOGOS_002_adhoc.md", &mut v_engine, "LOGOS_002");
    let hominem_ctx = compile_spec_file("../public-logoslib/specs/LOGOS_003_adhominem.md", &mut v_engine, "LOGOS_003");

    println!("Registered Core Lemmas in Active Memory Library: {:?}", v_engine.compile_dictionary.keys());

    // Execute intake cross-reference simulation on Fallacy #3
    if let Some(ctx) = hominem_ctx {
        println!("\n[Simulating Intake Analysis for incoming user submission text...]");
        let verdict = v_engine.cross_reference_submission(&ctx);
        
        println!("\n--- Core Engine Epistemic Verdict Report ---");
        match verdict {
            engine::VerificationStatus::SorryFree => println!("Verdict: SORRY-FREE LAKEBUILD VALIDATED. Cryptographic signature generated."),
            engine::VerificationStatus::StructuralFallacyDetected { code, error_context, .. } => {
                println!("Verdict: COMPILATION ABORTED (Structural Logical Fallacy Detected)");
                println!("Trigger Code : {}", code);
                println!("Error Spec   : {}", error_context);
            }
            engine::VerificationStatus::BoundedWithStubs(stubs) => println!("Verdict: BUILD BOUNDED. Missing dependency tracks: {:?}", stubs),
        }
    }
}
