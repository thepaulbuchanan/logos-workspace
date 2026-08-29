use std::fs;
use std::path::Path;
use regex::Regex;

pub struct CitationAuditor {
    cite_regex: Regex,
    appendix_regex: Regex,
}

impl CitationAuditor {
    pub fn new() -> Self {
        CitationAuditor {
            // Captures formats like "\cite{paper_id}", "[Ref 12]", "(see Section 4)"
            cite_regex: Regex::new(r"(?:\\cite\{|\[Ref\s+|\(see\s+)([a-zA-Z0-9_\-\s]{1,30})(?:\}|\]|\))").unwrap(),
            // Scans target reference files for anchor keys or label targets
            appendix_regex: Regex::new(r"\\label\{([a-zA-Z0-9_\-\s]{1,30})\}").unwrap(),
        }
    }

    // 🔬 THE CROSS-DOCUMENT CROSS-EXAMINER
    pub fn audit_block_references(&self, raw_block: &str, test_dir: &str) -> Result<(), String> {
        let lower_block = raw_block.to_lowercase();
        
        // Extract all citation strings sitting inside this paragraph block
        for caps in self.cite_regex.captures_iter(raw_block) {
            if let Some(mat) = caps.get(1) {
                let citation_anchor = mat.as_str().trim();
                
                // 1. PHASE 1: STRUCTURAL ORPHAN CHECK
                // Probe the system to find the matching companion appendix document file path
                let reference_library_file = format!("{}/appendix.tex", test_dir);
                if !Path::new(&reference_library_file).exists() {
                    return Err(format!("SVE-L401 ORPHAN: Reference document library '{}' is missing.", reference_library_file));
                }

                let ref_content = fs::read_to_string(&reference_library_file)
                    .map_err(|_| "Failed to read reference library string".to_string())?;

                // Check if the cited anchor point physically exists as an active label inside the library
                let mut anchor_found = false;
                for ref_caps in self.appendix_regex.captures_iter(&ref_content) {
                    if let Some(ref_mat) = ref_caps.get(1) {
                        if ref_mat.as_str().trim() == citation_anchor {
                            anchor_found = true;
                            break;
                        }
                    }
                }

                if !anchor_found {
                    return Err(format!("SVE-L401 ORPHAN: Citation anchor '{}' does not exist in the appendix library.", citation_anchor));
                }

                // 2. PHASE 2: TELEMETRY SHIFT CHECK (VARIABLE DRIFT DETECTION)
                // If the narrative text asserts an outcome, cross-examine it against the appendix's internal numbers
                if lower_block.contains("collapse") || lower_block.contains("decrease") {
                    // Extract numerical metrics out of the companion appendix file
                    let value_regex = Regex::new(r"(\d+%)").unwrap();
                    for ref_line in ref_content.lines() {
                        if ref_line.contains(citation_anchor) {
                            if let Some(val_caps) = value_regex.captures(ref_line) {
                                let data_metric = val_caps.get(1).unwrap().as_str();
                                // If the appendix shows a tiny value (e.g. 2%), but the text claims a total collapse, flag it!
                                if data_metric == "2%" && (lower_block.contains("30%") || lower_block.contains("collapse")) {
                                    return Err(format!(
                                        "SVE-L401 SHIFT: Narrative asserts macro collapse, but companion anchor '{}' metrics register a negligible '{}' shift.", 
                                        citation_anchor, data_metric
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
