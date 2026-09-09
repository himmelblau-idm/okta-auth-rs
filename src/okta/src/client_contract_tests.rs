use super::tests::{client, form};
use super::*;
use crate::{
    test_server::{Reply, Server},
    RequestOutcome,
};

fn failure(
    error: OktaError,
    kind: FailureKind,
    stage: FailureStage,
    status: Option<u16>,
) -> OperationFailure {
    let OktaError::OperationFailed(failure) = error else {
        panic!("unclassified error: {error:?}")
    };
    assert_eq!(failure.kind, kind, "{failure:?}");
    assert_eq!(failure.stage, stage, "{failure:?}");
    assert_eq!(failure.http_status, status, "{failure:?}");
    if status.is_some() {
        assert_eq!(failure.outcome, RequestOutcome::ResponseReceived);
    }
    failure
}
fn discovery(origin: &str) -> Reply {
    Reply::ok(
        "GET /oauth2/default/.well-known/openid-configuration",
        json!({"issuer":format!("{origin}/oauth2/default"),"code_challenge_methods_supported":["S256"]}),
    )
}
fn interact() -> Reply {
    Reply::ok(
        "POST /oauth2/default/v1/interact",
        json!({"interaction_handle":"fixture-interaction"}),
    )
}
fn identify(origin: &str, state: &str) -> Reply {
    Reply::ok(
        "POST /idp/idx/introspect",
        form(
            origin,
            state,
            "identify",
            json!([{"name":"identifier","required":true}]),
        ),
    )
}
fn credential(server: &Server) -> RefreshCredential {
    RefreshCredential::import(
        format!("{}/oauth2/default", server.origin),
        "fixture-client".into(),
        "fixture-refresh-secret".into(),
    )
}

#[tokio::test]
async fn interact_status_and_codes_survive_all_response_shapes() {
    for status in [404, 405, 501, 408, 429, 500, 502, 503, 504] {
        for shape in ["json", "html", "empty", "malformed"] {
            let server = Server::new(|origin| {
                let reply = Reply::ok(
                    "POST /oauth2/default/v1/interact",
                    json!({"error":"unauthorized_client"}),
                )
                .status(status);
                let reply = match shape {
                    "html" => reply.raw(
                        "text/html",
                        b"<html>fixture-response-secret</html>".to_vec(),
                    ),
                    "empty" => reply.raw("application/json", vec![]),
                    "malformed" => reply.raw("application/json", b"{bad".to_vec()),
                    _ => reply,
                };
                vec![discovery(origin), reply]
            });
            let app = client(&server);
            let f = failure(
                app.initiate_auth_flow(AuthOptions::default())
                    .await
                    .unwrap_err(),
                FailureKind::Http,
                FailureStage::Interact,
                Some(status),
            );
            assert_eq!(
                f.oauth_error.as_deref(),
                if shape == "json" {
                    Some("unauthorized_client")
                } else {
                    None
                }
            );
            assert!(!format!("{f:?} {f}").contains("fixture-response-secret"));
            assert_eq!(server.finish().len(), 2);
        }
    }
    for code in [
        "unsupported_grant_type",
        "unauthorized_client",
        "invalid_client",
        "invalid_grant",
    ] {
        for description in [false, true] {
            let server = Server::new(|origin| {
                let mut body = json!({"error":code});
                if description {
                    body["error_description"] =
                        json!("fixture-password-secret https://secret.invalid/path");
                }
                vec![
                    discovery(origin),
                    Reply::ok("POST /oauth2/default/v1/interact", body).status(400),
                ]
            });
            let f = failure(
                client(&server)
                    .initiate_auth_flow(AuthOptions::default())
                    .await
                    .unwrap_err(),
                FailureKind::Http,
                FailureStage::Interact,
                Some(400),
            );
            assert_eq!(f.oauth_error.as_deref(), Some(code));
            assert!(!format!("{f:?} {f}").contains("secret"));
            server.finish();
        }
    }
}

