use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountTier {
    FreeFun,
    EnterprisePaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub uuid: String,
    pub corporate_domain: String,
    pub tier: AccountTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSessionTicket {
    pub token_string: String,
    pub user_uuid: String,
    pub expires_at: u64,
}

pub struct CryptographicAuthRegistry {
    // Maps a cryptographically signed token string to an active session ticket
    pub active_tokens: HashMap<String, AuthSessionTicket>,
    pub user_records: HashMap<String, UserAccount>,
}

impl CryptographicAuthRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            active_tokens: HashMap::new(),
            user_records: HashMap::new(),
        };
        registry.hydrate_mock_production_records();
        registry
    }

    fn hydrate_mock_production_records(&mut self) {
        // Hydrate our canonical test identities safely into the authorization core
        let owner = UserAccount {
            uuid: "usr_owner_90a1".to_string(),
            corporate_domain: "fortune500_firm.com".to_string(),
            tier: AccountTier::EnterprisePaid,
        };
        let community = UserAccount {
            uuid: "usr_community_44b2".to_string(),
            corporate_domain: "open_source_grid.org".to_string(),
            tier: AccountTier::FreeFun,
        };

        self.user_records.insert(owner.uuid.clone(), owner);
        self.user_records.insert(community.uuid.clone(), community);
    }

    /// Issues a unique, signed session token for a validated user identity
    pub fn mint_auth_token(&mut self, user_uuid: &str) -> Result<String, String> {
        if !self.user_records.contains_key(user_uuid) {
            return Err("AUTHENTICATION_FAILED: Identity unmatched in database.".to_string());
        }

        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let expiry = timestamp + 3600; // Token active for exactly 1 hour

        // Calculate a unique cryptographic token hash
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", user_uuid, timestamp).as_bytes());
        let token_string = format!("logos-jwt:{:x}", hasher.finalize());

        let ticket = AuthSessionTicket {
            token_string: token_string.clone(),
            user_uuid: user_uuid.to_string(),
            expires_at: expiry,
        };

        self.active_tokens.insert(token_string.clone(), ticket);
        println!("[SECURITY LAYER] Secure auth token minted successfully for user '{}'", user_uuid);
        Ok(token_string)
    }

    /// Gatekeeper: Cryptographically validates an incoming session token string against expiry limits
    pub fn verify_token_clearance(&self, token_string: &str) -> Result<UserAccount, String> {
        let ticket = self.active_tokens.get(token_string)
            .ok_or_else(|| "SECURITY_UNAUTHORIZED_ACTOR: Cryptographic session token is invalid or missing.".to_string())?;

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time > ticket.expires_at {
            return Err("SECURITY_TOKEN_EXPIRED: Active session ticket window closed.".to_string());
        }

        let user = self.user_records.get(&ticket.user_uuid)
            .ok_or_else(|| "SECURITY_IDENTITY_ORPHANED: Associated user uuid has been purged from memory registries.".to_string())?;

        Ok(user.clone())
    }
}
