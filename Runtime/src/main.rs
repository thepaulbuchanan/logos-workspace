mod latex;
mod engine;
mod refactor;
mod registry;
mod lean;
mod pdf;
mod critic;
mod citation; // 🟢 RESTORED: Natively registers the cross-document citation auditor

use std::fs;
use std::path::{Path, PathBuf};
use engine::HeraclitusCore;
use pdf::PdfExtractorCore;


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
    let (test_dir, logos_dir) = locate_base_paths();
    let target_filename = "manuscript.tex";
    let target_path = format!("{}/{}", test_dir, target_filename);

    if !Path::new(&target_path).exists() {
        eprintln!("⚠️ HERACLITUS PATH FAULT: Unable to find target file at '{}'", target_path);
        std::process::exit(1);
    }

    if target_path.ends_with(".pdf") {
        eprintln!("🚀 STAGE A: ACTIVATING INVARIANT SCANNER");
        let extractor = PdfExtractorCore::new();
        if let Ok(paragraphs) = extractor.scan_pdf_to_paragraphs(&target_path) {
            let discovered = extractor.extract_fallacy_nodes(&paragraphs);
            for node in discovered {
                let out_name = format!("{}/{}_auto_generated.md", logos_dir, node.id);
                let file_path = PathBuf::from(out_name);
                if !file_path.exists() {
                    let draft_content = format!(
                        "---\nlemma_id: {}\nname: {}\ntriggers: {:?}\nep_hash: \n---\n\n\
                         ### 1. Human Readable Specification\nAuto-extracted from reference materials.\n\n\
                         ### 2. Verification Context\n{}",
                        node.id, node.name, node.candidate_triggers, node.body
                    );
                    let _ = fs::write(file_path, draft_content);
                }
            }
        }
        return;
    }

    eprintln!("⚙️ STAGE C: INITIALISING THE HERACLITUS REFACTOR CORE");
    let compiler = HeraclitusCore::new();
    let file_content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(_) => std::process::exit(1),
    };

    let mut manifest_summary = format!(
        "# HERACLITUS EPISTEMIC MANIFEST SUMMARY REPORT\n\nTarget File Ingested: {}\nStatus: EVALUATION COMPLETE\n\n## Epistemic Audit Ledger:\n\n", 
        target_path
    );
    let mut sve_script_output = "-- HERACLITUS SCRIPT: INVARIANT LOGOS LEDGER\n-- VERSION: v1.0.0-ALPHA\n\n".to_string();

    // 🔬 LINE-AWARE PARAGRAPH PARSING CORE
    let mut current_block = String::new();
    let mut block_start_line = 1;
    let mut global_line_counter = 1;
    let mut block_index = 1;

    for line in file_content.lines() {
        if line.trim().is_empty() {
            if !current_block.trim().is_empty() {
                // Evaluate block with exact file coordinate trackers passed into index space
                let (_, summary_str, sve_str, ir_trace) = compiler.evaluate_block(&current_block, block_start_line);
                if !ir_trace.is_empty() {
                    // Reformat the trace to print line limits explicitly to the terminal
                    println!("[Line {} -> Chunk {}] {}", block_start_line, block_index, ir_trace);
                    if !summary_str.is_empty() { manifest_summary.push_str(&summary_str); }
                    if !sve_str.is_empty() { sve_script_output.push_str(&sve_str); }
                    block_index += 1;
                }
                current_block.clear();
            }
            block_start_line = global_line_counter + 1;
        } else {
            if current_block.is_empty() {
                block_start_line = global_line_counter;
            }
            current_block.push_str(line);
            current_block.push('\n');
        }
        global_line_counter += 1;
    }

    // Process final trailing block trailing elements if buffer remains full
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
