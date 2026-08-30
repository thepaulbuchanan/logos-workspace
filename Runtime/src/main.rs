mod latex;
mod engine;
mod refactor;
mod registry;
mod lean;
mod pdf;
mod critic;
mod citation;
mod logoslang;
mod wiki; // 🟢 RESTORED: Registers the Wikipedia ingestion submodule to clear E0432

use std::fs;
use std::path::Path;
use engine::HeraclitusCore;
use pdf::PdfExtractorCore;
use wiki::WikipediaInglector;

fn locate_base_paths() -> (String, String) {
    let test_paths = vec!["Test", "../Test", "../../Test"];
    let mut selected_test = "Test".to_string();
    for p in test_paths {
        if Path::new(p).exists() && Path::new(p).is_dir() { selected_test = p.to_string(); break; }
    }
    let logos_paths = vec!["LogosLib", "../LogosLib", "../../LogosLib"];
    let mut selected_logos = "LogosLib".to_string();
    for p in logos_paths {
        if Path::new(p).exists() && Path::new(p).is_dir() { selected_logos = p.to_string(); break; }
    }
    (selected_test, selected_logos)
}

fn main() {
    let (test_dir, _logos_dir) = locate_base_paths();
    
    // 🟢 DYNAMIC MANUSCRIPT MODE: Wired for real-time local canvas stream updates
    let target_filename = "manuscript.tex";

    let compiler = HeraclitusCore::new();
    let _pdf_safe_linter = PdfExtractorCore::new();

    if target_filename.starts_with("http") || target_filename.contains("wikipedia.org") {
        eprintln!("🚀 WIKIPEDIA GROUND-TRUTH PIPELINE INGESTION RUNNING OVER ACTIVE LOOP...");
        match WikipediaInglector::fetch_article_text(target_filename) {
            Ok(paragraphs) => {
                let mut wiki_summary = format!(
                    "# HERACLITUS SOVEREIGN EVIDENCE LEDGER: WIKIPEDIA COMPILATION\nSource Target URL: {}\nStatus: INDEPENDENT AUDIT COMPLETE\n\n## Epistemic Audit Log By Category:\n\n", 
                    target_filename
                );
                let mut chunk_id = 1;
                // FIX: Enforced correct sized iteration over the String vector to clear E0277
                for p in &paragraphs {
                    let (_, summary_str, _, ir_trace) = compiler.evaluate_block(p, chunk_id);
                    if ir_trace.is_empty() { continue; }
                    println!("[Line {} -> Chunk {}] {}", chunk_id * 2, chunk_id, ir_trace);
                    if !summary_str.is_empty() { wiki_summary.push_str(&summary_str); }
                    chunk_id += 1;
                }
                let out_report = format!("{}/HERACLITUS_WIKI_REPORT.md", test_dir);
                let _ = fs::write(&out_report, wiki_summary);
            }
            Err(e) => eprintln!("🔴 Wikipedia Ingestion Failure: {}", e)
        }
        return;
    }

    let target_path = format!("{}/{}", test_dir, target_filename);
    if !Path::new(&target_path).exists() { std::process::exit(1); }
    
    let file_content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(_) => std::process::exit(1),
    };

    let mut manifest_summary = format!(
        "# HERACLITUS EPISTEMIC MANIFEST SUMMARY REPORT\n\nTarget File Ingested: {}\nStatus: EVALUATION COMPLETE\n\n## Epistemic Audit Ledger:\n\n", 
        target_path
    );
    let mut sve_script_output = "-- HERACLITUS SCRIPT: INVARIANT LOGOS LEDGER\n-- VERSION: v1.0.0-ALPHA\n\n".to_string();

    let mut current_block = String::new();
    let mut block_start_line = 1;
    let mut global_line_counter = 1;
    let mut block_index = 1;

    for line in file_content.lines() {
        if line.trim().is_empty() {
            if !current_block.trim().is_empty() {
                let (_, summary_str, sve_str, ir_trace) = compiler.evaluate_block(&current_block, block_start_line);
                if !ir_trace.is_empty() {
                    println!("[Line {} -> Chunk {}] {}", block_start_line, block_index, ir_trace);
                    if !summary_str.is_empty() { manifest_summary.push_str(&summary_str); }
                    if !sve_str.is_empty() { sve_script_output.push_str(&sve_str); }
                    block_index += 1;
                }
                current_block.clear();
            }
            block_start_line = global_line_counter + 1;
        } else {
            if current_block.is_empty() { block_start_line = global_line_counter; }
            current_block.push_str(line);
            current_block.push('\n');
        }
        global_line_counter += 1;
    }

    if !current_block.trim().is_empty() {
        let (_, summary_str, sve_str, ir_trace) = compiler.evaluate_block(&current_block, block_start_line);
        if !ir_trace.is_empty() {
            println!("[Line {} -> Chunk {}] {}", block_start_line, block_index, ir_trace);
            if !summary_str.is_empty() { manifest_summary.push_str(&summary_str); }
            if !sve_str.is_empty() { sve_script_output.push_str(&sve_str); }
        }
    }

    let _ = fs::write(format!("{}/HERACLITUS_MANIFEST_SUMMARY.md", test_dir), manifest_summary);
    let _ = fs::write(format!("{}/Validated.sve", test_dir), sve_script_output);
}