#[tokio::test]
async fn safe_codes_retry_headers_and_discovery_failures() {
    for code in [
        "E0000004".to_string(),
        "A".repeat(128),
        "A".repeat(129),
        "bad code".into(),
        "secret/url".into(),
        "é".into(),
        "".into(),
    ] {
        let server = Server::new(|_| {
            vec![Reply::ok(
                "GET /oauth2/default/.well-known/openid-configuration",
                json!({"errorCode":code,"errorSummary":"fixture-password-secret"}),
            )
            .status(429)
            .header("Retry-After", "17")]
        });
        let f = failure(
            client(&server)
                .initiate_auth_flow(AuthOptions::default())
                .await
                .unwrap_err(),
            FailureKind::Http,
            FailureStage::Discovery,
            Some(429),
        );
        assert_eq!(f.okta_error_code, crate::failure::safe_code(Some(&code)));
        assert_eq!(f.retry_after, Some(Duration::from_secs(17)));
        assert_eq!(f.oauth_error, None);
        assert!(!format!("{f:?} {f}").contains("fixture-password-secret"));
        server.finish();
    }
}

#[tokio::test]
async fn successful_unusable_responses_and_nested_introspection() {
    for body in [json!({}), json!(null), json!({"interaction_handle":false})] {
        let server = Server::new(|origin| {
            vec![
                discovery(origin),
                Reply::ok("POST /oauth2/default/v1/interact", body),
            ]
        });
        failure(
            client(&server)
                .initiate_auth_flow(AuthOptions::default())
                .await
                .unwrap_err(),
            FailureKind::Protocol,
            FailureStage::Interact,
            Some(200),
        );
        server.finish();
    }
    for status in [200, 404, 503] {
        let server = Server::new(|origin| {
            vec![
                discovery(origin),
                interact(),
                Reply::ok("POST /idp/idx/introspect", json!({"errorCode":"E0000004"}))
                    .status(status),
            ]
        });
        failure(
            client(&server)
                .initiate_auth_flow(AuthOptions::default())
                .await
                .unwrap_err(),
            if status == 200 {
                FailureKind::Protocol
            } else {
                FailureKind::Http
            },
            FailureStage::Introspect,
            Some(status),
        );
        server.finish();
    }
}

#[tokio::test]
async fn invalid_credentials_reconcile_and_invalidate_steps() {
    let server = Server::new(|origin| {
        let mut rejected = form(
            origin,
            "state-two",
            "identify",
            json!([{"name":"identifier","required":true}]),
        );
        rejected["messages"] = json!({"type":"array","value":[{"class":"ERROR","message":"Invalid credentials","i18n":{"key":"errors.E0000004"}}]});
        vec![
            discovery(origin),
            interact(),
            identify(origin, "state-one"),
            Reply::ok("POST /idp/idx/identify", rejected).status(401),
        ]
    });
    let app = client(&server);
    let mut flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let old = flow.steps()[0].id().clone();
    app.continue_auth_flow(
        &mut flow,
        &old,
        AuthInput::Form(FormInput::new(json!({"identifier":"fixture-password-secret"})).unwrap()),
    )
    .await
    .unwrap();
    assert_eq!(flow.status(), AuthStatus::Pending);
    assert!(flow.messages().is_some());
    assert_ne!(flow.steps()[0].id(), &old);
    assert!(matches!(
        app.continue_auth_flow(&mut flow, &old, AuthInput::Form(FormInput::empty()))
            .await,
        Err(OktaError::StaleStep)
    ));
    server.finish();
}

#[tokio::test]
async fn unusable_submission_stays_uncertain_until_reconciled() {
    let server = Server::new(|origin| {
        vec![
            discovery(origin),
            interact(),
            identify(origin, "state-one"),
            Reply::ok("POST /idp/idx/identify", json!({})).status(503),
            Reply::ok("POST /idp/idx/introspect", json!({})).status(502),
            identify(origin, "state-two"),
        ]
    });
    let app = client(&server);
    let mut flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let old = flow.steps()[0].id().clone();
    failure(
        app.continue_auth_flow(&mut flow, &old, AuthInput::Form(FormInput::empty()))
            .await
            .unwrap_err(),
        FailureKind::Http,
        FailureStage::Remediation,
        Some(503),
    );
    assert_eq!(flow.status(), AuthStatus::Uncertain);
    assert!(matches!(
        app.continue_auth_flow(&mut flow, &old, AuthInput::Form(FormInput::empty()))
            .await,
        Err(OktaError::Uncertain)
    ));
    failure(
        app.introspect_auth_flow(&mut flow).await.unwrap_err(),
        FailureKind::Http,
        FailureStage::Introspect,
        Some(502),
    );
    assert_eq!(flow.status(), AuthStatus::Uncertain);
    app.introspect_auth_flow(&mut flow).await.unwrap();
    assert_eq!(flow.status(), AuthStatus::Pending);
    assert_ne!(flow.steps()[0].id(), &old);
    let requests = server.finish();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.starts_with("POST /idp/idx/identify"))
            .count(),
        1
    );
}

