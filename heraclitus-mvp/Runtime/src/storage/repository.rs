use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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

pub struct CertificateLedger {
    pub db_directory: String,
    pub records: HashMap<String, Vec<VerificationCertificate>>,
}

impl CertificateLedger {
    pub fn new(db_path: &str) -> Self {
        let path = Path::new(db_path);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to construct system database storage folder hierarchy.");
        }

        Self {
            db_directory: db_path.to_string(),
            records: HashMap::new(),
        }
    }

    pub fn persist_certificate_to_disk(&mut self, project_id: &str, cert: VerificationCertificate) {
        let timeline = self.records.entry(project_id.to_string()).or_insert_with(Vec::new);
        timeline.push(cert.clone());

        let file_path = Path::new(&self.db_directory).join(format!("{}.json", cert.certificate_id));
        let json_payload = cert.to_json_payload();
        
        fs::write(&file_path, json_payload)
            .expect("Failed to write certificate token record directly to system database disk storage.");
        println!("[STORAGE COMPONENT] Successfully persisted signed certificate token to disk storage path: {:?}", file_path);
    }

    /// NEW AUTOMATED BACKGROUND BACKUP LAYER: Persists live working workspace file streams to hard drive
    pub fn backup_active_project_files(&self, project_id: &str, file_name: &str, raw_content: &str) {
        let backups_dir = Path::new(&self.db_directory).join("project_backups").join(project_id);
        if !backups_dir.exists() {
            fs::create_dir_all(&backups_dir).expect("Failed to establish secure disk target for backup channels.");
        }

        let target_file_path = backups_dir.join(file_name);
        fs::write(&target_file_path, raw_content)
            .expect("Failed to flush active document delta memory maps to hardware partition.");
        println!("[DISK SAVER] Flushed real-time workspace snapshot for '{}' ──► {:?}", file_name, target_file_path);
    }

    pub fn get_history_count_from_disk(&self, project_id: &str) -> usize {
        let mut count = 0;
        let path = Path::new(&self.db_directory);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name.contains(project_id) && file_name.ends_with(".json") {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
