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
    pub actor_uuid: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveCursorCoordinates {
    pub line_index: usize,
    pub character_offset: usize,
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
    pub active_cursor: Option<LiveCursorCoordinates>,
}

pub struct LiveWorkspaceSession {
    pub active_project: VerificationProject,
    pub diagnostic_timeline: Vec<Vec<ParagraphDiagnostic>>,
    pub active_collaborators: HashMap<String, CollaboratorSession>,
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

    pub fn register_collaborator(&mut self, session: CollaboratorSession) {
        println!(
            "[WORKSPACE SECURITY] User '{}' ({:?}) attached with workspace authority role '{:?}'",
            session.user_uuid, session.account_tier, session.active_role
        );
        self.active_collaborators.insert(session.user_uuid.clone(), session);
    }

    pub fn update_collaborator_cursor(&mut self, user_uuid: &str, coords: LiveCursorCoordinates) -> Result<(), String> {
        if let Some(collaborator) = self.active_collaborators.get_mut(user_uuid) {
            println!(
                "[CURSOR SYNC] User '{}' repositioned to index mapping coords -> [Line: {}, Offset: {}]",
                user_uuid, coords.line_index, coords.character_offset
            );
            collaborator.active_cursor = Some(coords);
            Ok(())
        } else {
            Err("USER_NOT_FOUND".to_string())
        }
    }

    pub fn process_shared_stream_mutation(&mut self, event: EditorStreamEvent) -> Result<Vec<String>, String> {
        let user_session = self.active_collaborators.get(&event.actor_uuid)
            .ok_or_else(|| "ACCESS_DENIED: Unauthenticated actor string detected.".to_string())?;

        if user_session.active_role == WorkspaceRole::Viewer {
            return Err("ACCESS_DENIED: Viewer tier lacks permission variables to mutate file buffers.".to_string());
        }

        if let Some(lock_owner) = self.active_file_locks.get(&event.file_target) {
            if lock_owner != &event.actor_uuid {
                return Err(format!("FILE_LOCKED: Content buffer is currently being edited by user '{}'", lock_owner));
            }
        } else {
            self.active_file_locks.insert(event.file_target.clone(), event.actor_uuid.clone());
        }

        println!(
            "[WORKSPACE STREAM] Applying text delta from editor '{}' to file '{}'...",
            event.actor_uuid, event.file_target
        );
        
        let mut updated_content = String::new();
        for file in &mut self.active_project.files {
            if file.name == event.file_target {
                file.raw_content = event.line_delta.clone();
                updated_content = file.raw_content.clone();
            }
        }

        self.active_file_locks.remove(&event.file_target);

        Ok(updated_content
            .split("\n\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }
}
