use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationCertificate {
    pub certificate_id: String,
    pub target_project_id: String,
    pub sha256_document_hash: String,
    pub compilation_timestamp: u64,
    pub compiled_lemma_count: usize,
    pub cryptographic_seal_signature: String,
}

impl VerificationCertificate {
    /// Generates a verifiable cryptographic token sealing the state of the document logic
    pub fn generate_seal(project_id: &str, raw_document_text: &str, lemma_count: usize) -> Self {
        // Calculate document content hash
        let mut hasher = Sha256::new();
        hasher.update(raw_document_text.as_bytes());
        let doc_hash = format!("{:x}", hasher.finalize());

        // Get system timestamp parameters
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        let cert_id = format!("CERT-{}-{}", project_id, timestamp);

        // Compute private key signature payload (Simulating engine signing key)
        let mut sig_hasher = Sha256::new();
        sig_hasher.update(format!("{}{}{}", cert_id, doc_hash, lemma_count).as_bytes());
        let seal_signature = format!("logos-signed:{:x}", sig_hasher.finalize());

        Self {
            certificate_id: cert_id,
            target_project_id: project_id.to_string(),
            sha256_document_hash: doc_hash,
            compilation_timestamp: timestamp,
            compiled_lemma_count: lemma_count,
            cryptographic_seal_signature: seal_signature,
        }
    }

    /// Exports the verifiable token block to a standard JSON format string
    pub fn to_json_payload(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
