mod latex;
mod engine;
mod refactor;
mod registry;
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
    
    // 🔍 CHOOSE TEST TARGET: Toggle this filename to run different simulation tracks
    // Target 1: The Standard Tex Manuscript
    let target_filename = "manuscript.tex";
    // Target 2: The Master Fallacies Reference Sheet (Uncomment to test the recursive loop!)
    // let target_filename = "Master_List_of_Logical_Fallacies.pdf";

    let target_path = format!("{}/{}", test_dir, target_filename);

    if !Path::new(&target_path).exists() {
        eprintln!("⚠️ HERACLITUS PATH FAULT: Unable to find target file at '{}'", target_path);
        std::process::exit(1);
    }

    // 🚨 IF THE INGESTION TARGET IS A BINARY PDF -> EXECUTE RECURSIVE SUBMISSION GENERATION
    if target_path.ends_with(".pdf") {
        println!("🚀 HERACLITUS RECURSIVE INVARIANT SCANNER ACTIVATED");
        println!(" Ingrained Source Target: {}", target_path);

        let extractor = PdfExtractorCore::new();
        match extractor.scan_pdf_to_paragraphs(&target_path) {
            Ok(paragraphs) => {
                let discovered_fallacies = extractor.extract_fallacy_nodes(&paragraphs);
                println!("🔍 Analyzed PDF content -> Found {} Fallacy Candidate Nodes.", discovered_fallacies.len());

                for node in discovered_fallacies {
                    let out_file_name = format!("{}/{}_auto_generated.md", logos_dir, node.id);
                    let file_path = PathBuf::from(out_file_name);

                    // If the lemma file does not exist yet in LogosLib, automatically compile and commit it!
                    if !file_path.exists() {
                        let calculated_hash = refactor::LemmaRefactor::calculate_hash(&node.id, &node.candidate_triggers);
                        refactor::LemmaRefactor::compile_and_lock(
                            &node.id,
                            &node.name,
                            &node.candidate_triggers,
                            &calculated_hash,
                            &node.body,
                            &file_path
                        );
                    }
                }
                println!("🔒 Automated LogosLib Library Sync Complete.");
            }
            Err(err) => eprintln!("🔴 PDF Extraction Failure Loop: {}", err),
        }
        return;
    }

    // Default Fallback Track: Processing the active LaTeX manuscript
    let file_content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(_) => std::process::exit(1),
    };

    let compiler = HeraclitusCore::new();
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
        if !sve_str.is_empty() { sve_script_output.push_str(&sve_str); }
        clean_paragraph_count += 1;
    }

    let _ = fs::write(format!("{}/HERACLITUS_MANIFEST_SUMMARY.md", test_dir), manifest_summary);
    let _ = fs::write(format!("{}/Validated.sve", test_dir), sve_script_output);
}
