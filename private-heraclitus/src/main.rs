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

fn compile_spec_file(path_str: &str, engine: &mut engine::VerificationEngine, id: &str) -> Option<ast::CompilerContext> {
    let spec_path = Path::new(path_str);
    if !spec_path.exists() { return None; }
    let file_content = fs::read_to_string(spec_path).expect("Unable to read file");
    
    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&file_content) {
        if let Ok(parsed_tree) = parser::SVEParser::parse(parser::Rule::program, &code_block) {
            let mut context = ast::CompilerContext::new();
            parser::build_ast(parsed_tree, &mut context);
            engine.register_lemma(id.to_string(), context.clone());
            return Some(context);
        }
    }
    None
}

fn main() {
    println!("==================================================");
    println!("=== HERACLITUS SAAS ENTERPRISE PRODUCT PLATFORM ===");
    println!("==================================================");

    let current_session_user = auth::UserAccount {
        uuid: "usr_90a1-f3b5-77c8-9d2e".to_string(),
        corporate_domain: "fortune500_firm.com".to_string(),
        tier: auth::AccountTier::EnterprisePaid,
    };

    let active_project = dashboard::VerificationProject {
        project_id: "prj_001_annual_report".to_string(),
        owner_uuid: current_session_user.uuid.clone(),
        files: vec![dashboard::ProjectFile {
            name: "financial_disclosure.docx".to_string(),
            raw_content: "All regional operations comply cleanly with capital allocation constraints and baseline standards.".to_string(),
        }],
        historical_runs_count: 14,
    };

    let mut v_engine = engine::VerificationEngine::new();
    compile_spec_file("../public-logoslib/specs/LOGOS_001_accident.md", &mut v_engine, "LOGOS_001");
    compile_spec_file("../public-logoslib/specs/LOGOS_002_adhoc.md", &mut v_engine, "LOGOS_002");
    compile_spec_file("../public-logoslib/specs/LOGOS_003_adhominem.md", &mut v_engine, "LOGOS_003");
    
    let lemma_count = v_engine.compile_dictionary.len();
    println!("Active Operational Verification Core Library Loaded (Count: {}).", lemma_count);

    // Generate and sign the project logic certificate
    println!("\nGenerating Cryptographic Logic Seal Verification Certificate...");
    let document_text = &active_project.files[0].raw_content;
    let certificate = storage::VerificationCertificate::generate_seal(
        &active_project.project_id, 
        document_text, 
        lemma_count
    );

    println!("\n--- Verifiable Epistemic Token Export Payload ---");
    println!("{}", certificate.to_json_payload());
    println!("--------------------------------------------------");
}
