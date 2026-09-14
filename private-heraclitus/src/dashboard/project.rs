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

/// Represents a live editing event transaction (Overleaf-style update stream)
#[derive(Debug, Clone)]
pub struct EditorStreamEvent {
    pub file_target: String,
    pub line_delta: String,
}

pub struct LiveWorkspaceSession {
    pub active_project: VerificationProject,
    pub diagnostic_timeline: Vec<Vec<ParagraphDiagnostic>>,
}

impl LiveWorkspaceSession {
    pub fn new(project: VerificationProject) -> Self {
        Self {
            active_project: project,
            diagnostic_timeline: Vec::new(),
        }
    }

    /// Reactive Pipeline: Simulates a user typing live inside the editor interface.
    /// Overwrites content buffers and cleanly splits text into a paragraph vector.
    pub fn process_stream_mutation(&mut self, event: EditorStreamEvent) -> Vec<String> {
        println!("[WORKSPACE STREAM] Applying incoming text delta to '{}'...", event.file_target);
        
        for file in &mut self.active_project.files {
            if file.name == event.file_target {
                // Append or mutate content stream buffer
                file.raw_content = event.line_delta.clone();
            }
        }

        // Return tokenised segments parsed by line breaks
        self.active_project.files[0].raw_content
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
pub mod project;
