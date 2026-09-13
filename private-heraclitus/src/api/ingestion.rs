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

    /// Module 2 Input Pre-processor: Normalises multi-format bytes into clean, 
    /// sequential paragraph strings for the semantic runtime core.
    pub fn extract_clean_paragraphs(&self) -> Vec<String> {
        match self.input_format {
            IngestionType::RawTextStream | IngestionType::OverleafLiveCode => {
                // Convert raw bytes directly to UTF-8 text strings
                let raw_text = String::from_utf8_lossy(&self.byte_payload).to_string();
                raw_text
                    .split("\n\n") // Segment text cleanly by paragraph boundaries
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
            IngestionType::AttachmentPDF => {
                // Mock execution hook for pdf-extract / local multi-modal processing
                vec![
                    "Paragraph 1: Ingested text segment from PDF payload format.".to_string(),
                    "Paragraph 2: Secondary data frame extracted from document tree.".to_string(),
                ]
            }
            IngestionType::AttachmentDocx => {
                // Mock execution hook for docx-rs file decoders
                vec![
                    "Paragraph 1: Corporate data asset successfully pulled from docx file block.".to_string(),
                ]
            }
        }
    }
}
