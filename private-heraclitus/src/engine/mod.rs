pub mod topology;
pub mod evaluator;
pub mod agent;
pub mod lexicon;

use crate::ast::CompilerContext;
use sha2::Digest;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnifiedLemma {
    pub id: String,
    pub name: String,
    pub triggers: Vec<String>,
    pub hash: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParagraphDiagnostic {
    pub paragraph_index: usize,
    pub segment_text: String,
    pub status: String,
    pub violation_code: Option<String>,
    pub diagnostic_details: Option<String>,
    pub generated_sve_block: String,
    pub ir_trace_log: String,
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

pub struct VerificationEngine {
    pub compile_dictionary: HashMap<String, CompilerContext>,
    pub structural_registry: HashMap<String, String>,
    pub global_thesaurus: lexicon::LogosLibThesaurus,
    pub active_lemmas: Vec<UnifiedLemma>,
}

impl VerificationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            compile_dictionary: HashMap::new(),
            structural_registry: HashMap::new(),
            global_thesaurus: lexicon::LogosLibThesaurus::new(),
            active_lemmas: Vec::new(),
        };
        engine.bootstrap_logos_lib_front_matter();
        engine
    }

    pub fn register_lemma(&mut self, id: String, ctx: CompilerContext) {
        let structural_hash = self.compute_structural_hash(&ctx);
        self.structural_registry.insert(structural_hash, id.clone());
        self.compile_dictionary.insert(id, ctx);
    }

    pub fn bootstrap_logos_lib_front_matter(&mut self) {
        use std::fs;
        let paths = vec!["../public-logoslib/specs", "public-logoslib/specs", "specs"];
        let mut target_dir = "";
        for p in paths {
            if std::path::Path::new(p).exists() && std::path::Path::new(p).is_dir() { 
                target_dir = p; 
                break; 
            }
        }
        if target_dir.is_empty() { return; }

        if let Ok(entries) = fs::read_dir(target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        self.process_and_lock_lemma_meta(&content);
                    }
                }
            }
        }
    }

    fn process_and_lock_lemma_meta(&mut self, content: &str) {
        if !content.starts_with("---") { return; }
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 { return; }
        
        let yaml_payload = parts[1];
        
        let mut id = String::new();
        let mut name = String::new();
        let mut triggers = Vec::new();

        for line in yaml_payload.lines() {
            if !line.contains(':') { continue; }
            let kv: Vec<&str> = line.splitn(2, ':').collect();
            let key = kv[0].trim();
            let val = kv[1].trim();

            match key {
                "id" | "lemma_id" => id = val.to_string(),
                "name" => name = val.to_string(),
                "aliases" | "triggers" => {
                    let cleaned = val.replace('[', "").replace(']', "").replace('"', "").replace('\'', "");
                    triggers = cleaned.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
                }
                _ => {}
            }
        }

        if !id.is_empty() {
            let mut hasher = sha2::Sha256::new();
            hasher.update(format!("{}{:?}", id, triggers).as_bytes());
            let hash = format!("{:x}", hasher.finalize());

            if triggers.is_empty() {
                triggers.push(id.to_lowercase());
            }

            self.active_lemmas.push(UnifiedLemma { id, name, triggers, hash });
        }
    }

    pub fn compute_structural_hash(&self, ctx: &CompilerContext) -> String {
        let mut hasher = sha2::Sha256::new();
        for node in &ctx.ast_nodes {
            match node {
                crate::ast::ASTNode::Declaration { is_constant, data_type, .. } => {
                    hasher.update(format!("DECL:{}:{:?}|", is_constant, data_type).as_bytes());
                }
                crate::ast::ASTNode::Definition { args, return_type, .. } => {
                    hasher.update(format!("DEF:ARGS:{:?}:RET:{:?}|", args.len(), return_type).as_bytes());
                }
                crate::ast::ASTNode::Assertion { tactic, .. } => {
                    hasher.update(format!("ASSERT:{}|", tactic).as_bytes());
                }
            }
        }
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_dependency_topology(&self, dependencies: Vec<(String, String)>) -> Result<Vec<String>, Vec<String>> {
        topology::verify_dependency_topology(dependencies)
    }

    pub fn verify_paper_lake_build(&self, paragraphs: &[String], project_id: &str) -> (Vec<ParagraphDiagnostic>, evaluator::LakeBuildVerdict) {
        evaluator::verify_paper_lake_build(paragraphs, &self.active_lemmas, &self.global_thesaurus, project_id)
    }

    pub fn execute_library_lock_pass(&self, target_lock_path: &str, specs_dir_path: &str) -> LibraryManifestLock {
        agent::execute_library_lock_pass(self, target_lock_path, specs_dir_path)
    }

    pub fn dispatch_zulip_alert(&self, target_stream: &str, target_topic: &str, alert_details: &str) -> String {
        let payload = crate::engine::evaluator::LakeBuildVerdict {
            is_prose_sorry_free: false,
            generated_sve_contract: format!("### 🚨 EMERGENCY SECURITY DISPATCH\nContext: {}\nDetails: {}", target_topic, alert_details),
            external_kernel_handshake_ready: false,
        };
        serde_json::to_string_pretty(&payload).unwrap()
    }
}
