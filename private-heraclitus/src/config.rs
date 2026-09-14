use crate::engine::VerificationEngine;
use crate::dashboard::project::LiveWorkspaceSession;
use crate::auth::user::CryptographicAuthRegistry;

pub struct AppState {
    pub engine: VerificationEngine,
    pub session: std::sync::Mutex<LiveWorkspaceSession>,
    pub auth_registry: std::sync::Mutex<CryptographicAuthRegistry>,
}
