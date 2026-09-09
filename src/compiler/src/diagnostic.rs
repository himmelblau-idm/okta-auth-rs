//! Reviewed native annotations. No generated function numbers or error-message matching.
use crate::{frontend::Graph, Result};

pub struct Hook {
    pub module: &'static str,
    pub symbol: &'static str,
    pub params: usize,
    pub stage: Option<&'static str>,
    pub validation: bool,
    pub finish: &'static str,
}

pub const HOOKS: &[Hook] = &[
    Hook {
        module: "lib/oidc/util/prepareTokenParams.ts",
        symbol: "validateCodeChallengeMethod",
        params: 2,
        stage: Some("Discovery"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/endpoints/well-known.ts",
        symbol: "getWellKnown",
        params: 2,
        stage: Some("Discovery"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/endpoints/well-known.ts",
        symbol: "getKey",
        params: 3,
        stage: Some("Jwks"),
        validation: true,
        finish: "normal",
    },
    Hook {
        module: "lib/idx/interact.ts",
        symbol: "interact",
        params: 2,
        stage: Some("Interact"),
        validation: false,
        finish: "interact",
    },
    Hook {
        module: "lib/idx/introspect.ts",
        symbol: "introspect",
        params: 2,
        stage: Some("Introspect"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/idx/idxState/v1/generateIdxAction.ts",
        symbol: "generateDirectFetch",
        params: 2,
        stage: Some("Remediation"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/exchangeCodeForTokens.ts",
        symbol: "exchangeCodeForTokens",
        params: 3,
        stage: Some("TokenExchange"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/endpoints/token.ts",
        symbol: "postToTokenEndpoint",
        params: 3,
        stage: Some("TokenExchange"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/renewTokensWithRefresh.ts",
        symbol: "renewTokensWithRefresh",
        params: 3,
        stage: Some("Refresh"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/endpoints/token.ts",
        symbol: "postRefreshToken",
        params: 3,
        stage: Some("Refresh"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/revokeToken.ts",
        symbol: "revokeToken",
        params: 2,
        stage: Some("Revoke"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/getUserInfo.ts",
        symbol: "getUserInfo",
        params: 3,
        stage: Some("UserInfo"),
        validation: true,
        finish: "normal",
    },
    Hook {
        module: "lib/idx/handleInteractionCodeRedirect.ts",
        symbol: "handleInteractionCodeRedirect",
        params: 2,
        stage: Some("Callback"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/idx/emailVerify.ts",
        symbol: "handleEmailVerifyCallback",
        params: 2,
        stage: Some("Callback"),
        validation: false,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/verifyToken.ts",
        symbol: "verifyToken",
        params: 3,
        stage: None,
        validation: true,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/decodeToken.ts",
        symbol: "decodeToken",
        params: 1,
        stage: None,
        validation: true,
        finish: "normal",
    },
    Hook {
        module: "lib/oidc/util/validateClaims.ts",
        symbol: "validateClaims",
        params: 3,
        stage: None,
        validation: true,
        finish: "normal",
    },
    Hook {
        module: "lib/http/request.ts",
        symbol: "formatError",
        params: 2,
        stage: None,
        validation: false,
        finish: "transfer",
    },
    Hook {
        module: "lib/oidc/handleOAuthResponse.ts",
        symbol: "handleOAuthResponse",
        params: 4,
        stage: None,
        validation: false,
        finish: "scopes",
    },
    Hook {
        module: "lib/idx/idxState/index.ts",
        symbol: "makeIdxState",
        params: 4,
        stage: None,
        validation: false,
        finish: "idx",
    },
];

pub fn validate(graph: &Graph) -> Result<()> {
    let hashes: std::collections::BTreeMap<String, String> =
        serde_json::from_str(include_str!("diagnostic-sources.json")).map_err(|e| e.to_string())?;
    for hook in HOOKS {
        if !graph
            .selected
            .contains(&(hook.module.into(), hook.symbol.into()))
        {
            continue;
        }
        let module = graph
            .modules
            .get(hook.module)
            .ok_or("missing annotated module")?;
        if hashes.get(hook.module) != Some(&crate::hash(module.source.as_bytes())) {
            return Err(format!("{} diagnostic source hash changed; review annotations before updating diagnostic-sources.json", hook.module));
        }
        let mut node = module
            .definitions
            .get(hook.symbol)
            .ok_or("missing annotated declaration")?;
        if node["type"] == "VariableDeclarator" {
            node = &node["init"];
        }
        if !matches!(
            node["type"].as_str(),
            Some("FunctionDeclaration" | "FunctionExpression" | "ArrowFunctionExpression")
        ) || node["params"].as_array().map(Vec::len) != Some(hook.params)
        {
            return Err(format!(
                "{}:{} diagnostic annotation requires source review",
                hook.module, hook.symbol
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn annotated_graph() -> Graph {
        let source =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../vendor/okta-auth-js");
        let mut graph = Graph::new(&source).unwrap();
        graph.load("lib/idx/interact.ts").unwrap();
        graph
            .selected
            .insert(("lib/idx/interact.ts".into(), "interact".into()));
        graph
    }
    #[test]
    fn reviewed_annotation_and_source_shape() {
        let mut graph = annotated_graph();
        validate(&graph).unwrap();
        graph
            .modules
            .get_mut("lib/idx/interact.ts")
            .unwrap()
            .definitions
            .get_mut("interact")
            .unwrap()["params"] = serde_json::json!([]);
        assert!(validate(&graph)
            .unwrap_err()
            .contains("requires source review"));
    }
    #[test]
    fn changed_annotation_source_requires_explicit_review() {
        let mut graph = annotated_graph();
        graph
            .modules
            .get_mut("lib/idx/interact.ts")
            .unwrap()
            .source
            .push_str("// changed");
        assert!(validate(&graph)
            .unwrap_err()
            .contains("source hash changed"));
    }
    #[test]
    fn annotations_are_unique_and_do_not_match_unrelated_symbols() {
        let mut names = std::collections::BTreeSet::new();
        for hook in HOOKS {
            assert!(names.insert((hook.module, hook.symbol)));
        }
        assert!(!HOOKS
            .iter()
            .any(|h| h.module == "unrelated.ts" && h.symbol == "interact"));
    }
}
