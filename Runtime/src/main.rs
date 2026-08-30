mod latex;
mod engine;
mod refactor;
mod registry;
mod citation;
mod critic;
mod lean;
mod pdf;

use std::fs;
use std::path::{Path, PathBuf};
use engine::HeraclitusCore;
use pdf::PdfExtractorCore;

fn locate_base_paths() -> (String, String) {
    let test_paths = vec!["Test", "../Test", "../../Test"];
    let mut selected_test = "Test".to_string();
    for p in test_paths {
        if Path::new(p).exists() && Path::new(p).is_dir() {
            selected_test = p.to_string();
            break;
        }
    }

    let logos_paths = vec!["LogosLib", "../LogosLib", "../../LogosLib"];
    let mut selected_logos = "LogosLib".to_string();
    for p in logos_paths {
        if Path::new(p).exists() && Path::new(p).is_dir() {
            selected_logos = p.to_string();
            break;
        }
    }

    (selected_test, selected_logos)
}

fn main() {
    let (test_dir, logos_dir) = locate_base_paths();
    
    // 🔍 SIMULATION TOGGLE
    let target_filename = "manuscript.tex";
    // let target_filename = "300_Fallacies.pdf";

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
            eprintln!("🔍 Discovered {} Fallacy Candidates. Writing Uncommitted Drafts...", discovered.len());

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
            eprintln!("🟢 Stage A Complete: Uncommitted Drafts Staged in LogosLib.");
        }
        return;
    }

    // FIX: Route log headers to standard error to clear standard output pipes
    eprintln!("⚙️ STAGE C: INITIALISING THE HERACLITUS REFACTOR CORE");
    
    let compiler = HeraclitusCore::new();
    let file_content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(_) => std::process::exit(1),
    };

    let paragraphs: Vec<&str> = file_content.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    let mut manifest_summary = format!(
        "# HERACLITUS EPISTEMIC MANIFEST SUMMARY REPORT\n\nTarget File Ingested: {}\nStatus: EVALUATION COMPLETE\n\n## Epistemic Audit Ledger:\n\n", 
        target_path
    );
    
    let mut sve_script_output = "-- HERACLITUS SCRIPT: INVARIANT LOGOS LEDGER\n-- VERSION: v1.0.0-ALPHA\n\n".to_string();
    let mut clean_paragraph_count = 1;

    for para in paragraphs {
        let (_, summary_str, sve_str, ir_trace) = compiler.evaluate_block(para, clean_paragraph_count);
        if ir_trace.is_empty() { continue; }
        
        println!("{}", ir_trace);
        
        if !summary_str.is_empty() { manifest_summary.push_str(&summary_str); }
        if !sve_script_output.is_empty() { sve_script_output.push_str(&sve_str); }
        clean_paragraph_count += 1;
    }

    let _ = fs::write(format!("{}/HERACLITUS_MANIFEST_SUMMARY.md", test_dir), manifest_summary);
    let _ = fs::write(format!("{}/Validated.sve", test_dir), sve_script_output);
}
