#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationCertificate {
    pub certificate_id: String,
    pub target_project_id: String,
    pub sha256_document_hash: String,
    pub compilation_timestamp: u64,
    pub compiled_lemma_count: usize,
    pub cryptographic_seal_signature: String,
}
