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
}

impl PdfExtractorCore {
    pub fn new() -> Self {
        PdfExtractorCore {
            header_regex: Regex::new(r"^(?P<num>\d+)\.\s+(?P<title>[^:\n\(\]]+)").unwrap(),
            trigger_cleaner: Regex::new(r#"[."';:,\(\)!\?]"#).unwrap(),
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
                
                let lemma_id = format!("SVE-L1{:02}", number_id.parse::<usize>().unwrap_or(0));
                
                let cleaned_text = self.trigger_cleaner.replace_all(block, "");
                let words: Vec<&str> = cleaned_text.split_whitespace().collect();
                let mut candidate_triggers = Vec::new();
                
                // FIX: Correctly extract indices sequentially from the string slice array vector
                if words.len() >= 2 {
                    candidate_triggers.push(format!("{} {}", words[0], words[1]).to_lowercase());
                } else if !words.is_empty() {
                    candidate_triggers.push(words[0].to_lowercase());
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
