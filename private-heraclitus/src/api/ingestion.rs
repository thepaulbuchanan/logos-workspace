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
    pub fn extract_clean_paragraphs(&self) -> Vec<String> {
        // This pipeline will house structural converters like pdf-extract or docx-rs
        // For the current engine interface sandbox, it loops clean textual mock inputs
        vec![]
    }
}
