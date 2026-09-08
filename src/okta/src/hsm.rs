//! Caller-owned HSM storage, matching libhimmelblau's sealing convention.
use crate::{AuthContinuation, RefreshCredential};
use kanidm_hsm_crypto::{
    provider::{BoxedDynTpm, Tpm},
    structures::{SealedData, StorageKey},
};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

#[derive(Debug, thiserror::Error)]
pub enum SealError {
    #[error("credential serialization failed")]
    Serialization,
    #[error("HSM operation failed")]
    Hsm,
    #[error("sealed credential belongs to a different client configuration")]
    Configuration,
    #[error("unsupported sealed credential version")]
    Version,
}
#[derive(Serialize, Deserialize)]
struct Envelope<T> {
    version: u32,
    credential: T,
}
pub fn seal_refresh_credential(
    credential: &RefreshCredential,
    tpm: &mut BoxedDynTpm,
    key: &StorageKey,
) -> Result<SealedData, SealError> {
    let data = serde_json::to_vec(&Envelope {
        version: 1,
        credential,
    })
    .map(Zeroizing::new)
    .map_err(|_| SealError::Serialization)?;
    tpm.seal_data(key, data).map_err(|_| SealError::Hsm)
}
pub fn unseal_refresh_credential(
    sealed: &SealedData,
    issuer: &str,
    client_id: &str,
    tpm: &mut BoxedDynTpm,
    key: &StorageKey,
) -> Result<RefreshCredential, SealError> {
    let data = tpm.unseal_data(key, sealed).map_err(|_| SealError::Hsm)?;
    let envelope: Envelope<RefreshCredential> =
        serde_json::from_slice(&data).map_err(|_| SealError::Serialization)?;
    if envelope.version != 1 {
        return Err(SealError::Version);
    }
    if envelope.credential.issuer != issuer || envelope.credential.client_id != client_id {
        return Err(SealError::Configuration);
    }
    Ok(envelope.credential)
}
pub fn seal_auth_continuation(
    continuation: &AuthContinuation,
    tpm: &mut BoxedDynTpm,
    key: &StorageKey,
) -> Result<SealedData, SealError> {
    let data = serde_json::to_vec(&Envelope {
        version: 1,
        credential: continuation.as_bytes(),
    })
    .map(Zeroizing::new)
    .map_err(|_| SealError::Serialization)?;
    tpm.seal_data(key, data).map_err(|_| SealError::Hsm)
}
pub fn unseal_auth_continuation(
    sealed: &SealedData,
    tpm: &mut BoxedDynTpm,
    key: &StorageKey,
) -> Result<AuthContinuation, SealError> {
    let data = tpm.unseal_data(key, sealed).map_err(|_| SealError::Hsm)?;
    let envelope: Envelope<Vec<u8>> =
        serde_json::from_slice(&data).map_err(|_| SealError::Serialization)?;
    if envelope.version != 1 {
        return Err(SealError::Version);
    }
    AuthContinuation::from_bytes(&envelope.credential).map_err(|_| SealError::Serialization)
}
#[cfg(test)]
mod tests {
    use super::*;
    use kanidm_hsm_crypto::{provider::SoftTpm, AuthValue};
    use std::str::FromStr;
    fn key(tpm: &mut BoxedDynTpm) -> StorageKey {
        let auth = AuthValue::from_str(&AuthValue::generate().unwrap()).unwrap();
        let loadable = tpm.root_storage_key_create(&auth).unwrap();
        tpm.root_storage_key_load(&auth, &loadable).unwrap()
    }
    #[test]
    fn round_trip_and_wrong_key() {
        let mut tpm = BoxedDynTpm::new(SoftTpm::new());
        let key1 = key(&mut tpm);
        let key2 = key(&mut tpm);
        let credential = RefreshCredential::import(
            "https://tenant.example".into(),
            "client".into(),
            "sensitive-fixture".into(),
        );
        let sealed = seal_refresh_credential(&credential, &mut tpm, &key1).unwrap();
        let exported = serde_json::to_string(&sealed).unwrap();
        assert!(!exported.contains("sensitive-fixture"));
        let restored = unseal_refresh_credential(
            &sealed,
            credential.issuer(),
            credential.client_id(),
            &mut tpm,
            &key1,
        )
        .unwrap();
        assert_eq!(restored.secret, credential.secret);
        assert!(matches!(
            unseal_refresh_credential(&sealed, credential.issuer(), "other", &mut tpm, &key1),
            Err(SealError::Configuration)
        ));
        assert!(unseal_refresh_credential(
            &sealed,
            credential.issuer(),
            credential.client_id(),
            &mut tpm,
            &key2
        )
        .is_err());
        assert!(!format!("{credential:?}").contains("sensitive-fixture"));
        let continuation = AuthContinuation::from_bytes(b"sensitive-continuation").unwrap();
        let sealed = seal_auth_continuation(&continuation, &mut tpm, &key1).unwrap();
        let restored = unseal_auth_continuation(&sealed, &mut tpm, &key1).unwrap();
        assert_eq!(restored.as_bytes(), continuation.as_bytes());
        assert!(!serde_json::to_string(&sealed)
            .unwrap()
            .contains("sensitive-continuation"));
    }
}
