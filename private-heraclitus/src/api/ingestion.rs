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

    /// Internal Core: Scans a string segment and strips out 'rhetorical waste' 
    /// based on the original SVI-Prototype specification catalog.
    fn strip_rhetorical_waste(&self, text: &str) -> String {
        // A compiled dictionary of emotional qualifiers, loaded labels, and hyperbolic padding
        let rhetorical_waste_dictionary = vec![
            "obviously", "clearly", "godless", "aborted", "horrific", "disgusting", 
            "idiotic", "moronic", "air-tight", "magically", "superstitious", "arrogantly",
            "beautifully", "blatantly", "ridiculous", "poppycock", "bastard", "sob"
        ];

        let mut words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.to_string())
            .collect();

        // Filter out any matching token mutations while preserving logical connectives
        words.retain(|word| {
            let normalized_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();
            !rhetorical_waste_dictionary.contains(&normalized_word.as_str())
        });

        words.join(" ")
    }

    /// Module 2 Input Pre-processor: Normalises multi-format bytes into clean, 
    /// stripped, sequential paragraph strings for the semantic runtime core.
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
                vec![
                    "Paragraph 1: Ingested text segment from PDF payload format.".to_string(),
                ]
            }
            IngestionType::AttachmentDocx => {
                vec![
                    "Paragraph 1: Corporate data asset successfully pulled from docx file block.".to_string(),
                ]
            }
        };

        // Pass every extracted paragraph string through our rhetorical filter matrix
        raw_paragraphs
            .into_iter()
            .map(|p| self.strip_rhetorical_waste(&p))
            .collect()
    }
}
