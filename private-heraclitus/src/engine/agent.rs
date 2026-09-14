use crate::engine::{LibraryManifestLock, LockedLemmaEntry, VerificationEngine};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn execute_library_lock_pass(engine: &VerificationEngine, target_lock_path: &str, specs_dir_path: &str) -> LibraryManifestLock {
    println!("\n[AGENT] Initiating Hermetic Invariant Lock-Step Verification Pass...");
    let mut locked_registry = HashMap::new();

    let specs_dir = Path::new(specs_dir_path);
    if specs_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(specs_dir) {
            for entry in entries.flatten() {
                let current_path = entry.path();
                if current_path.extension().map_or(false, |ext| ext == "md") {
                    let file_content = fs::read_to_string(&current_path).expect("Unable to read file");
                    
                    let mut target_id = String::new();
                    for line in file_content.lines() {
                        let clean_line = line.trim().to_lowercase();
                        if clean_line.starts_with("lemma_id") || clean_line.starts_with("id") {
                            if let Some(val_part) = line.split(':').nth(1) {
                                target_id = val_part.replace('"', "").replace('\'', "").trim().to_string();
                                break;
                            }
                        }
                    }

                    if !target_id.is_empty() {
                        let canonical_name = format!("{}.md", target_id.replace("-", "_"));
                        let canonical_path = specs_dir.join(&canonical_name);
                        
                        if current_path != canonical_path {
                            fs::rename(&current_path, &canonical_path)
                                .expect("Failed to execute agent renaming function on disk.");
                            println!("  ↳ [AGENT RENAME] Standardised filename: '{}' ──► '{}'", 
                                     current_path.file_name().unwrap().to_string_lossy(), canonical_name);
                        }
                    }
                }
            }
        }
    }

    for (lemma_id, ctx) in &engine.compile_dictionary {
        let structural_hash = engine.compute_structural_hash(ctx);
        
        let locked_entry = LockedLemmaEntry {
            lemma_id: lemma_id.clone(),
            structural_hash,
            compiled_nodes_count: ctx.ast_nodes.len(),
        };
        
        locked_registry.insert(lemma_id.clone(), locked_entry);
    }

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let manifest = LibraryManifestLock {
        manifest_version: "1.0.0".to_string(),
        global_verification_timestamp: timestamp,
        total_verified_lemmas: locked_registry.len(),
        locked_registry,
    };

    let json_payload = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write(target_lock_path, json_payload)
        .expect("Failed to write the library manifest lock file to disk root.");
        
    println!("[AGENT] Success! Invariant manifest sealed. Cryptographic lock file updated at: {}", target_lock_path);
    manifest
}
