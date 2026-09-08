//! Caller-driven authentication flows. Authentication decisions execute generated code.
use crate::{sdk::Sdk, RefreshCredential};
use okta_native_support::{HttpTransport, V};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};
use std::collections::BTreeMap;
use std::time::Duration;
use zeroize::{Zeroize, Zeroizing};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub issuer: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub timeout: Duration,
    pub additional_trusted_origins: Vec<String>,
    /// Permit HTTP only on loopback addresses, for an explicitly configured local test server.
    pub allow_loopback_http: bool,
}
impl ClientConfig {
    pub fn new(
        issuer: impl Into<String>,
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            issuer: issuer.into(),
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
            scopes: vec!["openid".into(), "profile".into(), "offline_access".into()],
            timeout: Duration::from_secs(30),
            additional_trusted_origins: vec![],
            allow_loopback_http: false,
        }
    }
}
#[derive(Default)]
pub struct AuthOptions {
    pub login_hint: Option<String>,
    pub recovery_token: Option<Zeroizing<String>>,
    pub activation_token: Option<Zeroizing<String>>,
}
impl std::fmt::Debug for AuthOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AuthOptions { .. }")
    }
}
#[derive(Debug, thiserror::Error)]
pub enum OktaError {
    #[error("invalid client configuration")]
    Configuration,
    #[error("step belongs to a different or older flow response")]
    StaleStep,
    #[error("flow belongs to another client configuration")]
    WrongClient,
    #[error("flow has an uncertain outcome; introspect before continuing")]
    Uncertain,
    #[error("flow has been canceled")]
    Canceled,
    #[error("input conflicts with a reserved SDK option")]
    ReservedInput,
    #[error("authentication operation failed")]
    Authentication,
    #[error("generated SDK returned an unsupported result")]
    UnsupportedResult,
    #[error("sensitive continuation is invalid")]
    InvalidContinuation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthStatus {
    Pending,
    Success,
    Terminal,
    Canceled,
    Uncertain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepKind {
    Form,
    Poll,
    Redirect,
    External,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepId {
    flow: [u8; 16],
    generation: u64,
    index: usize,
}

fn erase(value: &mut Json) {
    match value {
        Json::String(s) => s.zeroize(),
        Json::Array(a) => a.iter_mut().for_each(erase),
        Json::Object(o) => o.values_mut().for_each(erase),
        _ => {}
    }
}
#[derive(Clone)]
struct SensitiveJson(Json);
impl Drop for SensitiveJson {
    fn drop(&mut self) {
        erase(&mut self.0)
    }
}
impl std::fmt::Debug for SensitiveJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

#[derive(Clone, Debug)]
pub struct AuthStep {
    id: StepId,
    pub kind: StepKind,
    data: SensitiveJson,
}
impl AuthStep {
    pub fn id(&self) -> &StepId {
        &self.id
    }
    pub fn name(&self) -> &str {
        self.data.0["name"].as_str().unwrap_or("")
    }
    /// Complete generated schema, including nested inputs, options, and messages.
    pub fn schema(&self) -> &Json {
        &self.data.0
    }
}
/// Form values are deliberately separate from SDK configuration options.
#[derive(Debug)]
pub struct FormInput(SensitiveJson);
impl FormInput {
    pub fn new(values: Json) -> Result<Self, OktaError> {
        let fields = values.as_object().ok_or(OktaError::ReservedInput)?;
        let reserved = [
            "step",
            "flow",
            "state",
            "stateHandle",
            "version",
            "withCredentials",
            "useGenericRemediator",
            "enableLegacyMode",
            "autoRemediate",
            "remediators",
            "actions",
            "exchangeCodeForTokens",
            "scopes",
            "nonce",
            "ignoreSignature",
            "clientId",
            "clientSecret",
            "issuer",
            "redirectUri",
            "dpop",
            "recoveryToken",
            "activationToken",
        ];
        if fields.keys().any(|name| reserved.contains(&name.as_str())) {
            return Err(OktaError::ReservedInput);
        }
        Ok(Self(SensitiveJson(values)))
    }
    pub fn empty() -> Self {
        Self(SensitiveJson(json!({})))
    }
}
#[derive(Debug)]
pub enum AuthInput {
    Form(FormInput),
    Poll,
}

pub struct UserToken {
    data: SensitiveJson,
    refresh: Option<RefreshCredential>,
}
impl std::fmt::Debug for UserToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UserToken { [redacted] }")
    }
}
impl UserToken {
    pub fn access_token(&self) -> Option<&str> {
        self.data.0["accessToken"]["accessToken"].as_str()
    }
    pub fn id_token(&self) -> Option<&str> {
        self.data.0["idToken"]["idToken"].as_str()
    }
    /// Claims are exposed only after generated signature and claim verification succeeds.
    pub fn claims(&self) -> Option<&Json> {
        self.data.0.get("idToken").and_then(|v| v.get("claims"))
    }
    pub fn refresh_credential(&self) -> Option<&RefreshCredential> {
        self.refresh.as_ref()
    }
}

/// Opaque, sensitive state used to move an unfinished transaction between processes.
pub struct AuthContinuation(Zeroizing<Vec<u8>>);
impl std::fmt::Debug for AuthContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AuthContinuation([redacted])")
    }
}
impl AuthContinuation {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OktaError> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(OktaError::InvalidContinuation);
        }
        Ok(Self(Zeroizing::new(bytes.to_vec())))
    }
}
#[derive(Serialize, Deserialize)]
struct ContinuationEnvelope {
    schema: u32,
    binding: String,
    upstream_commit: String,
    stores: BTreeMap<String, Json>,
    cookies: Vec<u8>,
}

