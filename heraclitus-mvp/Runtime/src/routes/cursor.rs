use axum::{extract::State, Json};
use std::sync::Arc;
use crate::config::AppState;
use crate::dashboard::{LiveCursorCoordinates, CollaboratorSession, WorkspaceRole};
use crate::auth::AccountTier;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CursorUpdateRequest {
    pub user_uuid: String,
    pub line_index: usize,
    pub character_offset: usize,
}

pub async fn handle_cursor_sync(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CursorUpdateRequest>,
) -> Json<String> {
    let mut session_lock = state.session.lock().unwrap();
    
    let coords = LiveCursorCoordinates {
        line_index: payload.line_index,
        character_offset: payload.character_offset,
    };

    match session_lock.update_collaborator_cursor(&payload.user_uuid, coords) {
        Ok(_) => Json("{\"status\": \"SYNC_SUCCESS\"}".to_string()),
        Err(_) => {
            session_lock.register_collaborator(CollaboratorSession {
                user_uuid: payload.user_uuid.clone(),
                account_tier: AccountTier::FreeFun,
                active_role: WorkspaceRole::Editor,
                active_cursor: None,
            });
            Json("{\"status\": \"REGISTERED_AND_SYNCED\"}".to_string())
        }
    }
}
