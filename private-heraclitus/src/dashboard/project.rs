use crate::auth::AccountTier;
use crate::engine::ParagraphDiagnostic;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct EditorStreamEvent {
    pub file_target: String,
    pub line_delta: String,
    pub actor_uuid: String, // Tracks exactly which collaborator pushed the update
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WorkspaceRole {
    Owner,
    Editor,
    Viewer,
}

#[derive(Debug, Clone)]
pub struct CollaboratorSession {
    pub user_uuid: String,
    pub account_tier: AccountTier,
    pub active_role: WorkspaceRole,
}

pub struct LiveWorkspaceSession {
    pub active_project: VerificationProject,
    pub diagnostic_timeline: Vec<Vec<ParagraphDiagnostic>>,
    // Registry of actively authenticated workspace collaborators
    pub active_collaborators: HashMap<String, CollaboratorSession>,
    // File buffer locks preventing simultaneous mutation collisions
    pub active_file_locks: HashMap<String, String>, 
}

impl LiveWorkspaceSession {
    pub fn new(project: VerificationProject) -> Self {
        Self {
            active_project: project,
            diagnostic_timeline: Vec::new(),
            active_collaborators: HashMap::new(),
            active_file_locks: HashMap::new(),
        }
    }

    /// Access Management: Securely registers a new corporate or community collaborator to the live workspace
    pub fn register_collaborator(&mut self, session: CollaboratorSession) {
        println!(
            "[WORKSPACE SECURITY] User '{}' ({:?}) attached to session with role '{:?}'",
            session.user_uuid, session.account_tier, session.active_role
        );
        self.active_collaborators.insert(session.user_uuid.clone(), session);
    }

    /// Reactive Multi-User Pipeline: Acquires a document write-lock and applies typing deltas
    pub fn process_shared_stream_mutation(&mut self, event: EditorStreamEvent) -> Result<Vec<String>, String> {
        // Step 1: Verify user possesses authorization clearance to mutate the buffer
        let user_session = self.active_collaborators.get(&event.actor_uuid)
            .ok_or_else(|| "ACCESS_DENIED: Unauthenticated actor string detected.".to_string())?;

        if user_session.active_role == WorkspaceRole::Viewer {
            return Err("ACCESS_DENIED: Viewer tier lacks permission variables to mutate file buffers.".to_string());
        }

        // Step 2: Operational Mutex Check - Prevent cross-user editing collisions
        if let Some(lock_owner) = self.active_file_locks.get(&event.file_target) {
            if lock_owner != &event.actor_uuid {
                return Err(format!("FILE_LOCKED: Content buffer is currently being edited by user '{}'", lock_owner));
            }
        } else {
            // Acquire volatile write lock
            self.active_file_locks.insert(event.file_target.clone(), event.actor_uuid.clone());
        }

        println!(
            "[WORKSPACE STREAM] Applying text delta from editor '{}' to file '{}'...",
            event.actor_uuid, event.file_target
        );
        
        for file in &mut self.active_project.files {
            if file.name == event.file_target {
                file.raw_content = event.line_delta.clone();
            }
        }

        // Release lock context on completion of transaction frame (simulating a stream flush)
        self.active_file_locks.remove(&event.file_target);

        Ok(self.active_project.files[0].raw_content
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }
}