#[tokio::test]
async fn resume_io_is_not_an_invalid_envelope() {
    let server = Server::new(|origin| {
        vec![
            discovery(origin),
            interact(),
            identify(origin, "state-one"),
            Reply::ok("POST /idp/idx/introspect", json!({})).status(503),
        ]
    });
    let app = client(&server);
    let flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let continuation = app.export_auth_flow(&flow).unwrap();
    let mut envelope: Json = serde_json::from_slice(continuation.as_bytes()).unwrap();
    // Force actual introspection rather than restoring the cached IDX response.
    envelope["stores"]["getIdxResponseStorage"] = json!({});
    for field in ["schema", "binding", "upstream_commit", "build"] {
        let mut bad = envelope.clone();
        bad[field] = if field == "schema" {
            json!(1)
        } else {
            json!("wrong")
        };
        let bad = AuthContinuation::from_bytes(&serde_json::to_vec(&bad).unwrap()).unwrap();
        assert!(matches!(
            app.resume_auth_flow(&bad).await,
            Err(OktaError::InvalidContinuation)
        ));
    }
    let valid = AuthContinuation::from_bytes(&serde_json::to_vec(&envelope).unwrap()).unwrap();
    failure(
        app.resume_auth_flow(&valid).await.unwrap_err(),
        FailureKind::Http,
        FailureStage::Introspect,
        Some(503),
    );
    assert!(matches!(
        AuthContinuation::from_bytes(&vec![0; 4 * 1024 * 1024 + 1]),
        Err(OktaError::InvalidContinuation)
    ));
    server.finish();
}

#[tokio::test]
async fn token_metadata_explicit_scopes_and_rotation() {
    for returned_scope in [Some("openid email"), None] {
        for rotate in [true, false] {
            let server = Server::new(|_| {
                let mut response = json!({"access_token":"fixture-access-secret","expires_in":3600,"token_type":"Bearer"});
                if let Some(scopes) = returned_scope {
                    response["scope"] = json!(scopes);
                }
                if rotate {
                    response["refresh_token"] = json!("fixture-rotated-secret");
                }
                vec![Reply::ok("POST /oauth2/default/v1/token", response)]
            });
            let app = client(&server);
            let before = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let cred = credential(&server);
            let token = app
                .refresh_tokens_with_scopes(&cred, &["openid".into(), "email".into()])
                .await
                .unwrap();
            assert_eq!(token.token_type(), Some("Bearer"));
            let expected_scopes =
                returned_scope.map(|_| vec![String::from("openid"), String::from("email")]);
            assert_eq!(token.scopes(), expected_scopes.as_deref());
            assert!(token.expires_at().unwrap() >= before + 3600);
            assert!(token.expires_at().unwrap() <= before + 3602);
            assert_eq!(
                token.refresh_credential().unwrap().expose_secret(),
                if rotate {
                    "fixture-rotated-secret"
                } else {
                    "fixture-refresh-secret"
                }
            );
            assert!(!format!("{token:?} {cred:?}").contains("secret"));
            let requests = server.finish();
            assert!(requests[0].contains("scope=openid%20email"));
            assert_eq!(requests.len(), 1);
        }
    }
}

