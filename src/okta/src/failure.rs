//! Safe, structured evidence about a failed protocol operation.
use std::{fmt, time::Duration};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FailureKind {
    Transport,
    Http,
    Protocol,
    TokenValidation,
    Trust,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FailureStage {
    Discovery,
    Interact,
    Introspect,
    Remediation,
    TokenExchange,
    Refresh,
    UserInfo,
    Jwks,
    Revoke,
    Callback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestOutcome {
    NotSent,
    ResponseReceived,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct OperationFailure {
    pub kind: FailureKind,
    pub stage: FailureStage,
    pub http_status: Option<u16>,
    pub oauth_error: Option<String>,
    pub okta_error_code: Option<String>,
    pub retry_after: Option<Duration>,
    pub outcome: RequestOutcome,
}

impl OperationFailure {
    pub(crate) fn local(kind: FailureKind, stage: FailureStage) -> Self {
        Self {
            kind,
            stage,
            http_status: None,
            oauth_error: None,
            okta_error_code: None,
            retry_after: None,
            outcome: RequestOutcome::NotSent,
        }
    }
}

impl fmt::Display for OperationFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Public fields are caller-constructible; never render their free-form strings.
        write!(
            f,
            "{:?} failed ({:?}, {:?}",
            self.stage, self.kind, self.outcome
        )?;
        if let Some(status) = self.http_status {
            write!(f, ", HTTP {status}")?;
        }
        f.write_str(")")
    }
}

pub(crate) fn safe_code(value: Option<&str>) -> Option<String> {
    value
        .filter(|s| {
            !s.is_empty()
                && s.len() <= 128
                && s.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"_-.".contains(&b))
        })
        .map(str::to_owned)
}
