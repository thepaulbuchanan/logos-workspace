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
    println!("=== HERACLITUS LIVE PRODUCT WORKSPACE RUNTIME ===");
    println!("==================================================");

    // 1. Ingest Libraries
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

    // 2. Initialize Session User & Base Project Profiles
    let session_user = auth::UserAccount {
        uuid: "usr_90a1-f3b5-77c8-9d2e".to_string(),
        corporate_domain: "fortune500_firm.com".to_string(),
        tier: auth::AccountTier::EnterprisePaid,
    };

    let base_project = dashboard::VerificationProject {
        project_id: "prj_001_annual_report".to_string(),
        owner_uuid: session_user.uuid.clone(),
        files: vec![dashboard::ProjectFile {
            name: "draft.tex".to_string(),
            raw_content: String::new(),
        }],
        historical_runs_count: 14,
    };

    println!("User Dashboard Connected: {}", session_user.corporate_domain);
    
    // Start Overleaf live editor thread
    let mut session = dashboard::project::LiveWorkspaceSession::new(base_project);

    // 3. Simulate Live Streaming Typestream Event Transaction
    let incoming_mutation = dashboard::project::EditorStreamEvent {
        file_target: "draft.tex".to_string(),
        line_delta: "We operate standard manufacturing guidelines across all regional locations. Therefore, surgeons operating inside medical units must obey factory uniform standards, ignoring operational constraints.\n\nThe market strategy proposed by the compliance auditor is clean and structurally solid, aligning directly with capital distribution parameters.".to_string(),
    };

    // Run string stream slice normalisation loops
    let active_paragraphs = session.process_stream_mutation(incoming_mutation);
    println!("Live Stream Parser: Isolate-processed {} text paragraphs.", active_paragraphs.len());

    // 4. Execute Multi-Paragraph Epistemic Verification Pass
    // Hook loaded contexts as test tracking mock inputs to evaluate paragraph runs
    let mut mock_stream = Vec::new();
    if let Some(ctx1) = v_engine.compile_dictionary.get("LOGOS_001") { mock_stream.push(ctx1.clone()); }
    if let Some(ctx2) = v_engine.compile_dictionary.get("LOGOS_002") { mock_stream.push(ctx2.clone()); }

    println!("\nExecuting Epistemic Validation over Stream Buffers...");
    let reports = v_engine.verify_document_narrative(&active_paragraphs, &mock_stream);

    let mut compilation_failed = false;
    for report in reports {
        println!("  [Index #{}] Verdict: {}", report.paragraph_index, report.status);
        if report.status == "FAILED" {
            compilation_failed = true;
            println!("    ↳ Exception: {}", report.diagnostic_details.unwrap());
        }
    }

    // 5. Generate signed certificate IFF build achieves zero error state
    if !compilation_failed {
        println!("\nGenerating Cryptographic Logic Seal Verification Certificate...");
        let certificate = storage::VerificationCertificate::generate_seal(
            &session.active_project.project_id,
            &session.active_project.files[0].raw_content,
            v_engine.compile_dictionary.len()
        );
        println!("\n--- Verifiable Epistemic Token Export Payload ---\n{}", certificate.to_json_payload());
    } else {
        println!("\n[Build Warning] Cryptographic seal blocked. Clear logic compilation errors inside editor window to get signature stamp.");
    }
    println!("--------------------------------------------------");
}
