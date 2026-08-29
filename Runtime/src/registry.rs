use std::fs;
use std::path::PathBuf;
use crate::refactor::LemmaRefactor;

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
                        if let Some(lemma) = Self::parse_or_compile(&content, path) {
                            loaded_lemmas.push(lemma);
                        }
                    }
                }
            }
        }
        loaded_lemmas
    }

    fn parse_or_compile(content: &str, file_path: PathBuf) -> Option<UnifiedLemma> {
        if !content.starts_with("---") { return None; }
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() < 3 { return None; }
        
        // Isolate the front-matter block payload string segment explicitly
        let yaml_text = parts[1];
        let remaining_body = parts[2..].join("---");
        
        let mut id = String::new();
        let mut name = String::new();
        let mut hash = String::new();
        let mut triggers = Vec::new();

        // FIX: Call .lines() directly on the raw extracted string segment text
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

        if hash.is_empty() && !id.is_empty() && !triggers.is_empty() {
            let calculated_hash = LemmaRefactor::calculate_hash(&id, &triggers);
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
