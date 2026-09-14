#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AccountTier {
    FreeFun,
    EnterprisePaid,
    PowerDeveloper,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAccount {
    pub uuid: String,
    pub corporate_domain: String,
    pub tier: AccountTier,
}