#[tokio::test]
async fn default_scopes_binding_and_rejected_refresh() {
    let server = Server::new(|_| {
        vec![Reply::ok(
            "POST /oauth2/default/v1/token",
            json!({"error":"invalid_grant"}),
        )
        .status(400)]
    });
    let app = client(&server);
    let wrong = RefreshCredential::import(
        "https://other.invalid".into(),
        "fixture-client".into(),
        "secret".into(),
    );
    assert!(matches!(
        app.refresh_tokens_with_scopes(&wrong, &[]).await,
        Err(OktaError::WrongClient)
    ));
    let cred = credential(&server);
    let f = failure(
        app.refresh_tokens(&cred).await.unwrap_err(),
        FailureKind::Http,
        FailureStage::Refresh,
        Some(400),
    );
    assert_eq!(f.oauth_error.as_deref(), Some("invalid_grant"));
    assert_eq!(cred.expose_secret(), "fixture-refresh-secret");
    let requests = server.finish();
    assert!(requests[0].contains("scope=openid%20profile%20offline_access"));
    assert_eq!(requests.len(), 1);
}

fn validation_app(server: &Server) -> (PublicClientApplication, RefreshCredential) {
    let mut config = ClientConfig::new(
        format!("{}/oauth2/default", server.origin),
        "client",
        "http://localhost/callback",
    );
    config.allow_loopback_http = true;
    let cred = RefreshCredential::import(
        config.issuer.clone(),
        config.client_id.clone(),
        "fixture-refresh-secret".into(),
    );
    (PublicClientApplication::new(config).unwrap(), cred)
}
fn signed_fixture() -> (String, Json) {
    let fixture: Json =
        serde_json::from_str(include_str!("../../../tests/fixtures/signature.json")).unwrap();
    (
        fixture["token"].as_str().unwrap().into(),
        fixture["key"].clone(),
    )
}
fn token_reply(token: &str) -> Reply {
    Reply::ok(
        "POST /oauth2/default/v1/token",
        json!({"access_token":"fixture-access-secret","id_token":token,"refresh_token":"fixture-rotated-secret","expires_in":3600,"token_type":"Bearer"}),
    )
}
fn validation_discovery(origin: &str) -> Reply {
    Reply::ok(
        "GET /oauth2/default/.well-known/openid-configuration",
        json!({"issuer":"https://example.invalid","jwks_uri":format!("{origin}/oauth2/default/v1/keys")}),
    )
}
fn keys(key: &Json) -> Reply {
    Reply::ok(
        "GET /oauth2/default/v1/keys",
        json!({"keys":[{"kid":"offline-test","alg":"RS256","use":"sig","kty":"RSA","n":key["n"],"e":key["e"]}]}),
    )
}

#[tokio::test]
async fn nested_discovery_and_jwks_failures_keep_their_own_evidence() {
    let (token, _) = signed_fixture();
    for stage in [FailureStage::Discovery, FailureStage::Jwks] {
        let server = Server::new(|origin| {
            let mut replies = vec![token_reply(&token)];
            if stage == FailureStage::Discovery {
                replies.push(
                    Reply::ok(
                        "GET /oauth2/default/.well-known/openid-configuration",
                        json!({"error":"discovery_failure"}),
                    )
                    .status(503),
                );
            } else {
                replies.push(validation_discovery(origin));
                replies.push(
                    Reply::ok(
                        "GET /oauth2/default/v1/keys",
                        json!({"error":"jwks_failure"}),
                    )
                    .status(502),
                );
            }
            replies
        });
        let (app, cred) = validation_app(&server);
        failure(
            app.refresh_tokens(&cred).await.unwrap_err(),
            FailureKind::Http,
            stage,
            Some(if stage == FailureStage::Discovery {
                503
            } else {
                502
            }),
        );
        server.finish();
    }
}

#[tokio::test]
async fn claims_signatures_and_userinfo_subject_are_validation_failures() {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    let (token, key) = signed_fixture();
    for (claim, value) in [
        ("iss", json!("wrong")),
        ("aud", json!("wrong")),
        ("exp", json!(0)),
        ("iat", json!(4102444801u64)),
        ("sub", json!("tampered")),
    ] {
        let mut parts: Vec<String> = token.split('.').map(str::to_owned).collect();
        let mut claims: Json =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(&parts[1]).unwrap()).unwrap();
        claims[claim] = value;
        parts[1] = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).unwrap());
        let tampered = parts.join(".");
        let server = Server::new(|origin| {
            let mut replies = vec![token_reply(&tampered), validation_discovery(origin)];
            if claim == "sub" {
                replies.push(keys(&key));
            }
            replies
        });
        let (app, cred) = validation_app(&server);
        failure(
            app.refresh_tokens(&cred).await.unwrap_err(),
            FailureKind::TokenValidation,
            FailureStage::Refresh,
            Some(200),
        );
        assert_eq!(cred.expose_secret(), "fixture-refresh-secret");
        server.finish();
    }
    let server = Server::new(|origin| {
        vec![
            token_reply(&token),
            validation_discovery(origin),
            keys(&key),
            Reply::ok(
                "GET /oauth2/default/v1/userinfo",
                json!({"sub":"different-user"}),
            ),
        ]
    });
    let (app, cred) = validation_app(&server);
    let token = app.refresh_tokens(&cred).await.unwrap();
    failure(
        app.user_info(&token).await.unwrap_err(),
        FailureKind::TokenValidation,
        FailureStage::UserInfo,
        Some(200),
    );
    server.finish();
}

