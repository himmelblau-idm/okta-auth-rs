//! Partial evaluation for the fixed native public-client configuration.
//! No authentication response, input, URL, or protocol decision is synthesized.
use crate::frontend::{kind, name};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn literal(value: bool, source: &Value) -> Value {
    json!({"type":"Literal","value":value,"start":source["start"],"end":source["end"]})
}
fn boolean(v: &Value) -> Option<bool> {
    if kind(v) == "Literal" {
        return v["value"].as_bool();
    }
    if kind(v) == "LogicalExpression" {
        let a = boolean(&v["left"]);
        let b = boolean(&v["right"]);
        return match (v["operator"].as_str(), a, b) {
            (Some("&&"), Some(false), _) | (Some("&&"), _, Some(false)) => Some(false),
            (Some("||"), Some(true), _) | (Some("||"), _, Some(true)) => Some(true),
            (Some("&&"), Some(true), b) | (Some("||"), Some(false), b) => b,
            _ => None,
        };
    }
    None
}

pub fn specialize(
    v: &mut Value,
    references: &BTreeSet<u32>,
    browser_references: &BTreeSet<u32>,
    globals: &BTreeMap<u32, String>,
    allow_dpop: bool,
    changes: &mut Vec<Value>,
) {
    let original = v.clone();
    let start = v["start"].as_u64().map(|n| n as u32);
    let is_reference = start.is_some_and(|s| references.contains(&s));
    let native_flag = if allow_dpop && kind(v) == "Identifier" && is_reference && name(v) == "dpop"
    {
        Some(false)
    } else if allow_dpop
        && kind(v) == "MemberExpression"
        && v["computed"] != true
        && name(&v["property"]) == "dpop"
        && kind(&v["object"]) == "MemberExpression"
        && name(&v["object"]["property"]) == "options"
    {
        Some(false)
    } else {
        None
    };
    if let Some(value) = native_flag {
        changes.push(json!({"start":v["start"],"reason":"native public-client dpop=false"}));
        *v = literal(value, v);
        return;
    }
    // Calls to the imported environment detector are the only folded calls.
    if kind(v) == "CallExpression"
        && kind(&v["callee"]) == "Identifier"
        && v["callee"]["start"]
            .as_u64()
            .is_some_and(|s| browser_references.contains(&(s as u32)))
        && v["arguments"].as_array().is_some_and(Vec::is_empty)
    {
        changes
            .push(json!({"start":v["start"],"reason":"native environment has no document/window"}));
        *v = literal(false, v);
        return;
    }
    if kind(v) == "UnaryExpression"
        && v["operator"] == "typeof"
        && kind(&v["argument"]) == "Identifier"
        && v["argument"]["start"]
            .as_u64()
            .and_then(|n| globals.get(&(n as u32)))
            .is_some_and(|n| n == "window" || n == "document")
    {
        *v = json!({"type":"Literal","value":"undefined","start":v["start"],"end":v["end"]});
        return;
    }
    if matches!(kind(v), "ObjectPattern" | "ArrayPattern") {
        return;
    }
    if let Some(values) = v.as_array_mut() {
        for value in values {
            specialize(
                value,
                references,
                browser_references,
                globals,
                allow_dpop,
                changes,
            );
        }
    }
    if let Some(object) = v.as_object_mut() {
        let computed = object.get("computed") == Some(&Value::Bool(true));
        for (key, value) in object {
            if matches!(
                key.as_str(),
                "id" | "params"
                    | "typeAnnotation"
                    | "returnType"
                    | "typeParameters"
                    | "typeArguments"
            ) || (!computed && matches!(key.as_str(), "key" | "property"))
            {
                continue;
            }
            specialize(
                value,
                references,
                browser_references,
                globals,
                allow_dpop,
                changes,
            );
        }
    }
    let replacement=match kind(v){
        "IfStatement"=>boolean(&v["test"]).map(|b|{
            let branch=if b{v["consequent"].clone()}else if v["alternate"].is_null(){json!({"type":"EmptyStatement"})}else{v["alternate"].clone()};
            // Keep evaluation (and exceptions) of a nonliteral test.
            if kind(&v["test"])=="Literal"{branch}else{json!({"type":"BlockStatement","body":[{"type":"ExpressionStatement","expression":v["test"]},branch]})}
        }),
        "ConditionalExpression"=>boolean(&v["test"]).map(|b|v[if b{"consequent"}else{"alternate"}].clone()),
        "LogicalExpression"=>{
            let left=boolean(&v["left"]);let right=boolean(&v["right"]);
            match(v["operator"].as_str(),left,right){
                (Some("&&"),Some(false),_)|(Some("||"),Some(true),_)=>Some(v["left"].clone()),
                (Some("&&"),Some(true),_)|(Some("||"),Some(false),_)=>Some(v["right"].clone()),
                // Do not drop evaluation of an unknown left operand.
                _=>None,
            }
        },
        "UnaryExpression" if v["operator"]=="!"=>boolean(&v["argument"]).map(|b|literal(!b,v)),
        "BinaryExpression" if kind(&v["left"])=="Literal"&&kind(&v["right"])=="Literal"=>match v["operator"].as_str(){Some("===")=>Some(literal(v["left"]["value"]==v["right"]["value"],v)),Some("!==")=>Some(literal(v["left"]["value"]!=v["right"]["value"],v)),_=>None},
        _=>None,
    };
    if let Some(replacement) = replacement {
        changes.push(json!({"start":original["start"],"reason":"constant branch evaluation"}));
        *v = replacement;
    }
}

pub fn validate_environment_detector(program: &Value) -> crate::Result<()> {
    use crate::frontend::array;
    let function = array(program, "body")
        .iter()
        .map(|v| {
            if kind(v) == "ExportNamedDeclaration" {
                &v["declaration"]
            } else {
                v
            }
        })
        .find(|v| kind(v) == "FunctionDeclaration" && name(&v["id"]) == "isBrowser")
        .ok_or("native profile requires the upstream isBrowser definition")?;
    let statements = array(&function["body"], "body");
    let probe = |v: &Value, global: &str| {
        kind(v) == "BinaryExpression"
            && v["operator"] == "!=="
            && kind(&v["left"]) == "UnaryExpression"
            && v["left"]["operator"] == "typeof"
            && name(&v["left"]["argument"]) == global
            && v["right"]["value"] == "undefined"
    };
    if function["async"] == true
        || function["generator"] == true
        || !array(function, "params").is_empty()
        || statements.len() != 1
        || kind(&statements[0]) != "ReturnStatement"
    {
        return Err("isBrowser changed: native specialization requires review".into());
    }
    let expression = &statements[0]["argument"];
    if kind(expression) != "LogicalExpression"
        || expression["operator"] != "&&"
        || !probe(&expression["left"], "document")
        || !probe(&expression["right"], "window")
    {
        return Err("isBrowser changed: native specialization requires review".into());
    }
    Ok(())
}
