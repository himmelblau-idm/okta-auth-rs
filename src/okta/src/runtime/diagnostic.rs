//! Operation-local evidence. Context is captured by futures, never a process-wide last error.
use super::{Engine, JResult, V};
use crate::{FailureKind, FailureStage, OperationFailure};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(super) struct Context {
    pub epoch: u64,
    pub stage: FailureStage,
    pub kind: FailureKind,
    pub explicit_stage: bool,
    pub request: Arc<Mutex<Option<OperationFailure>>>,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            epoch: 0,
            stage: FailureStage::Remediation,
            kind: FailureKind::Protocol,
            explicit_stage: false,
            request: Arc::default(),
        }
    }
}

impl Engine {
    /// Used only by generated, source-validated protocol annotations.
    pub fn diagnostic_frame(
        &self,
        stage: Option<FailureStage>,
        validation: bool,
        protocol: bool,
    ) -> Self {
        let mut cx = self.clone();
        // Each invocation gets its own frame; sibling promises cannot overwrite
        // each other's request evidence, even at the same protocol stage.
        cx.1.request = Arc::new(Mutex::new(self.1.request.lock().unwrap().clone()));
        if let Some(stage) = stage {
            cx.1.explicit_stage = true;
            if stage != cx.1.stage {
                cx.1.stage = stage;
                cx.1.request = Arc::default();
                cx.1.kind = FailureKind::Protocol;
            }
        }
        if protocol {
            cx.1.kind = FailureKind::Protocol;
        }
        if validation {
            cx.1.kind = FailureKind::TokenValidation;
        }
        cx
    }

    pub(crate) fn request_evidence(&self, failure: OperationFailure) {
        if matches!(
            failure.stage,
            FailureStage::TokenExchange | FailureStage::Refresh | FailureStage::Revoke
        ) && failure.outcome != crate::RequestOutcome::NotSent
        {
            self.0
                .lock()
                .unwrap()
                .mutations
                .insert(failure.stage as u8, failure.clone());
        }
        *self.1.request.lock().unwrap() = Some(failure);
    }

    pub fn diagnostic_caught(&self, error: &V) {
        self.adopt_evidence(error);
    }

    pub(crate) fn adopt_evidence(&self, value: &V) {
        if let Some(failure) = self
            .evidence(value)
            .filter(|f| !self.1.explicit_stage || f.stage == self.1.stage)
        {
            *self.1.request.lock().unwrap() = Some(failure);
        }
    }

    pub(crate) fn mutation_evidence(&self) -> Option<OperationFailure> {
        self.0
            .lock()
            .unwrap()
            .mutations
            .get(&(self.1.stage as u8))
            .cloned()
    }

