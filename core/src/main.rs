mod latex;
mod engine;
mod refactor;

use std::fs;
use engine::HeraclitusCore;

fn main() {
    let target_file = "tests/manuscript.tex".to_string();

    let file_content = match fs::read_to_string(&target_file) {
        Ok(content) => content,
        Err(_) => { std::process::exit(1); }
    };

    let compiler = HeraclitusCore::new();
    let paragraphs: Vec<&str> = file_content.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    let mut manifest_summary = format!(
        "# HERACLITUS EPISTEMIC MANIFEST SUMMARY REPORT\n\nTarget File Ingested: {}\nStatus: EVALUATION COMPLETE\n\n## Epistemic Audit Ledger:\n\n", 
        target_file
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

    let _ = fs::write("tests/HERACLITUS_MANIFEST_SUMMARY.md", manifest_summary);
    let _ = fs::write("tests/Validated.sve", sve_script_output);
}
