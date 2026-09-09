//! HTTP platform service. Request serialization and response interpretation are generated.
use super::{Engine, JResult, V};
use reqwest::{
    header::{HeaderName, HeaderValue},
    redirect::Policy,
    Client, Url,
};
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
        let builder = || {
            Client::builder()
                .use_rustls_tls()
                .timeout(timeout)
                .redirect(Policy::none())
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
        let url = Url::parse(&args.first().unwrap_or(&V::Undefined).text())
            .map_err(|_| cx.error("invalid request URL"))?;
        if !self.origins.contains(&url.origin().ascii_serialization())
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(cx.error("request origin is not trusted"));
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
        let mut response = request
            .send()
            .await
            .map_err(|_| cx.error("native request failed"))?;
        let status = response.status();
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
            let value = serde_json::from_str(&cx.lookup(scope, "body")?.text())
                .map(|v| cx.from_json(&v))
                .map_err(|_| cx.error("invalid JSON response"));
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
        Ok(result)
    }
}
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
        let cx = self.clone();
        Ok(self.schedule(Box::pin(async move { transport.request(&cx, args).await })))
    }
}
