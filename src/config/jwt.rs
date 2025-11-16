#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JwtConfig {
    secret: String,
    access_token_expires_in: i64,
    refresh_token_expires_in: i64,
}

impl JwtConfig {
    pub fn get_jwt_secret(&self) -> String {
        self.secret.clone()
    }

    pub fn get_access_token_expiry(&self) -> i64 {
        self.access_token_expires_in
    }

    pub fn get_refresh_token_expiry(&self) -> i64 {
        self.refresh_token_expires_in
    }
}
