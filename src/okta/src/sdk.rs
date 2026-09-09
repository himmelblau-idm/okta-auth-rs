//! Wiring for generated functions; platform storage contains no protocol decisions.
use crate::runtime::{Engine, HttpTransport, JResult, V};
use std::collections::BTreeMap;

pub(crate) struct Sdk {
    pub engine: Engine,
    pub client: V,
    pub exports: BTreeMap<String, V>,
    pub storage: BTreeMap<String, V>,
}
fn constant(cx: &Engine, value: V) -> V {
    fn call(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
        cx.lookup(scope, "value")
    }
    let scope = cx.scope(None);
    cx.define(scope, "value", value);
    cx.function(scope, call)
}
fn bind(cx: &Engine, sdk: V, function: V) -> V {
    fn call(cx: &Engine, scope: usize, _this: V, mut args: Vec<V>) -> JResult {
        args.insert(0, cx.lookup(scope, "sdk")?);
        cx.call(cx.lookup(scope, "function")?, V::Undefined, args)
    }
    let scope = cx.scope(None);
    cx.define(scope, "sdk", sdk);
    cx.define(scope, "function", function);
    cx.function(scope, call)
}
fn storage(cx: &Engine) -> JResult {
    fn get(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
        Ok(cx.from_json(&cx.to_json(&cx.lookup(scope, "data")?)?))
    }
    fn set(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
        let data = cx.from_json(&cx.to_json(args.first().unwrap_or(&V::Undefined))?);
        cx.assign(scope, "data", data)?;
        Ok(V::Undefined)
    }
    fn clear(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
        if let Some(key) = args.first().filter(|v| !v.nullish()) {
            cx.delete(&cx.lookup(scope, "data")?, &key.text())?;
        } else {
            cx.assign(scope, "data", cx.object())?;
        }
        Ok(V::Undefined)
    }
    fn update(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
        let value = cx.from_json(&cx.to_json(args.get(1).unwrap_or(&V::Undefined))?);
        cx.put(
            &cx.lookup(scope, "data")?,
            &args.first().unwrap_or(&V::Undefined).text(),
            value,
        )?;
        Ok(V::Undefined)
    }
    let scope = cx.scope(None);
    cx.define(scope, "data", cx.object());
    let object = cx.object();
    for (name, code) in [
        ("getStorage", get as crate::runtime::Code),
        ("setStorage", set),
        ("clearStorage", clear),
        ("updateStorage", update),
        ("delete", clear),
        ("set", update),
    ] {
        cx.put(&object, name, cx.function(scope, code))?;
    }
    Ok(object)
}
impl Sdk {
    pub fn new(
        options: serde_json::Value,
        origin: &str,
        transport: HttpTransport,
    ) -> Result<Self, V> {
        let cx = Engine::new();
        cx.set_http_transport(transport);
        let exports = crate::generated::initialize(&cx)?;
        let client = cx.object();
        let config = cx.from_json(&options);
        cx.put(&client, "options", config.clone())?;
        let get = |name: &str| {
            exports
                .get(name)
                .cloned()
                .ok_or_else(|| cx.error(&format!("missing generated export {name}")))
        };
        let manager = cx.object();
        let mut stores = BTreeMap::new();
        for name in [
            "getTransactionStorage",
            "getIdxResponseStorage",
            "getHttpCache",
            "getSharedTansactionStorage",
        ] {
            let store = storage(&cx)?;
            cx.put(&manager, name, constant(&cx, store.clone()))?;
            stores.insert(name.to_string(), store);
        }
        cx.put(&client, "storageManager", manager.clone())?;
        let ctor = cx.call(get("createIdxTransactionManager")?, V::Undefined, vec![])?;
        let tm_options = cx.object();
        cx.put(&tm_options, "storageManager", manager)?;
        cx.put(&tm_options, "enableSharedStorage", V::Bool(false))?;
        cx.put(&tm_options, "saveLastResponse", V::Bool(true))?;
        cx.put(
            &client,
            "transactionManager",
            cx.construct(ctor, vec![tm_options])?,
        )?;
        let storage_util = cx.object();
        cx.put(&storage_util, "storage", storage(&cx)?)?;
        cx.put(&config, "storageUtil", storage_util)?;
        cx.put(&config, "httpRequestClient", get("fetchRequest")?)?;
        let agent = cx.object();
        cx.put(&agent,"getHttpHeader",constant(&cx,cx.from_json(&serde_json::json!({"X-Okta-User-Agent-Extended":"okta-auth-rs-translator/0.1"}))))?;
        cx.put(&client, "_oktaUserAgent", agent)?;
        cx.put(&client, "getIssuerOrigin", constant(&cx, V::string(origin)))?;
        let features = cx.object();
        for name in [
            "isPKCESupported",
            "isTokenVerifySupported",
            "isHTTPS",
            "hasTextEncoder",
        ] {
            cx.put(&features, name, constant(&cx, V::Bool(true)))?;
        }
        cx.put(&features, "isDPoPSupported", constant(&cx, V::Bool(false)))?;
        cx.put(&client, "features", features)?;
        let token = cx.object();
        for (name, entry) in [
            ("prepareTokenParams", "prepareTokenParams"),
            ("exchangeCodeForTokens", "exchangeCodeForTokens"),
            ("verify", "verifyToken"),
        ] {
            cx.put(&token, name, bind(&cx, client.clone(), get(entry)?))?;
        }
        cx.put(&token, "decode", get("decodeToken")?)?;
        cx.put(&client, "token", token)?;
        let idx = cx.object();
        for (name, entry) in [
            ("proceed", "proceed"),
            ("canProceed", "canProceed"),
            ("makeIdxResponse", "makeIdxState"),
        ] {
            cx.put(&idx, name, bind(&cx, client.clone(), get(entry)?))?;
        }
        cx.put(&client, "idx", idx)?;
        let tokens = storage(&cx)?;
        let token_manager = cx.object();
        cx.put(&token_manager, "getTokens", cx.get(&tokens, "getStorage")?)?;
        cx.put(&token_manager, "setTokens", cx.get(&tokens, "setStorage")?)?;
        fn update_refresh(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
            cx.method(
                cx.lookup(scope, "store")?,
                "updateStorage",
                vec![
                    V::string("refreshToken"),
                    args.first().cloned().unwrap_or(V::Undefined),
                ],
            )
        }
        fn remove_refresh(cx: &Engine, scope: usize, _this: V, _args: Vec<V>) -> JResult {
            cx.method(
                cx.lookup(scope, "store")?,
                "clearStorage",
                vec![V::string("refreshToken")],
            )
        }
        let scope = cx.scope(None);
        cx.define(scope, "store", tokens.clone());
        cx.put(
            &token_manager,
            "updateRefreshToken",
            cx.function(scope, update_refresh),
        )?;
        cx.put(
            &token_manager,
            "removeRefreshToken",
            cx.function(scope, remove_refresh),
        )?;
        cx.put(&client, "tokenManager", token_manager)?;
        stores.insert("tokens".into(), tokens);
        Ok(Self {
            engine: cx,
            client,
            exports,
            storage: stores,
        })
    }
    pub async fn call(&self, name: &str, mut args: Vec<V>) -> JResult {
        let f = self
            .exports
            .get(name)
            .cloned()
            .ok_or_else(|| self.engine.error("missing generated entry point"))?;
        args.insert(0, self.client.clone());
        self.engine.execute(f, V::Undefined, args).await
    }
    pub fn export_storage(&self) -> Result<BTreeMap<String, serde_json::Value>, V> {
        self.storage
            .iter()
            .map(|(name, store)| {
                let value = self.engine.method(store.clone(), "getStorage", vec![])?;
                Ok((name.clone(), self.engine.to_json(&value)?))
            })
            .collect()
    }
    pub fn import_storage(&self, values: &BTreeMap<String, serde_json::Value>) -> Result<(), V> {
        if values.len() != self.storage.len()
            || values.keys().any(|name| !self.storage.contains_key(name))
        {
            return Err(self.engine.error("invalid storage snapshot"));
        }
        for (name, value) in values {
            let store = self
                .storage
                .get(name)
                .ok_or_else(|| self.engine.error("invalid storage snapshot"))?;
            self.engine.method(
                store.clone(),
                "setStorage",
                vec![self.engine.from_json(value)],
            )?;
        }
        Ok(())
    }
    pub fn storage_value(&self, name: &str) -> Result<serde_json::Value, V> {
        let store = self
            .storage
            .get(name)
            .ok_or_else(|| self.engine.error("missing storage area"))?;
        let value = self.engine.method(store.clone(), "getStorage", vec![])?;
        self.engine.to_json(&value)
    }
}
impl Drop for Sdk {
    fn drop(&mut self) {
        self.engine.cancel_pending();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        time::{Duration, Instant},
    };
    #[tokio::test]
    async fn discovery_interact_and_identify_over_real_http() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let issuer = format!("{origin}/oauth2/default");
        let responses = vec![
            (
                "GET /oauth2/default/.well-known/openid-configuration",
                json!({"issuer":issuer,"code_challenge_methods_supported":["S256"]}),
            ),
            (
                "POST /oauth2/default/v1/interact",
                json!({"interaction_handle":"fixture-interaction"}),
            ),
            (
                "POST /idp/idx/introspect",
                json!({"version":"1.0.0","stateHandle":"fixture-state","remediation":{"type":"array","value":[{"name":"identify","rel":["create-form"],"href":format!("{origin}/idp/idx/identify"),"method":"POST","value":[{"name":"identifier","required":true},{"name":"stateHandle","value":"fixture-state","mutable":false}]}]}}),
            ),
        ];
        let server = std::thread::spawn(move || {
            let mut requests = vec![];
            for (prefix, body) in responses {
                let deadline = Instant::now() + Duration::from_secs(5);
                let (mut stream, _) = loop {
                    match listener.accept() {
                        Ok(s) => break s,
                        Err(e)
                            if e.kind() == std::io::ErrorKind::WouldBlock
                                && Instant::now() < deadline =>
                        {
                            std::thread::sleep(Duration::from_millis(1))
                        }
                        Err(e) => panic!("missing request {prefix}: {e}"),
                    }
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = vec![];
                let mut buffer = [0; 4096];
                loop {
                    let n = stream.read(&mut buffer).unwrap();
                    assert_ne!(n, 0);
                    bytes.extend_from_slice(&buffer[..n]);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..end]);
                        let length = header
                            .lines()
                            .find_map(|line| {
                                line.to_ascii_lowercase()
                                    .strip_prefix("content-length:")
                                    .map(|s| s.trim().parse::<usize>().unwrap())
                            })
                            .unwrap_or(0);
                        if bytes.len() >= end + 4 + length {
                            break;
                        }
                    }
                }
                let request = String::from_utf8(bytes).unwrap();
                eprintln!("received {prefix}");
                assert!(request.starts_with(prefix), "{request}");
                requests.push(request);
                let body = body.to_string();
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();
            }
            requests
        });
        let transport =
            HttpTransport::new(&[origin.clone()], Duration::from_secs(3), true).unwrap();
        let sdk=Sdk::new(json!({"issuer":issuer,"clientId":"fixture-client","redirectUri":"http://localhost/callback","scopes":["openid"],"pkce":true,"dpop":false,"ignoreSignature":false,"maxClockSkew":0,"idx":{"useGenericRemediator":true,"enableLegacyMode":false}}),&origin,transport).unwrap_or_else(|e|panic!("initialize: {}",e.text()));
        let result = tokio::time::timeout(
            Duration::from_secs(4),
            sdk.call("startTransaction", vec![sdk.engine.object()]),
        )
        .await
        .expect("flow must complete")
        .unwrap_or_else(|e| {
            panic!(
                "flow error: {} {}",
                e.text(),
                sdk.engine.to_json(&e).unwrap().to_string()
            )
        });
        let result = sdk.engine.to_json(&result).unwrap();
        assert_eq!(result["status"], "PENDING");
        assert_eq!(result["availableSteps"][0]["name"], "identify");
        let requests = server.join().unwrap();
        assert!(requests[1].contains("code_challenge_method=S256"));
        assert!(requests[2].contains("fixture-interaction"));
        assert!(!requests[1].contains("client_secret"));
    }
}
