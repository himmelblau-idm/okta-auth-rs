//! Oxc is confined to this frontend. The emitter consumes an owned ESTree model.
use crate::{hash, Manifest, Result};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_resolver::{ResolveOptions, Resolver};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
pub fn kind(v: &Value) -> &str {
    v["type"].as_str().unwrap_or("")
}
pub fn name(v: &Value) -> &str {
    v["name"]
        .as_str()
        .or_else(|| v["value"].as_str())
        .unwrap_or("")
}
pub fn array<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    v[key].as_array().map(Vec::as_slice).unwrap_or(&[])
}
pub fn walk(v: &Value, f: &mut impl FnMut(&Value)) {
    f(v);
    if let Some(a) = v.as_array() {
        for item in a {
            walk(item, f);
        }
    }
    if let Some(o) = v.as_object() {
        for (k, item) in o {
            if !matches!(
                k.as_str(),
                "typeAnnotation"
                    | "returnType"
                    | "typeParameters"
                    | "typeArguments"
                    | "superTypeParameters"
                    | "implements"
            ) {
                walk(item, f);
            }
        }
    }
}
pub struct Module {
    pub source: String,
    pub definitions: BTreeMap<String, Value>,
    pub imports: BTreeMap<String, (String, String)>,
    pub exports: BTreeMap<String, (String, String)>,
    pub stars: Vec<String>,
    pub effects: Vec<Value>,
    pub references: BTreeMap<u32, String>,
    pub globals: BTreeMap<u32, String>,
    pub profile_changes: Vec<Value>,
}
pub struct Graph {
    root: PathBuf,
    pub native_profile: bool,
    resolver: Resolver,
    pub modules: BTreeMap<String, Module>,
    pub selected: BTreeSet<(String, String)>,
    pub edges: BTreeMap<(String, String), BTreeSet<(String, String)>>,
    pub links: BTreeMap<(String, String), (String, String)>,
    pub namespaces: BTreeMap<(String, String), BTreeMap<String, (String, String)>>,
}
impl Graph {
    pub fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            native_profile: false,
            root: root.canonicalize().map_err(|e| e.to_string())?,
            resolver: Resolver::new(ResolveOptions {
                extensions: vec![".ts".into(), ".js".into(), ".json".into()],
                ..Default::default()
            }),
            modules: BTreeMap::new(),
            selected: BTreeSet::new(),
            edges: BTreeMap::new(),
            links: BTreeMap::new(),
            namespaces: BTreeMap::new(),
        })
    }
    fn resolve_path(&self, module: &str, spec: &str) -> Result<String> {
        if !spec.starts_with('.') {
            return Err(format!("{module}: unmapped package import {spec}"));
        }
        let from = self.root.join(module);
        let result = self
            .resolver
            .resolve(from.parent().unwrap(), spec)
            .map_err(|e| format!("{module}: cannot resolve {spec}: {e}"))?;
        result
            .path()
            .strip_prefix(&self.root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .map_err(|_| format!("{module}: import escapes source root: {spec}"))
    }
    pub fn load(&mut self, path: &str) -> Result<()> {
        if self.modules.contains_key(path) {
            return Ok(());
        }
        let file = self
            .root
            .join(path)
            .canonicalize()
            .map_err(|e| format!("{path}: {e}"))?;
        if !file.starts_with(&self.root) {
            return Err(format!("source escapes root: {path}"));
        }
        let source = fs::read_to_string(file).map_err(|e| e.to_string())?;
        let mut m = Module {
            source: source.clone(),
            definitions: BTreeMap::new(),
            imports: BTreeMap::new(),
            exports: BTreeMap::new(),
            stars: vec![],
            effects: vec![],
            references: BTreeMap::new(),
            globals: BTreeMap::new(),
            profile_changes: vec![],
        };
        if path.ends_with(".json") {
            let v: Value = serde_json::from_str(&source).map_err(|e| e.to_string())?;
            m.definitions
                .insert("default".into(), json!({"type":"NativeJson","value":v}));
            m.exports
                .insert("default".into(), (path.into(), "default".into()));
            self.modules.insert(path.into(), m);
            return Ok(());
        }
        let alloc = Allocator::default();
        let ret = Parser::new(
            &alloc,
            &source,
            SourceType::from_path(path).map_err(|e| e.to_string())?,
        )
        .parse();
        if !ret.errors.is_empty() {
            return Err(format!("{path}: parse errors: {:?}", ret.errors));
        }
        let semantics = SemanticBuilder::new()
            .with_check_syntax_error(true)
            .build(&ret.program);
        if !semantics.errors.is_empty() {
            return Err(format!("{path}: binding errors: {:?}", semantics.errors));
        }
        let semantic = &semantics.semantic;
        let scoping = semantic.scoping();
        let mut all_references = BTreeSet::new();
        for node in semantic.nodes().iter() {
            if let AstKind::IdentifierReference(id) = node.kind() {
                let reference = scoping
                    .get_reference(id.reference_id.get().expect("bound identifier reference"));
                if reference.is_type() {
                    continue;
                }
                all_references.insert(id.span.start);
                if let Some(symbol) = reference.symbol_id() {
                    if scoping.symbol_scope_id(symbol) == scoping.root_scope_id() {
                        m.references
                            .insert(id.span.start, scoping.symbol_name(symbol).to_string());
                    }
                } else {
                    m.globals.insert(id.span.start, id.name.to_string());
                }
            }
        }
        let mut ast: Value = serde_json::from_str(&ret.program.to_pretty_estree_ts_json(false))
            .map_err(|e| e.to_string())?;
        if self.native_profile {
            let mut browser_names = BTreeSet::new();
            if path == "lib/features.ts" {
                crate::profile::validate_environment_detector(&ast)?;
                browser_names.insert("isBrowser".to_string());
            }
            for item in array(&ast, "body") {
                if kind(item) == "ImportDeclaration" && item["importKind"] != "type" {
                    if self.resolve_path(path, item["source"]["value"].as_str().unwrap())?
                        == "lib/features.ts"
                    {
                        for specifier in array(item, "specifiers") {
                            if name(&specifier["imported"]) == "isBrowser" {
                                self.load("lib/features.ts")?;
                                browser_names.insert(name(&specifier["local"]).to_string());
                            }
                        }
                    }
                }
            }
            let browser_references = m
                .references
                .iter()
                .filter(|(_, n)| browser_names.contains(*n))
                .map(|(span, _)| *span)
                .collect();
            let allow_dpop = matches!(
                path,
                "lib/oidc/getUserInfo.ts"
                    | "lib/oidc/mixin/index.ts"
                    | "lib/oidc/exchangeCodeForTokens.ts"
                    | "lib/oidc/renewTokensWithRefresh.ts"
                    | "lib/oidc/util/defaultTokenParams.ts"
                    | "lib/oidc/util/prepareTokenParams.ts"
                    | "lib/oidc/endpoints/token.ts"
                    | "lib/oidc/handleOAuthResponse.ts"
            );
            crate::profile::specialize(
                &mut ast,
                &all_references,
                &browser_references,
                &m.globals,
                allow_dpop,
                &mut m.profile_changes,
            );
        }
        for item in array(&ast, "body") {
            match kind(item) {
                "ImportDeclaration" => {
                    if item["importKind"] == "type" {
                        continue;
                    }
                    let target =
                        self.resolve_path(path, item["source"]["value"].as_str().unwrap())?;
                    for s in array(item, "specifiers") {
                        if s["importKind"] == "type" {
                            continue;
                        }
                        let imported = match kind(s) {
                            "ImportDefaultSpecifier" => "default",
                            "ImportNamespaceSpecifier" => "*",
                            _ => name(&s["imported"]),
                        };
                        m.imports
                            .insert(name(&s["local"]).into(), (target.clone(), imported.into()));
                    }
                    if array(item, "specifiers").is_empty() {
                        m.effects.push(item.clone());
                    }
                }
                "ExportAllDeclaration" => {
                    if item["exportKind"] != "type" {
                        m.stars.push(
                            self.resolve_path(path, item["source"]["value"].as_str().unwrap())?,
                        );
                    }
                }
                "ExportNamedDeclaration" => {
                    if item["exportKind"] == "type" {
                        continue;
                    }
                    if !item["declaration"].is_null() {
                        add_definition(&mut m, path, &item["declaration"], true)?;
                    }
                    for s in array(item, "specifiers") {
                        let target = if item["source"].is_null() {
                            path.into()
                        } else {
                            self.resolve_path(path, item["source"]["value"].as_str().unwrap())?
                        };
                        m.exports.insert(
                            name(&s["exported"]).into(),
                            (target, name(&s["local"]).into()),
                        );
                    }
                }
                "ExportDefaultDeclaration" => {
                    let d = &item["declaration"];
                    if kind(d) == "Identifier" {
                        m.exports
                            .insert("default".into(), (path.into(), name(d).into()));
                    } else {
                        m.definitions.insert("default".into(), d.clone());
                        m.exports
                            .insert("default".into(), (path.into(), "default".into()));
                    }
                }
                _ => add_definition(&mut m, path, item, false)?,
            }
        }
        self.modules.insert(path.into(), m);
        Ok(())
    }
    pub fn binding(&mut self, module: &str, symbol: &str) -> Result<(String, String)> {
        self.binding_inner(module, symbol, &mut BTreeSet::new())
    }
    fn binding_inner(
        &mut self,
        module: &str,
        symbol: &str,
        visited: &mut BTreeSet<(String, String)>,
    ) -> Result<(String, String)> {
        if !visited.insert((module.into(), symbol.into())) {
            return Err(format!("cyclic/unresolved export {module}::{symbol}"));
        }
        self.load(module)?;
        let m = &self.modules[module];
        if m.definitions.contains_key(symbol) {
            return Ok((module.into(), symbol.into()));
        }
        if let Some((p, s)) = m
            .imports
            .get(symbol)
            .or_else(|| m.exports.get(symbol))
            .cloned()
        {
            return self.binding_inner(&p, &s, visited);
        }
        let stars = m.stars.clone();
        let mut found = vec![];
        for star in stars {
            if let Ok(v) = self.binding_inner(&star, symbol, &mut visited.clone()) {
                found.push(v);
            }
        }
        found.sort();
        found.dedup();
        match found.len() {
            1 => Ok(found.remove(0)),
            0 => Err(format!("unresolved symbol {module}::{symbol}")),
            _ => Err(format!("ambiguous export {module}::{symbol}")),
        }
    }
    pub fn select(&mut self, module: &str, symbol: &str) -> Result<()> {
        let key = self.binding(module, symbol)?;
        if !self.selected.insert(key.clone()) {
            return Ok(());
        }
        let m = &self.modules[&key.0];
        let definition = m.definitions[&key.1].clone();
        let mut names = BTreeSet::new();
        walk(&definition, &mut |node| {
            if kind(node) == "Identifier" {
                if let Some(n) = node["start"]
                    .as_u64()
                    .and_then(|s| m.references.get(&(s as u32)))
                {
                    names.insert(n.clone());
                }
            }
        });
        let candidates: Vec<_> = names
            .into_iter()
            .filter(|n| m.definitions.contains_key(n) || m.imports.contains_key(n))
            .collect();
        for n in candidates {
            if let Some((target, star)) = self.modules[&key.0].imports.get(&n).cloned() {
                if star == "*" {
                    let mut members = BTreeSet::new();
                    let mut dynamic = false;
                    let reference_names = &self.modules[&key.0].references;
                    let mut allowed = BTreeSet::new();
                    walk(&definition, &mut |v| {
                        if kind(v) == "MemberExpression"
                            && kind(&v["object"]) == "Identifier"
                            && v["object"]["start"]
                                .as_u64()
                                .and_then(|s| reference_names.get(&(s as u32)))
                                == Some(&n)
                        {
                            allowed.insert(v["object"]["start"].as_u64().unwrap());
                            if v["computed"] == true {
                                dynamic = true;
                            } else {
                                members.insert(name(&v["property"]).to_owned());
                            }
                        }
                    });
                    walk(&definition, &mut |v| {
                        if kind(v) == "Identifier" {
                            if let Some(span) = v["start"].as_u64() {
                                if reference_names.get(&(span as u32)) == Some(&n)
                                    && !allowed.contains(&span)
                                {
                                    dynamic = true;
                                }
                            }
                        }
                    });
                    if dynamic || members.is_empty() {
                        return Err(format!(
                            "{}: namespace {n} has unresolved dynamic use",
                            key.0
                        ));
                    }
                    for member in members {
                        let dep = self.binding(&target, &member)?;
                        self.namespaces
                            .entry((key.0.clone(), n.clone()))
                            .or_default()
                            .insert(member, dep.clone());
                        self.edges
                            .entry(key.clone())
                            .or_default()
                            .insert(dep.clone());
                        self.select(&dep.0, &dep.1)?;
                    }
                    continue;
                }
            }
            let dep = self.binding(&key.0, &n)?;
            if self.modules[&key.0].imports.contains_key(&n) {
                self.links.insert((key.0.clone(), n), dep.clone());
            }
            self.edges
                .entry(key.clone())
                .or_default()
                .insert(dep.clone());
            self.select(&dep.0, &dep.1)
                .map_err(|e| format!("{e}\n  required by {}::{}", key.0, key.1))?;
        }
        Ok(())
    }
    pub fn inventory(&self, manifest: &Manifest, manifest_hash: &str) -> Value {
        let mut syntax: BTreeMap<String, usize> = BTreeMap::new();
        let mut regexes = BTreeSet::new();
        for (p, s) in &self.selected {
            walk(&self.modules[p].definitions[s], &mut |v| {
                if !kind(v).is_empty() {
                    *syntax.entry(kind(v).to_string()).or_default() += 1;
                }
                if !v["regex"].is_null() {
                    regexes.insert(v["regex"].to_string());
                }
            });
        }
        json!({"schema":1,"upstream_version":manifest.upstream_version,"upstream_commit":manifest.upstream_commit,"manifest_sha256":manifest_hash,
            "syntax":syntax,"regexes":regexes,
            "native_profile":self.native_profile,
            "profile_changes":self.modules.iter().filter(|(_,m)|!m.profile_changes.is_empty()).map(|(p,m)|(p,&m.profile_changes)).collect::<BTreeMap<_,_>>(),
            "module_effects":self.modules.iter().filter(|(_,m)|!m.effects.is_empty()).map(|(p,m)|(p,&m.effects)).collect::<BTreeMap<_,_>>(),
            "frontend":{"oxc":"0.75.0","resolver":"11.9.0"},
            "sources":self.modules.iter().map(|(p,m)|(p,hash(m.source.as_bytes()))).collect::<BTreeMap<_,_>>(),
            "selected":self.selected.iter().map(|(p,s)|format!("{p}::{s}")).collect::<Vec<_>>(),
            "unselected":self.modules.iter().flat_map(|(p,m)|m.definitions.keys().filter(|s|!self.selected.contains(&(p.clone(),(*s).clone()))).map(move|s|format!("{p}::{s}"))).collect::<Vec<_>>()})
    }
    pub fn validate(&self) -> Result<()> {
        fn effectful(v: &Value) -> bool {
            if matches!(
                kind(v),
                "FunctionDeclaration" | "FunctionExpression" | "ArrowFunctionExpression"
            ) {
                return false;
            }
            if matches!(
                kind(v),
                "CallExpression"
                    | "NewExpression"
                    | "AssignmentExpression"
                    | "UpdateExpression"
                    | "AwaitExpression"
            ) {
                return true;
            }
            if matches!(kind(v), "ClassDeclaration" | "ClassExpression") {
                return array(&v["body"], "body").iter().any(|m| {
                    kind(m) == "StaticBlock"
                        || m["static"] == true
                            && kind(m) == "PropertyDefinition"
                            && effectful(&m["value"])
                });
            }
            if let Some(items) = v.as_array() {
                return items.iter().any(effectful);
            }
            v.as_object().is_some_and(|o| o.values().any(effectful))
        }
        fn native_path(node: &Value, module: &Module) -> Option<String> {
            match kind(node) {
                "Identifier" => node["start"]
                    .as_u64()
                    .and_then(|s| module.globals.get(&(s as u32)))
                    .cloned(),
                "MemberExpression" => {
                    let parent = native_path(&node["object"], module)?;
                    let property = if node["computed"] == true {
                        node["property"]["value"].as_str()?
                    } else {
                        name(&node["property"])
                    };
                    Some(format!("{parent}.{property}"))
                }
                "TSNonNullExpression" | "TSAsExpression" | "ParenthesizedExpression" => {
                    native_path(&node["expression"], module)
                }
                _ => None,
            }
        }
        for (path, module) in &self.modules {
            if let Some(effect) = module.effects.first() {
                return Err(format!(
                    "{path}:{}: module side effect requires explicit lowering",
                    effect["start"]
                ));
            }
            if self.selected.iter().any(|(p, _)| p == path) {
                for (symbol, definition) in &module.definitions {
                    if !self.selected.contains(&(path.clone(), symbol.clone()))
                        && effectful(definition)
                    {
                        return Err(format!("{path}:{}: unselected initializer {symbol} may have side effects; explicit lowering is required",definition["start"]));
                    }
                }
            }
        }
        let supported = [
            "Array",
            "Boolean",
            "Date",
            "Error",
            "JSON",
            "Math",
            "NaN",
            "Number",
            "Object",
            "Promise",
            "String",
            "TextEncoder",
            "Uint8Array",
            "URL",
            "atob",
            "btoa",
            "decodeURIComponent",
            "encodeURIComponent",
            "escape",
            "global",
            "undefined",
            "document",
            "window",
            "console",
            "arguments",
            "crypto",
        ];
        let members = [
            "Array.from",
            "Array.isArray",
            "Array.prototype",
            "Array.prototype.slice",
            "Array.prototype.slice.call",
            "Object.prototype",
            "Object.prototype.toString",
            "Object.prototype.toString.call",
            "Object.prototype.hasOwnProperty",
            "Object.prototype.hasOwnProperty.call",
            "Object.keys",
            "Object.values",
            "Object.entries",
            "Object.assign",
            "Object.setPrototypeOf",
            "JSON.parse",
            "JSON.stringify",
            "Math.floor",
            "Math.ceil",
            "Math.random",
            "Number.isInteger",
            "Date.now",
            "String.fromCharCode",
            "String.fromCharCode.apply",
            "Uint8Array.from",
            "Promise.resolve",
            "Promise.reject",
            "crypto.subtle",
            "crypto.subtle.digest",
            "crypto.subtle.importKey",
            "crypto.subtle.verify",
            "crypto.getRandomValues",
            "global.fetch",
            "arguments.length",
        ];
        for (path, symbol) in &self.selected {
            let module = &self.modules[path];
            let mut error = None;
            walk(&module.definitions[symbol], &mut |v| {
                if kind(v) == "Identifier" {
                    if let Some(global) = v["start"]
                        .as_u64()
                        .and_then(|start| module.globals.get(&(start as u32)))
                    {
                        if !supported.contains(&global.as_str()) {
                            error = Some(format!(
                                "{path}:{}: unmapped native global {global}, required by {symbol}",
                                v["start"]
                            ));
                        }
                    }
                }
            });
            walk(&module.definitions[symbol], &mut |v| {
                if kind(v) == "MemberExpression" {
                    if let Some(member) = native_path(v, module) {
                        if !members.contains(&member.as_str()) {
                            error=Some(format!("{path}:{}: unmapped native property {member}, required by {symbol}",v["start"]));
                        }
                    }
                }
                if kind(v) == "NewExpression" {
                    if let Some(constructor) = native_path(&v["callee"], module) {
                        if !["Error", "TextEncoder", "Uint8Array", "URL"]
                            .contains(&constructor.as_str())
                        {
                            error = Some(format!(
                                "{path}:{}: unmapped native constructor {constructor}",
                                v["start"]
                            ));
                        }
                    }
                }
            });
            if let Some(error) = error {
                return Err(error);
            }
        }
        Ok(())
    }
}
fn add_definition(m: &mut Module, path: &str, d: &Value, export: bool) -> Result<()> {
    if d["declare"] == true
        || matches!(
            kind(d),
            "TSInterfaceDeclaration" | "TSTypeAliasDeclaration" | "EmptyStatement"
        )
    {
        return Ok(());
    }
    if kind(d) == "VariableDeclaration" {
        for decl in array(d, "declarations") {
            if kind(&decl["id"]) != "Identifier" {
                return Err(format!("{path}: top-level binding pattern needs lowering"));
            }
            let n = name(&decl["id"]).to_string();
            m.definitions.insert(n.clone(), decl["init"].clone());
            if export {
                m.exports.insert(n.clone(), (path.into(), n));
            }
        }
    } else if matches!(
        kind(d),
        "FunctionDeclaration" | "ClassDeclaration" | "TSEnumDeclaration"
    ) {
        let n = name(&d["id"]).to_string();
        m.definitions.insert(n.clone(), d.clone());
        if export {
            m.exports.insert(n.clone(), (path.into(), n));
        }
    } else {
        m.effects.push(d.clone());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Source(PathBuf);
    impl Source {
        fn new(files: &[(&str, &str)]) -> Self {
            let path = std::env::temp_dir().join(format!(
                "okta-compiler-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            for (name, source) in files {
                let file = path.join(name);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                fs::write(file, source).unwrap();
            }
            Self(path)
        }
        fn graph(&self) -> Graph {
            Graph::new(&self.0).unwrap()
        }
    }
    impl Drop for Source {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn references_respect_lexical_shadowing_and_property_names() {
        let source=Source::new(&[("entry.ts","export function entry(x:number){const helper=()=>x+1;return {helper:helper()};} function helper(){return unsupported();}")]);
        let mut graph = source.graph();
        graph.select("entry.ts", "entry").unwrap();
        graph.validate().unwrap();
        assert_eq!(graph.selected.len(), 1);
    }
    #[test]
    fn native_unknown_operation_is_a_build_failure() {
        let source = Source::new(&[(
            "entry.ts",
            "export function entry(){return Math.round(1.5);}",
        )]);
        let mut graph = source.graph();
        graph.select("entry.ts", "entry").unwrap();
        assert!(graph.validate().unwrap_err().contains("Math.round"));
    }
    #[test]
    fn side_effectful_unselected_initializer_is_not_silently_dropped() {
        let source = Source::new(&[(
            "entry.ts",
            "const ignored=mutate(); export function entry(){return 1;}",
        )]);
        let mut graph = source.graph();
        graph.select("entry.ts", "entry").unwrap();
        assert!(graph
            .validate()
            .unwrap_err()
            .contains("unselected initializer"));
    }
    #[test]
    fn module_effect_is_a_build_failure() {
        let source = Source::new(&[(
            "entry.ts",
            "globalThis.mutated=true; export function entry(){return 1;}",
        )]);
        let mut graph = source.graph();
        graph.select("entry.ts", "entry").unwrap();
        assert!(graph.validate().unwrap_err().contains("module side effect"));
    }
    #[test]
    fn namespace_escape_is_rejected_even_with_static_uses() {
        let source=Source::new(&[("entry.ts","import * as ns from './dep'; export function entry(){const escaped=ns;return ns.f();}"),("dep.ts","export function f(){return 1;}")]);
        assert!(source
            .graph()
            .select("entry.ts", "entry")
            .unwrap_err()
            .contains("namespace"));
    }
    #[test]
    fn ambiguous_reexports_are_rejected() {
        let source = Source::new(&[
            ("entry.ts", "export * from './a'; export * from './b';"),
            ("a.ts", "export const entry=1;"),
            ("b.ts", "export const entry=2;"),
        ]);
        assert!(source
            .graph()
            .select("entry.ts", "entry")
            .unwrap_err()
            .contains("ambiguous"));
    }
    #[test]
    fn changed_native_detector_fails_instead_of_being_replaced() {
        let source=Source::new(&[("entry.ts","import {isBrowser} from './lib/features'; export function entry(){return isBrowser();}"),("lib/features.ts","export function isBrowser(){log();return true;}")]);
        let mut graph = source.graph();
        graph.native_profile = true;
        assert!(graph
            .select("entry.ts", "entry")
            .unwrap_err()
            .contains("specialization requires review"));
    }
    #[test]
    fn local_browser_named_function_is_not_specialized() {
        let source = Source::new(&[(
            "entry.ts",
            "export function entry(){const isBrowser=()=>true;return isBrowser();}",
        )]);
        let mut graph = source.graph();
        graph.native_profile = true;
        graph.select("entry.ts", "entry").unwrap();
        assert!(graph.modules["entry.ts"].profile_changes.is_empty());
    }
    #[test]
    fn unrelated_dpop_binding_is_not_specialized() {
        let source = Source::new(&[(
            "entry.ts",
            "export function entry(dpop:boolean){return dpop ? 1 : 2;}",
        )]);
        let mut graph = source.graph();
        graph.native_profile = true;
        graph.select("entry.ts", "entry").unwrap();
        assert!(graph.modules["entry.ts"].profile_changes.is_empty());
    }
}
