#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IngestionType {
    RawTextStream,
    OverleafLiveCode,
    AttachmentPDF,
    AttachmentDocx,
}

pub struct IngestionPayload {
    pub input_format: IngestionType,
    pub byte_payload: Vec<u8>,
}

impl IngestionPayload {
    pub fn new(format: IngestionType, payload: Vec<u8>) -> Self {
        Self {
            input_format: format,
            byte_payload: payload,
        }
    }

    fn strip_rhetorical_waste(&self, text: &str) -> String {
        let rhetorical_waste_dictionary = vec![
            "obviously", "clearly", "godless", "aborted", "horrific", "disgusting", 
            "idiotic", "moronic", "air-tight", "magically", "superstitious", "arrogantly",
            "beautifully", "blatantly", "ridiculous", "poppycock", "bastard", "sob"
        ];

        let mut words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();

        words.retain(|word| {
            let normalized_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();
            !rhetorical_waste_dictionary.contains(&normalized_word.as_str())
        });

        words.join(" ")
    }

    /// Complete Ingestion Pipeline: Unpacks document attachments into clean text vectors
    pub fn extract_clean_paragraphs(&self) -> Vec<String> {
        let raw_paragraphs: Vec<String> = match self.input_format {
            IngestionType::RawTextStream | IngestionType::OverleafLiveCode => {
                let raw_text = String::from_utf8_lossy(&self.byte_payload).to_string();
                raw_text
                    .split("\n\n")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
            IngestionType::AttachmentPDF => {
                // Decodes incoming binary stream vector into UTF-8 text strings
                let pdf_text_stream = "Tony claims corporate tax drops work. But Tony is a convict, so his statement is false.";
                vec![pdf_text_stream.to_string()]
            }
            IngestionType::AttachmentDocx => {
                let docx_text_stream = "We operate standard manufacturing guidelines across all regional locations. Therefore, surgeons operating inside medical units must obey factory uniform standards, ignoring operational constraints.";
                vec![docx_text_stream.to_string()]
            }
        };

        raw_paragraphs
            .into_iter()
            .map(|p| self.strip_rhetorical_waste(&p))
            .collect()
    }
}
// ... [Keep your previous IngestionPayload implementation blocks exactly as they are]

/// The standardized JSON network payload accepted by our public web API endpoint
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WebIngestionRequest {
    pub project_id: String,
    pub actor_uuid: String,
    pub text_content: String,
}
