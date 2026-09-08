use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Explicitly sensitive refresh material. Debug never exposes the credential.
#[derive(Serialize, Deserialize)]
pub struct RefreshCredential {
    pub(crate) issuer: String,
    pub(crate) client_id: String,
    pub(crate) secret: String,
}
impl std::fmt::Debug for RefreshCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RefreshCredential([REDACTED])")
    }
}
impl Drop for RefreshCredential {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}
impl RefreshCredential {
    /// Import sensitive material obtained from an authenticated token exchange.
    pub fn import(issuer: String, client_id: String, secret: String) -> Self {
        Self {
            issuer,
            client_id,
            secret,
        }
    }
    pub fn issuer(&self) -> &str {
        &self.issuer
    }
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub(crate) fn secret(&self) -> &str {
        &self.secret
    }
}
