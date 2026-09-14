pub mod topology;
pub mod evaluator;
pub mod agent;
pub mod lexicon;

use crate::ast::{ASTNode, CompilerContext};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParagraphDiagnostic {
    pub paragraph_index: usize,
    pub segment_text: String,
    pub status: String,
    pub violation_code: Option<String>,
    pub diagnostic_details: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LockedLemmaEntry {
    pub lemma_id: String,
    pub structural_hash: String,
    pub compiled_nodes_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryManifestLock {
    pub manifest_version: String,
    pub global_verification_timestamp: u64,
    pub total_verified_lemmas: usize,
    pub locked_registry: HashMap<String, LockedLemmaEntry>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ZulipWebhookPayload {
    pub stream: String,
    pub topic: String,
    pub content: String,
}

pub struct VerificationEngine {
    pub compile_dictionary: HashMap<String, CompilerContext>,
    pub structural_registry: HashMap<String, String>,
    pub global_thesaurus: lexicon::LogosLibThesaurus,
}

impl VerificationEngine {
    pub fn new() -> Self {
        Self {
            compile_dictionary: HashMap::new(),
            structural_registry: HashMap::new(),
            global_thesaurus: lexicon::LogosLibThesaurus::new(),
        }
    }

    pub fn register_lemma(&mut self, id: String, ctx: CompilerContext) {
        let structural_hash = self.compute_structural_hash(&ctx);
        self.structural_registry.insert(structural_hash, id.clone());
        self.compile_dictionary.insert(id, ctx);
    }

    pub fn compute_structural_hash(&self, ctx: &CompilerContext) -> String {
        let mut hasher = Sha256::new();
        for node in &ctx.ast_nodes {
            match node {
                ASTNode::Declaration { is_constant, data_type, .. } => {
                    hasher.update(format!("DECL:{}:{:?}|", is_constant, data_type).as_bytes());
                }
                ASTNode::Definition { args, return_type, .. } => {
                    hasher.update(format!("DEF:ARGS:{:?}:RET:{:?}|", args.len(), return_type).as_bytes());
                }
                ASTNode::Assertion { tactic, .. } => {
                    hasher.update(format!("ASSERT:{}|", tactic).as_bytes());
                }
            }
        }
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_dependency_topology(&self, dependencies: Vec<(String, String)>) -> Result<Vec<String>, Vec<String>> {
        topology::verify_dependency_topology(dependencies)
    }

    pub fn verify_document_narrative(&self, paragraphs: &[String]) -> Vec<ParagraphDiagnostic> {
        evaluator::verify_document_narrative(paragraphs, &self.compile_dictionary, &self.global_thesaurus)
    }

    pub fn execute_library_lock_pass(&self, target_lock_path: &str, specs_dir_path: &str) -> LibraryManifestLock {
        agent::execute_library_lock_pass(self, target_lock_path, specs_dir_path)
    }

    /// Automation Hook: Compiles a security/structural anomaly report and dispatches a simulated webhook
    pub fn dispatch_zulip_alert(&self, target_stream: &str, target_topic: &str, alert_details: &str) -> String {
        let payload = ZulipWebhookPayload {
            stream: target_stream.to_string(),
            topic: target_topic.to_string(),
            content: format!(
                "### 🚨 HERACLITUS EMERGENCY RESPONSE EXCEPTION WARNING\n\n\
                **Target Context:** {}\n\n\
                **Automated Diagnostic Data:**\n\
                ```text\n\
                {}\n\
                ```\n\n\
                *Security firewall intercept active. Session transaction blocked and logged.*",
                target_topic, alert_details
            ),
        };

        let json_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("\n[WEBSERVER WEBHOOK AGENT] Formatting outbound JSON security event payload...");
        println!("[WEBSERVER WEBHOOK AGENT] Piping HTTP POST payload to endpoint channel: `#stream/{}`", payload.stream);
        json_payload
    }
}