#[tokio::test]
async fn untrusted_jwks_and_malformed_success_are_not_outages() {
    let (token, _) = signed_fixture();
    let server = Server::new(|_| {
        vec![
            token_reply(&token),
            Reply::ok(
                "GET /oauth2/default/.well-known/openid-configuration",
                json!({"issuer":"https://example.invalid","jwks_uri":"https://untrusted.invalid/fixture-secret"}),
            ),
        ]
    });
    let (app, cred) = validation_app(&server);
    let f = failure(
        app.refresh_tokens(&cred).await.unwrap_err(),
        FailureKind::Trust,
        FailureStage::Jwks,
        None,
    );
    assert_eq!(f.outcome, RequestOutcome::NotSent);
    assert!(!format!("{f:?} {f}").contains("untrusted"));
    server.finish();
    for stage in [
        FailureStage::Refresh,
        FailureStage::Discovery,
        FailureStage::Jwks,
    ] {
        let server = Server::new(|origin| {
            let mut replies = if stage == FailureStage::Refresh {
                vec![]
            } else {
                vec![token_reply(&token)]
            };
            if stage == FailureStage::Jwks {
                replies.push(validation_discovery(origin));
            }
            replies.push(
                Reply::ok(
                    match stage {
                        FailureStage::Refresh => "POST /oauth2/default/v1/token",
                        FailureStage::Discovery => {
                            "GET /oauth2/default/.well-known/openid-configuration"
                        }
                        _ => "GET /oauth2/default/v1/keys",
                    },
                    json!({}),
                )
                .raw("text/html", b"<html>fixture-secret</html>".to_vec()),
            );
            replies
        });
        let (app, cred) = validation_app(&server);
        failure(
            app.refresh_tokens(&cred).await.unwrap_err(),
            FailureKind::Protocol,
            stage,
            Some(200),
        );
        server.finish();
    }
}

#[tokio::test]
async fn concurrent_flows_and_later_success_do_not_share_diagnostics() {
    let server = Server::new(|origin| {
        vec![
            discovery(origin),
            discovery(origin),
            Reply::ok("POST /oauth2/default/v1/interact", json!({"error":"first"})).status(404),
            Reply::ok(
                "POST /oauth2/default/v1/interact",
                json!({"error":"second"}),
            )
            .status(503),
            discovery(origin),
            interact(),
            identify(origin, "state"),
        ]
    });
    let app = client(&server);
    let (a, b) = tokio::join!(
        app.initiate_auth_flow(AuthOptions::default()),
        app.initiate_auth_flow(AuthOptions::default())
    );
    let mut codes = vec![];
    for result in [a, b] {
        let OktaError::OperationFailed(f) = result.unwrap_err() else {
            panic!("untyped failure")
        };
        assert_eq!(f.stage, FailureStage::Interact);
        assert_eq!(f.kind, FailureKind::Http);
        codes.push((f.http_status.unwrap(), f.oauth_error.unwrap()));
    }
    codes.sort();
    assert_eq!(codes, vec![(404, "first".into()), (503, "second".into())]);
    let mut flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    assert_eq!(flow.status(), AuthStatus::Pending);
    app.abandon_auth_flow(&mut flow).unwrap();
    assert_eq!(flow.status(), AuthStatus::Canceled);
    assert!(flow.steps().is_empty());
    assert!(flow.sdk.storage.is_empty());
    server.finish();
}