    pub(crate) fn current_evidence(&self) -> OperationFailure {
        let mut failure = self
            .1
            .request
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| OperationFailure::local(self.1.kind, self.1.stage));
        if failure.http_status.is_some_and(|s| (200..300).contains(&s)) {
            failure.kind = self.1.kind;
        }
        failure
    }

    fn evidence_key(value: &V) -> Option<(u8, usize)> {
        match value {
            V::Object(id) => Some((0, *id)),
            V::Array(id) => Some((1, *id)),
            V::NativeError(s) => Some((2, Arc::as_ptr(s) as usize)),
            V::String(s) => Some((3, Arc::as_ptr(s) as usize)),
            _ => None,
        }
    }

    pub(crate) fn attach_evidence(&self, value: &V, failure: OperationFailure) {
        if let Some(key) = Self::evidence_key(value) {
            // Retain the value until the next operation, preventing pointer reuse.
            self.0
                .lock()
                .unwrap()
                .evidence
                .entry(key)
                .or_insert((value.clone(), failure));
        }
    }

    pub(crate) fn evidence(&self, value: &V) -> Option<OperationFailure> {
        let key = Self::evidence_key(value)?;
        self.0
            .lock()
            .unwrap()
            .evidence
            .get(&key)
            .map(|(_, f)| f.clone())
    }

    pub(crate) fn operation_error(&self, value: &V, stage: FailureStage) -> crate::OktaError {
        let mut failure = self
            .evidence(value)
            .unwrap_or_else(|| OperationFailure::local(FailureKind::Protocol, stage));
        // Callback OAuth errors and WWW-Authenticate normalization may have no
        // JSON response body. Inspect only known structured code properties.
        match self.get(value, "name").ok().map(|v| v.text()).as_deref() {
            Some("OAuthError") if failure.oauth_error.is_none() => {
                failure.oauth_error = self.get(value, "error").ok().and_then(|v| match v {
                    V::String(s) => crate::failure::safe_code(Some(&s)),
                    _ => None,
                });
            }
            Some("AuthApiError") if failure.okta_error_code.is_none() => {
                failure.okta_error_code = self.get(value, "errorCode").ok().and_then(|v| match v {
                    V::String(s) => crate::failure::safe_code(Some(&s)),
                    _ => None,
                });
            }
            _ => {}
        }
        self.clear_diagnostics();
        crate::OktaError::OperationFailed(failure)
    }

    pub(crate) fn clear_diagnostics(&self) {
        self.0.lock().unwrap().evidence.clear();
    }

    /// Complete a generated function without making diagnostic fields enumerable.
    pub fn diagnostic_result(&self, result: JResult) -> JResult {
        match &result {
            Err(error) => self.attach_evidence(error, self.current_evidence()),
            Ok(value) if self.1.request.lock().unwrap().is_some() => {
                self.attach_evidence(value, self.current_evidence());
            }
            _ => {}
        }
        result
    }

    /// Preserve the original failure when SDK code constructs a new error object.
    pub fn diagnostic_transfer(&self, source: &V, result: JResult) -> JResult {
        if let Some(failure) = self.evidence(source) {
            if let Ok(value) = &result {
                self.attach_evidence(value, failure);
            }
        }
        self.diagnostic_result(result)
    }

    /// Native token metadata is separate from Auth JS's requested-scope fallback.
    pub fn diagnostic_token_scopes(&self, response: &V, result: JResult) -> JResult {
        if result.is_ok() && self.get(response, "access_token")?.truthy() {
            let scopes = match self.get(response, "scope")? {
                V::String(s) => Some(s.split_ascii_whitespace().map(str::to_owned).collect()),
                _ => None,
            };
            // One successful token result per serialized public operation. This
            // separate result channel survives the SDK's storage JSON copies
            // without adding properties to translated or persisted token data.
            self.0.lock().unwrap().granted_scopes = scopes;
        }
        self.diagnostic_result(result)
    }

    pub(crate) fn take_token_scopes(&self) -> Option<Vec<String>> {
        self.0.lock().unwrap().granted_scopes.take()
    }

    pub fn diagnostic_oauth_input(&self, input: &V) -> JResult {
        // The callback/code branch is interpreted by the generated SDK. A token
        // endpoint response must contain an access token before it can succeed.
        if self.get(input, "code")?.truthy() || self.get(input, "interaction_code")?.truthy() {
            return Ok(V::Undefined);
        }
        if !matches!(self.get(input, "access_token")?, V::String(ref s) if !s.is_empty()) {
            return self.diagnostic_result(Err(self.error("invalid token response")));
        }
        Ok(V::Undefined)
    }

    pub fn diagnostic_idx_input(&self, input: &V) -> JResult {
        if !matches!(input, V::Object(_)) || !matches!(self.get(input, "version")?, V::String(_)) {
            let error = self.error("invalid IDX response");
            let failure = self
                .evidence(input)
                .unwrap_or_else(|| self.current_evidence());
            self.attach_evidence(&error, failure);
            return Err(error);
        }
        Ok(V::Undefined)
    }

    pub fn diagnostic_interact_result(&self, result: JResult) -> JResult {
        if let Ok(value) = &result {
            if !matches!(self.get(value, "interactionHandle")?, V::String(ref s) if !s.is_empty()) {
                return self.diagnostic_result(Err(self.error("invalid interact response")));
            }
        }
        self.diagnostic_result(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestOutcome;

    #[test]
    fn derived_transactions_keep_their_response_stage_but_validation_does_not_adopt_jwks() {
        let root = Engine::new();
        let response = root.object();
        let mut evidence = OperationFailure::local(FailureKind::Protocol, FailureStage::Introspect);
        evidence.http_status = Some(200);
        evidence.outcome = RequestOutcome::ResponseReceived;
        root.attach_evidence(&response, evidence);
        root.adopt_evidence(&response);
        assert_eq!(root.current_evidence().stage, FailureStage::Introspect);
        let validation = root.diagnostic_frame(Some(FailureStage::Refresh), true, false);
        let jwks = root.object();
        let mut evidence = OperationFailure::local(FailureKind::Protocol, FailureStage::Jwks);
        evidence.http_status = Some(200);
        root.attach_evidence(&jwks, evidence);
        validation.adopt_evidence(&jwks);
        assert_eq!(validation.current_evidence().stage, FailureStage::Refresh);
        assert_eq!(
            validation.current_evidence().kind,
            FailureKind::TokenValidation
        );
    }

    #[test]
    fn same_stage_siblings_and_wrapper_reconstruction_preserve_request_identity() {
        let root = Engine::new();
        let first = root.diagnostic_frame(Some(FailureStage::Refresh), false, false);
        let second = root.diagnostic_frame(Some(FailureStage::Refresh), false, false);
        let mut failure = OperationFailure::local(FailureKind::Http, FailureStage::Refresh);
        failure.http_status = Some(429);
        failure.outcome = RequestOutcome::ResponseReceived;
        first.request_evidence(failure.clone());
        let original = first.error("fixture-secret");
        first.attach_evidence(&original, failure);
        let mut success = OperationFailure::local(FailureKind::Protocol, FailureStage::Refresh);
        success.http_status = Some(200);
        success.outcome = RequestOutcome::ResponseReceived;
        second.request_evidence(success);
        assert_eq!(first.current_evidence().http_status, Some(429));
        let wrapper = second.object();
        second
            .put(&wrapper, "name", V::string("AuthApiError"))
            .unwrap();
        let wrapper = second.diagnostic_transfer(&original, Ok(wrapper)).unwrap();
        assert_eq!(
            second.to_json(&wrapper).unwrap(),
            serde_json::json!({"name":"AuthApiError"})
        );
        let error = root.operation_error(&wrapper, FailureStage::Callback);
        let crate::OktaError::OperationFailed(f) = error else {
            panic!("missing evidence")
        };
        assert_eq!(f.http_status, Some(429));
        assert_eq!(f.stage, FailureStage::Refresh);
        assert!(!format!("{f:?} {f}").contains("secret"));
        assert!(root.evidence(&wrapper).is_none());
    }
}
