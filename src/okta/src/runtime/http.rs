//! HTTP platform service. Request serialization and response interpretation are generated.
use super::{Engine, JResult, V};
use crate::{FailureKind, OperationFailure, RequestOutcome};
use reqwest::{
    header::{HeaderName, HeaderValue},
    redirect::Policy,
    Client, Url,
};
use std::time::SystemTime;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone)]
pub struct HttpTransport {
    credentialed: Client,
    anonymous: Client,
    jar: Arc<CookieJar>,
    origins: Arc<Vec<String>>,
    max_response_bytes: usize,
}
#[derive(Default)]
struct CookieJar(Mutex<cookie_store::CookieStore>);
impl reqwest::cookie::CookieStore for CookieJar {
    fn set_cookies(&self, headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let mut jar = self.0.lock().unwrap();
        for header in headers {
            if let Ok(text) = header.to_str() {
                let _ = jar.parse(text, url);
            }
        }
    }
    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let jar = self.0.lock().unwrap();
        let cookies = jar
            .get_request_values(url)
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ");
        if cookies.is_empty() {
            None
        } else {
            let mut header = HeaderValue::from_str(&cookies).ok()?;
            header.set_sensitive(true);
            Some(header)
        }
    }
}
impl HttpTransport {
    /// Each instance owns a separate cookie jar. Redirects are returned to source logic.
    pub fn new(
        origins: &[String],
        timeout: Duration,
        allow_loopback_http: bool,
    ) -> Result<Self, String> {
        if origins.is_empty() {
            return Err("at least one trusted origin is required".into());
        }
        let origins = origins
            .iter()
            .map(|origin| {
                let url = Url::parse(origin).map_err(|_| "invalid trusted origin")?;
                let loopback = url.host_str().is_some_and(|host| {
                    host == "localhost"
                        || host
                            .parse::<std::net::IpAddr>()
                            .is_ok_and(|ip| ip.is_loopback())
                });
                if url.scheme() != "https"
                    && !(allow_loopback_http && url.scheme() == "http" && loopback)
                {
                    return Err("trusted origin must use HTTPS");
                }
                if !url.username().is_empty() || url.password().is_some() {
                    return Err("URL credentials are forbidden");
                }
                Ok(url.origin().ascii_serialization())
            })
            .collect::<Result<Vec<_>, &str>>()
            .map_err(str::to_string)?;
        let jar = Arc::new(CookieJar::default());
        #[cfg(feature = "proxyable")]
        let proxy = std::env::var("HTTPS_PROXY")
            .ok()
            .or_else(|| std::env::var("ALL_PROXY").ok())
            .map(reqwest::Proxy::https)
            .transpose()
            .map_err(|_| "invalid HTTPS proxy configuration")?;
        let builder = || {
            let builder = Client::builder()
                .use_rustls_tls()
                .timeout(timeout)
                .redirect(Policy::none())
                .retry(reqwest::retry::never());
            // Match libhimmelblau's opt-in interception feature. Certificate
            // verification is relaxed only when an explicit proxy is configured.
            #[cfg(feature = "proxyable")]
            let builder = match &proxy {
                Some(proxy) => builder
                    .proxy(proxy.clone())
                    .danger_accept_invalid_certs(true),
                None => builder,
            };
            builder
        };
        let credentialed = builder()
            .cookie_provider(jar.clone())
            .build()
            .map_err(|_| "HTTP client initialization failed")?;
        let anonymous = builder()
            .build()
            .map_err(|_| "HTTP client initialization failed")?;
        Ok(Self {
            credentialed,
            anonymous,
            jar,
            origins: Arc::new(origins),
            max_response_bytes: 4 * 1024 * 1024,
        })
    }
    pub fn export_cookies(&self) -> Result<zeroize::Zeroizing<Vec<u8>>, String> {
        let jar = self.jar.0.lock().unwrap();
        let cookies = jar.iter_unexpired().collect::<Vec<_>>();
        serde_json::to_vec(&cookies)
            .map(zeroize::Zeroizing::new)
            .map_err(|_| "cookie serialization failed".into())
    }
    pub fn import_cookies(&self, data: &[u8]) -> Result<(), String> {
        if data.len() > 4 * 1024 * 1024 {
            return Err("cookie data exceeds limit".into());
        }
        let restored =
            cookie_store::serde::json::load(data).map_err(|_| "invalid cookie snapshot")?;
        *self.jar.0.lock().unwrap() = restored;
        Ok(())
    }
    async fn request(&self, cx: &Engine, args: Vec<V>) -> JResult {
        // A source-level retry (including an unexpected DPoP challenge) must not
        // reuse an interaction code or a potentially rotated refresh credential.
        if let Some(failure) = cx.mutation_evidence() {
            let error = cx.error("mutation replay prohibited");
            cx.attach_evidence(&error, failure);
            return Err(error);
        }
        cx.request_evidence(OperationFailure::local(FailureKind::Protocol, cx.1.stage));
        let url = Url::parse(&args.first().unwrap_or(&V::Undefined).text())
            .map_err(|_| cx.error("invalid request URL"))?;
        if !self.origins.contains(&url.origin().ascii_serialization())
            || !url.username().is_empty()
            || url.password().is_some()
        {
            let mut failure = cx.current_evidence();
            failure.kind = FailureKind::Trust;
            cx.request_evidence(failure);
            return cx.diagnostic_result(Err(cx.error("request origin is not trusted")));
        }
        let options = args.get(1).cloned().unwrap_or(V::Undefined);
        let method = reqwest::Method::from_bytes(cx.get(&options, "method")?.text().as_bytes())
            .map_err(|_| cx.error("invalid request method"))?;
        let client = if cx.get(&options, "credentials")?.text() == "include" {
            &self.credentialed
        } else {
            &self.anonymous
        };
        let mut request = client.request(method, url);
        let headers = cx.get(&options, "headers")?;
        if !headers.nullish() {
            for (name, value) in cx.entries(&headers)? {
                let name = HeaderName::from_bytes(name.as_bytes())
                    .map_err(|_| cx.error("invalid request header"))?;
                let mut value = HeaderValue::from_str(&value.text())
                    .map_err(|_| cx.error("invalid request header"))?;
                value.set_sensitive(true);
                request = request.header(name, value);
            }
        }
        let body = cx.get(&options, "body")?;
        if !body.nullish() {
            if !matches!(body, V::String(_)) {
                return Err(cx.error("generated fetch body must be a string"));
            }
            request = request.body(body.text());
        }
        let request = request.build().map_err(|_| cx.error("invalid request"))?;
        let mut pending = cx.current_evidence();
        pending.outcome = RequestOutcome::Unknown;
        cx.request_evidence(pending);
        let mut response = client.execute(request).await.map_err(|error| {
            let mut failure = cx.current_evidence();
            failure.kind = classify_send_error(&error);
            failure.outcome = RequestOutcome::Unknown;
            cx.request_evidence(failure.clone());
            let error = cx.error("native request failed");
            cx.attach_evidence(&error, failure);
            error
        })?;
        let status = response.status();
        let mut failure = OperationFailure::local(
            if status.is_success() {
                FailureKind::Protocol
            } else {
                FailureKind::Http
            },
            cx.1.stage,
        );
        failure.http_status = Some(status.as_u16());
        failure.outcome = RequestOutcome::ResponseReceived;
        failure.retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| retry_after(h, SystemTime::now()));
        cx.request_evidence(failure);
        let headers = cx.object();
        for (name, value) in response.headers() {
            cx.put(
                &headers,
                name.as_str(),
                V::string(
                    value
                        .to_str()
                        .map_err(|_| cx.error("invalid response header"))?,
                ),
            )?;
        }
        let mut bytes = zeroize::Zeroizing::new(Vec::new());
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| cx.error("native response read failed"))?
        {
            if chunk.len() > self.max_response_bytes.saturating_sub(bytes.len()) {
                return Err(cx.error("response exceeds configured limit"));
            }
            bytes.extend_from_slice(&chunk);
        }
        if let Ok(body) = serde_json::from_slice::<serde_json::Value>(&bytes) {
            let mut failure = cx.current_evidence();
            failure.oauth_error =
                crate::failure::safe_code(body.get("error").and_then(|v| v.as_str()));
            failure.okta_error_code =
                crate::failure::safe_code(body.get("errorCode").and_then(|v| v.as_str()));
            cx.request_evidence(failure);
            if status.is_success() {
                let usable = (body.is_object() || cx.1.stage == crate::FailureStage::Revoke)
                    && match cx.1.stage {
                        crate::FailureStage::Interact => body
                            .get("interaction_handle")
                            .and_then(|v| v.as_str())
                            .is_some_and(|s| !s.is_empty()),
                        crate::FailureStage::Discovery => {
                            body.get("issuer").is_some_and(|v| v.is_string())
                        }
                        crate::FailureStage::Jwks => body.get("keys").is_some_and(|v| v.is_array()),
                        _ => true,
                    };
                if !usable {
                    let mut failure = cx.current_evidence();
                    failure.kind = FailureKind::Protocol;
                    let error = cx.error("invalid protocol response");
                    cx.attach_evidence(&error, failure);
                    return Err(error);
                }
            }
        }
        let body_text = V::string(String::from_utf8_lossy(&bytes).into_owned());
        let scope = cx.scope(None);
        cx.define(scope, "headers", headers);
        cx.define(scope, "body", body_text);
        fn get(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
            let value = cx.get(
                &cx.lookup(scope, "headers")?,
                &args
                    .first()
                    .unwrap_or(&V::Undefined)
                    .text()
                    .to_ascii_lowercase(),
            )?;
            Ok(if matches!(value, V::Undefined) {
                V::Null
            } else {
                value
            })
        }
        fn entries(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
            Ok(cx.array(
                cx.entries(&cx.lookup(scope, "headers")?)?
                    .into_iter()
                    .map(|(k, v)| cx.array(vec![V::string(k), v]))
                    .collect(),
            ))
        }
        fn text(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
            let body = cx.lookup(scope, "body")?;
            Ok(cx.promise(Box::pin(async move { Ok(body) })))
        }
        fn json(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
            let body = cx.lookup(scope, "body")?.text();
            // Revocation has no required response representation. Treat an
            // empty successful body as the empty result expected by httpRequest.
            if body.is_empty()
                && cx.1.stage == crate::FailureStage::Revoke
                && cx
                    .current_evidence()
                    .http_status
                    .is_some_and(|s| (200..300).contains(&s))
            {
                let empty = cx.object();
                return Ok(cx.promise(Box::pin(async move { Ok(empty) })));
            }
            let value = serde_json::from_str(&body)
                .map(|v| cx.from_json(&v))
                .map_err(|_| {
                    let error = cx.error("invalid JSON response");
                    let mut failure = cx.current_evidence();
                    if failure.http_status.is_some_and(|s| (200..300).contains(&s)) {
                        failure.kind = FailureKind::Protocol;
                    }
                    cx.attach_evidence(&error, failure);
                    error
                });
            Ok(cx.promise(Box::pin(async move { value })))
        }
        let header_api = cx.object();
        cx.put(&header_api, "get", cx.function(scope, get))?;
        cx.put(&header_api, "entries", cx.function(scope, entries))?;
        let result = cx.object();
        cx.put(&result, "headers", header_api)?;
        cx.put(&result, "status", V::Number(status.as_u16() as f64))?;
        cx.put(&result, "ok", V::Bool(status.is_success()))?;
        cx.put(&result, "text", cx.function(scope, text))?;
        cx.put(&result, "json", cx.function(scope, json))?;
        cx.attach_evidence(&result, cx.current_evidence());
        Ok(result)
    }
}

