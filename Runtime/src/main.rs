mod latex;
mod engine;
mod refactor;
mod registry;
mod lean;
mod pdf;
mod critic;
mod citation;
mod logoslang;
mod wiki;

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
    let (default_test_dir, _logos_dir) = locate_base_paths();
    
    // 🔍 DYNAMIC WORKSPACE ARGUMENT EXTRACTOR
    // Automatically intercepts project workspace paths piped downstream from the server
    let args: Vec<String> = std::env::args().collect();
    let target_dir = if args.len() > 1 {
        args[1].clone()
    } else {
        default_test_dir
    };

    let target_filename = "manuscript.tex";
    let target_path = format!("{}/{}", target_dir, target_filename);

    let compiler = HeraclitusCore::new();
    let _pdf_safe_linter = PdfExtractorCore::new();

    if target_path.starts_with("http") || target_path.contains("wikipedia.org") {
        eprintln!("🚀 WIKIPEDIA GROUND-TRUTH PIPELINE INGESTION RUNNING OVER ACTIVE LOOP...");
        match WikipediaInglector::fetch_article_text(&target_path) {
            Ok(paragraphs) => {
                let mut wiki_summary = format!(
                    "# HERACLITUS SOVEREIGN EVIDENCE LEDGER: WIKIPEDIA COMPILATION\nSource Target URL: {}\nStatus: INDEPENDENT AUDIT COMPLETE\n\n## Epistemic Audit Log By Category:\n\n", 
                    target_path
                );
                let mut chunk_id = 1;
                for p in &paragraphs {
                    let (_, summary_str, _, ir_trace) = compiler.evaluate_block(p, chunk_id);
                    if ir_trace.is_empty() { continue; }
                    println!("[Line {} -> Chunk {}] {}", chunk_id * 2, chunk_id, ir_trace);
                    if !summary_str.is_empty() { wiki_summary.push_str(&summary_str); }
                    chunk_id += 1;
                }
            }
            Err(e) => eprintln!("🔴 Wikipedia Ingestion Failure: {}", e)
        }
        return;
    }

    if !Path::new(&target_path).exists() {
        // Fallback seed guard to ensure the binary never exits or crashes out-of-bounds
        let _ = fs::create_dir_all(&target_dir);
        let _ = fs::write(&target_path, "% Heraclitus Workspace Target Guard\n");
    }
    
    let file_content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(_) => std::process::exit(1),
    };

    let mut current_block = String::new();
    let mut block_start_line = 1;
    let mut global_line_counter = 1;
    let mut block_index = 1;

        for line in file_content.lines() {
        if line.trim().is_empty() {
            if !current_block.trim().is_empty() {
                // FIX: Added underscores to satisfy strict compiler linter requirements
                let (_, _summary_str, _sve_str, ir_trace) = compiler.evaluate_block(&current_block, block_start_line);
                if !ir_trace.is_empty() {
                    println!("[Line {} -> Chunk {}] {}", block_start_line, block_index, ir_trace);
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
        let (_, _summary_str, _sve_str, ir_trace) = compiler.evaluate_block(&current_block, block_start_line);
        if !ir_trace.is_empty() {
            println!("[Line {} -> Chunk {}] {}", block_start_line, block_index, ir_trace);
        }
    }
}
