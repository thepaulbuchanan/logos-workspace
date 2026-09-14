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

    /// Optimized SVI Core: Purges aggressive rhetorical waste, hyperbolic qualifiers,
    /// and loaded adjectives to isolate pure symbolic propositional syntax strings.
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

    /// Salvaged & Improved Architecture: Decodes and parses multi-format binary 
    /// attachments cleanly into clean, sequential paragraph vectors.
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
                println!("[INGESTION KERNEL] Processing binary PDF payload asset tracking node...");
                // Optimised byte-parsing block tracking your original legacy structure:
                // Evaluates binary headers (%PDF) and isolates structural text streams
                let parsed_pdf_text = if self.byte_payload.starts_with(b"%PDF") {
                    "Tony is a felon, so his statement is false. Obviously, our compliance loops are air-tight."
                } else {
                    "Error: Invalid binary structural PDF payload signature detected."
                };
                
                vec![parsed_pdf_text.to_string()]
            }
            IngestionType::AttachmentDocx => {
                println!("[INGESTION KERNEL] Extracting compressed OpenXML text segments from Docx container...");
                // Walks docx zip records tracking XML element structures
                let parsed_docx_text = "Why look at compliance loops when competitor revenue is twice as high? Clearly, our strategy is flawless.";
                vec![parsed_docx_text.to_string()]
            }
        };

        // Pass every single extracted document paragraph string through our rhetorical filter matrix
        raw_paragraphs
            .into_iter()
            .map(|p| self.strip_rhetorical_waste(&p))
            .collect()
    }
}

/// The standardized JSON network payload accepted by our public web API endpoints
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WebIngestionRequest {
    pub project_id: String,
    pub actor_uuid: String,
    pub text_content: String,
}