pub struct AuthFlow {
    sdk: Sdk,
    binding: String,
    id: [u8; 16],
    generation: u64,
    status: AuthStatus,
    in_flight: bool,
    closed: bool,
    current: V,
    steps: Vec<AuthStep>,
    step_actions: Vec<V>,
    snapshot: SensitiveJson,
    tokens: Option<UserToken>,
}
impl std::fmt::Debug for AuthFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthFlow")
            .field("status", &self.status())
            .field("generation", &self.generation)
            .field("steps", &self.steps.len())
            .finish()
    }
}
impl AuthFlow {
    pub fn status(&self) -> AuthStatus {
        if self.in_flight {
            AuthStatus::Uncertain
        } else {
            self.status
        }
    }
    pub fn steps(&self) -> &[AuthStep] {
        &self.steps
    }
    pub fn messages(&self) -> Option<&Json> {
        self.snapshot.0.get("messages")
    }
    pub fn context(&self) -> Option<&Json> {
        self.snapshot.0.get("context")
    }
    pub fn tokens(&self) -> Option<&UserToken> {
        self.tokens.as_ref()
    }
}

pub struct PublicClientApplication {
    config: ClientConfig,
    binding: String,
    origin: String,
}
impl PublicClientApplication {
    pub fn new(mut config: ClientConfig) -> Result<Self, OktaError> {
        let issuer = url::Url::parse(&config.issuer).map_err(|_| OktaError::Configuration)?;
        let redirect =
            url::Url::parse(&config.redirect_uri).map_err(|_| OktaError::Configuration)?;
        if config.client_id.is_empty()
            || issuer.query().is_some()
            || issuer.fragment().is_some()
            || !issuer.username().is_empty()
            || issuer.password().is_some()
            || redirect.fragment().is_some()
            || !redirect.username().is_empty()
            || redirect.password().is_some()
        {
            return Err(OktaError::Configuration);
        }
        config.issuer = config.issuer.trim_end_matches('/').to_string();
        let origin = issuer.origin().ascii_serialization();
        let binding=serde_json::to_string(&json!({"issuer":config.issuer,"clientId":config.client_id,"redirectUri":config.redirect_uri,"scopes":config.scopes,"trustedOrigins":config.additional_trusted_origins,"allowLoopbackHttp":config.allow_loopback_http})).map_err(|_|OktaError::Configuration)?;
        let client = Self {
            config,
            binding,
            origin,
        };
        let _ = client.transport()?;
        Ok(client)
    }
    fn transport(&self) -> Result<HttpTransport, OktaError> {
        let mut origins = vec![self.origin.clone()];
        origins.extend(self.config.additional_trusted_origins.clone());
        HttpTransport::new(
            &origins,
            self.config.timeout,
            self.config.allow_loopback_http,
        )
        .map_err(|_| OktaError::Configuration)
    }
    fn sdk(&self) -> Result<Sdk, OktaError> {
        Sdk::new(json!({"issuer":self.config.issuer,"clientId":self.config.client_id,"redirectUri":self.config.redirect_uri,"scopes":self.config.scopes,"pkce":true,"dpop":false,"ignoreSignature":false,"ignoreLifetime":false,"maxClockSkew":300,"decodeAccessTokens":false,"idx":{"useGenericRemediator":true,"enableLegacyMode":false}}),&self.origin,self.transport()?).map_err(|_|OktaError::UnsupportedResult)
    }
    fn check(&self, flow: &AuthFlow, allow_uncertain: bool) -> Result<(), OktaError> {
        if flow.binding != self.binding {
            return Err(OktaError::WrongClient);
        }
        if flow.closed || flow.status == AuthStatus::Canceled {
            return Err(OktaError::Canceled);
        }
        if flow.in_flight && !allow_uncertain {
            return Err(OktaError::Uncertain);
        }
        Ok(())
    }
    pub async fn initiate_auth_flow(&self, options: AuthOptions) -> Result<AuthFlow, OktaError> {
        let sdk = self.sdk()?;
        let mut id = [0; 16];
        SystemRandom::new()
            .fill(&mut id)
            .map_err(|_| OktaError::Configuration)?;
        let args = json!({"exchangeCodeForTokens":true,"loginHint":options.login_hint,"recoveryToken":options.recovery_token.as_deref().map(String::as_str),"activationToken":options.activation_token.as_deref().map(String::as_str)});
        let input = sdk.engine.from_json(&args);
        let mut flow = AuthFlow {
            sdk,
            binding: self.binding.clone(),
            id,
            generation: 0,
            status: AuthStatus::Pending,
            in_flight: true,
            closed: false,
            current: V::Undefined,
            steps: vec![],
            step_actions: vec![],
            snapshot: SensitiveJson(json!({})),
            tokens: None,
        };
        let response = flow
            .sdk
            .call("startTransaction", vec![input])
            .await
            .map_err(|_| OktaError::Authentication)?;
        self.accept(&mut flow, response)?;
        Ok(flow)
    }
    pub async fn continue_auth_flow(
        &self,
        flow: &mut AuthFlow,
        step: &StepId,
        input: AuthInput,
    ) -> Result<(), OktaError> {
        self.check(flow, false)?;
        if step.flow != flow.id || step.generation != flow.generation {
            return Err(OktaError::StaleStep);
        }
        let selected = flow.steps.get(step.index).ok_or(OktaError::StaleStep)?;
        if matches!(selected.kind, StepKind::Redirect | StepKind::External) {
            return Err(OktaError::ReservedInput);
        }
        let values = match input {
            AuthInput::Form(form) => form.0.clone(),
            AuthInput::Poll if selected.kind == StepKind::Poll => SensitiveJson(json!({})),
            AuthInput::Poll => return Err(OktaError::ReservedInput),
        };
        let args = flow.sdk.engine.from_json(&values.0);
        let action = flow.step_actions[step.index].clone();
        flow.in_flight = true;
        let response = flow
            .sdk
            .engine
            .execute(action, V::Undefined, vec![args])
            .await
            .map_err(|_| OktaError::Authentication)?;
        self.accept(flow, response)
    }
    pub async fn cancel_auth_flow(&self, flow: &mut AuthFlow) -> Result<(), OktaError> {
        self.check(flow, false)?;
        let cancel = flow
            .steps
            .iter()
            .find(|step| step.name() == "cancel")
            .ok_or(OktaError::UnsupportedResult)?
            .id
            .clone();
        self.continue_auth_flow(flow, &cancel, AuthInput::Form(FormInput::empty()))
            .await?;
        flow.closed = true;
        Ok(())
    }
    /// Discard local state without making a network request.
    pub fn abandon_auth_flow(&self, flow: &mut AuthFlow) -> Result<(), OktaError> {
        self.check(flow, true)?;
        flow.sdk.engine.cancel_pending();
        flow.status = AuthStatus::Canceled;
        flow.in_flight = false;
        flow.closed = true;
        flow.steps.clear();
        flow.step_actions.clear();
        flow.tokens = None;
        flow.snapshot = SensitiveJson(json!({}));
        Ok(())
    }
    /// Reconcile after a canceled future or uncertain transport outcome; never replay a form submission.
    pub async fn introspect_auth_flow(&self, flow: &mut AuthFlow) -> Result<(), OktaError> {
        self.check(flow, true)?;
        flow.in_flight = true;
        let manager = flow
            .sdk
            .engine
            .get(&flow.sdk.client, "transactionManager")
            .map_err(|_| OktaError::UnsupportedResult)?;
        flow.sdk
            .engine
            .method(manager, "clearIdxResponse", vec![])
            .map_err(|_| OktaError::UnsupportedResult)?;
        let args = flow
            .sdk
            .engine
            .from_json(&json!({"autoRemediate":false,"exchangeCodeForTokens":false}));
        let response = flow
            .sdk
            .call("proceed", vec![args])
            .await
            .map_err(|_| OktaError::Authentication)?;
        self.accept(flow, response)
    }
    /// Continue an email verification callback using the transaction already held by this flow.
    pub async fn handle_email_verify_callback(
        &self,
        flow: &mut AuthFlow,
        query_or_fragment: &str,
    ) -> Result<(), OktaError> {
        self.check(flow, false)?;
        flow.in_flight = true;
        let input = V::string(query_or_fragment);
        let response = flow
            .sdk
            .call("handleEmailVerifyCallback", vec![input])
            .await
            .map_err(|_| OktaError::Authentication)?;
        if response.nullish() {
            flow.in_flight = false;
            return Err(OktaError::UnsupportedResult);
        }
        self.accept(flow, response)
    }
    /// Serialize an unfinished transaction. The returned bytes contain PKCE and session secrets.
    pub fn export_auth_flow(&self, flow: &AuthFlow) -> Result<AuthContinuation, OktaError> {
        self.check(flow, false)?;
        if flow.status != AuthStatus::Pending {
            return Err(OktaError::InvalidContinuation);
        }
        let envelope = ContinuationEnvelope {
            schema: 1,
            binding: self.binding.clone(),
            upstream_commit: crate::generated::UPSTREAM_COMMIT.to_string(),
            stores: flow
                .sdk
                .export_storage()
                .map_err(|_| OktaError::InvalidContinuation)?,
            cookies: flow
                .sdk
                .engine
                .export_cookies()
                .map_err(|_| OktaError::InvalidContinuation)?
                .to_vec(),
        };
        let bytes = serde_json::to_vec(&envelope).map_err(|_| OktaError::InvalidContinuation)?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(OktaError::InvalidContinuation);
        }
        Ok(AuthContinuation(Zeroizing::new(bytes)))
    }
    /// Restore an unfinished transaction and reconstruct its current generated steps.
    pub async fn resume_auth_flow(
        &self,
        continuation: &AuthContinuation,
    ) -> Result<AuthFlow, OktaError> {
        let envelope = self.decode_continuation(continuation)?;
        let sdk = self.restore_sdk(&envelope)?;
        let mut id = [0; 16];
        SystemRandom::new()
            .fill(&mut id)
            .map_err(|_| OktaError::Configuration)?;
        let mut flow = AuthFlow {
            sdk,
            binding: self.binding.clone(),
            id,
            generation: 0,
            status: AuthStatus::Pending,
            in_flight: true,
            closed: false,
            current: V::Undefined,
            steps: vec![],
            step_actions: vec![],
            snapshot: SensitiveJson(json!({})),
            tokens: None,
        };
        let args = flow
            .sdk
            .engine
            .from_json(&json!({"autoRemediate":false,"exchangeCodeForTokens":true}));
        let response = flow
            .sdk
            .call("proceed", vec![args])
            .await
            .map_err(|_| OktaError::InvalidContinuation)?;
        self.accept(&mut flow, response)?;
        Ok(flow)
    }
    /// Finish a caller-managed browser handoff. The URL must target the configured redirect URI.
    pub async fn handle_interaction_code_callback(
        &self,
        continuation: &AuthContinuation,
        callback_url: &str,
    ) -> Result<UserToken, OktaError> {
        let expected =
            url::Url::parse(&self.config.redirect_uri).map_err(|_| OktaError::Configuration)?;
        let actual = url::Url::parse(callback_url).map_err(|_| OktaError::Authentication)?;
        if expected.scheme() != actual.scheme()
            || expected.host_str() != actual.host_str()
            || expected.port_or_known_default() != actual.port_or_known_default()
            || expected.path() != actual.path()
        {
            return Err(OktaError::Authentication);
        }
        let envelope = self.decode_continuation(continuation)?;
        let sdk = self.restore_sdk(&envelope)?;
        sdk.call(
            "handleInteractionCodeRedirect",
            vec![V::string(callback_url)],
        )
        .await
        .map_err(|_| OktaError::Authentication)?;
        let data = sdk
            .storage_value("tokens")
            .map_err(|_| OktaError::UnsupportedResult)?;
        if !data.is_object() {
            return Err(OktaError::UnsupportedResult);
        }
        Ok(self.token(data))
    }
    /// Renew access and ID tokens through the translated refresh-token implementation.
    pub async fn refresh_tokens(
        &self,
        credential: &RefreshCredential,
    ) -> Result<UserToken, OktaError> {
        if credential.issuer() != self.config.issuer
            || credential.client_id() != self.config.client_id
        {
            return Err(OktaError::WrongClient);
        }
        let sdk = self.sdk()?;
        let urls = sdk
            .call("getOAuthUrls", vec![sdk.engine.object()])
            .await
            .map_err(|_| OktaError::Authentication)?;
        let urls = sdk
            .engine
            .to_json(&urls)
            .map_err(|_| OktaError::UnsupportedResult)?;
        let refresh = json!({"refreshToken":credential.secret(),"scopes":self.config.scopes,"tokenUrl":urls["tokenUrl"],"authorizeUrl":urls["authorizeUrl"],"issuer":urls["issuer"]});
        let params = json!({"scopes":self.config.scopes});
        let response = sdk
            .call(
                "renewTokensWithRefresh",
                vec![
                    sdk.engine.from_json(&params),
                    sdk.engine.from_json(&refresh),
                ],
            )
            .await
            .map_err(|_| OktaError::Authentication)?;
        let mut data = sdk
            .engine
            .to_json(&response)
            .map_err(|_| OktaError::UnsupportedResult)?;
        if data.get("refreshToken").is_none() {
            data.as_object_mut()
                .ok_or(OktaError::UnsupportedResult)?
                .insert("refreshToken".into(), refresh);
        }
        Ok(self.token(data))
    }
    /// Revoke the refresh credential at Okta's translated revocation endpoint.
    pub async fn revoke_refresh_token(
        &self,
        credential: &RefreshCredential,
    ) -> Result<(), OktaError> {
        if credential.issuer() != self.config.issuer
            || credential.client_id() != self.config.client_id
        {
            return Err(OktaError::WrongClient);
        }
        let sdk = self.sdk()?;
        let token = sdk
            .engine
            .from_json(&json!({"refreshToken":credential.secret()}));
        sdk.call("revokeToken", vec![token])
            .await
            .map_err(|_| OktaError::Authentication)?;
        Ok(())
    }
    /// Revoke this token's access token at Okta's translated revocation endpoint.
    pub async fn revoke_access_token(&self, token: &UserToken) -> Result<(), OktaError> {
        let access = token
            .data
            .0
            .get("accessToken")
            .cloned()
            .ok_or(OktaError::UnsupportedResult)?;
        let sdk = self.sdk()?;
        sdk.call("revokeToken", vec![sdk.engine.from_json(&access)])
            .await
            .map_err(|_| OktaError::Authentication)?;
        Ok(())
    }
    /// Fetch user claims and enforce the translated access-token/ID-token subject match.
    pub async fn user_info(&self, token: &UserToken) -> Result<Json, OktaError> {
        let access = token
            .data
            .0
            .get("accessToken")
            .ok_or(OktaError::UnsupportedResult)?;
        let id = token
            .data
            .0
            .get("idToken")
            .ok_or(OktaError::UnsupportedResult)?;
        let sdk = self.sdk()?;
        let response = sdk
            .call(
                "getUserInfo",
                vec![sdk.engine.from_json(access), sdk.engine.from_json(id)],
            )
            .await
            .map_err(|_| OktaError::Authentication)?;
        sdk.engine
            .to_json(&response)
            .map_err(|_| OktaError::UnsupportedResult)
    }
    fn decode_continuation(
        &self,
        continuation: &AuthContinuation,
    ) -> Result<ContinuationEnvelope, OktaError> {
        let envelope: ContinuationEnvelope = serde_json::from_slice(continuation.as_bytes())
            .map_err(|_| OktaError::InvalidContinuation)?;
        if envelope.schema != 1
            || envelope.binding != self.binding
            || envelope.upstream_commit != crate::generated::UPSTREAM_COMMIT
        {
            return Err(OktaError::InvalidContinuation);
        }
        Ok(envelope)
    }
    fn restore_sdk(&self, envelope: &ContinuationEnvelope) -> Result<Sdk, OktaError> {
        let sdk = self.sdk()?;
        sdk.import_storage(&envelope.stores)
            .map_err(|_| OktaError::InvalidContinuation)?;
        sdk.engine
            .import_cookies(&envelope.cookies)
            .map_err(|_| OktaError::InvalidContinuation)?;
        Ok(sdk)
    }
    fn token(&self, data: Json) -> UserToken {
        let refresh = data["refreshToken"]["refreshToken"].as_str().map(|secret| {
            RefreshCredential::import(
                self.config.issuer.clone(),
                self.config.client_id.clone(),
                secret.to_string(),
            )
        });
        UserToken {
            data: SensitiveJson(data),
            refresh,
        }
    }
    fn accept(&self, flow: &mut AuthFlow, response: V) -> Result<(), OktaError> {
        let cx = &flow.sdk.engine;
        let snapshot = cx
            .to_json(&response)
            .map_err(|_| OktaError::UnsupportedResult)?;
        let status = match snapshot["status"].as_str() {
            Some("PENDING") => AuthStatus::Pending,
            Some("SUCCESS") => AuthStatus::Success,
            Some("TERMINAL") => AuthStatus::Terminal,
            Some("CANCELED") => AuthStatus::Canceled,
            _ => return Err(OktaError::UnsupportedResult),
        };
        flow.generation = flow
            .generation
            .checked_add(1)
            .ok_or(OktaError::UnsupportedResult)?;
        let available = cx
            .get(&response, "availableSteps")
            .map_err(|_| OktaError::UnsupportedResult)?;
        let mut steps = vec![];
        let mut step_actions = vec![];
        if !available.nullish() {
            for item in cx
                .items(&available)
                .map_err(|_| OktaError::UnsupportedResult)?
            {
                let data = cx
                    .to_json(&item)
                    .map_err(|_| OktaError::UnsupportedResult)?;
                let name = data["name"].as_str().ok_or(OktaError::UnsupportedResult)?;
                let action = cx
                    .get(&item, "action")
                    .map_err(|_| OktaError::UnsupportedResult)?;
                let kind = if name == "poll" || name.ends_with("-poll") {
                    StepKind::Poll
                } else if !action.nullish() {
                    StepKind::Form
                } else if data.get("href").is_some() {
                    StepKind::Redirect
                } else {
                    StepKind::External
                };
                step_actions.push(action);
                steps.push(AuthStep {
                    id: StepId {
                        flow: flow.id,
                        generation: flow.generation,
                        index: steps.len(),
                    },
                    kind,
                    data: SensitiveJson(data),
                });
            }
        }
        flow.tokens = snapshot
            .get("tokens")
            .filter(|v| v.is_object())
            .cloned()
            .map(|data| self.token(data));
        flow.snapshot = SensitiveJson(snapshot);
        flow.current = response;
        flow.steps = steps;
        flow.step_actions = step_actions;
        flow.status = status;
        flow.in_flight = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_server::{Reply, Server};
    pub(super) fn form(origin: &str, state: &str, name: &str, fields: Json) -> Json {
        let mut fields = fields.as_array().unwrap().clone();
        fields.push(json!({"name":"stateHandle","mutable":false,"value":state}));
        json!({"version":"1.0.0","stateHandle":state,"remediation":{"type":"array","value":[{"name":name,"rel":["create-form"],"href":format!("{origin}/idp/idx/{name}"),"method":"POST","value":fields}]},"cancel":{"name":"cancel","rel":["create-form"],"href":format!("{origin}/idp/idx/cancel"),"method":"POST","value":[{"name":"stateHandle","mutable":false,"value":state}]}})
    }
    pub(super) fn client(server: &Server) -> PublicClientApplication {
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "fixture-client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        PublicClientApplication::new(config).unwrap()
    }
    #[tokio::test]
    async fn native_mfa_and_stale_step_and_cancel() {
        let server = Server::new(|origin| {
            vec![
                Reply::ok(
                    "GET /oauth2/default/.well-known/openid-configuration",
                    json!({"issuer":format!("{origin}/oauth2/default"),"code_challenge_methods_supported":["S256"]}),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/interact",
                    json!({"interaction_handle":"interaction-one"}),
                ),
                Reply::ok(
                    "POST /idp/idx/introspect",
                    form(
                        origin,
                        "state-one",
                        "identify",
                        json!([{"name":"identifier","required":true}]),
                    ),
                )
                .header("Set-Cookie", "idx-session=cookie-fixture; Path=/; HttpOnly"),
                Reply::ok(
                    "POST /idp/idx/identify",
                    form(
                        origin,
                        "state-two",
                        "challenge-authenticator",
                        json!([{"name":"credentials","type":"object","form":{"value":[{"name":"passcode","secret":true,"required":true}]}}]),
                    ),
                ),
                Reply::ok(
                    "POST /idp/idx/challenge-authenticator",
                    form(origin, "state-three", "challenge-poll", json!([])),
                ),
                Reply::ok("POST /idp/idx/cancel", json!({"version":"1.0.0"})),
            ]
        });
        let client = client(&server);
        let flow = client
            .initiate_auth_flow(AuthOptions::default())
            .await
            .unwrap();
        let continuation = client.export_auth_flow(&flow).unwrap();
        assert!(!format!("{continuation:?}").contains("state-one"));
        let mut damaged = continuation.as_bytes().to_vec();
        damaged.push(b'x');
        let damaged = AuthContinuation::from_bytes(&damaged).unwrap();
        assert!(matches!(
            client.resume_auth_flow(&damaged).await,
            Err(OktaError::InvalidContinuation)
        ));
        let mut other_config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "other-client",
            "http://localhost/callback",
        );
        other_config.allow_loopback_http = true;
        let other = PublicClientApplication::new(other_config).unwrap();
        assert!(matches!(
            other.resume_auth_flow(&continuation).await,
            Err(OktaError::InvalidContinuation)
        ));
        let mut flow = client.resume_auth_flow(&continuation).await.unwrap();
        let identify = flow
            .steps()
            .iter()
            .find(|s| s.name() == "identify")
            .unwrap()
            .id()
            .clone();
        let result = client
            .continue_auth_flow(
                &mut flow,
                &identify,
                AuthInput::Form(FormInput::new(json!({"identifier":"fixture-user"})).unwrap()),
            )
            .await;
        assert!(result.is_ok(), "{result:?}");
        assert!(matches!(
            client
                .continue_auth_flow(&mut flow, &identify, AuthInput::Form(FormInput::empty()))
                .await,
            Err(OktaError::StaleStep)
        ));
        let otp = flow
            .steps()
            .iter()
            .find(|s| s.name() == "challenge-authenticator")
            .unwrap()
            .id()
            .clone();
        client
            .continue_auth_flow(
                &mut flow,
                &otp,
                AuthInput::Form(
                    FormInput::new(json!({"credentials":{"passcode":"fixture-otp"}})).unwrap(),
                ),
            )
            .await
            .unwrap();
        assert!(flow.steps().iter().any(|s| s.kind == StepKind::Poll));
        assert!(!format!("{flow:?}").contains("fixture-otp"));
        client.cancel_auth_flow(&mut flow).await.unwrap();
        assert_eq!(flow.status(), AuthStatus::Terminal);
        assert!(flow.closed);
        let requests = server.finish();
        assert_eq!(requests.len(), 6);
        let body = |i: usize| {
            serde_json::from_str::<Json>(requests[i].split_once("\r\n\r\n").unwrap().1).unwrap()
        };
        assert_eq!(body(3)["stateHandle"], "state-one");
        assert!(requests[3]
            .to_ascii_lowercase()
            .contains("cookie: idx-session=cookie-fixture"));
        assert_eq!(body(4)["stateHandle"], "state-two");
        assert_eq!(body(4)["credentials"]["passcode"], "fixture-otp");
        assert_eq!(body(5)["stateHandle"], "state-three");
    }
    #[tokio::test]
    async fn idx_success_exchanges_interaction_code_for_tokens() {
        let server = Server::new(|origin| {
            vec![
                Reply::ok(
                    "GET /oauth2/default/.well-known/openid-configuration",
                    json!({"issuer":format!("{origin}/oauth2/default"),"code_challenge_methods_supported":["S256"]}),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/interact",
                    json!({"interaction_handle":"success-interaction"}),
                ),
                Reply::ok(
                    "POST /idp/idx/introspect",
                    form(
                        origin,
                        "success-state",
                        "identify",
                        json!([{"name":"identifier","required":true}]),
                    ),
                ),
                Reply::ok(
                    "POST /idp/idx/identify",
                    json!({"version":"1.0.0","successWithInteractionCode":{"name":"success-with-interaction-code","value":[{"name":"interaction_code","value":"final-code"}]}}),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/token",
                    json!({"access_token":"final-access","expires_in":3600,"token_type":"Bearer","scope":"profile"}),
                ),
            ]
        });
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "fixture-client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        config.scopes = vec!["profile".into()];
        let client = PublicClientApplication::new(config).unwrap();
        let mut flow = client
            .initiate_auth_flow(AuthOptions::default())
            .await
            .unwrap();
        let identify = flow.steps()[0].id().clone();
        client
            .continue_auth_flow(
                &mut flow,
                &identify,
                AuthInput::Form(FormInput::new(json!({"identifier":"fixture"})).unwrap()),
            )
            .await
            .unwrap();
        assert_eq!(flow.status(), AuthStatus::Success);
        assert_eq!(
            flow.tokens().and_then(UserToken::access_token),
            Some("final-access")
        );
        let requests = server.finish();
        assert_eq!(requests.len(), 5);
        assert!(requests[4].contains("interaction_code=final-code"));
    }
}

