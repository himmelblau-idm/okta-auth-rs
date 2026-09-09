//! WebAuthn request/response bridge. The wire-shape conversions are generated from Auth JS.
use crate::runtime::{Engine, JResult, V};
use serde_json::Value as Json;
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
pub enum WebAuthnError {
    #[error("invalid WebAuthn request")]
    InvalidRequest,
    #[error("authenticator operation failed: {0}")]
    Authenticator(String),
    #[error("translated WebAuthn conversion failed")]
    Translation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssertionCredential {
    pub id: String,
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttestationCredential {
    pub id: String,
    pub client_data_json: Vec<u8>,
    pub attestation_object: Vec<u8>,
    pub transports: Vec<String>,
}

/// Pluggable authenticator boundary. Implementations may use USB, a platform API, or test hardware.
pub trait WebAuthnAuthenticator {
    fn get(&mut self, origin: &str, request: &Json) -> Result<AssertionCredential, WebAuthnError>;
    fn create(
        &mut self,
        origin: &str,
        request: &Json,
    ) -> Result<AttestationCredential, WebAuthnError>;
}

pub struct WebAuthnAdapter {
    engine: Engine,
    exports: BTreeMap<String, V>,
}
impl WebAuthnAdapter {
    pub fn new() -> Result<Self, WebAuthnError> {
        let engine = Engine::new();
        let exports =
            crate::generated::initialize(&engine).map_err(|_| WebAuthnError::Translation)?;
        Ok(Self { engine, exports })
    }
    fn call(&self, name: &str, args: Vec<V>) -> Result<V, WebAuthnError> {
        let function = self
            .exports
            .get(name)
            .cloned()
            .ok_or(WebAuthnError::Translation)?;
        self.engine
            .call(function, V::Undefined, args)
            .map_err(|_| WebAuthnError::Translation)
    }
    pub fn request_options(
        &self,
        challenge_data: &Json,
        enrollments: &Json,
    ) -> Result<Json, WebAuthnError> {
        let value = self.call(
            "buildCredentialRequestOptions",
            vec![
                self.engine.from_json(challenge_data),
                self.engine.from_json(enrollments),
            ],
        )?;
        self.engine
            .to_json(&value)
            .map_err(|_| WebAuthnError::Translation)
    }
    pub fn creation_options(
        &self,
        activation_data: &Json,
        enrollments: &Json,
    ) -> Result<Json, WebAuthnError> {
        let value = self.call(
            "buildCredentialCreationOptions",
            vec![
                self.engine.from_json(activation_data),
                self.engine.from_json(enrollments),
            ],
        )?;
        self.engine
            .to_json(&value)
            .map_err(|_| WebAuthnError::Translation)
    }
    pub fn get<A: WebAuthnAuthenticator>(
        &self,
        authenticator: &mut A,
        origin: &str,
        challenge_data: &Json,
        enrollments: &Json,
    ) -> Result<Json, WebAuthnError> {
        let request = self.request_options(challenge_data, enrollments)?;
        let credential = authenticator.get(origin, &request)?;
        self.assertion(&credential)
    }
    pub fn create<A: WebAuthnAuthenticator>(
        &self,
        authenticator: &mut A,
        origin: &str,
        activation_data: &Json,
        enrollments: &Json,
    ) -> Result<Json, WebAuthnError> {
        let request = self.creation_options(activation_data, enrollments)?;
        let credential = authenticator.create(origin, &request)?;
        self.attestation(&credential)
    }
    pub fn assertion(&self, credential: &AssertionCredential) -> Result<Json, WebAuthnError> {
        let response = self.engine.object();
        self.engine
            .put(
                &response,
                "clientDataJSON",
                self.engine
                    .from_json(&serde_json::json!(credential.client_data_json)),
            )
            .map_err(|_| WebAuthnError::Translation)?;
        self.engine
            .put(
                &response,
                "authenticatorData",
                self.engine
                    .from_json(&serde_json::json!(credential.authenticator_data)),
            )
            .map_err(|_| WebAuthnError::Translation)?;
        self.engine
            .put(
                &response,
                "signature",
                self.engine
                    .from_json(&serde_json::json!(credential.signature)),
            )
            .map_err(|_| WebAuthnError::Translation)?;
        let object = self.engine.object();
        self.engine
            .put(&object, "id", V::string(&credential.id))
            .map_err(|_| WebAuthnError::Translation)?;
        self.engine
            .put(&object, "response", response)
            .map_err(|_| WebAuthnError::Translation)?;
        let value = self.call("getAssertion", vec![object])?;
        self.engine
            .to_json(&value)
            .map_err(|_| WebAuthnError::Translation)
    }
    pub fn attestation(&self, credential: &AttestationCredential) -> Result<Json, WebAuthnError> {
        fn transports(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
            cx.lookup(scope, "transports")
        }
        let response = self.engine.object();
        self.engine
            .put(
                &response,
                "clientDataJSON",
                self.engine
                    .from_json(&serde_json::json!(credential.client_data_json)),
            )
            .map_err(|_| WebAuthnError::Translation)?;
        self.engine
            .put(
                &response,
                "attestationObject",
                self.engine
                    .from_json(&serde_json::json!(credential.attestation_object)),
            )
            .map_err(|_| WebAuthnError::Translation)?;
        if !credential.transports.is_empty() {
            let scope = self.engine.scope(None);
            self.engine.define(
                scope,
                "transports",
                self.engine
                    .from_json(&serde_json::json!(credential.transports)),
            );
            self.engine
                .put(
                    &response,
                    "getTransports",
                    self.engine.function(scope, transports),
                )
                .map_err(|_| WebAuthnError::Translation)?;
        }
        let object = self.engine.object();
        self.engine
            .put(&object, "id", V::string(&credential.id))
            .map_err(|_| WebAuthnError::Translation)?;
        self.engine
            .put(&object, "response", response)
            .map_err(|_| WebAuthnError::Translation)?;
        let value = self.call("getAttestation", vec![object])?;
        self.engine
            .to_json(&value)
            .map_err(|_| WebAuthnError::Translation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    struct Fixture;
    impl WebAuthnAuthenticator for Fixture {
        fn get(&mut self, _: &str, request: &Json) -> Result<AssertionCredential, WebAuthnError> {
            assert_eq!(
                request["publicKey"]["challenge"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap() as u8)
                    .collect::<Vec<_>>(),
                vec![1, 2, 3]
            );
            Ok(AssertionCredential {
                id: "credential".into(),
                client_data_json: vec![1, 2],
                authenticator_data: vec![3, 4],
                signature: vec![5, 6],
            })
        }
        fn create(&mut self, _: &str, _: &Json) -> Result<AttestationCredential, WebAuthnError> {
            unreachable!()
        }
    }
    #[test]
    fn translated_request_and_assertion_bridge() {
        let adapter = WebAuthnAdapter::new().unwrap();
        let value=adapter.get(&mut Fixture,"https://example.invalid",&json!({"challenge":"AQID","rpId":"example.invalid","userVerification":"preferred"}),&json!([])).unwrap();
        assert_eq!(
            value,
            json!({"id":"credential","clientData":"AQI=","authenticatorData":"AwQ=","signatureData":"BQY="})
        );
    }
}
