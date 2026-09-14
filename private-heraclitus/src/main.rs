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
    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
        lemma_id = file_stem.replace("-", "_");
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
    println!("=== HERACLITUS ENTERPRISE END-TO-END WORKSPACE ==");
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

    println!("Total Active Lemmas Ingested in Engine Core: {}", v_engine.compile_dictionary.len());

    // 1. Simulate user uploading an unstructured binary PDF attachment via the Ingestion API
    let mock_pdf_bytes = vec![0x25, 0x50, 0x44, 0x46]; // Raw PDF signature bytes
    let payload = api::IngestionPayload::new(api::IngestionType::AttachmentPDF, mock_pdf_bytes);
    let parsed_paragraphs = payload.extract_clean_paragraphs();

    println!("\n[Ingestion API Action]: Decoded uploaded PDF file attachment into clean text blocks.");

    // 2. Execute dynamic verification run
    println!("\nRunning Graph-Aware Dictionary Validation Pass...");
    let logs = v_engine.verify_document_narrative(&parsed_paragraphs);

    for log in logs {
        println!("--------------------------------------------------");
        println!("FRAME [#{}] | Status Verdict: {}", log.paragraph_index, log.status);
        println!("Scrubbed Prose Reference: \"{}\"", log.segment_text);
        if let Some(code) = log.violation_code {
            println!("  ↳ LOGICAL VIOLATION INTERCEPTED: {}", code);
            println!("    Details: {}", log.diagnostic_details.unwrap());
        }
    }
    println!("--------------------------------------------------");
}
