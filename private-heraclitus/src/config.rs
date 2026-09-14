use crate::engine::VerificationEngine;
use crate::dashboard::project::LiveWorkspaceSession;
use crate::auth::CryptographicAuthRegistry;
use std::sync::Mutex;

pub struct AppState {
    pub engine: VerificationEngine,
    pub session: Mutex<LiveWorkspaceSession>,
    pub auth_registry: Mutex<CryptographicAuthRegistry>,
}
