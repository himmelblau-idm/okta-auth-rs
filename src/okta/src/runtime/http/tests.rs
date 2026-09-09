use super::*;
use rustls::{pki_types::pem::PemObject, ServerConfig, ServerConnection, StreamOwned};
use serde_json::json;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    thread,
    time::Instant,
};

const CASE_ENV: &str = "OKTA_PROXY_TEST_CASE";

// Each case gets its own process so proxy environment changes cannot affect
// other tests or reqwest's cached system proxy configuration.
#[test]
fn proxy_configuration_and_tls() {
    for case in ["https", "all", "precedence", "invalid", "empty", "absent"] {
        let mut child = Command::new(std::env::current_exe().unwrap());
        child.args([
            "--exact",
            "runtime::http::tests::proxy_case",
            "--nocapture",
            "--test-threads=1",
        ]);
        for name in [
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
            "HTTP_PROXY",
            "http_proxy",
            "NO_PROXY",
            "no_proxy",
        ] {
            child.env_remove(name);
        }
        let output = child.env(CASE_ENV, case).output().unwrap();
        assert!(
            output.status.success(),
            "proxy case {case}:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn read_headers(stream: &mut impl Read) -> std::io::Result<String> {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") {
        let mut byte = [0];
        stream.read_exact(&mut byte)?;
        bytes.push(byte[0]);
        assert!(bytes.len() <= 16 * 1024);
    }
    Ok(String::from_utf8(bytes).unwrap())
}

fn accept(listener: &TcpListener) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                return stream;
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                assert!(Instant::now() < deadline, "missing fixture connection");
                thread::sleep(Duration::from_millis(5));
            }
            Err(error) => panic!("fixture accept failed: {error}"),
        }
    }
}

async fn get(transport: &HttpTransport, url: &str, credentials: &str) -> JResult {
    let cx = Engine::new();
    let result = transport
        .request(
            &cx,
            vec![
                V::string(url),
                cx.from_json(&json!({"method": "GET", "credentials": credentials})),
            ],
        )
        .await;
    if let Err(error) = &result {
        // With feature-disabled ambient proxy variables, the failure can occur
        // at CONNECT before TLS. The absent-proxy case reaches the certificate.
        if url == "https://untrusted.invalid/" || std::env::var(CASE_ENV).as_deref() == Ok("absent")
        {
            assert_eq!(cx.evidence(error).unwrap().kind, FailureKind::Trust);
        }
    }
    result
}

#[test]
fn proxy_case() {
    let Ok(case) = std::env::var(CASE_ENV) else {
        return;
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let proxy_url = format!("http://{address}");
    // This is the sole test in a child process; configure its environment before
    // starting the server thread or the async runtime.
    match case.as_str() {
        "https" => std::env::set_var("HTTPS_PROXY", &proxy_url),
        "all" => std::env::set_var("ALL_PROXY", &proxy_url),
        "precedence" => {
            std::env::set_var("HTTPS_PROXY", &proxy_url);
            std::env::set_var("ALL_PROXY", "http://[invalid");
        }
        "invalid" | "empty" => {
            std::env::set_var(
                "HTTPS_PROXY",
                if case == "empty" {
                    ""
                } else {
                    "http://[invalid-secret"
                },
            );
            std::env::set_var("ALL_PROXY", &proxy_url);
        }
        "absent" => {}
        _ => panic!("unknown proxy case"),
    }
    // An explicit libhimmelblau-style proxy must not silently bypass capture.
    std::env::set_var("NO_PROXY", "*");
    let proxied = cfg!(feature = "proxyable") && case != "absent";
    let origin = if proxied {
        "https://okta.invalid".to_string()
    } else {
        format!("https://{address}")
    };
    let transport = HttpTransport::new(&[origin.clone()], Duration::from_secs(3), false);
    if cfg!(feature = "proxyable") && matches!(case.as_str(), "invalid" | "empty") {
        assert_eq!(
            transport.err().unwrap(),
            "invalid HTTPS proxy configuration"
        );
        let config = crate::ClientConfig::new(&origin, "client", "http://localhost/callback");
        assert!(matches!(
            crate::PublicClientApplication::new(config),
            Err(crate::OktaError::Configuration)
        ));
        return;
    }
    let transport = transport.unwrap();
    // These fixtures contain only a self-signed test certificate and test key.
    let cert = rustls::pki_types::CertificateDer::from_pem_slice(include_bytes!(
        "../../../../../tests/fixtures/proxy-cert.pem"
    ))
    .unwrap();
    let key = rustls::pki_types::PrivateKeyDer::from_pem_slice(include_bytes!(
        "../../../../../tests/fixtures/proxy-key.pem"
    ))
    .unwrap();
    let config =
        ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)
            .unwrap();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        // Set a cookie, check credentialed and anonymous requests, then a fresh jar.
        for _ in 0..if proxied { 4 } else { 2 } {
            let mut socket = accept(&listener);
            if proxied {
                let connect = read_headers(&mut socket).unwrap();
                assert!(connect.starts_with("CONNECT okta.invalid:443 HTTP/1.1\r\n"));
                socket
                    .write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")
                    .unwrap();
            }
            let connection = ServerConnection::new(Arc::new(config.clone())).unwrap();
            let mut tls = StreamOwned::new(connection, socket);
            let request = read_headers(&mut tls);
            if !proxied {
                assert!(request.is_err(), "untrusted certificate was accepted");
                continue;
            }
            requests.push(request.unwrap().to_ascii_lowercase());
            tls.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nSet-Cookie: session=fixture; Path=/; Secure\r\nConnection: close\r\n\r\n{}").unwrap();
            tls.flush().unwrap();
        }
        requests
    });
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Origin rejection happens before any request reaches the proxy.
            assert!(get(&transport, "https://untrusted.invalid/", "include")
                .await
                .is_err());
            if proxied {
                assert!(get(&transport, &format!("{origin}/set"), "include")
                    .await
                    .is_ok());
                assert!(
                    get(&transport, &format!("{origin}/credentialed"), "include")
                        .await
                        .is_ok()
                );
                assert!(get(&transport, &format!("{origin}/anonymous"), "omit")
                    .await
                    .is_ok());
                let fresh =
                    HttpTransport::new(&[origin.clone()], Duration::from_secs(3), false).unwrap();
                assert!(get(&fresh, &format!("{origin}/fresh"), "include")
                    .await
                    .is_ok());
            } else {
                for credentials in ["include", "omit"] {
                    assert!(get(&transport, &origin, credentials).await.is_err());
                }
            }
        });
    let requests = server.join().unwrap();
    if proxied {
        assert_eq!(requests.len(), 4);
        assert!(!requests[0].contains("cookie:"));
        assert!(requests[1].contains("cookie: session=fixture\r\n"));
        assert!(!requests[2].contains("cookie:"));
        assert!(!requests[3].contains("cookie:"));
    }
}

