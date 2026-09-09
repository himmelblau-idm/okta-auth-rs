//! Small support layer for the dynamic operations observed in the selected source.
//! Functions are compiled Rust pointers; there is no AST or bytecode interpreter.
use futures_util::{future::Shared, FutureExt};
use indexmap::IndexMap;
use serde_json::{Map, Value as Json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use zeroize::Zeroizing;
pub type JFuture = Pin<Box<dyn Future<Output = JResult> + Send>>;
pub type AsyncCode = fn(Engine, usize, V, Vec<V>) -> JFuture;

#[derive(Clone)]
pub enum V {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(Arc<Zeroizing<String>>),
    NativeError(Arc<Zeroizing<String>>),
    Object(usize),
    Array(usize),
    Function(usize),
    Promise(usize),
    Regex(usize),
    Builtin(&'static str),
}
impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.typeof_())
    }
}
impl V {
    pub fn string(s: impl Into<String>) -> Self {
        Self::String(Arc::new(Zeroizing::new(s.into())))
    }
    pub fn error(s: impl Into<String>) -> Self {
        Self::NativeError(Arc::new(Zeroizing::new(s.into())))
    }
    pub fn truthy(&self) -> bool {
        match self {
            Self::Undefined | Self::Null => false,
            Self::Bool(v) => *v,
            Self::Number(n) => *n != 0.0 && !n.is_nan(),
            Self::String(s) => !s.is_empty(),
            _ => true,
        }
    }
    pub fn nullish(&self) -> bool {
        matches!(self, Self::Undefined | Self::Null)
    }
    pub fn typeof_(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Null
            | Self::NativeError(_)
            | Self::Object(_)
            | Self::Array(_)
            | Self::Promise(_)
            | Self::Regex(_) => "object",
            Self::Bool(_) => "boolean",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Function(_) | Self::Builtin(_) => "function",
        }
    }
    pub fn text(&self) -> String {
        match self {
            Self::Undefined => "undefined".into(),
            Self::Null => "null".into(),
            Self::Bool(v) => v.to_string(),
            Self::Number(n) => n.to_string(),
            Self::String(s) => s.to_string(),
            Self::NativeError(s) => format!("Error: {}", s.as_str()),
            _ => "[object Object]".into(),
        }
    }
    pub fn number(&self) -> f64 {
        match self {
            Self::Number(v) => *v,
            Self::Null => 0.0,
            Self::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Self::String(s) => {
                if s.trim().is_empty() {
                    0.0
                } else {
                    s.trim().parse().unwrap_or(f64::NAN)
                }
            }
            _ => f64::NAN,
        }
    }
}
pub type JResult = Result<V, V>;
pub type Code = fn(&Engine, usize, V, Vec<V>) -> JResult;
pub enum Completion {
    Normal,
    Return(V),
    Break,
    Continue,
}
#[derive(Clone, Copy)]
enum Callable {
    Sync(Code),
    Async(AsyncCode),
}
struct Function {
    code: Callable,
    scope: usize,
    properties: IndexMap<String, V>,
    lexical_this: Option<V>,
    context: super::diagnostic::Context,
}
struct Scope {
    parent: Option<usize>,
    values: BTreeMap<String, V>,
    aliases: BTreeMap<String, (usize, String)>,
}
struct RegExp {
    compiled: regex::Regex,
    global: bool,
    last_index: usize,
}
#[derive(Default)]
pub(super) struct Heap {
    objects: Vec<IndexMap<String, V>>,
    prototypes: BTreeMap<(bool, usize), V>,
    getters: BTreeMap<(bool, usize, String), V>,
    constructors: BTreeMap<usize, V>,
    arrays: Vec<Vec<V>>,
    array_properties: BTreeMap<usize, IndexMap<String, V>>,
    functions: Vec<Function>,
    scopes: Vec<Scope>,
    promises: Vec<Option<Shared<JFuture>>>,
    regexes: Vec<RegExp>,
    tasks: Vec<JFuture>,
    executing: bool,
    epoch: u64,
    pub(super) granted_scopes: Option<Vec<String>>,
    pub(super) mutations: BTreeMap<u8, crate::OperationFailure>,
    pub(super) evidence: BTreeMap<(u8, usize), (V, crate::OperationFailure)>,
    http: Option<super::HttpTransport>,
    error_objects: std::collections::BTreeSet<usize>,
    hidden: std::collections::BTreeSet<(usize, String)>,
}
/// A flow-local arena owns objects and closure environments, preserving aliases without reference cycles.
#[derive(Clone, Default)]
pub struct Engine(
    pub(super) Arc<Mutex<Heap>>,
    pub(super) super::diagnostic::Context,
);
impl Engine {
    pub fn regexp(&self, pattern: &str, flags: &str) -> JResult {
        if flags.chars().any(|c| !"gim".contains(c)) {
            return Err(V::error("unsupported regex flags"));
        }
        let pattern = pattern
            .replace("\\/", "/")
            .replace("\\w", "[A-Za-z0-9_]")
            .replace("\\d", "[0-9]");
        let compiled = regex::RegexBuilder::new(&pattern)
            .case_insensitive(flags.contains('i'))
            .multi_line(flags.contains('m'))
            .build()
            .map_err(|_| V::error("unsupported regular expression"))?;
        let mut h = self.0.lock().unwrap();
        let id = h.regexes.len();
        h.regexes.push(RegExp {
            compiled,
            global: flags.contains('g'),
            last_index: 0,
        });
        Ok(V::Regex(id))
    }
    fn regex_exec(&self, id: usize, text: &str) -> JResult {
        let (regex, start, global) = {
            let h = self.0.lock().unwrap();
            let r = &h.regexes[id];
            (
                r.compiled.clone(),
                if r.global { r.last_index } else { 0 },
                r.global,
            )
        };
        if start > text.len() || !text.is_char_boundary(start) {
            self.0.lock().unwrap().regexes[id].last_index = 0;
            return Ok(V::Null);
        }
        if let Some(caps) = regex.captures_at(text, start) {
            let whole = caps.get(0).unwrap();
            if global {
                self.0.lock().unwrap().regexes[id].last_index = whole.end();
            }
            let result = self.array(
                caps.iter()
                    .map(|m| m.map(|m| V::string(m.as_str())).unwrap_or(V::Undefined))
                    .collect(),
            );
            let groups = self.object();
            for n in regex.capture_names().flatten() {
                self.put(
                    &groups,
                    n,
                    caps.name(n)
                        .map(|m| V::string(m.as_str()))
                        .unwrap_or(V::Undefined),
                )?;
            }
            self.put(&result, "groups", groups)?;
            self.put(
                &result,
                "index",
                V::Number(text[..whole.start()].encode_utf16().count() as f64),
            )?;
            Ok(result)
        } else {
            if global {
                self.0.lock().unwrap().regexes[id].last_index = 0;
            }
            Ok(V::Null)
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_http_transport(&self, transport: super::HttpTransport) {
        self.0.lock().unwrap().http = Some(transport);
    }
    pub(crate) fn http_transport(&self) -> Option<super::HttpTransport> {
        self.0.lock().unwrap().http.clone()
    }
    pub fn error(&self, message: &str) -> V {
        V::error(message)
    }

    pub fn scope(&self, parent: Option<usize>) -> usize {
        let mut h = self.0.lock().unwrap();
        let id = h.scopes.len();
        h.scopes.push(Scope {
            parent,
            values: BTreeMap::new(),
            aliases: BTreeMap::new(),
        });
        id
    }
    pub fn link(&self, scope: usize, name: &str, target: usize, symbol: &str) {
        self.0.lock().unwrap().scopes[scope]
            .aliases
            .insert(name.into(), (target, symbol.into()));
    }
    pub fn define(&self, scope: usize, name: &str, v: V) {
        self.0.lock().unwrap().scopes[scope]
            .values
            .insert(name.into(), v);
    }
    pub fn lookup(&self, mut scope: usize, name: &str) -> JResult {
        let h = self.0.lock().unwrap();
        loop {
            if let Some(v) = h.scopes[scope].values.get(name) {
                return Ok(v.clone());
            }
            if let Some((target, symbol)) = h.scopes[scope].aliases.get(name) {
                let (target, symbol) = (*target, symbol.clone());
                drop(h);
                return self.lookup(target, &symbol);
            }
            if let Some(p) = h.scopes[scope].parent {
                scope = p
            } else {
                break;
            }
        }
        match name {
            "undefined" | "document" | "window" => Ok(V::Undefined),
            "NaN" => Ok(V::Number(f64::NAN)),
            "Object" => Ok(V::Builtin("Object")),
            "Array" => Ok(V::Builtin("Array")),
            "JSON" => Ok(V::Builtin("JSON")),
            "String" => Ok(V::Builtin("String")),
            "Number" => Ok(V::Builtin("Number")),
            "Boolean" => Ok(V::Builtin("Boolean")),
            "Math" => Ok(V::Builtin("Math")),
            "Promise" => Ok(V::Builtin("Promise")),
            "Error" => Ok(V::Builtin("Error")),
            _ => super::native::global(name)
                .ok_or_else(|| V::error(format!("unbound identifier {name}"))),
        }
    }
    pub fn assign(&self, mut scope: usize, name: &str, v: V) -> JResult {
        let mut h = self.0.lock().unwrap();
        loop {
            if h.scopes[scope].values.contains_key(name) {
                h.scopes[scope].values.insert(name.into(), v.clone());
                return Ok(v);
            }
            if let Some(p) = h.scopes[scope].parent {
                scope = p
            } else {
                break;
            }
        }
        Err(V::error(format!("assignment to unbound identifier {name}")))
    }
    pub fn object(&self) -> V {
        let mut h = self.0.lock().unwrap();
        let id = h.objects.len();
        h.objects.push(IndexMap::new());
        V::Object(id)
    }
    pub fn array(&self, values: Vec<V>) -> V {
        let mut h = self.0.lock().unwrap();
        let id = h.arrays.len();
        h.arrays.push(values);
        V::Array(id)
    }
    pub fn function(&self, scope: usize, code: Code) -> V {
        self.add_function(scope, Callable::Sync(code), None)
    }
    pub fn async_function(&self, scope: usize, code: AsyncCode) -> V {
        self.add_function(scope, Callable::Async(code), None)
    }
    pub fn arrow(&self, scope: usize, this: V, code: Code) -> V {
        self.add_function(scope, Callable::Sync(code), Some(this))
    }
    pub fn async_arrow(&self, scope: usize, this: V, code: AsyncCode) -> V {
        self.add_function(scope, Callable::Async(code), Some(this))
    }
    fn add_function(&self, scope: usize, code: Callable, lexical_this: Option<V>) -> V {
        let mut h = self.0.lock().unwrap();
        let id = h.functions.len();
        h.functions.push(Function {
            code,
            scope,
            properties: IndexMap::new(),
            lexical_this,
            context: self.1.clone(),
        });
        V::Function(id)
    }
    pub fn promise(&self, f: JFuture) -> V {
        let mut h = self.0.lock().unwrap();
        let id = h.promises.len();
        h.promises.push(Some(f.shared()));
        V::Promise(id)
    }
    // Await always introduces a scheduling boundary, including non-promises.
    pub async fn await_value(&self, mut value: V) -> JResult {
        let mut yielded = false;
        std::future::poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await;
        loop {
            if let V::Promise(id) = value {
                let promise = self
                    .0
                    .lock()
                    .unwrap()
                    .promises
                    .get(id)
                    .and_then(Clone::clone)
                    .ok_or_else(|| V::error("operation canceled"))?;
                value = promise.await?;
            } else {
                self.adopt_evidence(&value);
                return Ok(value);
            }
        }
    }
    fn identity(obj: &V) -> Option<(bool, usize)> {
        match obj {
            V::Object(id) => Some((false, *id)),
            V::Function(id) => Some((true, *id)),
            _ => None,
        }
    }
    /// Start an async function through its first await on the calling task.
    /// Remaining work is driven serially by execute; no detached Tokio tasks.
    pub(crate) fn schedule(&self, mut future: JFuture) -> V {
        let waker = futures_util::task::noop_waker();
        if let Poll::Ready(result) = future.as_mut().poll(&mut Context::from_waker(&waker)) {
            return self.promise(Box::pin(async move { result }));
        }
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.0.lock().unwrap().tasks.push(Box::pin(async move {
            let result = future.await;
            let _ = tx.send(result);
            Ok(V::Undefined)
        }));
        self.promise(Box::pin(async move {
            rx.await
                .unwrap_or_else(|_| Err(V::error("operation canceled")))
        }))
    }
    pub fn cancel_pending(&self) {
        let (tasks, promises) = {
            let mut h = self.0.lock().unwrap();
            (
                std::mem::take(&mut h.tasks),
                h.promises
                    .iter_mut()
                    .filter_map(Option::take)
                    .collect::<Vec<_>>(),
            )
        };
        drop(tasks);
        drop(promises);
    }
    /// Runs one entry point. Dropping this future cancels its pending native work.
    /// A caller must serialize executions on an Engine.
    pub async fn execute(&self, f: V, this: V, args: Vec<V>) -> JResult {
        struct Cancel(Engine, bool);
        impl Drop for Cancel {
            fn drop(&mut self) {
                self.0.cancel_pending();
                let mut heap = self.0 .0.lock().unwrap();
                heap.executing = false;
                if !self.1 {
                    heap.evidence.clear();
                    heap.granted_scopes = None;
                }
            }
        }
        {
            let mut h = self.0.lock().unwrap();
            if h.executing {
                return Err(V::error("concurrent execution on one flow"));
            }
            h.executing = true;
            h.epoch += 1;
            h.evidence.clear();
            h.mutations.clear();
            h.granted_scopes = None;
        }
        let mut cancel = Cancel(self.clone(), false);
        let mut cx = self.clone();
        cx.1 = super::diagnostic::Context::default();
        cx.1.epoch = self.0.lock().unwrap().epoch;
        let value = match cx.call(f, this, args) {
            Ok(value) => value,
            Err(error) => {
                cancel.1 = true;
                return Err(error);
            }
        };
        let mut result = Box::pin(self.await_value(value));
        let output = std::future::poll_fn(|context| {
            let tasks = std::mem::take(&mut self.0.lock().unwrap().tasks);
            let mut pending = vec![];
            for mut task in tasks {
                if task.as_mut().poll(context).is_pending() {
                    pending.push(task);
                }
            }
            let mut h = self.0.lock().unwrap();
            if !h.tasks.is_empty() {
                context.waker().wake_by_ref();
            }
            pending.append(&mut h.tasks);
            h.tasks = pending;
            drop(h);
            result.as_mut().poll(context)
        })
        .await;
        cancel.1 = true;
        output
    }
    pub fn set_prototype(&self, obj: &V, proto: V) -> JResult {
        let identity = Self::identity(obj).ok_or_else(|| V::error("invalid prototype target"))?;
        if !matches!(proto, V::Null | V::Object(_) | V::Function(_)) {
            return Err(V::error("invalid prototype"));
        }
        let mut h = self.0.lock().unwrap();
        let mut cursor = Some(proto.clone());
        while let Some(p) = cursor {
            if let Some(id) = Self::identity(&p) {
                if id == identity {
                    return Err(V::error("cyclic prototype"));
                }
                cursor = h.prototypes.get(&id).cloned();
            } else {
                break;
            }
        }
        h.prototypes.insert(identity, proto);
        Ok(obj.clone())
    }
    pub fn relation(&self, operator: &str, a: V, b: V) -> JResult {
        if operator == "instanceof" {
            if matches!(b, V::Builtin("Error")) {
                return Ok(V::Bool(
                    matches!(a, V::NativeError(_))
                        || matches!(a,V::Object(id) if self.0.lock().unwrap().error_objects.contains(&id)),
                ));
            }
            if !matches!(b, V::Function(_)) {
                return Err(V::error("unsupported instanceof constructor"));
            }
            let target = self.get(&b, "prototype")?;
            if Self::identity(&target).is_none() {
                return Err(V::error("invalid instanceof prototype"));
            }
            let h = self.0.lock().unwrap();
            let mut cursor = Self::identity(&a).and_then(|id| h.prototypes.get(&id));
            while let Some(p) = cursor {
                if eq(p, &target) {
                    return Ok(V::Bool(true));
                }
                cursor = Self::identity(p).and_then(|id| h.prototypes.get(&id));
            }
            return Ok(V::Bool(false));
        }
        if operator == "in" {
            let key = a.text();
            let mut cursor = b;
            loop {
                if self.entries(&cursor)?.iter().any(|(k, _)| k == &key) {
                    return Ok(V::Bool(true));
                }
                let h = self.0.lock().unwrap();
                let next = Self::identity(&cursor)
                    .and_then(|id| h.prototypes.get(&id))
                    .cloned();
                drop(h);
                if let Some(next) = next {
                    cursor = next;
                } else {
                    return Ok(V::Bool(false));
                }
            }
        }
        Err(V::error("unsupported relation"))
    }
    pub fn getter(&self, obj: &V, key: &str, f: V) -> JResult {
        let (function, id) =
            Self::identity(obj).ok_or_else(|| V::error("invalid getter target"))?;
        self.0
            .lock()
            .unwrap()
            .getters
            .insert((function, id, key.into()), f);
        Ok(V::Undefined)
    }
    pub fn new_target(&self, obj: &V) -> JResult {
        if let V::Object(id) = obj {
            Ok(self
                .0
                .lock()
                .unwrap()
                .constructors
                .get(id)
                .cloned()
                .unwrap_or(V::Undefined))
        } else {
            Ok(V::Undefined)
        }
    }
    pub fn construct(&self, f: V, args: Vec<V>) -> JResult {
        if let V::Builtin(b @ ("Uint8Array" | "TextEncoder" | "URL")) = f {
            return self.native_call(b, V::Undefined, args);
        }
        let this = self.object();
        let p = self.get(&f, "prototype")?;
        if !p.nullish() {
            self.set_prototype(&this, p)?;
        }
        if let V::Object(id) = this {
            self.0.lock().unwrap().constructors.insert(id, f.clone());
        }
        let result = self.call(f, this.clone(), args)?;
        if matches!(result, V::Object(_) | V::Array(_) | V::Function(_)) {
            Ok(result)
        } else {
            Ok(this)
        }
    }
    pub fn get(&self, obj: &V, key: &str) -> JResult {
        self.get_with_receiver(obj, key, obj)
    }
    fn get_with_receiver(&self, obj: &V, key: &str, receiver: &V) -> JResult {
        let h = self.0.lock().unwrap();
        if let Some((function, id)) = Self::identity(obj) {
            if let Some(f) = h.getters.get(&(function, id, key.into())).cloned() {
                drop(h);
                return self.call(f, receiver.clone(), vec![]);
            }
            let value = if function {
                h.functions[id].properties.get(key)
            } else {
                h.objects[id].get(key)
            };
            if let Some(value) = value {
                return Ok(value.clone());
            }
            if let Some(proto) = h.prototypes.get(&(function, id)).cloned() {
                drop(h);
                if matches!(proto, V::Null) {
                    return Ok(V::Undefined);
                }
                return self.get_with_receiver(&proto, key, receiver);
            }
            return Ok(if key == "toString" {
                V::Builtin("Object.toString")
            } else {
                V::Undefined
            });
        }
        match obj {
            V::NativeError(message) => Ok(match key {
                "message" => V::string(message.to_string()),
                "name" => V::string("Error"),
                _ => V::Undefined,
            }),
            V::Array(id) => {
                if key == "length" {
                    Ok(V::Number(h.arrays[*id].len() as f64))
                } else if key == "slice"
                    && !h
                        .array_properties
                        .get(id)
                        .is_some_and(|p| p.contains_key(key))
                {
                    Ok(V::Builtin("Array.slice"))
                } else {
                    Ok(key
                        .parse::<usize>()
                        .ok()
                        .and_then(|i| h.arrays[*id].get(i).cloned())
                        .or_else(|| h.array_properties.get(id).and_then(|p| p.get(key)).cloned())
                        .unwrap_or(V::Undefined))
                }
            }
            V::String(s) => {
                if key == "length" {
                    Ok(V::Number(s.encode_utf16().count() as f64))
                } else if let Ok(i) = key.parse::<usize>() {
                    match s.encode_utf16().nth(i) {
                        Some(c) => Ok(V::string(
                            String::from_utf16(&[c])
                                .map_err(|_| V::error("unpaired UTF-16 surrogate"))?,
                        )),
                        None => Ok(V::Undefined),
                    }
                } else {
                    Ok(V::Undefined)
                }
            }
            V::Builtin("global") if key == "fetch" => Ok(V::Builtin("fetch")),
            V::Builtin("crypto") if key == "subtle" => Ok(V::Builtin("crypto.subtle")),
            V::Builtin("String") if key == "fromCharCode" => Ok(V::Builtin("String.fromCharCode")),
            V::Builtin("Array") if key == "prototype" => Ok(V::Builtin("ArrayPrototype")),
            V::Builtin("ArrayPrototype") if key == "slice" => Ok(V::Builtin("Array.slice")),
            V::Builtin("Object") if key == "prototype" => Ok(V::Builtin("ObjectPrototype")),
            V::Builtin("ObjectPrototype") if key == "toString" => Ok(V::Builtin("Object.toString")),
            V::Builtin("ObjectPrototype") if key == "hasOwnProperty" => {
                Ok(V::Builtin("hasOwnProperty"))
            }
            V::Null | V::Undefined => Err(V::error(format!(
                "cannot read property {key} of null or undefined"
            ))),
            _ => Ok(V::Undefined),
        }
    }
    pub fn put(&self, obj: &V, key: &str, value: V) -> JResult {
        let mut h = self.0.lock().unwrap();
        match obj {
            V::Object(id) => {
                h.hidden.remove(&(*id, key.into()));
                h.objects[*id].insert(key.into(), value.clone());
            }
            V::Function(id) => {
                h.functions[*id]
                    .properties
                    .insert(key.into(), value.clone());
            }
            V::Array(id) => {
                if key == "length" {
                    h.arrays[*id].resize(value.number() as usize, V::Undefined);
                } else if let Ok(i) = key.parse::<usize>() {
                    if i >= h.arrays[*id].len() {
                        h.arrays[*id].resize(i + 1, V::Undefined);
                    }
                    h.arrays[*id][i] = value.clone();
                } else {
                    h.array_properties
                        .entry(*id)
                        .or_default()
                        .insert(key.into(), value.clone());
                }
            }
            _ => return Err(V::error("cannot assign property")),
        }
        Ok(value)
    }
    pub fn delete(&self, obj: &V, key: &str) -> JResult {
        let mut h = self.0.lock().unwrap();
        match obj {
            V::Object(id) => {
                h.objects[*id].shift_remove(key);
            }
            V::Function(id) => {
                h.functions[*id].properties.shift_remove(key);
            }
            _ => return Err(V::error("unsupported delete target")),
        }
        Ok(V::Bool(true))
    }
    pub fn entries(&self, obj: &V) -> Result<Vec<(String, V)>, V> {
        let h = self.0.lock().unwrap();
        let mut entries = match obj {
            V::Object(id) => h.objects[*id]
                .iter()
                .filter(|(k, _)| !h.hidden.contains(&(*id, (*k).clone())))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>(),
            V::Array(id) => h.arrays[*id]
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v.clone()))
                .collect(),
            V::Function(id) => h.functions[*id]
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            V::String(s) => s
                .chars()
                .enumerate()
                .map(|(i, c)| (i.to_string(), V::string(c.to_string())))
                .collect(),
            V::Null | V::Undefined => return Err(V::error("cannot enumerate null or undefined")),
            _ => vec![],
        };
        // JavaScript enumerates array-index keys first, numerically.
        entries.sort_by(|a, b| {
            fn index(s: &str) -> Option<u32> {
                s.parse::<u32>()
                    .ok()
                    .filter(|i| *i != u32::MAX && i.to_string() == s)
            }
            match (index(&a.0), index(&b.0)) {
                (Some(a), Some(b)) => a.cmp(&b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });
        Ok(entries)
    }
    pub fn spread(&self, target: &V, source: &V) -> Result<(), V> {
        if source.nullish() {
            return Ok(());
        }
        for (k, v) in self.entries(source)? {
            self.put(target, &k, v)?;
        }
        Ok(())
    }
    pub fn rest(&self, source: &V, exclude: &[String]) -> JResult {
        let out = self.object();
        for (k, v) in self.entries(source)? {
            if !exclude.contains(&k) {
                self.put(&out, &k, v)?;
            }
        }
        Ok(out)
    }
    pub fn items(&self, value: &V) -> Result<Vec<V>, V> {
        match value {
            V::Array(id) => Ok(self.0.lock().unwrap().arrays[*id].clone()),
            _ => Err(V::error("value is not iterable")),
        }
    }
    pub fn call(&self, f: V, this: V, args: Vec<V>) -> JResult {
        match f {
            V::Function(id) => {
                let (code, scope, lexical, context, epoch) = {
                    let h = self.0.lock().unwrap();
                    (
                        h.functions[id].code,
                        h.functions[id].scope,
                        h.functions[id].lexical_this.clone(),
                        h.functions[id].context.clone(),
                        h.epoch,
                    )
                };
                let this = lexical.unwrap_or(this);
                let mut cx = self.clone();
                if context.epoch != 0 && context.epoch == epoch {
                    cx.1 = context;
                }
                let request = cx.1.request.lock().unwrap().clone();
                cx.1.request = Arc::new(Mutex::new(request));
                // Parameters tie callbacks and error normalization to their
                // own response, independent of promise completion order.
                for arg in &args {
                    cx.adopt_evidence(arg);
                }
                match code {
                    Callable::Sync(code) => {
                        let result = code(&cx, scope, this, args);
                        if let Ok(value) = &result {
                            self.adopt_evidence(value);
                        }
                        result
                    }
                    Callable::Async(code) => {
                        let future = code(cx.clone(), scope, this, args);
                        Ok(self.schedule(future))
                    }
                }
            }
            V::Builtin("Object.toString") => {
                let tag = match this {
                    V::Undefined => "Undefined",
                    V::NativeError(_) => "Error",
                    V::Null => "Null",
                    V::String(_) => "String",
                    V::Number(_) => "Number",
                    V::Bool(_) => "Boolean",
                    V::Array(_) => "Array",
                    V::Promise(_) => "Promise",
                    V::Regex(_) => "RegExp",
                    V::Function(id) => match self.0.lock().unwrap().functions[id].code {
                        Callable::Async(_) => "AsyncFunction",
                        _ => "Function",
                    },
                    V::Builtin(_) => "Function",
                    _ => "Object",
                };
                Ok(V::string(format!("[object {tag}]")))
            }
            V::Builtin("String") => Ok(V::string(args.first().unwrap_or(&V::Undefined).text())),
            V::Builtin("Number") => Ok(V::Number(args.first().unwrap_or(&V::Number(0.0)).number())),
            V::Builtin("Boolean") => Ok(V::Bool(args.first().is_some_and(V::truthy))),
            V::Builtin("Error") => {
                let this = if matches!(this, V::Undefined) {
                    self.object()
                } else {
                    this
                };
                self.put(&this, "name", V::string("Error"))?;
                self.put(
                    &this,
                    "message",
                    args.first().cloned().unwrap_or_else(|| V::string("")),
                )?;
                if let V::Object(id) = this {
                    let mut h = self.0.lock().unwrap();
                    h.error_objects.insert(id);
                    h.hidden.insert((id, "message".into()));
                    h.hidden.insert((id, "name".into()));
                }
                Ok(this)
            }
            V::Builtin("hasOwnProperty") => {
                Ok(V::Bool(self.entries(&this)?.iter().any(|(k, _)| {
                    *k == args.first().unwrap_or(&V::Undefined).text()
                })))
            }
            V::Builtin(b) => self.native_call(b, this, args),
            _ => Err(V::error("value is not callable")),
        }
    }
    pub fn resolve_method(&self, obj: &V, key: &str) -> JResult {
        let value = self.get(obj, key)?;
        if !matches!(value, V::Undefined) {
            return Ok(value);
        }
        if matches!(
            obj,
            V::Array(_)
                | V::String(_)
                | V::Builtin(_)
                | V::Regex(_)
                | V::Promise(_)
                | V::Function(_)
                | V::Number(_)
        ) {
            fn invoke(cx: &Engine, scope: usize, this: V, args: Vec<V>) -> JResult {
                let method = cx.lookup(scope, "native_method")?.text();
                cx.method(this, &method, args)
            }
            let scope = self.scope(None);
            self.define(scope, "native_method", V::string(key));
            return Ok(self.function(scope, invoke));
        }
        Ok(V::Undefined)
    }
    pub fn method(&self, obj: V, key: &str, args: Vec<V>) -> JResult {
        let a = args.first().cloned().unwrap_or(V::Undefined);
        if let V::Regex(id) = obj {
            return match key {
                "exec" => self.regex_exec(id, &a.text()),
                "test" => Ok(V::Bool(!matches!(self.regex_exec(id, &a.text())?, V::Null))),
                _ => Err(V::error("unsupported regex method")),
            };
        }
        if matches!(obj, V::Promise(_)) && matches!(key, "then" | "catch" | "finally") {
            let (cx, kind) = (self.clone(), key.to_owned());
            return Ok(self.schedule(Box::pin(async move {
                let r = cx.await_value(obj).await;
                let handler = match (kind.as_str(), &r) {
                    ("then", Ok(_)) => Some(a),
                    ("then", Err(_)) => args.get(1).cloned(),
                    ("catch", Err(_)) => Some(a),
                    ("finally", _) => Some(a),
                    _ => None,
                };
                if let Some(f) = handler.filter(|v| matches!(v, V::Function(_) | V::Builtin(_))) {
                    let values = if kind == "finally" {
                        vec![]
                    } else {
                        vec![match &r {
                            Ok(v) | Err(v) => v.clone(),
                        }]
                    };
                    let value = cx.call(f, V::Undefined, values)?;
                    let value = cx.await_value(value).await?;
                    if kind != "finally" {
                        return Ok(value);
                    }
                }
                r
            })));
        }
        if key == "apply" {
            let values = args
                .get(1)
                .filter(|v| !v.nullish())
                .map(|v| self.items(v))
                .transpose()?
                .unwrap_or_default();
            return self.call(obj, a, values);
        }
        if key == "call" {
            return self.call(obj, a, args.into_iter().skip(1).collect());
        }
        if let V::Builtin(b) = obj {
            return match (b, key) {
                ("Promise", "resolve") => Ok(self.promise(Box::pin(async move { Ok(a) }))),
                ("Promise", "reject") => {
                    self.attach_evidence(&a, self.current_evidence());
                    Ok(self.promise(Box::pin(async move { Err(a) })))
                }
                ("Object", "setPrototypeOf") => {
                    self.set_prototype(&a, args.get(1).cloned().unwrap_or(V::Null))
                }
                ("Array", "isArray") => Ok(V::Bool(matches!(a, V::Array(_)))),
                ("Object", "keys") => Ok(self.array(
                    self.entries(&a)?
                        .into_iter()
                        .map(|(k, _)| V::string(k))
                        .collect(),
                )),
                ("Object", "values") => {
                    Ok(self.array(self.entries(&a)?.into_iter().map(|(_, v)| v).collect()))
                }
                ("Object", "entries") => Ok(self.array(
                    self.entries(&a)?
                        .into_iter()
                        .map(|(k, v)| self.array(vec![V::string(k), v]))
                        .collect(),
                )),
                ("Object", "assign") => {
                    for source in args.iter().skip(1) {
                        self.spread(&a, source)?;
                    }
                    Ok(a)
                }
                ("JSON", "stringify") => {
                    if matches!(a, V::Undefined | V::Function(_)) {
                        return Ok(V::Undefined);
                    }
                    Ok(V::string(
                        serde_json::to_string(&self.to_json(&a)?)
                            .map_err(|_| V::error("JSON serialization failed"))?,
                    ))
                }
                ("JSON", "parse") => {
                    let j =
                        serde_json::from_str(&a.text()).map_err(|_| V::error("invalid JSON"))?;
                    Ok(self.from_json(&j))
                }
                ("Number", "isInteger") => Ok(V::Bool(
                    matches!(a,V::Number(n) if n.is_finite()&&n.fract()==0.0),
                )),
                ("Math", "floor") => Ok(V::Number(a.number().floor())),
                _ => self.native_method(b, key, args),
            };
        }
        if let V::Array(id) = obj {
            match key {
                "push" => {
                    let mut h = self.0.lock().unwrap();
                    h.arrays[id].extend(args);
                    return Ok(V::Number(h.arrays[id].len() as f64));
                }
                "pop" => {
                    return Ok(self.0.lock().unwrap().arrays[id]
                        .pop()
                        .unwrap_or(V::Undefined))
                }
                "splice" => {
                    let mut h = self.0.lock().unwrap();
                    let values = &mut h.arrays[id];
                    let n = a.number() as isize;
                    let start = if n < 0 {
                        (values.len() as isize + n).max(0) as usize
                    } else {
                        n as usize
                    }
                    .min(values.len());
                    let count = args
                        .get(1)
                        .map(|v| v.number().max(0.0) as usize)
                        .unwrap_or(values.len() - start)
                        .min(values.len() - start);
                    let removed: Vec<_> = values
                        .splice(start..start + count, args.into_iter().skip(2))
                        .collect();
                    drop(h);
                    return Ok(self.array(removed));
                }
                _ => {}
            }
            let items = self.items(&obj)?;
            match key {
                "map" | "filter" | "forEach" | "find" | "findIndex" | "some" | "every" => {
                    let mut out = vec![];
                    for (i, v) in items.into_iter().enumerate() {
                        let r = self.call(
                            a.clone(),
                            V::Undefined,
                            vec![v.clone(), V::Number(i as f64), obj.clone()],
                        )?;
                        match key {
                            "map" => out.push(r),
                            "filter" => {
                                if r.truthy() {
                                    out.push(v)
                                }
                            }
                            "find" => {
                                if r.truthy() {
                                    return Ok(v);
                                }
                            }
                            "findIndex" => {
                                if r.truthy() {
                                    return Ok(V::Number(i as f64));
                                }
                            }
                            "some" => {
                                if r.truthy() {
                                    return Ok(V::Bool(true));
                                }
                            }
                            "every" => {
                                if !r.truthy() {
                                    return Ok(V::Bool(false));
                                }
                            }
                            _ => {}
                        }
                    }
                    return Ok(match key {
                        "map" | "filter" => self.array(out),
                        "some" => V::Bool(false),
                        "every" => V::Bool(true),
                        "findIndex" => V::Number(-1.0),
                        _ => V::Undefined,
                    });
                }
                "reduce" => {
                    let mut iter = items.into_iter().enumerate();
                    let mut acc = if args.len() > 1 {
                        args[1].clone()
                    } else {
                        iter.next()
                            .map(|(_, v)| v)
                            .ok_or_else(|| V::error("reduce of empty array"))?
                    };
                    for (i, v) in iter {
                        acc = self.call(
                            a.clone(),
                            V::Undefined,
                            vec![acc, v, V::Number(i as f64), obj.clone()],
                        )?;
                    }
                    return Ok(acc);
                }
                "includes" => return Ok(V::Bool(items.iter().any(|v| eq(v, &a)))),
                "indexOf" => {
                    return Ok(V::Number(
                        items
                            .iter()
                            .position(|v| eq(v, &a))
                            .map(|i| i as f64)
                            .unwrap_or(-1.0),
                    ))
                }
                "join" => {
                    return Ok(V::string(
                        items
                            .iter()
                            .map(|v| if v.nullish() { String::new() } else { v.text() })
                            .collect::<Vec<_>>()
                            .join(&if matches!(a, V::Undefined) {
                                ",".into()
                            } else {
                                a.text()
                            }),
                    ))
                }
                "concat" => {
                    let mut out = items;
                    for v in args {
                        if matches!(v, V::Array(_)) {
                            out.extend(self.items(&v)?);
                        } else {
                            out.push(v)
                        }
                    }
                    return Ok(self.array(out));
                }
                "slice" => {
                    let start = a.number().max(0.0) as usize;
                    let end = args
                        .get(1)
                        .map(|v| v.number().max(0.0) as usize)
                        .unwrap_or(items.len())
                        .min(items.len());
                    return Ok(self.array(items.get(start..end).unwrap_or(&[]).to_vec()));
                }
                _ => {}
            }
        }
        if let V::Number(n) = obj {
            if key == "toString" {
                let radix = if matches!(a, V::Undefined) {
                    10
                } else {
                    a.number() as u32
                };
                if radix == 10 {
                    return Ok(V::string(n.to_string()));
                }
                if !(2..=36).contains(&radix) || !n.is_finite() || n.fract() != 0.0 {
                    return Err(V::error("unsupported numeric radix conversion"));
                }
                let mut digits = vec![];
                let mut number = n.abs() as u64;
                loop {
                    digits.push(char::from_digit((number % radix as u64) as u32, radix).unwrap());
                    number /= radix as u64;
                    if number == 0 {
                        break;
                    }
                }
                if n < 0.0 {
                    digits.push('-');
                }
                digits.reverse();
                return Ok(V::string(digits.into_iter().collect::<String>()));
            }
        }
        if let V::String(s) = &obj {
            return match key {
                "substr" => {
                    let units: Vec<_> = s.encode_utf16().collect();
                    let n = a.number() as isize;
                    let start = if n < 0 {
                        (units.len() as isize + n).max(0) as usize
                    } else {
                        n as usize
                    }
                    .min(units.len());
                    let len = args
                        .get(1)
                        .map(|v| v.number().max(0.0) as usize)
                        .unwrap_or(units.len() - start)
                        .min(units.len() - start);
                    Ok(V::string(
                        String::from_utf16(&units[start..start + len])
                            .map_err(|_| V::error("unpaired UTF-16 surrogate"))?,
                    ))
                }
                "replace" => {
                    let replacement = args.get(1).cloned().unwrap_or(V::Undefined);
                    if let V::Regex(id) = a {
                        let (regex, global) = {
                            let h = self.0.lock().unwrap();
                            (h.regexes[id].compiled.clone(), h.regexes[id].global)
                        };
                        if matches!(replacement, V::Function(_)) {
                            return Err(V::error("replacement callback requires lowering"));
                        }
                        let text = if global {
                            regex
                                .replace_all(s.as_str(), replacement.text())
                                .into_owned()
                        } else {
                            regex.replace(s.as_str(), replacement.text()).into_owned()
                        };
                        Ok(V::string(text))
                    } else {
                        Ok(V::string(s.replacen(&a.text(), &replacement.text(), 1)))
                    }
                }
                "charAt" => Ok(V::string(
                    s.chars()
                        .nth(a.number() as usize)
                        .map(|c| c.to_string())
                        .unwrap_or_default(),
                )),
                "charCodeAt" => Ok(V::Number(
                    s.encode_utf16()
                        .nth(a.number() as usize)
                        .map(|c| c as f64)
                        .unwrap_or(f64::NAN),
                )),
                "indexOf" => Ok(V::Number(
                    s.find(&a.text())
                        .map(|i| s[..i].encode_utf16().count() as f64)
                        .unwrap_or(-1.0),
                )),
                "substring" | "slice" => {
                    let units: Vec<_> = s.encode_utf16().collect();
                    let index = |v: &V| {
                        let n = v.number() as isize;
                        if key == "slice" && n < 0 {
                            (units.len() as isize + n).max(0) as usize
                        } else {
                            n.max(0) as usize
                        }
                        .min(units.len())
                    };
                    let mut start = index(&a);
                    let mut end = args.get(1).map(index).unwrap_or(units.len());
                    if key == "substring" && start > end {
                        std::mem::swap(&mut start, &mut end);
                    }
                    Ok(V::string(
                        String::from_utf16(units.get(start..end).unwrap_or(&[]))
                            .map_err(|_| V::error("unpaired UTF-16 surrogate"))?,
                    ))
                }
                "includes" => Ok(V::Bool(s.contains(&a.text()))),
                "startsWith" => Ok(V::Bool(s.starts_with(&a.text()))),
                "endsWith" => Ok(V::Bool(s.ends_with(&a.text()))),
                "toLowerCase" => Ok(V::string(s.to_lowercase())),
                "toUpperCase" => Ok(V::string(s.to_uppercase())),
                "trim" => Ok(V::string(s.trim())),
                "split" => Ok(self.array(if matches!(a, V::Undefined) {
                    vec![obj.clone()]
                } else {
                    s.split(&a.text()).map(V::string).collect()
                })),
                _ => Err(V::error(format!("unsupported string method {key}"))),
            };
        }
        if key == "call" {
            return self.call(obj, a, args.into_iter().skip(1).collect());
        }
        let f = self.get(&obj, key)?;
        self.call(f, obj, args)
    }
    pub fn from_json(&self, j: &Json) -> V {
        match j {
            Json::Null => V::Null,
            Json::Bool(v) => V::Bool(*v),
            Json::Number(v) => V::Number(v.as_f64().unwrap()),
            Json::String(v) => V::string(v),
            Json::Array(v) => self.array(v.iter().map(|j| self.from_json(j)).collect()),
            Json::Object(v) => {
                let o = self.object();
                for (k, v) in v {
                    self.put(&o, k, self.from_json(v)).unwrap();
                }
                o
            }
        }
    }
    pub fn to_json(&self, v: &V) -> Result<Json, V> {
        self.json_inner(v, &mut vec![])
    }
    fn json_inner(&self, v: &V, stack: &mut Vec<(bool, usize)>) -> Result<Json, V> {
        let identity = match v {
            V::Array(i) => Some((true, *i)),
            V::Object(i) => Some((false, *i)),
            _ => None,
        };
        if let Some(id) = identity {
            if stack.contains(&id) {
                return Err(V::error("cyclic JSON value"));
            }
            stack.push(id);
        }
        let out = match v {
            V::Undefined | V::Null | V::Function(_) | V::Builtin(_) => Json::Null,
            V::NativeError(_) | V::Promise(_) | V::Regex(_) => Json::Object(Map::new()),
            V::Bool(b) => Json::Bool(*b),
            V::Number(n) => serde_json::Number::from_f64(*n)
                .map(Json::Number)
                .unwrap_or(Json::Null),
            V::String(s) => Json::String(s.to_string()),
            V::Array(_) => Json::Array(
                self.items(v)?
                    .iter()
                    .map(|v| self.json_inner(v, stack))
                    .collect::<Result<_, _>>()?,
            ),
            V::Object(_) => {
                let mut m = Map::new();
                for (k, v) in self.entries(v)? {
                    if !matches!(v, V::Undefined | V::Function(_) | V::Builtin(_)) {
                        m.insert(k, self.json_inner(&v, stack)?);
                    }
                }
                Json::Object(m)
            }
        };
        if identity.is_some() {
            stack.pop();
        }
        Ok(out)
    }
}
pub fn eq(a: &V, b: &V) -> bool {
    match (a, b) {
        (V::Undefined, V::Undefined) | (V::Null, V::Null) => true,
        (V::Bool(a), V::Bool(b)) => a == b,
        (V::Number(a), V::Number(b)) => a == b,
        (V::String(a), V::String(b)) => a == b,
        (V::Object(a), V::Object(b))
        | (V::Array(a), V::Array(b))
        | (V::Function(a), V::Function(b)) => a == b,
        _ => false,
    }
}
fn loose_eq(a: &V, b: &V) -> Result<bool, V> {
    if a.nullish() && b.nullish() {
        return Ok(true);
    }
    if a.typeof_() == b.typeof_() {
        return Ok(eq(a, b));
    }
    match (a, b) {
        (V::Bool(_), _) => loose_eq(&V::Number(a.number()), b),
        (_, V::Bool(_)) => loose_eq(a, &V::Number(b.number())),
        (V::String(_), V::Number(_)) | (V::Number(_), V::String(_)) => Ok(a.number() == b.number()),
        (V::Object(_) | V::Array(_) | V::Function(_), _)
        | (_, V::Object(_) | V::Array(_) | V::Function(_)) => {
            Err(V::error("object coercion requires lowering"))
        }
        _ => Ok(false),
    }
}
pub fn binary(op: &str, a: V, b: V) -> JResult {
    Ok(match op {
        "==" => V::Bool(loose_eq(&a, &b)?),
        "!=" => V::Bool(!loose_eq(&a, &b)?),
        "===" => V::Bool(eq(&a, &b)),
        "!==" => V::Bool(!eq(&a, &b)),
        "+" => {
            if matches!(a, V::String(_)) || matches!(b, V::String(_)) {
                V::string(a.text() + &b.text())
            } else {
                V::Number(a.number() + b.number())
            }
        }
        "-" => V::Number(a.number() - b.number()),
        "*" => V::Number(a.number() * b.number()),
        "/" => V::Number(a.number() / b.number()),
        "%" => V::Number(a.number() % b.number()),
        "<" => V::Bool(a.number() < b.number()),
        ">" => V::Bool(a.number() > b.number()),
        "<=" => V::Bool(a.number() <= b.number()),
        ">=" => V::Bool(a.number() >= b.number()),
        _ => return Err(V::error(format!("unsupported binary operator {op}"))),
    })
}
