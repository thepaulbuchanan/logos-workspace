use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ExtractedFallacyNode {
    pub id: String,
    pub name: String,
    pub body: String,
    pub candidate_triggers: Vec<String>,
}

pub struct PdfExtractorCore {
    header_regex: Regex,
    trigger_cleaner: Regex,
    number_stripper: Regex,
}

impl PdfExtractorCore {
    pub fn new() -> Self {
        PdfExtractorCore {
            // FIX: Captures up to triple-digit indexes safely (e.g., "145. The Worst-Case Fallacy", "300. New Lemma")
            header_regex: Regex::new(r"^(?P<num>\d{1,3})\.\s+(?P<title>[^:\n\(\]]+)").unwrap(),
            trigger_cleaner: Regex::new(r#"[."';:,\(\)!\?]"#).unwrap(),
            number_stripper: Regex::new(r"^\d{1,3}\.?\s*").unwrap(),
        }
    }

    pub fn scan_pdf_to_paragraphs<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<String>, String> {
        let mut file = File::open(file_path).map_err(|e| format!("IO Open Fault: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| format!("IO Read Fault: {}", e))?;

        let raw_text = match pdf_extract::extract_text_from_mem(&buffer) {
            Ok(text) => text,
            Err(_) => String::from_utf8_lossy(&buffer).into_owned()
        };

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

        for block in paragraphs {
            if let Some(caps) = self.header_regex.captures(block) {
                let number_id = caps.name("num").unwrap().as_str();
                let fallacy_name = caps.name("title").unwrap().as_str().trim();
                
                // Index format matches higher sequence counts cleanly
                let parsed_num = number_id.parse::<usize>().unwrap_or(0);
                let lemma_id = format!("SVE-L{:03}", parsed_num);
                
                let pure_body = self.number_stripper.replace(block, "");
                let cleaned_text = self.trigger_cleaner.replace_all(&pure_body, "");
                let words: Vec<&str> = cleaned_text.split_whitespace().collect();
                
                let mut candidate_triggers = Vec::new();
                
                if words.len() >= 2 {
                    let first_key = format!("{} {}", words[0].to_string().to_lowercase(), words[1].to_string().to_lowercase());
                    candidate_triggers.push(first_key);
                } else if !words.is_empty() {
                    candidate_triggers.push(words[0].to_string().to_lowercase());
                }

                detected_nodes.push(ExtractedFallacyNode {
                    id: lemma_id,
                    name: fallacy_name.to_string(),
                    body: block.clone(),
                    candidate_triggers,
                });
            }
        }
        detected_nodes
    }
}
