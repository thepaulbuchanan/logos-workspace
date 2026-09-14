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
    println!("=== HERACLITUS SAAS PLATFORM WITH DISK DB =======");
    println!("==================================================");

    let mut v_engine = engine::VerificationEngine::new();
    
    // Instantiate persistent database registry folder path
    let mut storage_ledger = storage::repository::CertificateLedger::new("./vault_database");

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

    let lemma_count = v_engine.compile_dictionary.len();
    println!("Active Operational Verification Core Library Loaded (Count: {}).", lemma_count);

    // 1. Simulate an incoming clean text block clearing validation entirely
    let clean_text = "The market strategy proposed by the compliance auditor is clean and structurally solid.";
    let payload = api::IngestionPayload::new(api::IngestionType::RawTextStream, clean_text.as_bytes().to_vec());
    let parsed_paragraphs = payload.extract_clean_paragraphs();

    println!("\nIngested Workspace Document Stream: \"{}\"", clean_text);
    let logs = v_engine.verify_document_narrative(&parsed_paragraphs);

    let mut compilation_failed = false;
    for log in logs {
        if log.status == "FAILED" { compilation_failed = true; }
    }

    // 2. Generate and write the certificate to disk on success
    if !compilation_failed {
        println!("\nGenerating Cryptographic Logic Seal Verification Certificate...");
        let certificate = storage::VerificationCertificate::generate_seal(
            "prj_001_annual_report", 
            clean_text, 
            lemma_count
        );

        // Execute active physical file persistence routine
        storage_ledger.persist_certificate_to_disk("prj_001_annual_report", certificate);
        
        println!("\n--------------------------------------------------");
        println!("Database Storage Audit Check: Project 'prj_001_annual_report' now tracks {} total historical validation certificate runs safely saved on disk.", 
                 storage_ledger.get_history_count_from_disk("prj_001_annual_report"));
        println!("--------------------------------------------------");
    }
}