#[cfg(test)]
mod cancellation_tests {
    use super::*;
    use crate::test_server::{Reply, Server};
    #[tokio::test]
    async fn dropped_future_requires_read_only_reconciliation() {
        fn response(origin: &str, state: &str, name: &str) -> Json {
            json!({"version":"1.0.0","stateHandle":state,"remediation":{"type":"array","value":[{"name":name,"rel":["create-form"],"href":format!("{origin}/idp/idx/{name}"),"method":"POST","value":[{"name":"identifier"},{"name":"stateHandle","mutable":false,"value":state}]}]}})
        }
        let server = Server::new(|origin| {
            vec![
                Reply::ok(
                    "GET /oauth2/default/.well-known/openid-configuration",
                    json!({"issuer":format!("{origin}/oauth2/default"),"code_challenge_methods_supported":["S256"]}),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/interact",
                    json!({"interaction_handle":"fixture-interaction"}),
                ),
                Reply::ok(
                    "POST /idp/idx/introspect",
                    response(origin, "state-one", "identify"),
                ),
                Reply {
                    prefix: "POST /idp/idx/identify",
                    body: response(origin, "state-two", "challenge-authenticator"),
                    status: 200,
                    delay: Duration::from_millis(200),
                    headers: vec![],
                },
                Reply::ok(
                    "POST /idp/idx/introspect",
                    response(origin, "state-two", "challenge-authenticator"),
                ),
            ]
        });
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        let client = PublicClientApplication::new(config).unwrap();
        let mut flow = client
            .initiate_auth_flow(AuthOptions::default())
            .await
            .unwrap();
        let step = flow.steps()[0].id().clone();
        let timed = tokio::time::timeout(
            Duration::from_millis(30),
            client.continue_auth_flow(
                &mut flow,
                &step,
                AuthInput::Form(FormInput::new(json!({"identifier":"fixture"})).unwrap()),
            ),
        )
        .await;
        assert!(timed.is_err());
        assert_eq!(flow.status(), AuthStatus::Uncertain);
        assert!(matches!(
            client
                .continue_auth_flow(&mut flow, &step, AuthInput::Form(FormInput::empty()))
                .await,
            Err(OktaError::Uncertain)
        ));
        client.introspect_auth_flow(&mut flow).await.unwrap();
        assert_eq!(flow.status(), AuthStatus::Pending);
        assert_eq!(flow.steps()[0].name(), "challenge-authenticator");
        assert_eq!(server.finish().len(), 5);
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use crate::test_server::{Reply, Server};
    #[tokio::test]
    async fn refresh_userinfo_and_revocation_use_generated_protocol_logic() {
        let fixture: Json =
            serde_json::from_str(include_str!("../../../tests/fixtures/signature.json")).unwrap();
        let jwt = fixture["token"].as_str().unwrap().to_string();
        let key = fixture["key"].clone();
        let server = Server::new(move |origin| {
            vec![
                Reply::ok(
                    "POST /oauth2/default/v1/token",
                    json!({"access_token":"access-one","id_token":jwt,"refresh_token":"refresh-two","expires_in":3600,"token_type":"Bearer","scope":"openid profile offline_access"}),
                ),
                Reply::ok(
                    "GET /oauth2/default/.well-known/openid-configuration",
                    json!({"issuer":"https://example.invalid","jwks_uri":format!("{origin}/oauth2/default/v1/keys")}),
                ),
                Reply::ok(
                    "GET /oauth2/default/v1/keys",
                    json!({"keys":[{"kid":"offline-test","alg":"RS256","use":"sig","kty":key["kty"],"n":key["n"],"e":key["e"]}]}),
                ),
                Reply::ok(
                    "GET /oauth2/default/v1/userinfo",
                    json!({"sub":"fixture","name":"Fixture User"}),
                ),
                Reply::ok("POST /oauth2/default/v1/revoke", json!({})),
                Reply::ok("POST /oauth2/default/v1/revoke", json!({})),
            ]
        });
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        let client = PublicClientApplication::new(config).unwrap();
        let original = RefreshCredential::import(
            format!("{}/oauth2/default", server.origin),
            "client".into(),
            "refresh-one".into(),
        );
        let tokens = client.refresh_tokens(&original).await.unwrap();
        assert_eq!(tokens.access_token(), Some("access-one"));
        assert_eq!(tokens.claims().unwrap()["sub"], "fixture");
        assert!(tokens.refresh_credential().is_some());
        let user = client.user_info(&tokens).await.unwrap();
        assert_eq!(user["name"], "Fixture User");
        client.revoke_access_token(&tokens).await.unwrap();
        client
            .revoke_refresh_token(tokens.refresh_credential().unwrap())
            .await
            .unwrap();
        let requests = server.finish();
        assert_eq!(requests.len(), 6);
        assert!(requests[0].contains("grant_type=refresh_token"));
        assert!(requests[0].contains("refresh_token=refresh-one"));
        assert!(requests[4].contains("token_type_hint=access_token"));
        assert!(requests[5].contains("token_type_hint=refresh_token"));
        assert!(!format!("{tokens:?}").contains("access-one"));
        assert!(!format!("{original:?}").contains("refresh-one"));
    }
    #[tokio::test]
    async fn interaction_callback_validates_state_and_exchanges_tokens() {
        let fixture: Json =
            serde_json::from_str(include_str!("../../../tests/fixtures/signature.json")).unwrap();
        let jwt = fixture["token"].as_str().unwrap().to_string();
        let key = fixture["key"].clone();
        let server = Server::new(move |origin| {
            vec![
                Reply::ok(
                    "GET /oauth2/default/.well-known/openid-configuration",
                    json!({"issuer":"https://example.invalid","jwks_uri":format!("{origin}/oauth2/default/v1/keys"),"code_challenge_methods_supported":["S256"]}),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/interact",
                    json!({"interaction_handle":"callback-interaction"}),
                ),
                Reply::ok(
                    "POST /idp/idx/introspect",
                    super::tests::form(
                        origin,
                        "callback-state-handle",
                        "identify",
                        json!([{"name":"identifier"}]),
                    ),
                ),
                Reply::ok(
                    "POST /oauth2/default/v1/token",
                    json!({"access_token":"callback-access","id_token":jwt,"refresh_token":"callback-refresh","expires_in":3600,"token_type":"Bearer","scope":"openid profile offline_access"}),
                ),
                Reply::ok(
                    "GET /oauth2/default/v1/keys",
                    json!({"keys":[{"kid":"offline-test","alg":"RS256","use":"sig","kty":key["kty"],"n":key["n"],"e":key["e"]}]}),
                ),
            ]
        });
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        let client = PublicClientApplication::new(config).unwrap();
        let flow = client
            .initiate_auth_flow(AuthOptions::default())
            .await
            .unwrap();
        let state = flow.snapshot.0["meta"]["state"]
            .as_str()
            .unwrap()
            .to_string();
        let continuation = client.export_auth_flow(&flow).unwrap();
        assert!(matches!(
            client
                .handle_interaction_code_callback(
                    &continuation,
                    "http://localhost/callback?state=wrong&interaction_code=code"
                )
                .await,
            Err(OktaError::Authentication)
        ));
        let callback = format!("http://localhost/callback?state={state}&interaction_code=code");
        let tokens = client
            .handle_interaction_code_callback(&continuation, &callback)
            .await
            .unwrap();
        assert_eq!(tokens.access_token(), Some("callback-access"));
        let requests = server.finish();
        assert_eq!(requests.len(), 5);
        assert!(requests[3].contains("interaction_code=code"));
    }
}
