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
    println!("=== HERACLITUS MULTI-USER WORKSPACE WITH ALERTS ==");
    println!("==================================================");

    let owner_user = auth::UserAccount {
        uuid: "usr_owner_90a1".to_string(),
        corporate_domain: "fortune500_firm.com".to_string(),
        tier: auth::AccountTier::EnterprisePaid,
    };

    let base_project = dashboard::VerificationProject {
        project_id: "prj_shared_report".to_string(),
        owner_uuid: owner_user.uuid.clone(),
        files: vec![dashboard::ProjectFile {
            name: "collab_draft.tex".to_string(),
            raw_content: String::new(),
        }],
        historical_runs_count: 5,
    };

    let mut session = dashboard::project::LiveWorkspaceSession::new(base_project);

    session.register_collaborator(dashboard::project::CollaboratorSession {
        user_uuid: owner_user.uuid.clone(),
        account_tier: owner_user.tier.clone(),
        active_role: dashboard::project::WorkspaceRole::Owner,
    });

    let external_editor_uuid = "usr_community_44b2".to_string();
    session.register_collaborator(dashboard::project::CollaboratorSession {
        user_uuid: external_editor_uuid.clone(),
        account_tier: auth::AccountTier::FreeFun,
        active_role: dashboard::project::WorkspaceRole::Editor,
    });

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
    
    let lock_file_path = "../public-logoslib/library_manifest.lock";
    v_engine.execute_library_lock_pass(lock_file_path);

    // INGESTION TEST CHANGE: Inputting a text block containing a Red Herring distraction
    let incoming_collab_bytes = b"Paragraph 1: Why focus on compliance tracking loops when our competitors are spending twice as much on standard marketing campaigns?";
    let document_payload = api::IngestionPayload::new(api::IngestionType::RawTextStream, incoming_collab_bytes.to_vec());
    let clean_paragraphs = document_payload.extract_clean_paragraphs();

    let stream_event = dashboard::project::EditorStreamEvent {
        file_target: "collab_draft.tex".to_string(),
        line_delta: clean_paragraphs.join("\n\n"),
        actor_uuid: external_editor_uuid.clone(),
    };

    println!("\nProcessing incoming workspace stream transaction...");
    if let Ok(active_paragraphs) = session.process_shared_stream_mutation(stream_event) {
        let diagnostics = v_engine.verify_document_narrative(&active_paragraphs);
        
        for log in diagnostics {
            println!("  [Index #{}] Verdict Status: {}", log.paragraph_index, log.status);
            if log.status == "FAILED" {
                let details = log.diagnostic_details.unwrap();
                let webhook_json = v_engine.dispatch_zulip_alert(
                    "logoslib-ci",
                    "STUB_DISCOVERY_ALERT",
                    &format!("Fallacy Blocked: {}\nContext Segment: {}", log.violation_code.unwrap(), details)
                );
                println!("\n--- Raw Transport Event Packet Payload ---\n{}", webhook_json);
            }
        }
    }
    println!("==================================================");
}
