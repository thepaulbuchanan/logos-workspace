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
    if !spec_path.exists() { 
        println!("[Loader Warning]: File target missing at {:?}", spec_path);
        return None; 
    }
    let file_content = fs::read_to_string(spec_path).expect("Unable to read file");
    
    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&file_content) {
        match parser::SVEParser::parse(parser::Rule::program, &code_block) {
            Ok(parsed_tree) => {
                let mut context = ast::CompilerContext::new();
                parser::build_ast(parsed_tree, &mut context);
                engine.register_lemma(id.to_string(), context.clone());
                return Some(context);
            }
            Err(e) => {
                println!("[Grammar Error]: Spec parsing failed inside target {}!\n{:?}\n", id, e);
                return None;
            }
        }
    }
    println!("[Extractor Warning]: Failed to locate a valid ```logos-spec block inside {}", id);
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

    let raw_uploaded_bytes = b"Paragraph 1: Invariant asset distribution details.\n\nParagraph 2: Secondary corporate statement text profile.";
    let document_payload = api::IngestionPayload::new(
        api::IngestionType::RawTextStream,
        raw_uploaded_bytes.to_vec()
    );

    let clean_paragraphs = document_payload.extract_clean_paragraphs();
    
    let mut active_project = dashboard::VerificationProject {
        project_id: "prj_001_annual_report".to_string(),
        owner_uuid: current_session_user.uuid.clone(),
        files: vec![dashboard::ProjectFile {
            name: "financial_disclosure.txt".to_string(),
            raw_content: clean_paragraphs.join("\n\n"),
        }],
        historical_runs_count: 14,
    };

    let mut v_engine = engine::VerificationEngine::new();
    let mut storage_ledger = storage::repository::CertificateLedger::new();
    
    println!("Ingesting Active Specifications Library...");
    compile_spec_file("../public-logoslib/specs/LOGOS_001_accident.md", &mut v_engine, "LOGOS_001");
    compile_spec_file("../public-logoslib/specs/LOGOS_002_adhoc.md", &mut v_engine, "LOGOS_002");
    compile_spec_file("../public-logoslib/specs/LOGOS_003_adhominem.md", &mut v_engine, "LOGOS_003");
    compile_spec_file("../public-logoslib/specs/LOGOS_006_baserate.md", &mut v_engine, "LOGOS_006");
    compile_spec_file("../public-logoslib/specs/LOGOS_007_beggingquestion.md", &mut v_engine, "LOGOS_007");
    compile_spec_file("../public-logoslib/specs/LOGOS_008_hastygeneralization.md", &mut v_engine, "LOGOS_008");
    
    let lemma_count = v_engine.compile_dictionary.len();
    println!("Active Operational Verification Core Library Loaded (Count: {}).", lemma_count);

    println!("\nGenerating Cryptographic Logic Seal Verification Certificate...");
    let document_text = &active_project.files[0].raw_content;
    let certificate = storage::VerificationCertificate::generate_seal(
        &active_project.project_id, 
        document_text, 
        lemma_count
    );

    storage_ledger.log_certificate(&active_project.project_id, certificate.clone());
    active_project.historical_runs_count += storage_ledger.get_history_count(&active_project.project_id) as u32;

    println!("\n--- Verifiable Epistemic Token Export Payload ---");
    println!("{}", certificate.to_json_payload());
    println!("--------------------------------------------------");
    println!("Database Ledger Check: Project '{}' has now tracked {} verified runs inside this workspace session storage ledger.", 
             active_project.project_id, storage_ledger.get_history_count(&active_project.project_id));
}
