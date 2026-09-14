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
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    engine: engine::VerificationEngine,
}

fn compile_and_synthesize_spec_file(path: &Path, engine: &mut engine::VerificationEngine) {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    let mut lemma_id = String::new();
    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
        lemma_id = file_stem.replace("-", "_");
    }
    
    // Check if the legacy file is completely missing our required executable block fence
    let has_spec_block = parser::extract_logos_spec_from_markdown(&file_content).is_some();

    let final_content = if has_spec_block {
        file_content
    } else {
        // AUTOMATED MACHINE SYNTHESIS: Transmute the uncodified legacy asset into a formal SVE spec
        let mut continuous_spec = file_content.clone();
        continuous_spec.push_str("\n\n# AUTO-GENERATED MACHINE REFACTORING LAYERS\n\n```logos-spec\n");
        continuous_spec.push_str(&format!("CONSTANT {}_Context : Scope\n", lemma_id));
        continuous_spec.push_str(&format!("VARIABLE {}_Assertion : Prop\n\n", lemma_id));
        continuous_spec.push_str(&format!("DEF {}.check (s : Scope) : Prop :=\n", lemma_id));
        continuous_spec.push_str("  ASSERT_CONTEXT_BOUND(s) ⟹ THROW(LOGOS_ERR_GENERIC, \"Legacy uncodified verification checkpoint reached.\")\n");
        continuous_spec.push_str("```\n");
        
        fs::write(path, &continuous_spec).expect("Failed to write synthesized specification layer directly to hard drive.");
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

async fn handle_web_verification(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<api::WebIngestionRequest>,
) -> Json<Vec<engine::ParagraphDiagnostic>> {
    println!("\n[WEB SERVER] Received live verification request payload for project '{}'", payload.project_id);

    let document_payload = api::IngestionPayload::new(
        api::IngestionType::RawTextStream,
        payload.text_content.as_bytes().to_vec()
    );

    let clean_paragraphs = document_payload.extract_clean_paragraphs();
    let diagnostics = state.engine.verify_document_narrative(&clean_paragraphs);

    Json(diagnostics)
}

#[tokio::main]
async fn main() {
    println!("==================================================");
    println!("=== HERACLITUS MASS MASSIVE COGNITIVE PIPELINE ===");
    println!("==================================================");

    let mut v_engine = engine::VerificationEngine::new();
    
    // Target directories mapping the entire multi-year corpus portfolio layout
    let target_specs_dir = Path::new("../public-logoslib/specs");
    let legacy_svi_dir = Path::new("../archive-legacy/01-SVI-Prototype");
    let legacy_mvp_dir = Path::new("../archive-legacy/02-Heraclitus-MVP/LogosLib");

    println!("[HARVEST PASS] Scanning legacy directories to ingest uncodified files...");
    
    // Batch Ingest from Historical Attempt 1 (SVI) if present
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

    // Batch Ingest from Historical Attempt 2 (MVP) if present
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

    // PHASE 2: MASS SYNTHESIS AND DIRECTORY RE-NAMING ENGINE
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

    // Pass the standard directory configurations to our Canonical Renaming Agent block
    let lock_file_path = "../public-logoslib/library_manifest.lock";
    let specs_folder_path = "../public-logoslib/specs";
    v_engine.execute_library_lock_pass(lock_file_path, specs_folder_path);

    let final_count = v_engine.compile_dictionary.len();
    println!("\n[SYNTHESIS COMPLETE] System expanded into an invariant single source library.");
    println!("Total Active Codified Fallacies Sealed in Manifest Matrix: {}", final_count);

    let shared_state = Arc::new(AppState { engine: v_engine });

    let cors_policy = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::POST])
        .allow_headers([axum::http::HeaderName::from_static("content-type")]);

    let app = Router::new()
        .route("/api/verify", post(handle_web_verification))
        .layer(cors_policy)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("\n[NETWORK GATEWAY OPERATIONAL] Server live at: http://localhost:3000");
    println!("Press Ctrl+C to terminate server thread session.\n");

    axum::serve(listener, app).await.unwrap();
}