#[test]
fn retry_after_is_bounded_and_handles_http_dates() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    for (header, expected) in [
        ("0", Some(0)),
        ("17", Some(17)),
        ("99999999999999999999999999", Some(86400)),
        ("-1", None),
        ("1.5", None),
        ("bad", None),
        ("", None),
    ] {
        assert_eq!(retry_after(header, now), expected.map(Duration::from_secs));
    }
    for (time, expected) in [
        (now - Duration::from_secs(1), 0),
        (now + Duration::from_secs(12), 12),
        (now + Duration::from_secs(864000), 86400),
    ] {
        assert_eq!(
            retry_after(&httpdate::fmt_http_date(time), now),
            Some(Duration::from_secs(expected))
        );
    }
    assert_eq!(
        retry_after("Sun, 06 Nov 1994 08:49:37 GMT", SystemTime::now()),
        Some(Duration::ZERO)
    );
}

async fn request_failure(origin: &str, timeout: Duration) -> crate::OperationFailure {
    fn fetch(cx: &Engine, _: usize, _: V, args: Vec<V>) -> JResult {
        cx.fetch(args)
    }
    let cx = Engine::new();
    cx.set_http_transport(HttpTransport::new(&[origin.into()], timeout, true).unwrap());
    let function = cx.function(cx.scope(None), fetch);
    let error = cx
        .execute(
            function,
            V::Undefined,
            vec![V::string(origin), cx.from_json(&json!({"method":"GET"}))],
        )
        .await
        .unwrap_err();
    let crate::OktaError::OperationFailed(failure) =
        cx.operation_error(&error, crate::FailureStage::Remediation)
    else {
        panic!("untyped error")
    };
    failure
}

#[tokio::test]
async fn body_failures_keep_status_and_response_outcome() {
    use crate::test_server::{Reply, Server};
    for status in [200, 404, 503] {
        for mode in ["truncated", "timeout", "oversize"] {
            let server = Server::new(|_| {
                let mut reply = Reply::ok("GET / ", json!({})).status(status);
                match mode {
                    "truncated" => reply.declared_length = Some(100),
                    "timeout" => reply.body_delay = Duration::from_millis(150),
                    _ => reply = reply.raw("application/json", vec![b'x'; 4 * 1024 * 1024 + 1]),
                }
                vec![reply]
            });
            let f = request_failure(
                &server.origin,
                Duration::from_millis(if mode == "timeout" { 30 } else { 2000 }),
            )
            .await;
            assert_eq!(
                f.kind,
                if status == 200 {
                    FailureKind::Protocol
                } else {
                    FailureKind::Http
                }
            );
            assert_eq!(f.http_status, Some(status));
            assert_eq!(f.outcome, RequestOutcome::ResponseReceived);
            server.finish();
        }
    }
}

#[tokio::test]
async fn connection_failure_has_no_response_and_no_replay_promise() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let f = request_failure(&origin, Duration::from_secs(1)).await;
    assert_eq!(f.kind, FailureKind::Transport);
    assert_eq!(f.http_status, None);
    assert_eq!(f.outcome, RequestOutcome::Unknown);
}

#[tokio::test]
async fn dns_failure_is_transport_without_system_dns() {
    struct Unavailable;
    impl reqwest::dns::Resolve for Unavailable {
        fn resolve(&self, _: reqwest::dns::Name) -> reqwest::dns::Resolving {
            Box::pin(async {
                Err(std::io::Error::new(std::io::ErrorKind::NotFound, "fixture DNS failure").into())
            })
        }
    }
    let origin = "https://dns-fixture.invalid";
    let mut transport =
        HttpTransport::new(&[origin.into()], Duration::from_secs(1), false).unwrap();
    transport.anonymous = Client::builder()
        .no_proxy()
        .dns_resolver(Arc::new(Unavailable))
        .build()
        .unwrap();
    let cx = Engine::new();
    let error = transport
        .request(
            &cx,
            vec![V::string(origin), cx.from_json(&json!({"method":"GET"}))],
        )
        .await
        .unwrap_err();
    let failure = cx.evidence(&error).unwrap();
    assert_eq!(failure.kind, FailureKind::Transport);
    assert_eq!(failure.outcome, RequestOutcome::Unknown);
    assert_eq!(failure.http_status, None);
}
