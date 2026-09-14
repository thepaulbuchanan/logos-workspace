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
    println!("=== HERACLITUS SAAS ENTERPRISE PRODUCTION LAYER ===");
    println!("==================================================");

    // PHASE 1: ACCOUNT SECURITY & DASHBOARD INITIALISATION
    let session_user = auth::UserAccount {
        uuid: "usr_90a1-f3b5-77c8-9d2e".to_string(),
        corporate_domain: "fortune500_firm.com".to_string(),
        tier: auth::AccountTier::EnterprisePaid,
    };

    let base_project = dashboard::VerificationProject {
        project_id: "prj_001_annual_report".to_string(),
        owner_uuid: session_user.uuid.clone(),
        files: vec![dashboard::ProjectFile {
            name: "annual_draft.tex".to_string(),
            raw_content: String::new(),
        }],
        historical_runs_count: 14,
    };

    println!("Secure Client Account Instantiated: {}", session_user.corporate_domain);
    let mut session = dashboard::project::LiveWorkspaceSession::new(base_project);
    let mut storage_ledger = storage::CertificateLedger::new("./vault_database");

    // PHASE 2: INVARIANT SPECIFICATION LEXICON TAMPER-AUDIT CHECK
    let mut v_engine = engine::VerificationEngine::new();
    let specs_dir = Path::new("../public-logoslib/specs");
    let lock_file_path = "../public-logoslib/library_manifest.lock";
    
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
    println!("Dynamic Directory Sweep Complete. Compiled Active Lemmas: {}", lemma_count);

    // Dynamic Tamper Audit Loop: Recalculate and verify against the lock manifest on disk
    println!("\n[SECURITY] Executing Hermetic Manifest Integrity Check...");
    if Path::new(lock_file_path).exists() {
        let lock_content = fs::read_to_string(lock_file_path).expect("Failed to read lock file");
        if let Ok(manifest) = serde_json::from_str::<engine::LibraryManifestLock>(&lock_content) {
            let mut tamper_detected = false;
            
            for (lemma_id, entry) in &manifest.locked_registry {
                if let Some(current_ctx) = v_engine.compile_dictionary.get(lemma_id) {
                    let current_hash = v_engine.compute_structural_hash(current_ctx);
                    if current_hash != entry.structural_hash {
                        eprintln!("\n[CRITICAL SECURITY EXCEPTION] MANIFEST_TAMPER_EXCEPTION Detected!");
                        eprintln!("  ↳ Target Lemma ID: {}", lemma_id);
                        eprintln!("  ↳ Expected Hash  : {}", entry.structural_hash);
                        eprintln!("  ↳ Computed Hash  : {}", current_hash);
                        tamper_detected = true;
                    }
                }
            }

            if tamper_detected {
                eprintln!("\n[FATAL] System Compilation Aborted. Cryptographic signatures mismatch with library lock file.");
                println!("==================================================");
                std::process::exit(1); // Force-halt the enterprise product thread
            } else {
                println!("  ↳ STATUS: VERIFIED. Manifest integrity matched. 0 Changes detected.");
            }
        }
    } else {
        println!("  ↳ WARNING: Manifest lock file missing. Regenerating standard lock map.");
        v_engine.execute_library_lock_pass(lock_file_path);
    }

    // PHASE 3: LIVE MULTI-FORMAT DOCUMENT INGESTION & EVALUATION LOOP
    let incoming_bytes = b"Paragraph 1: Tony claims corporate tax drops work. But Tony is a convict, so his statement is false.\n\nParagraph 2: The market strategy proposed by the compliance auditor is clean and structurally solid, aligning directly with capital parameters.";
    
    let document_payload = api::IngestionPayload::new(
        api::IngestionType::RawTextStream,
        incoming_bytes.to_vec()
    );

    let clean_paragraphs = document_payload.extract_clean_paragraphs();
    
    let mutation_event = dashboard::project::EditorStreamEvent {
        file_target: "annual_draft.tex".to_string(),
        line_delta: clean_paragraphs.join("\n\n"),
    };

    let active_paragraphs = session.process_stream_mutation(mutation_event);
    println!("\nExecuting Epistemic Validation over live pre-filtered document buffers...");
    let diagnostics = v_engine.verify_document_narrative(&active_paragraphs);

    let mut system_build_halted = false;
    for log in &diagnostics {
        println!("  ------------------------------------------------");
        println!("  FRAME [#{}] | Status Verdict: {}", log.paragraph_index, log.status);
        println!("  Scrubbed Text Segment: \"{}\"", log.segment_text);
        if let Some(code) = &log.violation_code {
            system_build_halted = true;
            println!("    ↳ LOGICAL FALLACY INTERCEPTED: {}", code);
            println!("      Details: {}", log.diagnostic_details.as_ref().unwrap());
        }
    }

    println!("  ------------------------------------------------");

    // PHASE 4: PERSISTENT CRYPTOGRAPHIC CORES SEALING
    if !system_build_halted {
        println!("\nGenerating Cryptographic Logic Seal Verification Certificate...");
        let certificate = storage::VerificationCertificate::generate_seal(
            &session.active_project.project_id,
            &session.active_project.files[0].raw_content,
            lemma_count
        );
        
        storage_ledger.persist_certificate_to_disk(&session.active_project.project_id, certificate);
    } else {
        println!("\n[Build Aborted] Cryptographic verification signature blocked due to critical logic anomalies in text stream.");
        println!("Total Verified Project Runs Saved on Disk: {} runs.", storage_ledger.get_history_count_from_disk(&session.active_project.project_id));
    }
    println!("==================================================");
}
