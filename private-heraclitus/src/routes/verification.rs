use axum::{extract::State, Json};
use std::sync::Arc;
use std::fs;
use std::path::Path;
use crate::config::AppState;
use crate::api::{IngestionPayload, IngestionType};
use crate::dashboard::project::EditorStreamEvent;
use crate::engine::evaluator::LakeBuildVerdict;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecureWebIngestionRequest {
    pub auth_token: String,
    pub project_id: String,
    pub text_content: String,
}

pub async fn handle_web_verification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SecureWebIngestionRequest>,
) -> Result<Json<LakeBuildVerdict>, (axum::http::StatusCode, String)> {
    println!("\n[SECURITY GATE] Received secure verification request payload for project '{}'", payload.project_id);

    let active_user = {
        let auth_lock = state.auth_registry.lock().unwrap();
        match auth_lock.verify_token_clearance(&payload.auth_token) {
            Ok(user) => user,
            Err(err_msg) => {
                eprintln!("\n[SECURITY ALERT] Unauthorized transaction blocked! Exception: {}", err_msg);
                let _ = state.engine.dispatch_zulip_alert("security-firewall", "ACCESS_VIOLATION", &err_msg);
                return Err((axum::http::StatusCode::UNAUTHORIZED, err_msg));
            }
        }
    };

    println!("[SECURITY GATE] Access Authorized. User identity confirmed as: '{}'", active_user.uuid);

    let document_payload = if payload.text_content.starts_with("%PDF") {
        IngestionPayload::new(IngestionType::AttachmentPDF, payload.text_content.as_bytes().to_vec())
    } else {
        IngestionPayload::new(IngestionType::RawTextStream, payload.text_content.as_bytes().to_vec())
    };

    let clean_paragraphs = document_payload.extract_clean_paragraphs();
    
    {
        let mut session_lock = state.session.lock().unwrap();
        let mutation = EditorStreamEvent {
            file_target: "collab_draft.tex".to_string(),
            line_delta: clean_paragraphs.join("\n\n"),
            actor_uuid: active_user.uuid.clone(),
        };
        let _ = session_lock.process_shared_stream_mutation(mutation);
    }

    let (diagnostics, verdict) = state.engine.verify_paper_lake_build(&clean_paragraphs, &payload.project_id);

    // SVI-main Output Generation Pipeline
    let mut manifest_summary = format!(
        "# HERACLITUS EPISTEMIC MANIFEST SUMMARY REPORT\n\nTarget Project: {}\nStatus: EVALUATION COMPLETE\n\n## Epistemic Audit Ledger:\n\n", 
        payload.project_id
    );
    let mut sve_script_output = "-- HERACLITUS SCRIPT: INVARIANT LOGOS LEDGER\n-- VERSION: v1.0.0-ALPHA\n\n".to_string();

    println!("\n--- [SVI-CORE] Live IR Trace Log Stream ---");
    for diag in &diagnostics {
        println!("{}", diag.ir_trace_log);
        manifest_summary.push_str(&diag.segment_text);
        sve_script_output.push_str(&diag.generated_sve_block);
    }
    println!("-------------------------------------------\n");

    let target_dir = Path::new("tests");
    if !target_dir.exists() { let _ = fs::create_dir_all(target_dir); }
    
    let _ = fs::write("tests/HERACLITUS_MANIFEST_SUMMARY.md", manifest_summary);
    let _ = fs::write("tests/Validated.sve", sve_script_output);
    println!("[SVI-LOGS] Physical ledger files generated under private-heraclitus/tests/");

    Ok(Json(verdict))
}