#[tokio::test]
async fn standalone_mutations_are_not_replayed_on_timeout_or_drop() {
    for revoke in [false, true] {
        let server = Server::new(|_| {
            let mut reply = Reply::ok(
                if revoke {
                    "POST /oauth2/default/v1/revoke"
                } else {
                    "POST /oauth2/default/v1/token"
                },
                json!({}),
            );
            reply.delay = Duration::from_millis(200);
            vec![reply]
        });
        let mut config = ClientConfig::new(
            format!("{}/oauth2/default", server.origin),
            "fixture-client",
            "http://localhost/callback",
        );
        config.allow_loopback_http = true;
        config.timeout = Duration::from_millis(30);
        let app = PublicClientApplication::new(config).unwrap();
        let cred = credential(&server);
        let error = if revoke {
            app.revoke_refresh_token(&cred).await.unwrap_err()
        } else {
            app.refresh_tokens(&cred).await.unwrap_err()
        };
        let f = failure(
            error,
            FailureKind::Transport,
            if revoke {
                FailureStage::Revoke
            } else {
                FailureStage::Refresh
            },
            None,
        );
        assert_eq!(f.outcome, RequestOutcome::Unknown);
        assert_eq!(server.finish().len(), 1);
    }
    let server = Server::new(|_| {
        let mut reply = Reply::ok("POST /oauth2/default/v1/token", json!({}));
        reply.delay = Duration::from_millis(200);
        vec![reply]
    });
    let app = client(&server);
    let cred = credential(&server);
    assert!(
        tokio::time::timeout(Duration::from_millis(30), app.refresh_tokens(&cred))
            .await
            .is_err()
    );
    assert_eq!(cred.expose_secret(), "fixture-refresh-secret");
    assert_eq!(server.finish().len(), 1);
}

#[tokio::test]
async fn dropped_code_exchange_never_replays_the_code() {
    let server = Server::new(|origin| {
        let mut token = Reply::ok("POST /oauth2/default/v1/token", json!({}));
        token.delay = Duration::from_millis(200);
        vec![
            discovery(origin),
            interact(),
            identify(origin, "state"),
            token,
        ]
    });
    let app = client(&server);
    let flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let continuation = app.export_auth_flow(&flow).unwrap();
    let state = flow.snapshot.0["meta"]["state"].as_str().unwrap();
    let callback =
        format!("http://localhost/callback?state={state}&interaction_code=fixture-one-use-code");
    assert!(tokio::time::timeout(
        Duration::from_millis(30),
        app.handle_interaction_code_callback(&continuation, &callback)
    )
    .await
    .is_err());
    let requests = server.finish();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.starts_with("POST /oauth2/default/v1/token"))
            .count(),
        1
    );
}

#[tokio::test]
async fn unexpected_dpop_challenge_does_not_retry_a_refresh() {
    let server = Server::new(|_| {
        vec![Reply::ok(
            "POST /oauth2/default/v1/token",
            json!({"error":"use_dpop_nonce","error_description":"fixture"}),
        )
        .status(400)
        .header("DPoP-Nonce", "fixture-nonce")]
    });
    let app = client(&server);
    let f = failure(
        app.refresh_tokens(&credential(&server)).await.unwrap_err(),
        FailureKind::Http,
        FailureStage::Refresh,
        Some(400),
    );
    assert_eq!(f.oauth_error.as_deref(), Some("use_dpop_nonce"));
    assert_eq!(server.finish().len(), 1);
}

#[tokio::test]
async fn callback_rejection_is_local_and_redacted() {
    let server =
        Server::new(|origin| vec![discovery(origin), interact(), identify(origin, "state")]);
    let app = client(&server);
    let mut flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let continuation = app.export_auth_flow(&flow).unwrap();
    for callback in [
        "not a URL",
        "https://secret.invalid/callback?fixture-secret",
        "http://localhost/callback?state=wrong&interaction_code=fixture-secret",
    ] {
        let f = failure(
            app.handle_interaction_code_callback(&continuation, callback)
                .await
                .unwrap_err(),
            FailureKind::Protocol,
            FailureStage::Callback,
            None,
        );
        assert_eq!(f.outcome, RequestOutcome::NotSent);
        assert!(!format!("{f:?} {f}").contains("secret"));
    }
    let f = failure(
        app.handle_email_verify_callback(&mut flow, "?state=wrong&otp=fixture-secret")
            .await
            .unwrap_err(),
        FailureKind::Protocol,
        FailureStage::Callback,
        None,
    );
    assert!(!format!("{f:?} {f}").contains("secret"));
    server.finish();
}

