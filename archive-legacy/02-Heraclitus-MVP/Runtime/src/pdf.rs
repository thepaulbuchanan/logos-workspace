use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

// Allow dead code for alternate compilation pipelines
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExtractedFallacyNode {
    pub id: String,
    pub name: String,
    pub body: String,
    pub candidate_triggers: Vec<String>,
}

#[allow(dead_code)]
pub struct PdfExtractorCore {
    header_regex: Regex,
    trigger_cleaner: Regex,
    binary_content_regex: Regex,
}

#[allow(dead_code)]
impl PdfExtractorCore {
    pub fn new() -> Self {
        PdfExtractorCore {
            header_regex: Regex::new(r"^(?P<title>[A-Z][a-zA-Z\s’'–-]{3,40})(?:\n|\s+\(|$)").unwrap(),
            trigger_cleaner: Regex::new(r#"[."';:,\(\)!\?]"#).unwrap(),
            binary_content_regex: Regex::new(r"BT\s+.*?([A-Za-z\s]{5,100}).*?ET").unwrap(),
        }
    }

    pub fn scan_pdf_to_paragraphs<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<String>, String> {
        let mut file = File::open(file_path).map_err(|e| format!("IO Open Fault: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| format!("IO Read Fault: {}", e))?;

        let mut raw_text = match pdf_extract::extract_text_from_mem(&buffer) {
            Ok(text) => text,
            Err(_) => String::new()
        };

        if raw_text.trim().len() < 100 {
            let binary_string = String::from_utf8_lossy(&buffer);
            let mut fallbacks = Vec::new();

            for caps in self.binary_content_regex.captures_iter(&binary_string) {
                if let Some(mat) = caps.get(1) {
                    let cleaned = mat.as_str().trim().to_string();
                    if cleaned.len() > 10 {
                        fallbacks.push(cleaned);
                    }
                }
            }
            
            if !fallbacks.is_empty() {
                return Ok(fallbacks);
            }

            raw_text = binary_string.into_owned();
        }

        let normalized = raw_text.replace("\r\n", "\n");
        let paragraphs: Vec<String> = normalized
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        Ok(paragraphs)
    }

    pub fn extract_fallacy_nodes(&self, paragraphs: &[String]) -> Vec<ExtractedFallacyNode> {
        let mut detected_nodes = Vec::new();
        let mut index_counter = 1;

        for block in paragraphs {
            if block.starts_with("Contents") || block.starts_with("Preface") || block.starts_with("Introduction") {
                continue;
            }

            if let Some(caps) = self.header_regex.captures(block) {
                let fallacy_name = caps.name("title").unwrap().as_str().trim();
                let lemma_id = format!("SVE-L3{:03}", index_counter);
                
                let cleaned_text = self.trigger_cleaner.replace_all(block, "");
                let words: Vec<&str> = cleaned_text.split_whitespace().collect();
                let mut candidate_triggers = Vec::new();
                
                // 🟢 FIX: Extract individual vector slice array index strings sequentially 
                if words.len() >= 2 {
                    let first_key = format!("{} {}", words[0].to_lowercase(), words[1].to_lowercase());
                    candidate_triggers.push(first_key);
                } else if !words.is_empty() {
                    candidate_triggers.push(words[0].to_lowercase());
                }

                if !candidate_triggers.is_empty() {
                    detected_nodes.push(ExtractedFallacyNode {
                        id: lemma_id,
                        name: fallacy_name.to_string(),
                        body: block.clone(),
                        candidate_triggers,
                    });
                    index_counter += 1;
                }
            }
        }
        
        if detected_nodes.is_empty() && !paragraphs.is_empty() {
            for (idx, block) in paragraphs.iter().take(50).enumerate() {
                let clean_name = format!("Scanned Fallacy Clause Element {}", idx + 1);
                let lemma_id = format!("SVE-L3{:03}", idx + 1);
                detected_nodes.push(ExtractedFallacyNode {
                    id: lemma_id,
                    name: clean_name,
                    body: block.clone(),
                    candidate_triggers: vec!["fallacy claim".to_string()],
                });
            }
        }

        detected_nodes
    }
}
