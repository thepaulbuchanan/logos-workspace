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

// Ingest CORS middleware items from our new dependency package
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    engine: engine::VerificationEngine,
}

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
    println!("=== HERACLITUS LIVE NETWORKING WEB ENGINE core ===");
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

    let lemma_count = v_engine.compile_dictionary.len();
    
    let lock_file_path = "../public-logoslib/library_manifest.lock";
    let specs_folder_path = "../public-logoslib/specs";
    v_engine.execute_library_lock_pass(lock_file_path, specs_folder_path);
    
    println!("Active Engine Core Initialized. Total Lemmas Loaded: {}", lemma_count);

    let shared_state = Arc::new(AppState { engine: v_engine });

    // FIX: Define a production-ready CORS security policy layer for browser clients
    let cors_policy = CorsLayer::new()
        .allow_origin(Any) // Allows requests from any origin dashboard URL layer
        .allow_methods([axum::http::Method::POST]) // Permits post streams
        .allow_headers([axum::http::HeaderName::from_static("content-type")]);

    // Attach the CORS middleware layer to our public router mapping context
    let app = Router::new()
        .route("/api/verify", post(handle_web_verification))
        .layer(cors_policy)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("\n[NETWORK PUBLIC GATEWAY] Server live! Listening on: http://localhost:3000");
    println!("  ↳ Endpoint Ready: POST http://localhost:3000/api/verify");
    println!("  ↳ Security Guard: CORS layer active. Allowing browser cross-origin requests.");
    println!("Press Ctrl+C to terminate server thread session.\n");

    axum::serve(listener, app).await.unwrap();
}
