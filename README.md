# okta-auth-rs

Native Okta Identity Engine authentication for Rust, generated from a pinned version of [Okta Auth JS](https://github.com/okta/okta-auth-js).

`okta-auth-rs` lets a Rust application drive Okta's Interaction Code flow directly. It is intended for caller-owned Linux authentication interfaces such as Himmelblau: the application receives each policy-driven remediation step, renders the required fields, submits the user's response, and receives validated OIDC tokens when authentication succeeds.

The runtime is native Rust. It does not bundle Node.js, execute JavaScript, call an AI service, or generate code at application startup.

## What it supports

- Identity Engine (IDX) Interaction Code authentication
- Policy-driven MFA, enrollment, recovery, unlock, and password-change forms
- Polling authenticators such as push verification
- Email verification callbacks
- Caller-managed redirect and external steps
- PKCE interaction-code exchange
- ID-token claim and RS256 signature verification
- Access-token refresh, revocation, and userinfo
- Serializable authentication continuations for process handoff
- A pluggable WebAuthn authenticator interface
- Optional USB CTAP2 security-key support
- Optional HSM or TPM sealing for refresh credentials and continuations

Classic Authentication API flows, DPoP, browser popup/iframe behavior, and Auth JS's browser session-management services are outside the current translation profile.

## Requirements

- Rust 1.85 or newer
- An Okta Identity Engine organization
- A public OIDC application and its client ID
- The Interaction Code grant enabled for the organization, application, and any custom authorization server being used
- A registered sign-in redirect URI

Okta documents the tenant-side setup in its [Interaction Code guide](https://developer.okta.com/docs/guides/implement-grant-type/interactioncode/main/). Interaction Code uses PKCE and can be enabled for either the org authorization server or a custom authorization server.

For a native application, configure at least:

1. **OIDC application type:** Native Application.
2. **Grant types:** Interaction Code. Enable Refresh Token if the application needs long-lived sessions.
3. **Sign-in redirect URI:** The exact URI the caller will later give to `handle_interaction_code_callback`.
4. **Assignments and policies:** Assign the intended users and configure the authenticators the Linux client must support.

The issuer is normally either the Okta organization URL:

```text
https://example.okta.com
```

or a custom authorization server:

```text
https://example.okta.com/oauth2/default
```

## Add the library

The merged public API is not yet published to crates.io. Add it as a path dependency from a checkout of this repository:

```toml
[dependencies]
okta = { package = "okta-auth-rs", path = "../okta-auth-rs/src/okta" }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The repository's `rust-toolchain.toml` selects Rust 1.85 automatically. Build the library with:

```sh
cargo build --locked -p okta-auth-rs
```

## Try the terminal client

The included terminal client displays the complete schema for every step and accepts form values as a JSON object:

```sh
cargo run --locked -p okta-auth-rs --example okta-mfa -- \
  https://example.okta.com/oauth2/default \
  YOUR_CLIENT_ID \
  http://127.0.0.1:8765/callback
```

For example, an identify step may ask for input resembling:

```json
{"identifier":"user@example.com"}
```

Always use the displayed schema as the source of truth. The required fields and available authenticators depend on the tenant's current policies and the user's enrollment state.

The example intentionally does not open a browser or run a callback listener. If Okta returns a redirect or external step, it prints the step and the size of the continuation that a production caller would protect and persist.

## Drive an authentication flow

Create one `PublicClientApplication` for an issuer, client ID, redirect URI, and scope set:

```rust
use okta::{AuthOptions, ClientConfig, PublicClientApplication};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let mut config = ClientConfig::new(
    "https://example.okta.com/oauth2/default",
    "YOUR_CLIENT_ID",
    "http://127.0.0.1:8765/callback",
);
config.scopes = vec![
    "openid".into(),
    "profile".into(),
    "offline_access".into(),
];

let client = PublicClientApplication::new(config)?;
let mut flow = client.initiate_auth_flow(AuthOptions::default()).await?;
# Ok(())
# }
```

`AuthOptions` can provide a `login_hint`, recovery token, or activation token. Recovery and activation values use `Zeroizing<String>` so their memory is cleared when dropped.

While the flow is pending, present `flow.steps()` to the user. Each `AuthStep` contains:

- `id()`: an opaque identifier valid only for this exact flow response
- `name()`: the upstream remediation name
- `kind`: `Form`, `Poll`, `Redirect`, or `External`
- `schema()`: the complete upstream-derived JSON schema, including nested inputs, options, and messages

A caller can submit a selected step with a helper like this:

```rust
use okta::{
    AuthFlow, AuthInput, FormInput, OktaError, PublicClientApplication, StepKind,
};
use serde_json::Value;

async fn submit_step(
    client: &PublicClientApplication,
    flow: &mut AuthFlow,
    selected_index: usize,
    form_values: Option<Value>,
) -> Result<(), OktaError> {
    let step = flow
        .steps()
        .get(selected_index)
        .ok_or(OktaError::StaleStep)?
        .clone();

    let input = match step.kind {
        StepKind::Form => AuthInput::Form(FormInput::new(
            form_values.ok_or(OktaError::ReservedInput)?,
        )?),
        StepKind::Poll => AuthInput::Poll,
        StepKind::Redirect | StepKind::External => {
            return Err(OktaError::UnsupportedResult);
        }
    };

    client.continue_auth_flow(flow, step.id(), input).await
}
```

Form values may contain nested JSON objects and arrays. Do not copy immutable protocol fields such as `stateHandle` out of the schema and submit them yourself; the generated action retains those values. `FormInput` rejects SDK configuration and protocol fields to prevent them from overriding trusted state.

After each operation, inspect `flow.status()`:

| Status | Caller action |
| --- | --- |
| `Pending` | Render the new messages and steps. |
| `Success` | Read the validated token set from `flow.tokens()`. |
| `Terminal` | Display the terminal messages and stop. |
| `Canceled` | Discard the flow. |
| `Uncertain` | Call `introspect_auth_flow`; do not repeat the submitted credential. |

Step IDs become stale whenever the server returns a new flow response. Always render and select from the latest `flow.steps()` value.

Server messages and additional context are available through `flow.messages()` and `flow.context()`.

## Handle cancellation and uncertain requests

`cancel_auth_flow` invokes the current Okta cancellation remediation when one is available. `abandon_auth_flow` clears local state without sending another request.

If a future performing a mutation is dropped, or its transport result is otherwise unknown, the flow reports `AuthStatus::Uncertain`. Reconcile it without replaying credentials:

```rust
if flow.status() == okta::AuthStatus::Uncertain {
    client.introspect_auth_flow(&mut flow).await?;
}
```

## Save and resume a flow

An unfinished flow can be serialized for handoff to another process:

```rust
let continuation = client.export_auth_flow(&flow)?;
save_sensitive_bytes(continuation.as_bytes())?;
```

Restore it using the same client configuration:

```rust
use okta::AuthContinuation;

let continuation = AuthContinuation::from_bytes(&load_sensitive_bytes()?)?;
let mut flow = client.resume_auth_flow(&continuation).await?;
```

`AuthContinuation` contains transaction state, the PKCE verifier, and cookies. Treat it as a credential. Its `Debug` output is redacted and its memory is zeroized on drop, but callers remain responsible for encryption, access control, and cleanup of persisted copies.

Continuations are limited to 4 MiB and bound to:

- the issuer, client ID, redirect URI, scopes, and trusted-origin configuration
- the exact generated Auth JS commit

A continuation fails closed if either binding changes or its serialized data is invalid.

## Complete a redirect callback

Some Okta policies may return a redirect or another external action. Save the continuation before handing control to the external component. When the registered callback arrives, pass its complete URL back with that continuation:

```rust
let continuation = client.export_auth_flow(&flow)?;

// Persist the continuation, complete the caller-managed handoff, then restore it.
let token = client
    .handle_interaction_code_callback(&continuation, callback_url)
    .await?;
```

The callback's scheme, host, port, and path must match the configured redirect URI. The translated code validates OAuth state and exchanges the interaction code using the saved PKCE verifier.

For email verification, retain or resume the associated flow and call:

```rust
client
    .handle_email_verify_callback(&mut flow, callback_query_or_fragment)
    .await?;
```

## Use and revoke tokens

On success, `UserToken` exposes the access token, ID token, verified ID-token claims, and an optional refresh credential:

```rust
let token = flow.tokens().ok_or("missing token set")?;

if let Some(claims) = token.claims() {
    println!("subject: {}", claims["sub"]);
}

let profile = client.user_info(token).await?;
client.revoke_access_token(token).await?;
```

ID-token claims are exposed only after translated claim validation and native RS256 signature verification succeed. `user_info` also requires the returned subject to match the validated ID token.

Use the refresh credential to obtain a new token set or revoke the session:

```rust
if let Some(refresh) = token.refresh_credential() {
    let refreshed = client.refresh_tokens(refresh).await?;
    // Replace the old token set and securely persist any rotated refresh credential.
    drop(refreshed);

    client.revoke_refresh_token(refresh).await?;
}
```

`UserToken`, `RefreshCredential`, and `AuthContinuation` redact secrets from `Debug`; sensitive strings and buffers are zeroized when dropped. Avoid logging values obtained from their explicit accessors.

## WebAuthn and security keys

`okta::webauthn::WebAuthnAdapter` converts IDX WebAuthn challenge data into authenticator requests and converts assertions or attestations back into the wire shape expected by the translated Okta logic.

Implement `WebAuthnAuthenticator` to connect the adapter to a platform authenticator, broker, or other hardware API. The trait receives the verified origin and generated request JSON and returns raw credential bytes.

For direct USB CTAP2 support, enable `webauthn-usb`:

```toml
okta = {
    package = "okta-auth-rs",
    path = "../okta-auth-rs/src/okta",
    features = ["webauthn-usb"],
}
```

```rust
use okta::webauthn::WebAuthnAdapter;
use okta::webauthn_usb::UsbAuthenticator;
use std::time::Duration;

let adapter = WebAuthnAdapter::new()?;
let mut authenticator = UsbAuthenticator::new(Duration::from_secs(30))?;

let assertion = adapter.get(
    &mut authenticator,
    "https://example.okta.com",
    challenge_data,
    enrollments,
)?;
```

On Linux, this feature requires the `libudev` development package. A CTAP PIN can be supplied as `Zeroizing<String>` with `UsbAuthenticator::set_pin`.

The caller extracts `challenge_data` and `enrollments` from the current step schema and submits the returned assertion or attestation in the form field named by that schema.

## Protect credentials with an HSM or TPM

Enable `hsm` to seal refresh credentials and authentication continuations with a caller-owned `kanidm-hsm-crypto` provider and `StorageKey`:

```toml
okta = {
    package = "okta-auth-rs",
    path = "../okta-auth-rs/src/okta",
    features = ["hsm"],
}
```

The `okta::hsm` module provides:

- `seal_refresh_credential` and `unseal_refresh_credential`
- `seal_auth_continuation` and `unseal_auth_continuation`

The application owns provider initialization, key lifecycle, and persistence of the resulting `SealedData`. Enable `tpm` in addition to `hsm` to select the system TPM backend; that feature requires the TPM2-TSS development libraries.

## Network and state boundaries

The native HTTP layer:

- uses `reqwest` with rustls
- accepts HTTPS issuers by default
- allows HTTP only for an explicitly enabled loopback test origin
- follows no redirects automatically
- restricts requests to the issuer and explicitly configured additional trusted origins
- limits response bodies to 4 MiB
- keeps cookies isolated per flow/runtime

Add another origin only when an Okta deployment intentionally returns an endpoint on that origin:

```rust
config.additional_trusted_origins = vec!["https://login.example.com".into()];
```

Review this list as a security boundary. Do not populate it from an untrusted server response.

## Error handling

Public errors are intentionally coarse and do not include credentials or server response bodies.

| Error | Meaning |
| --- | --- |
| `Configuration` | An issuer, redirect URI, client ID, trusted origin, or transport setting is invalid. |
| `StaleStep` | The selected step came from an older response or another flow. |
| `WrongClient` | A flow or refresh credential belongs to another client configuration. |
| `Uncertain` | A mutating request may have completed; introspection is required. |
| `ReservedInput` | Form data attempted to override SDK or protocol state, or used the wrong input kind. |
| `Authentication` | Okta, transport, callback, or token validation rejected the operation. |
| `UnsupportedResult` | The response cannot be represented by the current translation profile. |
| `InvalidContinuation` | Continuation data is malformed, oversized, or bound to another build/configuration. |

## Build and test the repository

The root Makefile provides maintenance shortcuts. Run `make help` to list
all targets. Rust commands require Cargo and the pinned Rust 1.85 toolchain with
rustfmt; the underlying Cargo commands can also be used directly.

```sh
make                     # Development build of the workspace
make release             # Optimized workspace build
make check-generated     # Verify generated code and provenance
make test                # Workspace tests with default features
make test-hsm            # Library tests with HSM support
```

Run the normal Rust 1.85 checks with:

```sh
cargo run --locked -p okta-translator -- check
cargo test --locked --workspace --all-targets
cargo test --locked -p okta-auth-rs --features hsm
```

The optional upstream/Rust parity suite needs Python 3, Node.js, and npm. Node is only a test oracle; it is never used by the translator or Rust runtime. Run `make parity` to install the locked oracle dependencies, build the example, and run the suite, or use:

```sh
npm ci --prefix tests/oracle
cargo build --locked -p okta-auth-rs --example parity
python3 tests/parity.py
```

Check optional platform integrations with the corresponding system development libraries installed:

```sh
cargo check --locked -p okta-auth-rs --features webauthn-usb
cargo check --locked -p okta-auth-rs --features tpm
```

Use `make fmt` to format Rust source, `make fmt-check` to check formatting, and
`make clean` to remove Cargo build artifacts.

To publish the library, prepare and commit the intended version and source changes,
configure Cargo registry credentials, then run:

```sh
make publish-dry-run
make publish
```

These wrap `cargo publish --locked -p okta-auth-rs` with and without `--dry-run`.
The translator is private and is not published. Version changes, commits, and tags
remain manual; publishing does not automatically regenerate code or run the test
suites.

## How translation works

The repository vendors Okta Auth JS at a pinned commit. `translation.json` selects the IDX and OIDC entry points required by this library.

```text
vendored TypeScript
        │
        ▼
 Oxc parser and resolver
        │
        ▼
selected dependency graph ──► validated native substitutions
        │
        ▼
 generated Rust functions
        │
        ▼
native support runtime ──► typed `okta` API
```

The compiler resolves imports and lexical bindings, computes the selected dependency closure, checks every syntax and native operation against the supported profile, and emits Rust. New syntax, module effects, ambiguous exports, native APIs, namespace escapes, or side-effectful omitted initializers cause generation to fail instead of being silently ignored.

The generated files are:

- [`src/okta/src/generated/mod.rs`](src/okta/src/generated/mod.rs)
- [`src/okta/src/generated/provenance.json`](src/okta/src/generated/provenance.json)

The provenance file records selected declarations, source hashes, syntax inventory, native-profile changes, and hashes of the compiler, manifest, runtime, and generated output.

## Update the pinned Auth JS source

An upstream update is a reviewed compiler change:

1. Replace `vendor/okta-auth-js` with the exact upstream source tree.
2. Calculate the new deterministic tree hash:

   ```sh
   make source-hash
   ```

3. Update the version, commit, and `source_tree_sha256` in `translation.json` and `translation.bootstrap.json`.
4. Inspect the selected graph and native substitutions:

   ```sh
   make inspect
   ```

5. Implement support for any newly rejected syntax, native operation, or module behavior.
6. Regenerate and verify exact reproducibility:

   ```sh
   make generate
   make check-generated
   ```

7. Run `make test`, `make test-hsm`, and `make parity`, plus the optional
   platform checks above where supported.

Source replacement and edits to both translation manifests remain manual. The
Makefile provides helpers for reviewing and validating the update.

Translation depends only on the checked-in compiler, manifest, and upstream source. A changed upstream requirement that the compiler cannot translate is a build failure requiring an explicit compiler or runtime update.

## License

Original Rust code is licensed under [LGPL-3.0-or-later](LICENSE). Vendored and translated Okta Auth JS material remains under Apache-2.0; the upstream [LICENSE](vendor/okta-auth-js/LICENSE) and [THIRD-PARTY-NOTICES](vendor/okta-auth-js/THIRD-PARTY-NOTICES) are preserved. Optional dependencies retain their respective licenses.