#[tokio::test]
async fn simultaneous_live_flows_keep_cookies_fields_and_steps_separate() {
    let server = Server::new(|origin| {
        vec![
            discovery(origin),
            interact(),
            identify(origin, "state-a").header("Set-Cookie", "session=cookie-a; Path=/"),
            discovery(origin),
            interact(),
            identify(origin, "state-b").header("Set-Cookie", "session=cookie-b; Path=/"),
            Reply::ok(
                "POST /idp/idx/identify",
                form(origin, "next-a", "identify", json!([{"name":"identifier"}])),
            ),
            Reply::ok(
                "POST /idp/idx/identify",
                form(origin, "next-b", "identify", json!([{"name":"identifier"}])),
            ),
        ]
    });
    let app = client(&server);
    let mut a = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let mut b = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let step_a = a.steps()[0].id().clone();
    let step_b = b.steps()[0].id().clone();
    assert!(matches!(
        app.continue_auth_flow(&mut a, &step_b, AuthInput::Form(FormInput::empty()))
            .await,
        Err(OktaError::StaleStep)
    ));
    for (flow, step, identifier) in [(&mut a, &step_a, "user-a"), (&mut b, &step_b, "user-b")] {
        app.continue_auth_flow(
            flow,
            step,
            AuthInput::Form(FormInput::new(json!({"identifier":identifier})).unwrap()),
        )
        .await
        .unwrap();
    }
    let requests = server.finish();
    for (index, cookie, other, state, identifier) in [
        (6, "cookie-a", "cookie-b", "state-a", "user-a"),
        (7, "cookie-b", "cookie-a", "state-b", "user-b"),
    ] {
        assert!(requests[index].contains(cookie));
        assert!(!requests[index].contains(other));
        let body: Json =
            serde_json::from_str(requests[index].split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(body["stateHandle"], state);
        assert_eq!(body["identifier"], identifier);
    }
}

#[tokio::test]
async fn callback_oauth_code_without_description_is_preserved() {
    let server =
        Server::new(|origin| vec![discovery(origin), interact(), identify(origin, "state")]);
    let app = client(&server);
    let flow = app
        .initiate_auth_flow(AuthOptions::default())
        .await
        .unwrap();
    let continuation = app.export_auth_flow(&flow).unwrap();
    let f = failure(
        app.handle_interaction_code_callback(
            &continuation,
            "http://localhost/callback?error=access_denied",
        )
        .await
        .unwrap_err(),
        FailureKind::Protocol,
        FailureStage::Callback,
        None,
    );
    assert_eq!(f.oauth_error.as_deref(), Some("access_denied"));
    server.finish();
}

#[tokio::test]
async fn empty_revocation_success_and_http_rejection_are_distinct() {
    for status in [200, 204, 401, 503] {
        let server = Server::new(|_| {
            vec![Reply::ok("POST /oauth2/default/v1/revoke", json!({}))
                .status(status)
                .raw("application/json", vec![])]
        });
        let app = client(&server);
        let result = app.revoke_refresh_token(&credential(&server)).await;
        if status < 300 {
            result.unwrap();
        } else {
            failure(
                result.unwrap_err(),
                FailureKind::Http,
                FailureStage::Revoke,
                Some(status),
            );
        }
        assert_eq!(server.finish().len(), 1);
    }
}

#[tokio::test]
async fn incompatible_pkce_metadata_is_a_discovery_protocol_failure() {
    let server = Server::new(|origin| {
        vec![Reply::ok(
            "GET /oauth2/default/.well-known/openid-configuration",
            json!({"issuer":format!("{origin}/oauth2/default"), "code_challenge_methods_supported":["plain"]}),
        )]
    });
    failure(
        client(&server)
            .initiate_auth_flow(AuthOptions::default())
            .await
            .unwrap_err(),
        FailureKind::Protocol,
        FailureStage::Discovery,
        Some(200),
    );
    assert_eq!(server.finish().len(), 1);
}
