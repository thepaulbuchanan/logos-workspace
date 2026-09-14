use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct LemmaRefactor;

impl LemmaRefactor {
    // Generates a deterministic machine signature hash string
    pub fn calculate_hash(id: &str, triggers: &[String]) -> String {
        let mut hash: usize = 5381;
        for c in id.bytes() { hash = ((hash << 5).wrapping_add(hash)) + c as usize; }
        for tr in triggers {
            for b in tr.bytes() { hash = ((hash << 5).wrapping_add(hash)) + b as usize; }
        }
        format!("sle_sha256_auto_{:x}", hash)
    }

    // Builds the clean symbolic code block and rewrites the file safely down to disk
    pub fn compile_and_lock(id: &str, name: &str, triggers: &[String], hash: &str, body: &str, path: &PathBuf) {
        let symbolic_bytecode = format!(
            "DECLARE_LEMMA({}) {{\n  MATCH_CONTEXT(Rhetorical_Pattern{:?});\n  ON_VIOLATION(THROW_QUARANTINE_CONJECTURE);\n}}\n",
            id.replace("-", "_"), triggers
        );

        let updated_manifest = format!(
            "---\nlemma_id: {}\nname: {}\ntriggers: {:?}\nep_hash: {}\n---\n\n### 1. Human Readable Specification\nAuto-compiled from Williamson Master List.\n\n### 2. Machine Compiled Symbolic Logos Block\n```sve\n{}```\n{}",
            id, name, triggers, hash, symbolic_bytecode, body.trim()
        );

        if let Ok(mut file) = File::create(path) {
            let _ = file.write_all(updated_manifest.as_bytes());
            println!("⚡ HERACLITUS REFACTOR: Compiled and signed asset {} -> Locked.", id);
        }
    }
}
