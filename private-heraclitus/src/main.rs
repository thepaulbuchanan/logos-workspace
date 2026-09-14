mod ast;
mod parser;
mod engine;
mod auth;
mod dashboard;
mod storage;
mod api;

use axum::{routing::post, Json, Router};
use pest::Parser;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    engine: engine::VerificationEngine,
    session: Mutex<dashboard::project::LiveWorkspaceSession>,
    auth_registry: Mutex<auth::CryptographicAuthRegistry>,
}

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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecureWebIngestionRequest {
    pub auth_token: String,
    pub project_id: String,
    pub text_content: String,
}

async fn handle_web_verification(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<SecureWebIngestionRequest>,
) -> Result<Json<engine::LakeBuildVerdict>, (axum::http::StatusCode, String)> {
    println!("\n[SECURITY GATE] Received secure verification request payload for project '{}'", payload.project_id);

    let active_user = {
        let auth_lock = state.auth_registry.lock().unwrap();
        match auth_lock.verify_token_clearance(&payload.auth_token) {
            Ok(user) => user,
            Err(err_msg) => {
                eprintln!("\n[SECURITY ALERT] Unauthorized transaction blocked! Exception: {}", err_msg);
                return Err((axum::http::StatusCode::UNAUTHORIZED, err_msg));
            }
        }
    };

    println!("[SECURITY GATE] Access Authorized. User identity confirmed as: '{}'", active_user.uuid);

    let document_payload = if payload.text_content.starts_with("%PDF") {
        api::IngestionPayload::new(
            api::IngestionType::AttachmentPDF,
            payload.text_content.as_bytes().to_vec()
        )
    } else {
        api::IngestionPayload::new(
            api::IngestionType::RawTextStream,
            payload.text_content.as_bytes().to_vec()
        )
    };

    let clean_paragraphs = document_payload.extract_clean_paragraphs();
    
    {
        let mut session_lock = state.session.lock().unwrap();
        let mutation = dashboard::project::EditorStreamEvent {
            file_target: "collab_draft.tex".to_string(),
            line_delta: clean_paragraphs.join("\n\n"),
            actor_uuid: active_user.uuid.clone(),
        };
        let _ = session_lock.process_shared_stream_mutation(mutation);
    }

    let (_diagnostics, verdict) = state.engine.verify_paper_lake_build(&clean_paragraphs, &payload.project_id);

    Ok(Json(verdict))
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct CursorUpdateRequest {
    pub user_uuid: String,
    pub line_index: usize,
    pub character_offset: usize,
}

async fn handle_cursor_sync(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<CursorUpdateRequest>,
) -> Json<String> {
    let mut session_lock = state.session.lock().unwrap();
    
    let coords = dashboard::LiveCursorCoordinates {
        line_index: payload.line_index,
        character_offset: payload.character_offset,
    };

    match session_lock.update_collaborator_cursor(&payload.user_uuid, coords) {
        Ok(_) => Json("{\"status\": \"SYNC_SUCCESS\"}".to_string()),
        Err(_) => {
            session_lock.register_collaborator(dashboard::CollaboratorSession {
                user_uuid: payload.user_uuid.clone(),
                account_tier: auth::AccountTier::FreeFun,
                active_role: dashboard::WorkspaceRole::Editor,
                active_cursor: None,
            });
            Json("{\"status\": \"REGISTERED_AND_SYNCED\"}".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("=== HERACLITUS MASS MASSIVE COGNITIVE PIPELINE ===");
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
    let _valid_test_token = auth_manager.mint_auth_token("usr_owner_90a1").unwrap();
    println!("  ↳ [BOOT DATA] Baseline production token token minted to console for local testing: {}\n", _valid_test_token);

    let shared_state = Arc::new(AppState { 
        engine: v_engine,
        session: Mutex::new(dashboard::project::LiveWorkspaceSession::new(base_project)),
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

    let cors_policy = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::POST])
        .allow_headers([axum::http::HeaderName::from_static("content-type")]);

    let app = Router::new()
        .route("/api/verify", post(handle_web_verification))
        .route("/api/cursor", post(handle_cursor_sync))
        .layer(cors_policy)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("\n[NETWORK GATEWAY OPERATIONAL] Server live at: http://localhost:3000");
    println!("Press Ctrl+C to terminate server thread session.\n");

    axum::serve(listener, app).await.unwrap();
}
