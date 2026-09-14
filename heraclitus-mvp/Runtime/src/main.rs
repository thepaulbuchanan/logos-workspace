mod ast;
mod parser;
mod engine;
mod auth;
mod dashboard;
mod storage;
mod api;
mod config;
mod routes;

use pest::Parser;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn compile_and_synthesize_spec_file(path: &Path, engine: &mut engine::VerificationEngine) {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    let mut lemma_id = String::new();
    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
        lemma_id = file_stem.replace("-", "_");
    }
    
    let has_spec_block = parser::extract_logos_spec_from_markdown(&file_content).is_some();

    let final_content = if has_spec_block {
        file_content
    } else {
        let mut continuous_spec = file_content.clone();
        continuous_spec.push_str("\n\n# AUTO-GENERATED MACHINE REFACTORING LAYERS\n\n```logos-spec\n");
        continuous_spec.push_str(&format!("CONSTANT {}_Context : Scope\n", lemma_id));
        continuous_spec.push_str(&format!("VARIABLE {}_Assertion : Prop\n\n", lemma_id));
        continuous_spec.push_str(&format!("DEF {}.check (s : Scope) : Prop :=\n", lemma_id));
        continuous_spec.push_str("  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, \"Legacy uncodified verification checkpoint reached.\")\n");
        continuous_spec.push_str("```\n");
        
        fs::write(path, &continuous_spec).expect("Failed to write synthesized specification layer.");
        continuous_spec
    };

    if let Some(code_block) = parser::extract_logos_spec_from_markdown(&final_content) {
        if let Ok(parsed_tree) = parser::SVEParser::parse(parser::Rule::program, &code_block) {
            let mut context = ast::CompilerContext::new();
            parser::build_ast(parsed_tree, &mut context);
            engine.register_lemma(lemma_id, context);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("=== HERACLITUS MODULAR NETWORKING PRODUCTION ====");
    println!("==================================================");

    let mut v_engine = engine::VerificationEngine::new();
    let target_specs_dir = Path::new("../public-logoslib/specs");
    let legacy_svi_dir = Path::new("../archive-legacy/01-SVI-Prototype");
    let legacy_mvp_dir = Path::new("../archive-legacy/02-Heraclitus-MVP/LogosLib");

    if legacy_svi_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(legacy_svi_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    let file_name = path.file_name().unwrap();
                    let dest = target_specs_dir.join(file_name);
                    if !dest.exists() { let _ = fs::copy(&path, &dest); }
                }
            }
        }
    }

    if legacy_mvp_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(legacy_mvp_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    let file_name = path.file_name().unwrap();
                    let dest = target_specs_dir.join(file_name);
                    if !dest.exists() { let _ = fs::copy(&path, &dest); }
                }
            }
        }
    }

    if target_specs_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(target_specs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    compile_and_synthesize_spec_file(&path, &mut v_engine);
                }
            }
        }
    }

    let lock_file_path = "../public-logoslib/library_manifest.lock";
    let specs_folder_path = "../public-logoslib/specs";
    v_engine.execute_library_lock_pass(lock_file_path, specs_folder_path);

    let base_project = dashboard::VerificationProject {
        project_id: "prj_shared_report".to_string(),
        owner_uuid: "usr_owner_90a1".to_string(),
        files: vec![dashboard::ProjectFile {
            name: "collab_draft.tex".to_string(),
            raw_content: "Initial structural asset draft template.".to_string(),
        }],
        historical_runs_count: 5,
    };

    let mut auth_manager = auth::CryptographicAuthRegistry::new();
    let valid_test_token = auth_manager.mint_auth_token("usr_owner_90a1").unwrap();
    println!("  ↳ [BOOT DATA] Baseline production token token minted to console for local testing: {}\n", valid_test_token);

    let shared_state = Arc::new(config::AppState { 
        engine: v_engine,
        session: Mutex::new(dashboard::LiveWorkspaceSession::new(base_project)),
        auth_registry: Mutex::new(auth_manager),
    });

    let saver_state = Arc::clone(&shared_state);
    tokio::spawn(async move {
        let storage_ledger = storage::CertificateLedger::new("./vault_database");
        println!("[WORKER INIT] Background Automated Disk-Saver Thread Active.");
        
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let session_lock = saver_state.session.lock().unwrap();
            let current_project = &session_lock.active_project;
            
            for file in &current_project.files {
                storage_ledger.backup_active_project_files(
                    &current_project.project_id, 
                    &file.name, 
                    &file.raw_content
                );
            }
        }
    });

    let app = routes::build_application_router(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("\n[NETWORK GATEWAY OPERATIONAL] Server live at: http://localhost:3000");
    println!("Press Ctrl+C to terminate server thread session.\n");

    axum::serve(listener, app).await.unwrap();
}
