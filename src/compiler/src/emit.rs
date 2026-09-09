use crate::{
    frontend::{array, kind, name, Graph},
    Manifest, Result,
};
use serde_json::Value;
use std::collections::BTreeMap;
fn q(s: &str) -> String {
    format!("{s:?}")
}
struct Emitter {
    module: String,
    functions: Vec<String>,
    next: usize,
    in_try: bool,
    asynchronous: bool,
    loop_depth: usize,
    break_label: Option<String>,
    chain_label: Option<String>,
}
impl Emitter {
    fn return_value(&self, expr: &str) -> String {
        if self.in_try {
            format!("return Ok(Completion::Return({expr}));\n")
        } else {
            format!("return Ok({expr});\n")
        }
    }
    fn assignment(&mut self, l: &Value, r: &str) -> Result<String> {
        if kind(l) == "Identifier" {
            Ok(format!("cx.assign(scope,{}, {r})?", q(name(l))))
        } else if kind(l) == "MemberExpression" {
            let (o, k) = self.member(l)?;
            Ok(format!(
                "{{let object={o};let key={k};cx.put(&object,&key,{r})?}}"
            ))
        } else {
            self.fail(l, "unsupported assignment target")
        }
    }
    fn enumeration(&mut self, v: &Value) -> Result<String> {
        let mut s = "{let enumeration=cx.object();".to_string();
        let mut next = 0;
        for m in array(&v["body"], "members") {
            let value = if m["initializer"].is_null() {
                let n = next;
                next += 1;
                format!("V::Number({n}f64)")
            } else {
                if let Some(n) = m["initializer"]["value"].as_i64() {
                    next = n + 1;
                }
                self.expr(&m["initializer"])?
            };
            let key = q(name(&m["id"]));
            s+=&format!("{{let value={value};if matches!(value,V::Number(_)){{cx.put(&enumeration,&value.text(),V::string({key}))?;}}cx.put(&enumeration,{key},value)?;}}");
        }
        Ok(s + "enumeration}")
    }
    fn try_statement(&mut self, v: &Value) -> Result<String> {
        let enclosing = self.in_try;
        self.in_try = true;
        let mut body = self.stmt(&v["block"])?;
        body += "Ok::<Completion,V>(Completion::Normal)";
        let wrap = |body: String, asynchronous: bool| {
            if asynchronous {
                format!("(async{{{body}}}).await")
            } else {
                format!("(||{{{body}}})()")
            }
        };
        let mut s = format!("{{let mut completion={};", wrap(body, self.asynchronous));
        if !v["handler"].is_null() {
            let mut catch = "let scope=cx.scope(Some(scope));".to_string();
            if !v["handler"]["param"].is_null() {
                catch += &self.bind(&v["handler"]["param"], "error")?;
            }
            catch += &self.stmt(&v["handler"]["body"])?;
            catch += "Ok::<Completion,V>(Completion::Normal)";
            s += &format!(
                "if let Err(error)=completion{{completion={};}}",
                wrap(catch, self.asynchronous)
            );
        }
        if !v["finalizer"].is_null() {
            let finally = self.stmt(&v["finalizer"])? + "Ok::<Completion,V>(Completion::Normal)";
            s+=&format!("let finalization={};if !matches!(finalization,Ok(Completion::Normal)){{completion=finalization;}}",wrap(finally,self.asynchronous));
        }
        self.in_try = enclosing;
        let break_code = if enclosing {
            "return Ok(Completion::Break);".into()
        } else if let Some(label) = &self.break_label {
            format!("break '{label};")
        } else if self.loop_depth > 0 {
            "break;".into()
        } else {
            "return Err(V::string(\"unexpected break\"));".into()
        };
        let continue_code = if enclosing {
            "return Ok(Completion::Continue);"
        } else if self.loop_depth > 0 {
            "continue;"
        } else {
            "return Err(V::string(\"unexpected continue\"));"
        };
        s+=&format!("match completion?{{Completion::Normal=>{{}},Completion::Return(value)=>{{{}}},Completion::Break=>{{{break_code}}},Completion::Continue=>{{{continue_code}}}}}}}\n",self.return_value("value"));
        Ok(s)
    }
    fn loop_statement(&mut self, v: &Value) -> Result<String> {
        self.loop_depth += 1;
        let prior = self.break_label.take();
        let result = if kind(v) == "ForStatement" {
            let init = if v["init"].is_null() {
                String::new()
            } else if kind(&v["init"]) == "VariableDeclaration" {
                self.stmt(&v["init"])?
            } else {
                format!("let _={};", self.expr(&v["init"])?)
            };
            let test = if v["test"].is_null() {
                "V::Bool(true)".into()
            } else {
                self.expr(&v["test"])?
            };
            let update = self.expr(&v["update"])?;
            let body = self.stmt(&v["body"])?;
            format!("{{let scope=cx.scope(Some(scope));{init}let mut first=true;loop{{if !first{{let _={update};}}first=false;if !{test}.truthy(){{break;}}{body}}}}}\n")
        } else {
            let binding = if kind(&v["left"]) == "VariableDeclaration" {
                &v["left"]["declarations"][0]["id"]
            } else {
                return self.fail(v, "for-in assignment needs lowering");
            };
            format!(
                "for(key,_)in cx.entries(&{})?{{let scope=cx.scope(Some(scope));{}{}}}\n",
                self.expr(&v["right"])?,
                self.bind(binding, "V::string(key)")?,
                self.stmt(&v["body"])?
            )
        };
        self.loop_depth -= 1;
        self.break_label = prior;
        Ok(result)
    }
    fn switch_statement(&mut self, v: &Value) -> Result<String> {
        let label = format!("switch{}", self.next);
        self.next += 1;
        let prior = self.break_label.replace(label.clone());
        let mut s = format!(
            "'{label}:{{let discriminant={};let mut matched=false;",
            self.expr(&v["discriminant"])?
        );
        for case in array(v, "cases") {
            let test = if case["test"].is_null() {
                "true".into()
            } else {
                format!(
                    "crate::runtime::eq(&discriminant,&{})",
                    self.expr(&case["test"])?
                )
            };
            s += &format!("if matched||{test}{{matched=true;");
            for statement in array(case, "consequent") {
                s += &self.stmt(statement)?;
            }
            s += "}";
        }
        self.break_label = prior;
        Ok(s + "}\n")
    }
    fn class(&mut self, v: &Value) -> Result<String> {
        let derived = !v["superClass"].is_null();
        let superclass = if derived {
            self.expr(&v["superClass"])?
        } else {
            "V::Undefined".into()
        };
        let mut constructor = None;
        let mut methods = vec![];
        let mut fields = vec![];
        let mut statics = vec![];
        for m in array(&v["body"], "body") {
            if m["declare"] == true {
                continue;
            }
            if m["computed"] == true {
                return self.fail(
                    m,
                    "computed class members require evaluation-order lowering",
                );
            }
            match kind(m) {
                "MethodDefinition" => {
                    if m["kind"] == "constructor" {
                        constructor = Some(m["value"].clone());
                    } else if m["kind"] == "method" || m["kind"] == "get" {
                        methods.push(m.clone());
                    } else {
                        return self.fail(m, "class setter requires lowering");
                    }
                }
                "PropertyDefinition" => {
                    if !m["value"].is_null() {
                        if m["static"] == true {
                            statics.push(m.clone());
                        } else {
                            fields.push(m.clone());
                        }
                    }
                }
                _ => return self.fail(m, "unsupported class member"),
            }
        }
        let mut ctor=constructor.unwrap_or_else(||if derived{serde_json::json!({"type":"FunctionExpression","params":[{"type":"RestElement","argument":{"type":"Identifier","name":"args"}}],"body":{"type":"BlockStatement","body":[{"type":"ExpressionStatement","expression":{"type":"CallExpression","callee":{"type":"Super"},"arguments":[{"type":"SpreadElement","argument":{"type":"Identifier","name":"args"}}]}}]}})}else{serde_json::json!({"type":"FunctionExpression","params":[],"body":{"type":"BlockStatement","body":[]}})});
        let statements = ctor["body"]["body"]
            .as_array_mut()
            .ok_or("class constructor must have block body")?;
        let position = if derived {
            statements
                .iter()
                .position(|s| {
                    kind(s) == "ExpressionStatement"
                        && kind(&s["expression"]) == "CallExpression"
                        && kind(&s["expression"]["callee"]) == "Super"
                })
                .ok_or_else(|| {
                    format!(
                        "{}: derived constructor needs a direct super call",
                        self.module
                    )
                })?
                + 1
        } else {
            0
        };
        for field in fields.into_iter().rev() {
            statements.insert(position,serde_json::json!({"type":"ExpressionStatement","expression":{"type":"AssignmentExpression","operator":"=","left":{"type":"MemberExpression","object":{"type":"ThisExpression"},"property":field["key"],"computed":false},"right":field["value"]}}));
        }
        let ctor = self.function(&ctor)?;
        let mut s=format!("{{let superclass={superclass};let scope=cx.scope(Some(scope));cx.define(scope,\"__super\",superclass.clone());let constructor={ctor};let prototype=cx.object();if !superclass.nullish(){{let parent=cx.get(&superclass,\"prototype\")?;if !parent.nullish(){{cx.set_prototype(&prototype,parent)?;}}if matches!(superclass,V::Function(_)){{cx.set_prototype(&constructor,superclass)?;}}}}cx.put(&constructor,\"prototype\",prototype.clone())?;");
        if !name(&v["id"]).is_empty() {
            s += &format!(
                "cx.define(scope,{},constructor.clone());",
                q(name(&v["id"]))
            );
        }
        for method in methods {
            let value = self.function(&method["value"])?;
            let target = if method["static"] == true {
                "constructor"
            } else {
                "prototype"
            };
            let operation = if method["kind"] == "get" {
                "getter"
            } else {
                "put"
            };
            s += &format!(
                "cx.{operation}(&{target},&{}, {})?;",
                self.key(&method)?,
                value
            );
        }
        for field in statics {
            s += &format!(
                "{{let this=constructor.clone();cx.put(&constructor,&{}, {})?;}}",
                self.key(&field)?,
                self.expr(&field["value"])?
            );
        }
        Ok(s + "constructor}")
    }
    fn fail<T>(&self, v: &Value, message: &str) -> Result<T> {
        Err(format!(
            "{}:{}: {message} ({})",
            self.module,
            v["start"],
            kind(v)
        ))
    }
    fn function(&mut self, v: &Value) -> Result<String> {
        if v["generator"] == true {
            return self.fail(
                v,
                "asynchronous/generator function requires additional lowering",
            );
        }
        let id = self.next;
        self.next += 1;
        let previous = (
            self.in_try,
            self.asynchronous,
            self.loop_depth,
            self.break_label.take(),
            self.chain_label.take(),
        );
        self.in_try = false;
        self.asynchronous = v["async"] == true;
        self.loop_depth = 0;
        let mut body = if kind(v) == "ArrowFunctionExpression" {
            String::new()
        } else {
            "cx.define(scope,\"arguments\",cx.array(args.clone()));\n".into()
        };
        for (i, p) in array(v, "params").iter().enumerate() {
            let value = if kind(p) == "RestElement" {
                format!("cx.array(args.iter().skip({i}).cloned().collect())")
            } else {
                format!("args.get({i}).cloned().unwrap_or(V::Undefined)")
            };
            body += &self.bind(p, &value)?;
        }
        if kind(&v["body"]) == "BlockStatement" {
            for s in array(&v["body"], "body") {
                body += &self.stmt(s)?;
            }
        } else {
            body += &format!("return Ok({});\n", self.expr(&v["body"])?);
        }
        let asynchronous = v["async"] == true;
        if asynchronous {
            self.functions.push(format!("fn f{id}(engine:Engine,parent:usize,this:V,args:Vec<V>)->crate::runtime::JFuture {{Box::pin(async move {{let cx=&engine;let scope=cx.scope(Some(parent));{body} Ok(V::Undefined)}})}}\n"));
        } else {
            self.functions.push(format!("fn f{id}(cx:&Engine,parent:usize,this:V,args:Vec<V>)->JResult {{\nlet scope=cx.scope(Some(parent));\n{body}\nOk(V::Undefined)\n}}\n"));
        }
        self.in_try = previous.0;
        self.asynchronous = previous.1;
        self.loop_depth = previous.2;
        self.break_label = previous.3;
        self.chain_label = previous.4;
        let prefix = if asynchronous { "async_" } else { "" };
        Ok(if kind(v) == "ArrowFunctionExpression" {
            format!("cx.{prefix}arrow(scope,this.clone(),f{id})")
        } else {
            format!("cx.{prefix}function(scope,f{id})")
        })
    }
    fn bind(&mut self, p: &Value, value: &str) -> Result<String> {
        Ok(match kind(p) {
            "Identifier" => format!("cx.define(scope,{},{});\n", q(name(p)), value),
            "RestElement" => self.bind(&p["argument"], value)?,
            "AssignmentPattern" => {
                let fallback = self.expr(&p["right"])?;
                format!("{{let value={value};let value=if matches!(value,V::Undefined){{{fallback}}}else{{value}};{}}}\n",self.bind(&p["left"],"value")?)
            }
            "ObjectPattern" => {
                let mut s = format!("{{let binding={value};\n");
                let mut keys = vec![];
                for prop in array(p, "properties") {
                    if kind(prop) == "RestElement" {
                        s += &self.bind(
                            &prop["argument"],
                            &format!("cx.rest(&binding,&vec![{}])?", keys.join(",")),
                        )?;
                    } else {
                        let k = self.key(prop)?;
                        keys.push(format!("{k}.to_string()"));
                        s += &self.bind(&prop["value"], &format!("cx.get(&binding,&{k})?"))?;
                    }
                }
                s + "}\n"
            }
            "ArrayPattern" => {
                let mut s = format!("{{let binding={value};\n");
                for (i, p) in array(p, "elements").iter().enumerate() {
                    if !p.is_null() {
                        s += &self.bind(p, &format!("cx.get(&binding,{})?", q(&i.to_string())))?;
                    }
                }
                s + "}\n"
            }
            _ => return self.fail(p, "unsupported binding pattern"),
        })
    }
    fn key(&mut self, v: &Value) -> Result<String> {
        if v["computed"] == true {
            Ok(format!("{}.text()", self.expr(&v["key"])?))
        } else {
            Ok(q(name(&v["key"])))
        }
    }
    fn member(&mut self, v: &Value) -> Result<(String, String)> {
        Ok((
            self.expr(&v["object"])?,
            if v["computed"] == true {
                format!("{}.text()", self.expr(&v["property"])?)
            } else {
                q(name(&v["property"]))
            },
        ))
    }
    fn args(&mut self, v: &Value) -> Result<String> {
        let mut s = "{let mut argv=Vec::new();".to_string();
        for a in array(v, "arguments") {
            if kind(a) == "SpreadElement" {
                s += &format!("argv.extend(cx.items(&{})?);", self.expr(&a["argument"])?);
            } else {
                s += &format!("argv.push({});", self.expr(a)?);
            }
        }
        Ok(s + "argv}")
    }
    fn expr(&mut self, v: &Value) -> Result<String> {
        if v.is_null() {
            return Ok("V::Undefined".into());
        }
        Ok(match kind(v) {
            "Identifier" => format!("cx.lookup(scope,{})?", q(name(v))),
            "ThisExpression" => "this.clone()".into(),
            "AwaitExpression" => format!("cx.await_value({}).await?", self.expr(&v["argument"])?),
            "Super" => "cx.lookup(scope,\"__super\")?".into(),
            "MetaProperty" => {
                if name(&v["meta"]) == "new" && name(&v["property"]) == "target" {
                    "cx.new_target(&this)?".into()
                } else {
                    return self.fail(v, "unsupported meta property");
                }
            }
            "NewExpression" => format!(
                "cx.construct({}, {})?",
                self.expr(&v["callee"])?,
                self.args(v)?
            ),
            "ClassDeclaration" | "ClassExpression" => self.class(v)?,
            "TSEnumDeclaration" => self.enumeration(v)?,
            "Literal" => {
                if !v["regex"].is_null() {
                    format!(
                        "cx.regexp({}, {})?",
                        q(v["regex"]["pattern"].as_str().unwrap()),
                        q(v["regex"]["flags"].as_str().unwrap())
                    )
                } else {
                    match &v["value"] {
                        Value::Null => "V::Null".into(),
                        Value::Bool(b) => format!("V::Bool({b})"),
                        Value::Number(n) => format!("V::Number({n}f64)"),
                        Value::String(s) => format!("V::string({})", q(s)),
                        _ => return self.fail(v, "unsupported literal"),
                    }
                }
            }
            "TSAsExpression"
            | "TSTypeAssertion"
            | "TSNonNullExpression"
            | "TSSatisfiesExpression"
            | "ParenthesizedExpression" => self.expr(&v["expression"])?,
            "ChainExpression" => {
                let label = format!("chain{}", self.next);
                self.next += 1;
                let prior = self.chain_label.replace(label.clone());
                let body = self.expr(&v["expression"])?;
                self.chain_label = prior;
                format!("'{label}:{{{body}}}")
            }
            "SequenceExpression" => {
                let mut body = "{".to_string();
                let expressions = array(v, "expressions");
                for (i, e) in expressions.iter().enumerate() {
                    if i + 1 == expressions.len() {
                        body += &self.expr(e)?;
                    } else {
                        body += &format!("let _={};", self.expr(e)?);
                    }
                }
                body + "}"
            }
            "FunctionDeclaration" | "FunctionExpression" | "ArrowFunctionExpression" => {
                self.function(v)?
            }
            "ObjectExpression" => {
                let mut s = "{let object=cx.object();".to_string();
                for p in array(v, "properties") {
                    if kind(p) == "SpreadElement" {
                        s += &format!("cx.spread(&object,&{})?;", self.expr(&p["argument"])?);
                    } else {
                        if p["kind"].as_str().is_some_and(|s| s != "init") {
                            return self.fail(p, "accessor property requires lowering");
                        }
                        s += &format!(
                            "cx.put(&object,&{}, {})?;",
                            self.key(p)?,
                            self.expr(&p["value"])?
                        );
                    }
                }
                s + "object}"
            }
            "ArrayExpression" => {
                let mut s = "{let mut values=vec![];".to_string();
                for a in array(v, "elements") {
                    if kind(a) == "SpreadElement" {
                        s += &format!("values.extend(cx.items(&{})?);", self.expr(&a["argument"])?);
                    } else {
                        s += &format!("values.push({});", self.expr(a)?);
                    }
                }
                s + "cx.array(values)}"
            }
            "MemberExpression" => {
                let (o, k) = self.member(v)?;
                if v["optional"] == true {
                    let label = self
                        .chain_label
                        .as_ref()
                        .ok_or("optional member outside chain")?;
                    format!("{{let object={o};if object.nullish(){{break '{label} V::Undefined;}}cx.get(&object,&{k})?}}")
                } else {
                    format!("cx.get(&{o},&{k})?")
                }
            }
            "CallExpression" => {
                let mut callee = &v["callee"];
                while matches!(
                    kind(callee),
                    "TSNonNullExpression" | "TSAsExpression" | "ParenthesizedExpression"
                ) {
                    callee = &callee["expression"];
                }
                let args = self.args(v)?;
                if kind(callee) == "Super" {
                    return Ok(format!(
                        "cx.call(cx.lookup(scope,\"__super\")?,this.clone(),{args})?"
                    ));
                }
                let optional = if v["optional"] == true {
                    format!(
                        "if function.nullish(){{break '{} V::Undefined;}}",
                        self.chain_label
                            .as_ref()
                            .ok_or("optional call outside chain")?
                    )
                } else {
                    String::new()
                };
                if kind(callee) == "MemberExpression" {
                    if kind(&callee["object"]) == "Super" {
                        let key = if callee["computed"] == true {
                            format!("{}.text()", self.expr(&callee["property"])?)
                        } else {
                            q(name(&callee["property"]))
                        };
                        return Ok(format!("{{let parent=cx.get(&cx.lookup(scope,\"__super\")?,\"prototype\")?;let function=cx.resolve_method(&parent,&{key})?;let argv={args};cx.call(function,this.clone(),argv)?}}"));
                    }
                    let (o, k) = self.member(callee)?;
                    let receiver_guard = if callee["optional"] == true {
                        format!(
                            "if receiver.nullish(){{break '{} V::Undefined;}}",
                            self.chain_label
                                .as_ref()
                                .ok_or("optional member call outside chain")?
                        )
                    } else {
                        String::new()
                    };
                    format!("{{let receiver={o};{receiver_guard}let method={k};let function=cx.resolve_method(&receiver,&method)?;{optional}let argv={args};cx.call(function,receiver,argv)?}}")
                } else {
                    let f = self.expr(callee)?;
                    format!("{{let function={f};{optional}let argv={args};cx.call(function,V::Undefined,argv)?}}")
                }
            }
            "LogicalExpression" => {
                let a = self.expr(&v["left"])?;
                let b = self.expr(&v["right"])?;
                match v["operator"].as_str().unwrap() {
                    "&&" => format!("{{let left={a};if left.truthy(){{{b}}}else{{left}}}}"),
                    "||" => format!("{{let left={a};if left.truthy(){{left}}else{{{b}}}}}"),
                    "??" => format!("{{let left={a};if left.nullish(){{{b}}}else{{left}}}}"),
                    _ => return self.fail(v, "unsupported logical operator"),
                }
            }
            "BinaryExpression" => {
                let op = v["operator"].as_str().unwrap();
                if ["instanceof", "in"].contains(&op) {
                    format!(
                        "cx.relation({}, {}, {})?",
                        q(op),
                        self.expr(&v["left"])?,
                        self.expr(&v["right"])?
                    )
                } else {
                    if ![
                        "==", "!=", "===", "!==", "+", "-", "*", "/", "%", "<", ">", "<=", ">=",
                    ]
                    .contains(&op)
                    {
                        return self.fail(v, "unsupported binary operator");
                    }
                    format!(
                        "binary({}, {}, {})?",
                        q(op),
                        self.expr(&v["left"])?,
                        self.expr(&v["right"])?
                    )
                }
            }
            "UnaryExpression" => match v["operator"].as_str().unwrap() {
                "!" => format!("V::Bool(!{}.truthy())", self.expr(&v["argument"])?),
                "typeof" => format!("V::string({}.typeof_())", self.expr(&v["argument"])?),
                "void" => format!("{{let _={};V::Undefined}}", self.expr(&v["argument"])?),
                "-" => format!("V::Number(-{}.number())", self.expr(&v["argument"])?),
                "+" => format!("V::Number({}.number())", self.expr(&v["argument"])?),
                "delete" => {
                    let (o, k) = self.member(&v["argument"])?;
                    format!("cx.delete(&{o},&{k})?")
                }
                _ => return self.fail(v, "unsupported unary operator"),
            },
            "UpdateExpression" => {
                let arg = &v["argument"];
                let old = self.expr(arg)?;
                let operation = if v["operator"] == "++" { "+" } else { "-" };
                let updated = format!("binary({},old.clone(),V::Number(1.0))?", q(operation));
                let assign = self.assignment(arg, &updated)?;
                format!(
                    "{{let old={old};let updated={assign};{} }}",
                    if v["prefix"] == true {
                        "updated"
                    } else {
                        "old"
                    }
                )
            }
            "AssignmentExpression" => {
                let mut r = self.expr(&v["right"])?;
                let l = &v["left"];
                if v["operator"] != "=" {
                    let op = v["operator"].as_str().unwrap().strip_suffix('=').unwrap();
                    if !["+", "-", "*", "/"].contains(&op) {
                        return self.fail(v, "unsupported compound assignment");
                    }
                    r = format!("binary({}, {}, {r})?", q(op), self.expr(l)?);
                }
                if kind(l) == "Identifier" {
                    format!("cx.assign(scope,{}, {r})?", q(name(l)))
                } else if kind(l) == "MemberExpression" {
                    let (o, k) = self.member(l)?;
                    format!(
                        "{{let object={o};let key={k};let value={r};cx.put(&object,&key,value)?}}"
                    )
                } else {
                    return self.fail(v, "destructuring assignment needs lowering");
                }
            }
            "ConditionalExpression" => format!(
                "if {}.truthy(){{{}}}else{{{}}}",
                self.expr(&v["test"])?,
                self.expr(&v["consequent"])?,
                self.expr(&v["alternate"])?
            ),
            "TemplateLiteral" => {
                let mut s = "{let mut text=String::new();".to_string();
                for (i, part) in array(v, "quasis").iter().enumerate() {
                    s += &format!(
                        "text.push_str({});",
                        q(part["value"]["cooked"].as_str().unwrap_or(""))
                    );
                    if let Some(e) = array(v, "expressions").get(i) {
                        s += &format!("text.push_str(&{}.text());", self.expr(e)?);
                    }
                }
                s + "V::string(text)}"
            }
            "NativeJson" => format!(
                "cx.from_json(&serde_json::from_str({}).expect(\"compiler-validated JSON\"))",
                q(&v["value"].to_string())
            ),
            _ => return self.fail(v, "unsupported reachable expression"),
        })
    }
    fn stmt(&mut self, v: &Value) -> Result<String> {
        Ok(match kind(v) {
            "VariableDeclaration" => {
                let mut s = String::new();
                for d in array(v, "declarations") {
                    let e = self.expr(&d["init"])?;
                    s += &self.bind(&d["id"], &e)?;
                }
                s
            }
            "ExpressionStatement" => format!("let _={};\n", self.expr(&v["expression"])?),
            "ReturnStatement" => {
                let expr = self.expr(&v["argument"])?;
                self.return_value(&expr)
            }
            "TryStatement" => self.try_statement(v)?,
            "ForStatement" | "ForInStatement" => self.loop_statement(v)?,
            "SwitchStatement" => self.switch_statement(v)?,
            "ThrowStatement" => format!("return Err({});\n", self.expr(&v["argument"])?),
            "BlockStatement" => {
                let mut s = String::from("{let scope=cx.scope(Some(scope));\n");
                for st in array(v, "body") {
                    s += &self.stmt(st)?;
                }
                s + "}\n"
            }
            "IfStatement" => {
                let mut s = format!(
                    "if {}.truthy(){{{}}}",
                    self.expr(&v["test"])?,
                    self.stmt(&v["consequent"])?
                );
                if !v["alternate"].is_null() {
                    s += &format!("else{{{}}}", self.stmt(&v["alternate"])?);
                }
                s + "\n"
            }
            "ForOfStatement" => {
                let binding = if kind(&v["left"]) == "VariableDeclaration" {
                    &v["left"]["declarations"][0]["id"]
                } else {
                    return self.fail(v, "for-of assignment requires lowering");
                };
                format!(
                    "for item in cx.items(&{})?{{let scope=cx.scope(Some(scope));{}{} }}\n",
                    self.expr(&v["right"])?,
                    self.bind(binding, "item")?,
                    self.stmt(&v["body"])?
                )
            }
            "WhileStatement" => format!(
                "while {}.truthy(){{{}}}\n",
                self.expr(&v["test"])?,
                self.stmt(&v["body"])?
            ),
            "ContinueStatement" => {
                if !v["label"].is_null() {
                    return self.fail(v, "labelled continue unsupported");
                }
                if self.in_try {
                    "return Ok(Completion::Continue);\n".into()
                } else {
                    "continue;\n".into()
                }
            }
            "BreakStatement" => {
                if !v["label"].is_null() {
                    return self.fail(v, "labelled break unsupported");
                }
                if self.in_try {
                    "return Ok(Completion::Break);\n".into()
                } else if let Some(label) = &self.break_label {
                    format!("break '{label};\n")
                } else {
                    "break;\n".into()
                }
            }
            "FunctionDeclaration" => {
                let f = self.function(v)?;
                format!("cx.define(scope,{}, {f});\n", q(name(&v["id"])))
            }
            "EmptyStatement" => String::new(),
            _ => return self.fail(v, "unsupported reachable statement"),
        })
    }
}
pub fn generate(graph: &Graph, manifest: &Manifest) -> Result<String> {
    let mut emitter = Emitter {
        module: String::new(),
        functions: vec![],
        next: 0,
        in_try: false,
        asynchronous: false,
        loop_depth: 0,
        break_label: None,
        chain_label: None,
    };
    let modules: BTreeMap<_, _> = graph
        .modules
        .keys()
        .enumerate()
        .map(|(i, p)| (p.clone(), i))
        .collect();
    let mut initialize = String::new();
    // Function declarations are hoisted. Other initializers follow dependencies,
    // so a class cannot be evaluated before its imported base class.
    fn order(
        key: &(String, String),
        graph: &Graph,
        done: &mut std::collections::BTreeSet<(String, String)>,
        active: &mut std::collections::BTreeSet<(String, String)>,
        out: &mut Vec<(String, String)>,
    ) -> Result<()> {
        if done.contains(key) {
            return Ok(());
        }
        if !active.insert(key.clone()) {
            return Err(format!(
                "initializer dependency cycle at {}::{}",
                key.0, key.1
            ));
        }
        if let Some(deps) = graph.edges.get(key) {
            for dep in deps {
                if dep != key {
                    order(dep, graph, done, active, out)?;
                }
            }
        }
        active.remove(key);
        done.insert(key.clone());
        out.push(key.clone());
        Ok(())
    }
    let mut ordered: Vec<_> = graph
        .selected
        .iter()
        .filter(|(p, s)| kind(&graph.modules[p].definitions[s]) == "FunctionDeclaration")
        .cloned()
        .collect();
    let mut done = ordered.iter().cloned().collect();
    for key in &graph.selected {
        order(key, graph, &mut done, &mut Default::default(), &mut ordered)?;
    }
    for (module, symbol) in &ordered {
        emitter.module = module.clone();
        let d = &graph.modules[module].definitions[symbol];
        let expr = emitter.expr(d)?;
        initialize += &format!(
            "{{let scope=scopes[{}];cx.define(scope,{},{});}}\n",
            modules[module],
            q(symbol),
            expr
        );
    }
    let mut aliases = String::new();
    for ((module, local), (target, symbol)) in &graph.links {
        aliases += &format!(
            "cx.link(scopes[{}],{},scopes[{}],{});\n",
            modules[module],
            q(local),
            modules[target],
            q(symbol)
        );
    }
    let mut namespaces = String::new();
    for ((module, local), members) in &graph.namespaces {
        namespaces += &format!("{{let namespace=cx.object();");
        for (name, (target, symbol)) in members {
            namespaces += &format!(
                "cx.put(&namespace,{},cx.lookup(scopes[{}],{})?)?;",
                q(name),
                modules[target],
                q(symbol)
            );
        }
        namespaces += &format!(
            "cx.define(scopes[{}],{},namespace);}}\n",
            modules[module],
            q(local)
        );
    }
    let mut exports = String::new();
    for e in &manifest.entries {
        exports += &format!(
            "exports.insert({}.to_string(),cx.lookup(scopes[{}],{})?);\n",
            q(e.public_name()),
            modules[&e.module],
            q(&e.symbol)
        );
    }
    Ok(format!("// Generated from Okta Auth JS {} ({}). DO NOT EDIT.\n// Upstream source: Apache-2.0; see LICENSE-APACHE-OKTA.\n#![allow(unused_variables,unused_mut,unused_labels,unused_assignments,unreachable_code)]\nuse crate::runtime::{{Engine,V,JResult,binary,Completion}};\npub const UPSTREAM_COMMIT:&str={:?};\n{}\npub fn initialize(cx:&Engine)->Result<std::collections::BTreeMap<String,V>,V>{{\nlet this=V::Undefined;let scopes:Vec<_>=(0..{}).map(|_|cx.scope(None)).collect();\n{aliases}{initialize}{namespaces}let mut exports=std::collections::BTreeMap::new();{exports}Ok(exports)}}\n",manifest.upstream_version,manifest.upstream_commit,manifest.upstream_commit,emitter.functions.join("\n"),modules.len()))
}
