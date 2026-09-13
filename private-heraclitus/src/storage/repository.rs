use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

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
    pub fn generate_seal(project_id: &str, raw_document_text: &str, lemma_count: usize) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(raw_document_text.as_bytes());
        let doc_hash = format!("{:x}", hasher.finalize());

        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cert_id = format!("CERT-{}-{}", project_id, timestamp);

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

    pub fn to_json_payload(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// The Enterprise Central Storage Ledger Tracking Project History (Module 3 addition)
pub struct CertificateLedger {
    pub records: HashMap<String, Vec<VerificationCertificate>>,
}

impl CertificateLedger {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Stores a freshly signed verification token into the project's historical timeline array
    pub fn log_certificate(&mut self, project_id: &str, cert: VerificationCertificate) {
        let timeline = self.records.entry(project_id.to_string()).or_insert_with(Vec::new);
        timeline.push(cert);
    }

    pub fn get_history_count(&self, project_id: &str) -> usize {
        self.records.get(project_id).map(|v| v.len()).unwrap_or(0)
    }
}