#[cfg(test)]
mod tests;
impl Engine {
    pub fn export_cookies(&self) -> Result<zeroize::Zeroizing<Vec<u8>>, String> {
        self.http_transport()
            .ok_or("no transport")?
            .export_cookies()
    }
    pub fn import_cookies(&self, data: &[u8]) -> Result<(), String> {
        self.http_transport()
            .ok_or("no transport")?
            .import_cookies(data)
    }
    pub(crate) fn fetch(&self, args: Vec<V>) -> JResult {
        let transport = self
            .http_transport()
            .ok_or_else(|| self.error("no HTTP transport configured"))?;
        let mut cx = self.clone();
        // A fresh slot identifies this specific request, not the last request
        // made by another callback. Values carry immutable evidence snapshots.
        cx.1.request = Arc::default();
        Ok(self.schedule(Box::pin(async move {
            let result = transport.request(&cx, args).await;
            // Body and header failures retain the request's already-received status.
            if let Err(error) = &result {
                let mut failure = cx.current_evidence();
                if failure.http_status.is_some_and(|s| (200..300).contains(&s)) {
                    failure.kind = FailureKind::Protocol;
                }
                cx.attach_evidence(error, failure);
            }
            result
        })))
    }
}

/// The public retry delay is capped at one day, including enormous decimal values.
fn retry_after(value: &str, now: SystemTime) -> Option<Duration> {
    const MAX: Duration = Duration::from_secs(24 * 60 * 60);
    let value = value.trim();
    if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) {
        let seconds = value.bytes().fold(0u64, |n, b| {
            n.saturating_mul(10).saturating_add(u64::from(b - b'0'))
        });
        return Some(Duration::from_secs(seconds).min(MAX));
    }
    let date = httpdate::parse_http_date(value).ok()?;
    Some(date.duration_since(now).unwrap_or_default().min(MAX))
}

fn classify_send_error(error: &reqwest::Error) -> FailureKind {
    fn tls_kind(error: &(dyn std::error::Error + 'static), depth: usize) -> Option<FailureKind> {
        if depth == 32 {
            return None;
        }
        if let Some(tls) = error.downcast_ref::<rustls::Error>() {
            return Some(match tls {
                rustls::Error::InvalidCertificate(_)
                | rustls::Error::InvalidCertRevocationList(_) => FailureKind::Trust,
                _ => FailureKind::Protocol,
            });
        }
        // io::Error::source can skip its boxed inner error, including nested io
        // wrappers around rustls. Inspect get_ref at every level, not just one.
        if let Some(inner) = error
            .downcast_ref::<std::io::Error>()
            .and_then(|e| e.get_ref())
        {
            if let Some(kind) = tls_kind(inner, depth + 1) {
                return Some(kind);
            }
        }
        error.source().and_then(|e| tls_kind(e, depth + 1))
    }
    if let Some(kind) = tls_kind(error, 0) {
        return kind;
    }
    if error.is_timeout() || error.is_connect() || error.is_request() {
        FailureKind::Transport
    } else {
        FailureKind::Protocol
    }
}
