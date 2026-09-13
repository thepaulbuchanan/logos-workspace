use crate::engine::ParagraphDiagnostic;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectFile {
    pub name: String,
    pub raw_content: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationProject {
    pub project_id: String,
    pub owner_uuid: String,
    pub files: Vec<ProjectFile>,
    pub historical_runs_count: u32,
}
