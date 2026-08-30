use std::fs;
use std::path::PathBuf;
use crate::refactor::LemmaRefactor;
use crate::critic::LemmaCriticEngine; // 🟢 Link our fresh Adversarial Critic layer

pub struct UnifiedLemma {
    pub id: String,
    pub name: String,
    pub triggers: Vec<String>,
    pub hash: String,
}

pub struct LibraryRegistry;

impl LibraryRegistry {
    pub fn audit_and_load(target_dir: &str) -> Vec<UnifiedLemma> {
        let mut loaded_lemmas = Vec::new();

        if let Ok(entries) = fs::read_dir(target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Pass the accumulating clean library down to run validation checks
                        if let Some(lemma) = Self::parse_or_compile(&content, path, &loaded_lemmas) {
                            loaded_lemmas.push(lemma);
                        }
                    }
                }
            }
        }
        loaded_lemmas
    }

    fn parse_or_compile(content: &str, file_path: PathBuf, extant_library: &[UnifiedLemma]) -> Option<UnifiedLemma> {
        if !content.starts_with("---") { return None; }
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 { return None; }
        
        let yaml_text = parts[1];
        let remaining_body = parts[2..].join("---");
        
        let mut id = String::new();
        let mut name = String::new();
        let mut hash = String::new();
        let mut triggers = Vec::new();

        for line in yaml_text.lines() {
            if !line.contains(':') { continue; }
            let kv: Vec<&str> = line.splitn(2, ':').collect();
            let key = kv[0].trim();
            let val = kv[1].trim();

            match key {
                "lemma_id" => id = val.to_string(),
                "name" => name = val.to_string(),
                "ep_hash" => hash = val.to_string(),
                "triggers" => {
                    let cleaned = val.replace('[', "").replace(']', "").replace('"', "").replace('\'', "");
                    triggers = cleaned.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
                }
                _ => {}
            }
        }

        // 🚨 ADVERSARIAL CRITIC GATEWAY FOR NEW UNHASHED ENTRIES
        if hash.is_empty() && !id.is_empty() && !triggers.is_empty() {
            // Run the proposal through our Strawman/Steelman agent tournament prior to compilation!
            if let Err(critic_fault) = LemmaCriticEngine::evaluate_proposal(&id, &triggers, extant_library) {
                eprintln!("🔴 HERACLITUS CRITIC SECURITY FAULT for {}: {}", id, critic_fault);
                return None; // Hard stop: Reject the corrupted proposal asset instantly!
            }
            
            let calculated_hash = LemmaRefactor::calculate_hash(&id, &name, &triggers, &remaining_body);
            LemmaRefactor::compile_and_lock(&id, &name, &triggers, &calculated_hash, &remaining_body, &file_path);
            hash = calculated_hash;
        }

        if !id.is_empty() && !triggers.is_empty() && !hash.is_empty() {
            Some(UnifiedLemma { id, name, triggers, hash })
        } else {
            None
        }
    }
}
