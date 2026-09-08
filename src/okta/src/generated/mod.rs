// Generated from Okta Auth JS 8.0.1 (f30496d66c42e1d0a4de56b00b1e4041dbd543ab). DO NOT EDIT.
// Upstream source: Apache-2.0; see vendor/okta-auth-js/LICENSE.
#![allow(
    unused_variables,
    unused_mut,
    unused_labels,
    unused_assignments,
    unreachable_code
)]
use okta_native_support::{binary, Completion, Engine, JResult, V};
pub const UPSTREAM_COMMIT: &str = "f30496d66c42e1d0a4de56b00b1e4041dbd543ab";
fn f0(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "b64", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = {
            let receiver = {
                let receiver = cx.lookup(scope, "b64")?;
                let method = "replace";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.regexp("\\+", "g")?);
                    argv.push(V::string("-"));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "replace";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.regexp("\\/", "g")?);
                argv.push(V::string("_"));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "replace";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.regexp("=+$", "")?);
            argv.push(V::string(""));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f1(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let function = cx.lookup(scope, "atob")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let function = cx.lookup(scope, "base64UrlToBase64")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "str")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f2(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "b64u", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = {
            let receiver = cx.lookup(scope, "b64u")?;
            let method = "replace";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.regexp("-", "g")?);
                argv.push(V::string("+"));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "replace";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.regexp("_", "g")?);
            argv.push(V::string("/"));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f4(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "c", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = cx.lookup(scope, "c")?;
        let method = "charCodeAt";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(0f64));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f3(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "b64u", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = cx.lookup(scope, "Uint8Array")?;
        let method = "from";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let function = cx.lookup(scope, "base64UrlDecode")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "b64u")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            argv.push(cx.arrow(scope, this.clone(), f4));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f5(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "b64u", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "b64", {
        let function = cx.lookup(scope, "base64UrlToBase64")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "b64u")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    'switch6: {
        let discriminant = binary(
            "%",
            cx.get(&cx.lookup(scope, "b64")?, &"length")?,
            V::Number(4f64),
        )?;
        let mut matched = false;
        if matched || okta_native_support::eq(&discriminant, &V::Number(0f64)) {
            matched = true;
            break 'switch6;
        }
        if matched || okta_native_support::eq(&discriminant, &V::Number(2f64)) {
            matched = true;
            let _ = cx.assign(
                scope,
                "b64",
                binary("+", cx.lookup(scope, "b64")?, V::string("=="))?,
            )?;
            break 'switch6;
        }
        if matched || okta_native_support::eq(&discriminant, &V::Number(3f64)) {
            matched = true;
            let _ = cx.assign(
                scope,
                "b64",
                binary("+", cx.lookup(scope, "b64")?, V::string("="))?,
            )?;
            break 'switch6;
        }
        if matched || true {
            matched = true;
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string("Not a valid Base64Url"));
                argv
            })?);
        }
    }
    cx.define(scope, "utf8", {
        let function = cx.lookup(scope, "atob")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "b64")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    {
        let mut completion = (|| {
            {
                let scope = cx.scope(Some(scope));
                return Ok(Completion::Return({
                    let function = cx.lookup(scope, "decodeURIComponent")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let function = cx.lookup(scope, "escape")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "utf8")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        });
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }));
            }
            Ok::<Completion, V>(Completion::Normal)
        })();
        if let Err(error) = completion {
            completion = (|| {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "e", error);
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(Completion::Return(cx.lookup(scope, "utf8")?));
                }
                Ok::<Completion, V>(Completion::Normal)
            })();
        }
        match completion? {
            Completion::Normal => {}
            Completion::Return(value) => {
                return Ok(value);
            }
            Completion::Break => {
                return Err(V::string("unexpected break"));
            }
            Completion::Continue => {
                return Err(V::string("unexpected continue"));
            }
        }
    }

    Ok(V::Undefined)
}

fn f8(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "s", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "byte", args.get(1).cloned().unwrap_or(V::Undefined));
    return Ok(binary("+", cx.lookup(scope, "s")?, {
        let receiver = cx.lookup(scope, "String")?;
        let method = "fromCharCode";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "byte")?);
            argv
        };
        cx.call(function, receiver, argv)?
    })?);

    Ok(V::Undefined)
}

fn f7(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "bin", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let function = cx.lookup(scope, "btoa")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let receiver = cx.construct(cx.lookup(scope, "Uint8Array")?, {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "bin")?);
                    argv
                })?;
                let method = "reduce";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f8));
                    argv.push(V::string(""));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f9(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "b64", {
        let function = cx.lookup(scope, "btoa")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "str")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let function = cx.lookup(scope, "base64ToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "b64")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f10(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "buffer",
        cx.construct(cx.lookup(scope, "Uint8Array")?, {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "str")?, &"length")?);
            argv
        })?,
    );
    {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "i", V::Number(0f64));
        let mut first = true;
        loop {
            if !first {
                let _ = {
                    let old = cx.lookup(scope, "i")?;
                    let updated =
                        cx.assign(scope, "i", binary("+", old.clone(), V::Number(1.0))?)?;
                    old
                };
            }
            first = false;
            if !binary(
                "<",
                cx.lookup(scope, "i")?,
                cx.get(&cx.lookup(scope, "str")?, &"length")?,
            )?
            .truthy()
            {
                break;
            }
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let object = cx.lookup(scope, "buffer")?;
                    let key = cx.lookup(scope, "i")?.text();
                    let value = {
                        let receiver = cx.lookup(scope, "str")?;
                        let method = "charCodeAt";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "i")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                    cx.put(&object, &key, value)?
                };
            }
        }
    }
    return Ok(cx.lookup(scope, "buffer")?);

    Ok(V::Undefined)
}

fn f12(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "arrayBuffer",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "intBuffer",
        cx.construct(cx.lookup(scope, "Uint8Array")?, {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "arrayBuffer")?);
            argv
        })?,
    );
    cx.define(scope, "firstHalf", {
        let receiver = cx.lookup(scope, "intBuffer")?;
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(0f64));
            argv.push(V::Number(16f64));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "hash", {
        let receiver = cx.get(&cx.lookup(scope, "String")?, &"fromCharCode")?;
        let method = "apply";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Null);
            argv.push(cx.lookup(scope, "firstHalf")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "b64u", {
        let function = cx.lookup(scope, "stringToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "hash")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok(cx.lookup(scope, "b64u")?);

    Ok(V::Undefined)
}

fn f11(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "buffer", {
        let receiver = cx.construct(cx.lookup(scope, "TextEncoder")?, {
            let mut argv = Vec::new();
            argv
        })?;
        let method = "encode";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "str")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok({
        let receiver = {
            let receiver = cx.get(&cx.lookup(scope, "crypto")?, &"subtle")?;
            let method = "digest";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("SHA-256"));
                argv.push(cx.lookup(scope, "buffer")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f12));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f14(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "cryptoKey",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "jwt", {
        let receiver = cx.lookup(scope, "idToken")?;
        let method = "split";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::string("."));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "payload", {
        let function = cx.lookup(scope, "stringToBuffer")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(binary(
                "+",
                binary(
                    "+",
                    cx.get(&cx.lookup(scope, "jwt")?, &V::Number(0f64).text())?,
                    V::string("."),
                )?,
                cx.get(&cx.lookup(scope, "jwt")?, &V::Number(1f64).text())?,
            )?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "b64Signature", {
        let function = cx.lookup(scope, "base64UrlDecode")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "jwt")?, &V::Number(2f64).text())?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "signature", {
        let function = cx.lookup(scope, "stringToBuffer")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "b64Signature")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let receiver = cx.get(&cx.lookup(scope, "crypto")?, &"subtle")?;
        let method = "verify";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "algo")?);
            argv.push(cx.lookup(scope, "cryptoKey")?);
            argv.push(cx.lookup(scope, "signature")?);
            argv.push(cx.lookup(scope, "payload")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f13(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idToken",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "key", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = cx.assign(scope, "key", {
        let function = cx.lookup(scope, "clone")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "key")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    })?;
    cx.define(scope, "algo", {
        let object = cx.object();
        cx.put(&object, &"name", V::string("RSASSA-PKCS1-v1_5"))?;
        cx.put(&object, &"hash", {
            let object = cx.object();
            cx.put(&object, &"name", V::string("SHA-256"))?;
            object
        })?;
        object
    });
    let _ = cx.delete(&cx.lookup(scope, "key")?, &"use")?;
    return Ok({
        let receiver = {
            let receiver = cx.get(&cx.lookup(scope, "crypto")?, &"subtle")?;
            let method = "importKey";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("jwk"));
                argv.push(cx.lookup(scope, "key")?);
                argv.push(cx.lookup(scope, "algo")?);
                argv.push(V::Bool(true));
                argv.push({
                    let mut values = vec![];
                    values.push(V::string("verify"));
                    cx.array(values)
                });
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f14));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f15(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(cx.relation(
        "instanceof",
        cx.lookup(scope, "obj")?,
        cx.lookup(scope, "AuthApiError")?,
    )?);

    Ok(V::Undefined)
}

fn f16(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(cx.relation(
        "instanceof",
        cx.lookup(scope, "obj")?,
        cx.lookup(scope, "OAuthError")?,
    )?);

    Ok(V::Undefined)
}

fn f17(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(cx.relation(
        "instanceof",
        cx.lookup(scope, "obj")?,
        cx.lookup(scope, "WWWAuthError")?,
    )?);

    Ok(V::Undefined)
}

fn f19(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "param", cx.get(&binding, "0")?);
        cx.define(scope, "value", cx.get(&binding, "1")?);
    }
    return Ok({
        let mut text = String::new();
        text.push_str("");
        text.push_str(&cx.lookup(scope, "param")?.text());
        text.push_str("=");
        text.push_str(
            &{
                let function = cx.lookup(scope, "encodeURIComponent")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "value")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
            .text(),
        );
        text.push_str("");
        V::string(text)
    });

    Ok(V::Undefined)
}

fn f21(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "result",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = cx.lookup(scope, "error")?;
        if left.truthy() {
            left
        } else {
            'chain22: {
                {
                    let object = cx.get(&cx.lookup(scope, "result")?, &"responseJSON")?;
                    if object.nullish() {
                        break 'chain22 V::Undefined;
                    }
                    cx.get(&object, &"error")?
                }
            }
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.lookup(scope, "result")?);
        }
    }
    return Ok(cx.lookup(scope, "result")?);

    Ok(V::Undefined)
}

fn f23(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "data", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let function = cx.lookup(scope, "formatResult")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "status")?);
            argv.push(cx.lookup(scope, "data")?);
            argv.push(cx.lookup(scope, "response")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f20(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "response",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "error",
        V::Bool(!cx.get(&cx.lookup(scope, "response")?, &"ok")?.truthy()),
    );
    cx.define(
        scope,
        "status",
        cx.get(&cx.lookup(scope, "response")?, &"status")?,
    );
    return Ok({
        let receiver = {
            let receiver = {
                let function = cx.lookup(scope, "readData")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "response")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            let method = "then";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f23));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f21));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f18(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "method",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "url", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "args", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "body", cx.get(&cx.lookup(scope, "args")?, &"data")?);
    cx.define(scope, "headers", {
        let left = cx.get(&cx.lookup(scope, "args")?, &"headers")?;
        if left.truthy() {
            left
        } else {
            {
                let object = cx.object();
                object
            }
        }
    });
    cx.define(scope, "contentType", {
        let left = {
            let left = cx.get(
                &cx.lookup(scope, "headers")?,
                &V::string("Content-Type").text(),
            )?;
            if left.truthy() {
                left
            } else {
                cx.get(
                    &cx.lookup(scope, "headers")?,
                    &V::string("content-type").text(),
                )?
            }
        };
        if left.truthy() {
            left
        } else {
            V::string("")
        }
    });
    if {
        let left = cx.lookup(scope, "body")?;
        if left.truthy() {
            binary(
                "!==",
                V::string(cx.lookup(scope, "body")?.typeof_()),
                V::string("string"),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            if {
                let receiver = cx.lookup(scope, "appJsonContentTypeRegex")?;
                let method = "test";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "contentType")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = cx.assign(scope, "body", {
                        let receiver = cx.lookup(scope, "JSON")?;
                        let method = "stringify";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "body")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    })?;
                }
            } else {
                if binary(
                    "===",
                    cx.lookup(scope, "contentType")?,
                    V::string("application/x-www-form-urlencoded"),
                )?
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "body", {
                            let receiver = {
                                let receiver = {
                                    let receiver = cx.lookup(scope, "Object")?;
                                    let method = "entries";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "body")?);
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                };
                                let method = "map";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.arrow(scope, this.clone(), f19));
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                            let method = "join";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(V::string("&"));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        })?;
                    }
                }
            }
        }
    }
    cx.define(
        scope,
        "fetch",
        cx.get(&cx.lookup(scope, "global")?, &"fetch")?,
    );
    cx.define(scope, "fetchPromise", {
        let function = cx.lookup(scope, "fetch")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "url")?);
            argv.push({
                let object = cx.object();
                cx.put(&object, &"method", cx.lookup(scope, "method")?)?;
                cx.put(
                    &object,
                    &"headers",
                    cx.get(&cx.lookup(scope, "args")?, &"headers")?,
                )?;
                cx.put(&object, &"body", cx.lookup(scope, "body")?)?;
                cx.put(
                    &object,
                    &"credentials",
                    if cx
                        .get(&cx.lookup(scope, "args")?, &"withCredentials")?
                        .truthy()
                    {
                        V::string("include")
                    } else {
                        V::string("omit")
                    },
                )?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if V::Bool(
        !cx.get(&cx.lookup(scope, "fetchPromise")?, &"finally")?
            .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "fetchPromise", {
                let receiver = cx.lookup(scope, "Promise")?;
                let method = "resolve";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "fetchPromise")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    }
    return Ok({
        let receiver = cx.lookup(scope, "fetchPromise")?;
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f20));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f24(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "status",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "response",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "isObject",
        binary(
            "===",
            V::string(cx.lookup(scope, "data")?.typeof_()),
            V::string("object"),
        )?,
    );
    cx.define(scope, "headers", {
        let object = cx.object();
        object
    });
    for item in cx.items(&{
        let receiver = cx.get(&cx.lookup(scope, "response")?, &"headers")?;
        let method = "entries";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    })? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "pair", item);
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "headers")?;
                let key = cx
                    .get(&cx.lookup(scope, "pair")?, &V::Number(0f64).text())?
                    .text();
                let value = cx.get(&cx.lookup(scope, "pair")?, &V::Number(1f64).text())?;
                cx.put(&object, &key, value)?
            };
        }
    }
    cx.define(scope, "result", {
        let object = cx.object();
        cx.put(
            &object,
            &"responseText",
            if cx.lookup(scope, "isObject")?.truthy() {
                {
                    let receiver = cx.lookup(scope, "JSON")?;
                    let method = "stringify";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "data")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
            } else {
                cx.lookup(scope, "data")?
            },
        )?;
        cx.put(&object, &"status", cx.lookup(scope, "status")?)?;
        cx.put(&object, &"headers", cx.lookup(scope, "headers")?)?;
        object
    });
    if cx.lookup(scope, "isObject")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "result")?;
                let key = "responseType";
                let value = V::string("json");
                cx.put(&object, &key, value)?
            };
            let _ = {
                let object = cx.lookup(scope, "result")?;
                let key = "responseJSON";
                let value = cx.lookup(scope, "data")?;
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok(cx.lookup(scope, "result")?);

    Ok(V::Undefined)
}

fn f26(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "e", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let object = cx.object();
        cx.put(&object, &"error", cx.lookup(scope, "e")?)?;
        cx.put(
            &object,
            &"errorSummary",
            V::string("Could not parse server response"),
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f25(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "response",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = {
            let receiver = cx.get(&cx.lookup(scope, "response")?, &"headers")?;
            let method = "get";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("Content-Type"));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        if left.truthy() {
            binary(
                ">=",
                {
                    let receiver = {
                        let receiver = {
                            let receiver = cx.get(&cx.lookup(scope, "response")?, &"headers")?;
                            let method = "get";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(V::string("Content-Type"));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        };
                        let method = "toLowerCase";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                    let method = "indexOf";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::string("application/json"));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::Number(0f64),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let receiver = {
                    let receiver = cx.lookup(scope, "response")?;
                    let method = "json";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
                let method = "catch";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f26));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let receiver = cx.lookup(scope, "response")?;
                let method = "text";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            });
        }
    }

    Ok(V::Undefined)
}

fn f27(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "url", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.assign(
        scope,
        "url",
        if {
            let function = cx.lookup(scope, "isAbsoluteUrl")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "url")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy()
        {
            cx.lookup(scope, "url")?
        } else {
            binary(
                "+",
                {
                    let receiver = cx.lookup(scope, "sdk")?;
                    let method = "getIssuerOrigin";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                cx.lookup(scope, "url")?,
            )?
        },
    )?;
    cx.define(scope, "getOptions", {
        let object = cx.object();
        cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
        cx.put(&object, &"method", V::string("GET"))?;
        object
    });
    let _ = {
        let receiver = cx.lookup(scope, "Object")?;
        let method = "assign";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "getOptions")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok({
        let function = cx.lookup(scope, "httpRequest")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "getOptions")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f29(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "resp", args.get(0).cloned().unwrap_or(V::Undefined));
    let _ = cx.assign(scope, "err", {
        let function = cx.lookup(scope, "formatError")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "resp")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    })?;
    if binary(
        "===",
        cx.get(&cx.lookup(scope, "err")?, &"errorCode")?,
        V::string("E0000011"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "storage")?;
                let method = "delete";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "STATE_TOKEN_KEY_NAME")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Err(cx.lookup(scope, "err")?);

    Ok(V::Undefined)
}

fn f31(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "item", args.get(0).cloned().unwrap_or(V::Undefined));
    let _ = {
        let object = cx.lookup(scope, "item")?;
        let key = "headers";
        let value = cx.get(&cx.lookup(scope, "resp")?, &"headers")?;
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f30(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "resp", args.get(0).cloned().unwrap_or(V::Undefined));
    let _ = cx.assign(
        scope,
        "res",
        cx.get(&cx.lookup(scope, "resp")?, &"responseText")?,
    )?;
    if {
        let left = cx.lookup(scope, "res")?;
        if left.truthy() {
            {
                let function = cx.lookup(scope, "isString")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "res")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "res", {
                let receiver = cx.lookup(scope, "JSON")?;
                let method = "parse";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "res")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
            if {
                let left = {
                    let left = cx.lookup(scope, "res")?;
                    if left.truthy() {
                        binary(
                            "===",
                            V::string(cx.lookup(scope, "res")?.typeof_()),
                            V::string("object"),
                        )?
                    } else {
                        left
                    }
                };
                if left.truthy() {
                    V::Bool(!cx.get(&cx.lookup(scope, "res")?, &"headers")?.truthy())
                } else {
                    left
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    if {
                        let receiver = cx.lookup(scope, "Array")?;
                        let method = "isArray";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "res")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let receiver = cx.lookup(scope, "res")?;
                                let method = "forEach";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.arrow(scope, this.clone(), f31));
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                        }
                    } else {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "res")?;
                                let key = "headers";
                                let value = cx.get(&cx.lookup(scope, "resp")?, &"headers")?;
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                }
            }
        }
    }
    if cx.lookup(scope, "saveAuthnState")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            if V::Bool(!cx.get(&cx.lookup(scope, "res")?, &"stateToken")?.truthy()).truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let receiver = cx.lookup(scope, "storage")?;
                        let method = "delete";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "STATE_TOKEN_KEY_NAME")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
            }
        }
    }
    if {
        let left = {
            let left = cx.lookup(scope, "res")?;
            if left.truthy() {
                cx.get(&cx.lookup(scope, "res")?, &"stateToken")?
            } else {
                left
            }
        };
        if left.truthy() {
            cx.get(&cx.lookup(scope, "res")?, &"expiresAt")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "storage")?;
                let method = "set";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "STATE_TOKEN_KEY_NAME")?);
                    argv.push(cx.get(&cx.lookup(scope, "res")?, &"stateToken")?);
                    argv.push(cx.get(&cx.lookup(scope, "res")?, &"expiresAt")?);
                    argv.push(cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"cookies")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    if {
        let left = cx.lookup(scope, "res")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "options")?, &"cacheResponse")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "httpCache")?;
                let method = "updateStorage";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "url")?);
                    argv.push({
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"expiresAt",
                            binary(
                                "+",
                                {
                                    let receiver = cx.lookup(scope, "Math")?;
                                    let method = "floor";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(binary(
                                            "/",
                                            {
                                                let receiver = cx.lookup(scope, "Date")?;
                                                let method = "now";
                                                let function =
                                                    cx.resolve_method(&receiver, &method)?;
                                                let argv = {
                                                    let mut argv = Vec::new();
                                                    argv
                                                };
                                                cx.call(function, receiver, argv)?
                                            },
                                            V::Number(1000f64),
                                        )?);
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                },
                                cx.lookup(scope, "DEFAULT_CACHE_DURATION")?,
                            )?,
                        )?;
                        cx.put(&object, &"response", cx.lookup(scope, "res")?)?;
                        object
                    });
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Ok(cx.lookup(scope, "res")?);

    Ok(V::Undefined)
}

fn f28(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.assign(scope, "options", {
        let left = cx.lookup(scope, "options")?;
        if left.truthy() {
            left
        } else {
            {
                let object = cx.object();
                object
            }
        }
    })?;
    if cx
        .get(
            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
            &"httpRequestInterceptors",
        )?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            for item in cx.items(&cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"httpRequestInterceptors",
            )?)? {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "interceptor", item);
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let function = cx.lookup(scope, "interceptor")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "options")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    };
                }
            }
        }
    }
    cx.define(scope, "url", cx.get(&cx.lookup(scope, "options")?, &"url")?);
    cx.define(
        scope,
        "method",
        cx.get(&cx.lookup(scope, "options")?, &"method")?,
    );
    cx.define(
        scope,
        "args",
        cx.get(&cx.lookup(scope, "options")?, &"args")?,
    );
    cx.define(
        scope,
        "saveAuthnState",
        cx.get(&cx.lookup(scope, "options")?, &"saveAuthnState")?,
    );
    cx.define(
        scope,
        "accessToken",
        cx.get(&cx.lookup(scope, "options")?, &"accessToken")?,
    );
    cx.define(
        scope,
        "withCredentials",
        binary(
            "===",
            cx.get(&cx.lookup(scope, "options")?, &"withCredentials")?,
            V::Bool(true),
        )?,
    );
    cx.define(
        scope,
        "storageUtil",
        cx.get(
            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
            &"storageUtil",
        )?,
    );
    cx.define(
        scope,
        "storage",
        cx.get(&cx.lookup(scope, "storageUtil")?, &"storage")?,
    );
    cx.define(scope, "httpCache", {
        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"storageManager")?;
        let method = "getHttpCache";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"cookies")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(
        scope,
        "pollingIntent",
        cx.get(&cx.lookup(scope, "options")?, &"pollingIntent")?,
    );
    cx.define(scope, "pollDelay", {
        let left = cx.get(
            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
            &"pollDelay",
        )?;
        if left.nullish() {
            V::Number(0f64)
        } else {
            left
        }
    });
    if cx
        .get(&cx.lookup(scope, "options")?, &"cacheResponse")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "cacheContents", {
                let receiver = cx.lookup(scope, "httpCache")?;
                let method = "getStorage";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            cx.define(
                scope,
                "cachedResponse",
                cx.get(
                    &cx.lookup(scope, "cacheContents")?,
                    &cx.lookup(scope, "url")?.text(),
                )?,
            );
            if {
                let left = cx.lookup(scope, "cachedResponse")?;
                if left.truthy() {
                    binary(
                        "<",
                        binary(
                            "/",
                            {
                                let receiver = cx.lookup(scope, "Date")?;
                                let method = "now";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            },
                            V::Number(1000f64),
                        )?,
                        cx.get(&cx.lookup(scope, "cachedResponse")?, &"expiresAt")?,
                    )?
                } else {
                    left
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok({
                        let receiver = cx.lookup(scope, "Promise")?;
                        let method = "resolve";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&cx.lookup(scope, "cachedResponse")?, &"response")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                }
            }
        }
    }
    cx.define(scope, "oktaUserAgentHeader", {
        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"_oktaUserAgent")?;
        let method = "getHttpHeader";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "headers", {
        let object = cx.object();
        cx.put(&object, &"Accept", V::string("application/json"))?;
        cx.put(&object, &"Content-Type", V::string("application/json"))?;
        cx.spread(&object, &cx.lookup(scope, "oktaUserAgentHeader")?)?;
        object
    });
    let _ = {
        let receiver = cx.lookup(scope, "Object")?;
        let method = "assign";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "headers")?);
            argv.push(cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"headers")?);
            argv.push(cx.get(&cx.lookup(scope, "options")?, &"headers")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };
    let _ = cx.assign(scope, "headers", {
        let function = cx.lookup(scope, "removeNils")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "headers")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    })?;
    if {
        let left = cx.lookup(scope, "accessToken")?;
        if left.truthy() {
            {
                let function = cx.lookup(scope, "isString")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "accessToken")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "headers")?;
                let key = V::string("Authorization").text();
                let value = binary("+", V::string("Bearer "), cx.lookup(scope, "accessToken")?)?;
                cx.put(&object, &key, value)?
            };
        }
    }
    cx.define(scope, "ajaxOptions", {
        let object = cx.object();
        cx.put(&object, &"headers", cx.lookup(scope, "headers")?)?;
        cx.put(&object, &"data", {
            let left = cx.lookup(scope, "args")?;
            if left.truthy() {
                left
            } else {
                cx.lookup(scope, "undefined")?
            }
        })?;
        cx.put(
            &object,
            &"withCredentials",
            cx.lookup(scope, "withCredentials")?,
        )?;
        object
    });
    cx.define(scope, "err", V::Undefined);
    cx.define(scope, "res", V::Undefined);
    cx.define(scope, "promise", V::Undefined);
    {
        let scope = cx.scope(Some(scope));
        let _ = {
            let left = cx.lookup(scope, "pollingIntent")?;
            if left.truthy() {
                V::Bool(false)
            } else {
                left
            }
        };
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "promise", {
                let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
                let method = "httpRequestClient";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "method")?);
                    argv.push(cx.lookup(scope, "url")?);
                    argv.push(cx.lookup(scope, "ajaxOptions")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    }
    return Ok({
        let receiver = {
            let receiver = cx.lookup(scope, "promise")?;
            let method = "then";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.function(scope, f30));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "catch";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f29));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f32(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "url", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "args", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(3).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.assign(
        scope,
        "url",
        if {
            let function = cx.lookup(scope, "isAbsoluteUrl")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "url")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy()
        {
            cx.lookup(scope, "url")?
        } else {
            binary(
                "+",
                {
                    let receiver = cx.lookup(scope, "sdk")?;
                    let method = "getIssuerOrigin";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                cx.lookup(scope, "url")?,
            )?
        },
    )?;
    cx.define(scope, "postOptions", {
        let object = cx.object();
        cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
        cx.put(&object, &"method", V::string("POST"))?;
        cx.put(&object, &"args", cx.lookup(scope, "args")?)?;
        cx.put(&object, &"saveAuthnState", V::Bool(true))?;
        object
    });
    let _ = {
        let receiver = cx.lookup(scope, "Object")?;
        let method = "assign";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "postOptions")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok({
        let function = cx.lookup(scope, "httpRequest")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "postOptions")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f34(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "options")?);
        argv
    })?;

    Ok(V::Undefined)
}

fn f35(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    let _ = {
        let parent = cx.get(&cx.lookup(scope, "__super")?, "prototype")?;
        let function = cx.resolve_method(&parent, &"clear")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, this.clone(), argv)?
    };
    if binary(
        "!==",
        cx.get(&cx.lookup(scope, "options")?, &"clearIdxResponse")?,
        V::Bool(false),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = this.clone();
                let method = "clearIdxResponse";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f36(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "data", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(!cx.get(&this.clone(), &"saveLastResponse")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    cx.define(scope, "storage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getIdxResponseStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if V::Bool(!cx.lookup(scope, "storage")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    let _ = {
        let receiver = cx.lookup(scope, "storage")?;
        let method = "setStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "data")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f37(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if V::Bool(!cx.get(&this.clone(), &"saveLastResponse")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Null);
        }
    }
    cx.define(scope, "storage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getIdxResponseStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if V::Bool(!cx.lookup(scope, "storage")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Null);
        }
    }
    cx.define(scope, "storedValue", {
        let receiver = cx.lookup(scope, "storage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if {
        let left = V::Bool(!cx.lookup(scope, "storedValue")?.truthy());
        if left.truthy() {
            left
        } else {
            V::Bool(
                !{
                    let function = cx.lookup(scope, "isRawIdxResponse")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.get(&cx.lookup(scope, "storedValue")?, &"rawIdxResponse")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
                .truthy(),
            )
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Null);
        }
    }
    if cx.lookup(scope, "options")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            {
                let binding = cx.lookup(scope, "options")?;
                cx.define(scope, "stateHandle", cx.get(&binding, &"stateHandle")?);
                cx.define(
                    scope,
                    "interactionHandle",
                    cx.get(&binding, &"interactionHandle")?,
                );
            }
            if {
                let left = {
                    let left = V::Bool(
                        !cx.get(&cx.lookup(scope, "options")?, &"useGenericRemediator")?
                            .truthy(),
                    );
                    if left.truthy() {
                        cx.lookup(scope, "stateHandle")?
                    } else {
                        left
                    }
                };
                if left.truthy() {
                    binary(
                        "!==",
                        cx.get(&cx.lookup(scope, "storedValue")?, &"stateHandle")?,
                        cx.lookup(scope, "stateHandle")?,
                    )?
                } else {
                    left
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(V::Null);
                }
            }
            if {
                let left = cx.lookup(scope, "interactionHandle")?;
                if left.truthy() {
                    binary(
                        "!==",
                        cx.get(&cx.lookup(scope, "storedValue")?, &"interactionHandle")?,
                        cx.lookup(scope, "interactionHandle")?,
                    )?
                } else {
                    left
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(V::Null);
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "storedValue")?);

    Ok(V::Undefined)
}

fn f38(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    if V::Bool(!cx.get(&this.clone(), &"saveLastResponse")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    cx.define(scope, "storage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getIdxResponseStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = 'chain39: {
        {
            let receiver = cx.lookup(scope, "storage")?;
            if receiver.nullish() {
                break 'chain39 V::Undefined;
            }
            let method = "clearStorage";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        }
    };

    Ok(V::Undefined)
}

fn f33(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "TransactionManager", {
        let function = cx.lookup(scope, "createTransactionManager")?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let superclass = cx.lookup(scope, "TransactionManager")?;
        let scope = cx.scope(Some(scope));
        cx.define(scope, "__super", superclass.clone());
        let constructor = cx.function(scope, f34);
        let prototype = cx.object();
        if !superclass.nullish() {
            let parent = cx.get(&superclass, "prototype")?;
            if !parent.nullish() {
                cx.set_prototype(&prototype, parent)?;
            }
            if matches!(superclass, V::Function(_)) {
                cx.set_prototype(&constructor, superclass)?;
            }
        }
        cx.put(&constructor, "prototype", prototype.clone())?;
        cx.define(scope, "IdxTransactionManager", constructor.clone());
        cx.put(&prototype, &"clear", cx.function(scope, f35))?;
        cx.put(&prototype, &"saveIdxResponse", cx.function(scope, f36))?;
        cx.put(&prototype, &"loadIdxResponse", cx.function(scope, f37))?;
        cx.put(&prototype, &"clearIdxResponse", cx.function(scope, f38))?;
        constructor
    });

    Ok(V::Undefined)
}

fn f40(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "auth1", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "auth2", args.get(1).cloned().unwrap_or(V::Undefined));
    if {
        let left = V::Bool(!cx.lookup(scope, "auth1")?.truthy());
        if left.truthy() {
            left
        } else {
            V::Bool(!cx.lookup(scope, "auth2")?.truthy())
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    if {
        let left = cx.get(&cx.lookup(scope, "auth1")?, &"id")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "auth2")?, &"id")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(binary(
                "===",
                cx.get(&cx.lookup(scope, "auth1")?, &"id")?,
                cx.get(&cx.lookup(scope, "auth2")?, &"id")?,
            )?);
        }
    }
    if {
        let left = cx.get(&cx.lookup(scope, "auth1")?, &"key")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "auth2")?, &"key")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(binary(
                "===",
                cx.get(&cx.lookup(scope, "auth1")?, &"key")?,
                cx.get(&cx.lookup(scope, "auth2")?, &"key")?,
            )?);
        }
    }
    return Ok(V::Bool(false));

    Ok(V::Undefined)
}

fn f41(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "incoming",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "authenticator", V::Undefined);
    if {
        let function = cx.lookup(scope, "isAuthenticator")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "incoming")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "authenticator", cx.lookup(scope, "incoming")?)?;
        }
    } else {
        if binary(
            "===",
            V::string(cx.lookup(scope, "incoming")?.typeof_()),
            V::string("string"),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "authenticator", {
                    let object = cx.object();
                    cx.put(&object, &"key", cx.lookup(scope, "incoming")?)?;
                    object
                })?;
            }
        } else {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "Error")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("Invalid format for authenticator"));
                    argv
                })?);
            }
        }
    }
    return Ok(cx.lookup(scope, "authenticator")?);

    Ok(V::Undefined)
}

fn f42(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "search",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        if {
            let function = cx.lookup(scope, "isEmailVerifyCallback")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "search")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                {
                    let binding = {
                        let function = cx.lookup(scope, "parseEmailVerifyCallback")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "search")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    };
                    cx.define(scope, "state", cx.get(&binding, &"state")?);
                    cx.define(scope, "otp", cx.get(&binding, &"otp")?);
                }
                if {
                    let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                    let method = "canProceed";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let object = cx.object();
                            cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
                            object
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        return Ok(cx
                            .await_value({
                                let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                                let method = "proceed";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push({
                                        let object = cx.object();
                                        cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
                                        cx.put(&object, &"otp", cx.lookup(scope, "otp")?)?;
                                        object
                                    });
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            })
                            .await?);
                    }
                } else {
                    {
                        let scope = cx.scope(Some(scope));
                        return Err(cx.construct(
                            cx.lookup(scope, "EmailVerifyCallbackError")?,
                            {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "state")?);
                                argv.push(cx.lookup(scope, "otp")?);
                                argv
                            },
                        )?);
                    }
                }
            }
        }
        Ok(V::Undefined)
    })
}

fn f43(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "urlPath",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let left = {
            let receiver = cx.regexp("(otp=)", "i")?;
            let method = "test";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "urlPath")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        if left.truthy() {
            {
                let receiver = cx.regexp("(state=)", "i")?;
                let method = "test";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "urlPath")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f44(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "urlPath",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let function = cx.lookup(scope, "urlParamsToObject")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "urlPath")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f45(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "url", args.get(1).cloned().unwrap_or(V::Undefined));
        cx.define(scope, "meta", {
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
            let method = "load";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        });
        if V::Bool(!cx.lookup(scope, "meta")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("No transaction data was found in storage"));
                    argv
                })?);
            }
        }
        {
            let binding = cx.lookup(scope, "meta")?;
            cx.define(scope, "codeVerifier", cx.get(&binding, &"codeVerifier")?);
            cx.define(scope, "savedState", cx.get(&binding, &"state")?);
        }
        {
            let binding = cx.construct(cx.lookup(scope, "URL")?, {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "url")?);
                argv
            })?;
            cx.define(scope, "searchParams", cx.get(&binding, &"searchParams")?);
        }
        cx.define(scope, "state", {
            let receiver = cx.lookup(scope, "searchParams")?;
            let method = "get";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("state"));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.define(scope, "interactionCode", {
            let receiver = cx.lookup(scope, "searchParams")?;
            let method = "get";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("interaction_code"));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.define(scope, "error", {
            let receiver = cx.lookup(scope, "searchParams")?;
            let method = "get";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("error"));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        if cx.lookup(scope, "error")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "OAuthError")?, {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "error")?);
                    argv.push({
                        let receiver = cx.lookup(scope, "searchParams")?;
                        let method = "get";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(V::string("error_description"));
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                    argv
                })?);
            }
        }
        if binary(
            "!==",
            cx.lookup(scope, "state")?,
            cx.lookup(scope, "savedState")?,
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string(
                        "State in redirect uri does not match with transaction state",
                    ));
                    argv
                })?);
            }
        }
        if V::Bool(!cx.lookup(scope, "interactionCode")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("Unable to parse interaction_code from the url"));
                    argv
                })?);
            }
        }
        {
            let binding = cx
                .await_value({
                    let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"token")?;
                    let method = "exchangeCodeForTokens";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let object = cx.object();
                            cx.put(
                                &object,
                                &"interactionCode",
                                cx.lookup(scope, "interactionCode")?,
                            )?;
                            cx.put(&object, &"codeVerifier", cx.lookup(scope, "codeVerifier")?)?;
                            object
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                })
                .await?;
            cx.define(scope, "tokens", cx.get(&binding, &"tokens")?);
        }
        let _ = {
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"tokenManager")?;
            let method = "setTokens";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "tokens")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        Ok(V::Undefined)
    })
}

fn f46(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "rawIdxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "toPersist",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "requestDidSucceed",
        args.get(3).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "version", {
        let left = 'chain47: {
            {
                let object = cx.lookup(scope, "rawIdxResponse")?;
                if object.nullish() {
                    break 'chain47 V::Undefined;
                }
                cx.get(&object, &"version")?
            }
        };
        if left.nullish() {
            cx.lookup(scope, "IDX_API_VERSION")?
        } else {
            left
        }
    });
    let _ = {
        let function = cx.lookup(scope, "validateVersionConfig")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "version")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    };
    {
        let binding = {
            let function = cx.lookup(scope, "parsersForVersion")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "version")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        cx.define(scope, "makeIdxState", cx.get(&binding, &"makeIdxState")?);
    }
    return Ok({
        let function = cx.lookup(scope, "makeIdxState")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push(cx.lookup(scope, "rawIdxResponse")?);
            argv.push(cx.lookup(scope, "toPersist")?);
            argv.push(cx.lookup(scope, "requestDidSucceed")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f48(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "version",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if V::Bool(!cx.lookup(scope, "version")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "Error")?, {
                let mut argv = Vec::new();
                argv.push(V::string("version is required"));
                argv
            })?);
        }
    }
    cx.define(scope, "cleanVersion", {
        let receiver = {
            let left = cx.lookup(scope, "version")?;
            if left.nullish() {
                V::string("")
            } else {
                left
            }
        };
        let method = "replace";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.regexp("[^0-9a-zA-Z._-]", "")?);
            argv.push(V::string(""));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if {
        let left = binary(
            "!==",
            cx.lookup(scope, "cleanVersion")?,
            cx.lookup(scope, "version")?,
        )?;
        if left.truthy() {
            left
        } else {
            V::Bool(!cx.lookup(scope, "version")?.truthy())
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "Error")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "invalid version supplied - version is required and uses semver syntax",
                ));
                argv
            })?);
        }
    }
    let _ = {
        let function = cx.lookup(scope, "parsersForVersion")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "version")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    };

    Ok(V::Undefined)
}

fn f51(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok(binary(
        "===",
        cx.get(&cx.lookup(scope, "remediation")?, &"name")?,
        cx.lookup(scope, "remediationChoice")?,
    )?);

    Ok(V::Undefined)
}

fn f50(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "remediationChoice",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "paramsFromUser", value);
        }
        cx.define(scope, "remediationChoiceObject", {
            let receiver = cx.lookup(scope, "remediations")?;
            let method = "find";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f51));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        if V::Bool(!cx.lookup(scope, "remediationChoiceObject")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let receiver = cx.lookup(scope, "Promise")?;
                    let method = "reject";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let mut text = String::new();
                            text.push_str("Unknown remediation choice: [");
                            text.push_str(&cx.lookup(scope, "remediationChoice")?.text());
                            text.push_str("]");
                            V::string(text)
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
            }
        }
        cx.define(
            scope,
            "actionFn",
            cx.get(&cx.lookup(scope, "remediationChoiceObject")?, &"action")?,
        );
        if binary(
            "!==",
            V::string(cx.lookup(scope, "actionFn")?.typeof_()),
            V::string("function"),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let receiver = cx.lookup(scope, "Promise")?;
                    let method = "reject";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let mut text = String::new();
                            text.push_str("Current remediation cannot make form submit action: [");
                            text.push_str(&cx.lookup(scope, "remediationChoice")?.text());
                            text.push_str("]");
                            V::string(text)
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
            }
        }
        return Ok({
            let receiver = cx.lookup(scope, "remediationChoiceObject")?;
            let method = "action";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "paramsFromUser")?);
                argv
            };
            cx.call(function, receiver, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f52(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "item", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "===",
        cx.get(&cx.lookup(scope, "item")?, &"name")?,
        V::string("interaction_code"),
    )?);

    Ok(V::Undefined)
}

fn f49(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "toPersist",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "requestDidSucceed",
        args.get(3).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "rawIdxResponse", cx.lookup(scope, "idxResponse")?);
    {
        let binding = {
            let function = cx.lookup(scope, "parseIdxResponse")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv.push(cx.lookup(scope, "toPersist")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        cx.define(scope, "remediations", cx.get(&binding, &"remediations")?);
        cx.define(scope, "context", cx.get(&binding, &"context")?);
        cx.define(scope, "actions", cx.get(&binding, &"actions")?);
    }
    cx.define(scope, "neededToProceed", {
        let mut values = vec![];
        values.extend(cx.items(&cx.lookup(scope, "remediations")?)?);
        cx.array(values)
    });
    cx.define(scope, "proceed", cx.async_function(scope, f50));
    cx.define(scope, "findCode", cx.arrow(scope, this.clone(), f52));
    cx.define(scope, "interactionCode", 'chain53: {
        {
            let object = {
                let receiver = {
                    let object = cx.get(
                        &cx.lookup(scope, "rawIdxResponse")?,
                        &"successWithInteractionCode",
                    )?;
                    if object.nullish() {
                        break 'chain53 V::Undefined;
                    }
                    cx.get(&object, &"value")?
                };
                if receiver.nullish() {
                    break 'chain53 V::Undefined;
                }
                let method = "find";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "findCode")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            if object.nullish() {
                break 'chain53 V::Undefined;
            }
            cx.get(&object, &"value")?
        }
    });
    return Ok({
        let object = cx.object();
        cx.put(&object, &"proceed", cx.lookup(scope, "proceed")?)?;
        cx.put(
            &object,
            &"neededToProceed",
            cx.lookup(scope, "neededToProceed")?,
        )?;
        cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
        cx.put(&object, &"context", cx.lookup(scope, "context")?)?;
        cx.put(&object, &"rawIdxState", cx.lookup(scope, "rawIdxResponse")?)?;
        cx.put(
            &object,
            &"interactionCode",
            cx.lookup(scope, "interactionCode")?,
        )?;
        cx.put(&object, &"toPersist", cx.lookup(scope, "toPersist")?)?;
        cx.put(
            &object,
            &"requestDidSucceed",
            cx.lookup(scope, "requestDidSucceed")?,
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f54(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "meta", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let object = cx.object();
        cx.put(&object, &"meta", cx.lookup(scope, "meta")?)?;
        cx.put(
            &object,
            &"interactionHandle",
            cx.get(&cx.lookup(scope, "meta")?, &"interactionHandle")?,
        )?;
        cx.put(
            &object,
            &"state",
            cx.get(&cx.lookup(scope, "meta")?, &"state")?,
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f55(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        let _ = cx.assign(scope, "options", {
            let function = cx.lookup(scope, "removeNils")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        })?;
        cx.define(scope, "meta", {
            let function = cx.lookup(scope, "getSavedTransactionMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        if 'chain56: {
            {
                let object = cx.lookup(scope, "meta")?;
                if object.nullish() {
                    break 'chain56 V::Undefined;
                }
                cx.get(&object, &"interactionHandle")?
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let function = cx.lookup(scope, "getResponse")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "meta")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
            }
        }
        let _ = cx.assign(
            scope,
            "meta",
            cx.await_value({
                let function = cx.lookup(scope, "createTransactionMeta")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push({
                        let object = cx.object();
                        cx.spread(&object, &cx.lookup(scope, "meta")?)?;
                        cx.spread(&object, &cx.lookup(scope, "options")?)?;
                        object
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        )?;
        cx.define(scope, "baseUrl", {
            let function = cx.lookup(scope, "getOAuthBaseUrl")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        {
            let binding = cx.lookup(scope, "meta")?;
            cx.define(scope, "clientId", cx.get(&binding, &"clientId")?);
            cx.define(scope, "redirectUri", cx.get(&binding, &"redirectUri")?);
            cx.define(scope, "state", cx.get(&binding, &"state")?);
            cx.define(scope, "scopes", cx.get(&binding, &"scopes")?);
            cx.define(
                scope,
                "withCredentials",
                cx.get(&binding, &"withCredentials")?,
            );
            cx.define(scope, "codeChallenge", cx.get(&binding, &"codeChallenge")?);
            cx.define(
                scope,
                "codeChallengeMethod",
                cx.get(&binding, &"codeChallengeMethod")?,
            );
            cx.define(
                scope,
                "activationToken",
                cx.get(&binding, &"activationToken")?,
            );
            cx.define(scope, "recoveryToken", cx.get(&binding, &"recoveryToken")?);
            cx.define(scope, "maxAge", cx.get(&binding, &"maxAge")?);
            cx.define(scope, "acrValues", cx.get(&binding, &"acrValues")?);
            cx.define(scope, "nonce", cx.get(&binding, &"nonce")?);
        }
        cx.define(scope, "clientSecret", {
            let left = cx.get(&cx.lookup(scope, "options")?, &"clientSecret")?;
            if left.truthy() {
                left
            } else {
                cx.get(
                    &cx.get(&cx.lookup(scope, "authClient")?, &"options")?,
                    &"clientSecret",
                )?
            }
        });
        let _ = cx.assign(scope, "withCredentials", {
            let left = cx.lookup(scope, "withCredentials")?;
            if left.nullish() {
                V::Bool(true)
            } else {
                left
            }
        })?;
        cx.define(scope, "url", {
            let mut text = String::new();
            text.push_str("");
            text.push_str(&cx.lookup(scope, "baseUrl")?.text());
            text.push_str("/v1/interact");
            V::string(text)
        });
        cx.define(scope, "params", {
            let object = cx.object();
            cx.put(&object, &"client_id", cx.lookup(scope, "clientId")?)?;
            cx.put(&object, &"scope", {
                let receiver = cx.lookup(scope, "scopes")?;
                let method = "join";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string(" "));
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
            cx.put(&object, &"redirect_uri", cx.lookup(scope, "redirectUri")?)?;
            cx.put(
                &object,
                &"code_challenge",
                cx.lookup(scope, "codeChallenge")?,
            )?;
            cx.put(
                &object,
                &"code_challenge_method",
                cx.lookup(scope, "codeChallengeMethod")?,
            )?;
            cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "activationToken")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"activation_token",
                            cx.lookup(scope, "activationToken")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "recoveryToken")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"recovery_token",
                            cx.lookup(scope, "recoveryToken")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "clientSecret")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"client_secret", cx.lookup(scope, "clientSecret")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "maxAge")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"max_age", cx.lookup(scope, "maxAge")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "acrValues")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"acr_values", cx.lookup(scope, "acrValues")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "nonce")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"nonce", cx.lookup(scope, "nonce")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            object
        });
        cx.define(scope, "headers", {
            let object = cx.object();
            cx.put(
                &object,
                &"Content-Type",
                V::string("application/x-www-form-urlencoded"),
            )?;
            object
        });
        cx.define(
            scope,
            "resp",
            cx.await_value({
                let function = cx.lookup(scope, "httpRequest")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push({
                        let object = cx.object();
                        cx.put(&object, &"method", V::string("POST"))?;
                        cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
                        cx.put(&object, &"headers", cx.lookup(scope, "headers")?)?;
                        cx.put(
                            &object,
                            &"withCredentials",
                            cx.lookup(scope, "withCredentials")?,
                        )?;
                        cx.put(&object, &"args", cx.lookup(scope, "params")?)?;
                        object
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        );
        cx.define(
            scope,
            "interactionHandle",
            cx.get(&cx.lookup(scope, "resp")?, &"interaction_handle")?,
        );
        cx.define(scope, "newMeta", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "meta")?)?;
            cx.put(
                &object,
                &"interactionHandle",
                cx.lookup(scope, "interactionHandle")?,
            )?;
            cx.put(
                &object,
                &"withCredentials",
                cx.lookup(scope, "withCredentials")?,
            )?;
            cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
            cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
            cx.put(
                &object,
                &"recoveryToken",
                cx.lookup(scope, "recoveryToken")?,
            )?;
            cx.put(
                &object,
                &"activationToken",
                cx.lookup(scope, "activationToken")?,
            )?;
            object
        });
        let _ = {
            let function = cx.lookup(scope, "saveTransactionMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "newMeta")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        return Ok({
            let function = cx.lookup(scope, "getResponse")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "newMeta")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f57(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        cx.define(scope, "rawIdxResponse", V::Undefined);
        cx.define(scope, "requestDidSucceed", V::Undefined);
        cx.define(scope, "savedIdxResponse", {
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
            let method = "loadIdxResponse";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, receiver, argv)?
        });
        if cx.lookup(scope, "savedIdxResponse")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "rawIdxResponse",
                    cx.get(&cx.lookup(scope, "savedIdxResponse")?, &"rawIdxResponse")?,
                )?;
                let _ = cx.assign(
                    scope,
                    "requestDidSucceed",
                    cx.get(&cx.lookup(scope, "savedIdxResponse")?, &"requestDidSucceed")?,
                )?;
            }
        }
        if V::Bool(!cx.lookup(scope, "rawIdxResponse")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "version", {
                    let left = cx.get(&cx.lookup(scope, "options")?, &"version")?;
                    if left.truthy() {
                        left
                    } else {
                        cx.lookup(scope, "IDX_API_VERSION")?
                    }
                });
                cx.define(scope, "domain", {
                    let function = cx.lookup(scope, "getOAuthDomain")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
                {
                    let binding = cx.lookup(scope, "options")?;
                    cx.define(
                        scope,
                        "interactionHandle",
                        cx.get(&binding, &"interactionHandle")?,
                    );
                    cx.define(scope, "stateHandle", cx.get(&binding, &"stateHandle")?);
                }
                cx.define(scope, "withCredentials", {
                    let left = cx.get(&cx.lookup(scope, "options")?, &"withCredentials")?;
                    if left.nullish() {
                        V::Bool(true)
                    } else {
                        left
                    }
                });
                {
                    let mut completion = (async {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = cx.assign(scope, "requestDidSucceed", V::Bool(true))?;
                            let _ = {
                                let function = cx.lookup(scope, "validateVersionConfig")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "version")?);
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            };
                            cx.define(scope, "url", {
                                let mut text = String::new();
                                text.push_str("");
                                text.push_str(&cx.lookup(scope, "domain")?.text());
                                text.push_str("/idp/idx/introspect");
                                V::string(text)
                            });
                            cx.define(
                                scope,
                                "body",
                                if cx.lookup(scope, "stateHandle")?.truthy() {
                                    {
                                        let object = cx.object();
                                        cx.put(
                                            &object,
                                            &"stateToken",
                                            cx.lookup(scope, "stateHandle")?,
                                        )?;
                                        object
                                    }
                                } else {
                                    {
                                        let object = cx.object();
                                        cx.put(
                                            &object,
                                            &"interactionHandle",
                                            cx.lookup(scope, "interactionHandle")?,
                                        )?;
                                        object
                                    }
                                },
                            );
                            cx.define(scope, "headers", {
                                let object = cx.object();
                                cx.put(&object, &"Content-Type", {
                                    let mut text = String::new();
                                    text.push_str("application/ion+json; okta-version=");
                                    text.push_str(&cx.lookup(scope, "version")?.text());
                                    text.push_str("");
                                    V::string(text)
                                })?;
                                cx.put(&object, &"Accept", {
                                    let mut text = String::new();
                                    text.push_str("application/ion+json; okta-version=");
                                    text.push_str(&cx.lookup(scope, "version")?.text());
                                    text.push_str("");
                                    V::string(text)
                                })?;
                                object
                            });
                            let _ = cx.assign(
                                scope,
                                "rawIdxResponse",
                                cx.await_value({
                                    let function = cx.lookup(scope, "httpRequest")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "authClient")?);
                                        argv.push({
                                            let object = cx.object();
                                            cx.put(&object, &"method", V::string("POST"))?;
                                            cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
                                            cx.put(
                                                &object,
                                                &"headers",
                                                cx.lookup(scope, "headers")?,
                                            )?;
                                            cx.put(
                                                &object,
                                                &"withCredentials",
                                                cx.lookup(scope, "withCredentials")?,
                                            )?;
                                            cx.put(&object, &"args", cx.lookup(scope, "body")?)?;
                                            object
                                        });
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                })
                                .await?,
                            )?;
                        }
                        Ok::<Completion, V>(Completion::Normal)
                    })
                    .await;
                    if let Err(error) = completion {
                        completion = (async {
                            let scope = cx.scope(Some(scope));
                            cx.define(scope, "err", error);
                            {
                                let scope = cx.scope(Some(scope));
                                if {
                                    let left = {
                                        let left = {
                                            let function = cx.lookup(scope, "isAuthApiError")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.lookup(scope, "err")?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        };
                                        if left.truthy() {
                                            cx.get(&cx.lookup(scope, "err")?, &"xhr")?
                                        } else {
                                            left
                                        }
                                    };
                                    if left.truthy() {
                                        {
                                            let function = cx.lookup(scope, "isRawIdxResponse")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.get(
                                                    &cx.get(&cx.lookup(scope, "err")?, &"xhr")?,
                                                    &"responseJSON",
                                                )?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        }
                                    } else {
                                        left
                                    }
                                }
                                .truthy()
                                {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        let _ = cx.assign(
                                            scope,
                                            "rawIdxResponse",
                                            cx.get(
                                                &cx.get(&cx.lookup(scope, "err")?, &"xhr")?,
                                                &"responseJSON",
                                            )?,
                                        )?;
                                        let _ =
                                            cx.assign(scope, "requestDidSucceed", V::Bool(false))?;
                                    }
                                } else {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Err(cx.lookup(scope, "err")?);
                                    }
                                }
                            }
                            Ok::<Completion, V>(Completion::Normal)
                        })
                        .await;
                    }
                    match completion? {
                        Completion::Normal => {}
                        Completion::Return(value) => {
                            return Ok(value);
                        }
                        Completion::Break => {
                            return Err(V::string("unexpected break"));
                        }
                        Completion::Continue => {
                            return Err(V::string("unexpected continue"));
                        }
                    }
                }
            }
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(
                scope,
                "withCredentials",
                cx.get(&binding, &"withCredentials")?,
            );
        }
        return Ok({
            let function = cx.lookup(scope, "makeIdxState")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "rawIdxResponse")?);
                argv.push({
                    let object = cx.object();
                    cx.put(
                        &object,
                        &"withCredentials",
                        cx.lookup(scope, "withCredentials")?,
                    )?;
                    object
                });
                argv.push(cx.lookup(scope, "requestDidSucceed")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f58(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "meta", {
        let function = cx.lookup(scope, "getSavedTransactionMeta")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok(V::Bool(
        !V::Bool(
            !{
                let left = cx.lookup(scope, "meta")?;
                if left.truthy() {
                    left
                } else {
                    cx.get(&cx.lookup(scope, "options")?, &"stateHandle")?
                }
            }
            .truthy(),
        )
        .truthy(),
    ));

    Ok(V::Undefined)
}

fn f59(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        if V::Bool(
            !{
                let function = cx.lookup(scope, "canProceed")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "options")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
            .truthy(),
        )
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string(
                        "Unable to proceed: saved transaction could not be loaded",
                    ));
                    argv
                })?);
            }
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(scope, "flow", cx.get(&binding, &"flow")?);
            cx.define(scope, "state", cx.get(&binding, &"state")?);
        }
        if V::Bool(!cx.lookup(scope, "flow")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "meta", {
                    let function = cx.lookup(scope, "getSavedTransactionMeta")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push({
                            let object = cx.object();
                            cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
                            object
                        });
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
                let _ = cx.assign(scope, "flow", 'chain60: {
                    {
                        let object = cx.lookup(scope, "meta")?;
                        if object.nullish() {
                            break 'chain60 V::Undefined;
                        }
                        cx.get(&object, &"flow")?
                    }
                })?;
            }
        }
        return Ok({
            let function = cx.lookup(scope, "run")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push({
                    let object = cx.object();
                    cx.spread(&object, &cx.lookup(scope, "options")?)?;
                    cx.put(&object, &"flow", cx.lookup(scope, "flow")?)?;
                    object
                });
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f62(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "action",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let left =
            V::Bool(!V::Bool(!cx.get(&cx.lookup(scope, "values")?, &"resend")?.truthy()).truthy());
        if left.truthy() {
            {
                let receiver = cx.lookup(scope, "action")?;
                let method = "includes";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("-resend"));
                    argv
                };
                cx.call(function, receiver, argv)?
            }
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f61(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "values",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "keys";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "find";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f62));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f63(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "options",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    if cx
        .get(&cx.lookup(scope, "idxResponse")?, &"interactionCode")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let object = cx.object();
                cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                object
            });
        }
    }
    if cx
        .get(&cx.lookup(scope, "options")?, &"enableLegacyMode")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let object = cx.object();
                cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                object
            });
        }
    }
    cx.define(
        scope,
        "gr",
        cx.construct(cx.lookup(scope, "GenericRemediator")?, {
            let mut argv = Vec::new();
            argv.push(cx.get(
                &cx.get(&cx.lookup(scope, "idxResponse")?, &"neededToProceed")?,
                &V::Number(0f64).text(),
            )?);
            argv.push({
                let object = cx.object();
                object
            });
            argv.push(cx.lookup(scope, "options")?);
            argv
        })?,
    );
    cx.define(scope, "nextStep", {
        let function = cx.lookup(scope, "getNextStep")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push(cx.lookup(scope, "gr")?);
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let object = cx.object();
        cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
        cx.put(&object, &"nextStep", cx.lookup(scope, "nextStep")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f65(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        cx.lookup(scope, "action")?,
    )?);

    Ok(V::Undefined)
}

fn f66(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "acc", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "curr", args.get(1).cloned().unwrap_or(V::Undefined));
    return Ok(if cx.lookup(scope, "acc")?.truthy() {
        binary(
            "+",
            binary("+", cx.lookup(scope, "acc")?, V::string(" ,"))?,
            cx.get(&cx.lookup(scope, "curr")?, &"name")?,
        )?
    } else {
        cx.get(&cx.lookup(scope, "curr")?, &"name")?
    });

    Ok(V::Undefined)
}

fn f64(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "idxResponse",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "values",
            args.get(2).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "options",
            args.get(3).cloned().unwrap_or(V::Undefined),
        );
        {
            let binding = cx.lookup(scope, "idxResponse")?;
            cx.define(
                scope,
                "neededToProceed",
                cx.get(&binding, &"neededToProceed")?,
            );
            cx.define(
                scope,
                "interactionCode",
                cx.get(&binding, &"interactionCode")?,
            );
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(scope, "flow", cx.get(&binding, &"flow")?);
            cx.define(
                scope,
                "useGenericRemediator",
                cx.get(&binding, &"useGenericRemediator")?,
            );
            cx.define(
                scope,
                "enableLegacyMode",
                cx.get(&binding, &"enableLegacyMode")?,
            );
        }
        cx.define(
            scope,
            "isStepMode",
            binary("!==", cx.lookup(scope, "enableLegacyMode")?, V::Bool(true))?,
        );
        if cx.lookup(scope, "interactionCode")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let object = cx.object();
                    cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                    object
                });
            }
        }
        if {
            let function = cx.lookup(scope, "isTerminalResponse")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let object = cx.object();
                    cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                    cx.put(&object, &"terminal", V::Bool(true))?;
                    object
                });
            }
        }
        cx.define(scope, "actionFromValues", {
            let function = cx.lookup(scope, "getActionFromValues")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "values")?);
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        cx.define(scope, "actionFromOptions", {
            let left = cx.get(&cx.lookup(scope, "options")?, &"actions")?;
            if left.truthy() {
                left
            } else {
                {
                    let mut values = vec![];
                    cx.array(values)
                }
            }
        });
        cx.define(scope, "actions", {
            let mut values = vec![];
            values.extend(cx.items(&cx.lookup(scope, "actionFromOptions")?)?);
            values.extend(cx.items(&{
                let left = {
                    let left = cx.lookup(scope, "actionFromValues")?;
                    if left.truthy() {
                        {
                            let mut values = vec![];
                            values.push(cx.lookup(scope, "actionFromValues")?);
                            cx.array(values)
                        }
                    } else {
                        left
                    }
                };
                if left.truthy() {
                    left
                } else {
                    {
                        let mut values = vec![];
                        cx.array(values)
                    }
                }
            })?);
            cx.array(values)
        });
        if binary(
            ">",
            cx.get(&cx.lookup(scope, "actions")?, &"length")?,
            V::Number(0f64),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                for item in cx.items(&cx.lookup(scope, "actions")?)? {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "action", item);
                    {
                        let scope = cx.scope(Some(scope));
                        cx.define(scope, "params", {
                            let object = cx.object();
                            object
                        });
                        if binary(
                            "!==",
                            V::string(cx.lookup(scope, "action")?.typeof_()),
                            V::string("string"),
                        )?
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = cx.assign(scope, "params", {
                                    let left = cx.get(&cx.lookup(scope, "action")?, &"params")?;
                                    if left.truthy() {
                                        left
                                    } else {
                                        {
                                            let object = cx.object();
                                            object
                                        }
                                    }
                                })?;
                                let _ = cx.assign(
                                    scope,
                                    "action",
                                    cx.get(&cx.lookup(scope, "action")?, &"name")?,
                                )?;
                            }
                        }
                        cx.define(scope, "valuesWithoutExecutedAction", {
                            let function = cx.lookup(scope, "removeActionFromValues")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "values")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        });
                        cx.define(scope, "optionsWithoutExecutedAction", {
                            let function = cx.lookup(scope, "removeActionFromOptions")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "options")?);
                                argv.push(cx.lookup(scope, "action")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        });
                        if binary(
                            "===",
                            V::string(
                                cx.get(
                                    &cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?,
                                    &cx.lookup(scope, "action")?.text(),
                                )?
                                .typeof_(),
                            ),
                            V::string("function"),
                        )?
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = cx.assign(
                                    scope,
                                    "idxResponse",
                                    cx.await_value({
                                        let receiver =
                                            cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?;
                                        let method = cx.lookup(scope, "action")?.text();
                                        let function = cx.resolve_method(&receiver, &method)?;
                                        let argv = {
                                            let mut argv = Vec::new();
                                            argv.push(cx.lookup(scope, "params")?);
                                            argv
                                        };
                                        cx.call(function, receiver, argv)?
                                    })
                                    .await?,
                                )?;
                                if binary(
                                    "===",
                                    cx.get(
                                        &cx.lookup(scope, "idxResponse")?,
                                        &"requestDidSucceed",
                                    )?,
                                    V::Bool(false),
                                )?
                                .truthy()
                                {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Ok({
                                            let function =
                                                cx.lookup(scope, "handleFailedResponse")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.lookup(scope, "authClient")?);
                                                argv.push(cx.lookup(scope, "idxResponse")?);
                                                argv.push(cx.lookup(scope, "options")?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        });
                                    }
                                }
                                if binary("===", cx.lookup(scope, "action")?, V::string("cancel"))?
                                    .truthy()
                                {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Ok({
                                            let object = cx.object();
                                            cx.put(
                                                &object,
                                                &"idxResponse",
                                                cx.lookup(scope, "idxResponse")?,
                                            )?;
                                            cx.put(&object, &"canceled", V::Bool(true))?;
                                            object
                                        });
                                    }
                                }
                                if cx.lookup(scope, "isStepMode")?.truthy() {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Ok({
                                            let function =
                                                cx.lookup(scope, "getGenericNextStep")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.lookup(scope, "authClient")?);
                                                argv.push(cx.lookup(scope, "idxResponse")?);
                                                argv.push(cx.lookup(scope, "options")?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        });
                                    }
                                }
                                return Ok({
                                    let function = cx.lookup(scope, "remediate")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "authClient")?);
                                        argv.push(cx.lookup(scope, "idxResponse")?);
                                        argv.push(cx.lookup(scope, "valuesWithoutExecutedAction")?);
                                        argv.push(
                                            cx.lookup(scope, "optionsWithoutExecutedAction")?,
                                        );
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                });
                            }
                        }
                        cx.define(scope, "remediationAction", {
                            let receiver = cx.lookup(scope, "neededToProceed")?;
                            let method = "find";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.arrow(scope, this.clone(), f65));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        });
                        if cx.lookup(scope, "remediationAction")?.truthy() {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = cx.assign(
                                    scope,
                                    "idxResponse",
                                    cx.await_value({
                                        let receiver = cx.lookup(scope, "idxResponse")?;
                                        let method = "proceed";
                                        let function = cx.resolve_method(&receiver, &method)?;
                                        let argv = {
                                            let mut argv = Vec::new();
                                            argv.push(cx.lookup(scope, "action")?);
                                            argv.push(cx.lookup(scope, "params")?);
                                            argv
                                        };
                                        cx.call(function, receiver, argv)?
                                    })
                                    .await?,
                                )?;
                                if binary(
                                    "===",
                                    cx.get(
                                        &cx.lookup(scope, "idxResponse")?,
                                        &"requestDidSucceed",
                                    )?,
                                    V::Bool(false),
                                )?
                                .truthy()
                                {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Ok({
                                            let function =
                                                cx.lookup(scope, "handleFailedResponse")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.lookup(scope, "authClient")?);
                                                argv.push(cx.lookup(scope, "idxResponse")?);
                                                argv.push(cx.lookup(scope, "options")?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        });
                                    }
                                }
                                if cx.lookup(scope, "isStepMode")?.truthy() {
                                    {
                                        let scope = cx.scope(Some(scope));
                                        return Ok({
                                            let function =
                                                cx.lookup(scope, "getGenericNextStep")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.lookup(scope, "authClient")?);
                                                argv.push(cx.lookup(scope, "idxResponse")?);
                                                argv.push(cx.lookup(scope, "options")?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        });
                                    }
                                }
                                return Ok({
                                    let function = cx.lookup(scope, "remediate")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "authClient")?);
                                        argv.push(cx.lookup(scope, "idxResponse")?);
                                        argv.push(cx.lookup(scope, "values")?);
                                        argv.push(
                                            cx.lookup(scope, "optionsWithoutExecutedAction")?,
                                        );
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                });
                            }
                        }
                    }
                }
                if {
                    let left = cx.lookup(scope, "isStepMode")?;
                    if left.truthy() {
                        V::Bool(!cx.get(&cx.lookup(scope, "options")?, &"step")?.truthy())
                    } else {
                        left
                    }
                }
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                            let mut argv = Vec::new();
                            argv.push(V::string("Unable to proceed with provided actions"));
                            argv
                        })?);
                    }
                }
            }
        }
        if {
            let left = {
                let left = V::Bool(!cx.lookup(scope, "useGenericRemediator")?.truthy());
                if left.truthy() {
                    cx.lookup(scope, "isStepMode")?
                } else {
                    left
                }
            };
            if left.truthy() {
                V::Bool(!cx.get(&cx.lookup(scope, "options")?, &"step")?.truthy())
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("No `step` or `action` provided"));
                    argv
                })?);
            }
        }
        cx.define(scope, "remediator", {
            let function = cx.lookup(scope, "getRemediator")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv.push(cx.lookup(scope, "values")?);
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        if V::Bool(!cx.lookup(scope, "remediator")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                if cx.get(&cx.lookup(scope, "options")?, &"step")?.truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "values", {
                            let function = cx.lookup(scope, "filterValuesForRemediation")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "idxResponse")?);
                                argv.push(cx.get(&cx.lookup(scope, "options")?, &"step")?);
                                argv.push(cx.lookup(scope, "values")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })?;
                        let _ = cx.assign(
                            scope,
                            "idxResponse",
                            cx.await_value({
                                let receiver = cx.lookup(scope, "idxResponse")?;
                                let method = "proceed";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.get(&cx.lookup(scope, "options")?, &"step")?);
                                    argv.push(cx.lookup(scope, "values")?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            })
                            .await?,
                        )?;
                        if binary(
                            "===",
                            cx.get(&cx.lookup(scope, "idxResponse")?, &"requestDidSucceed")?,
                            V::Bool(false),
                        )?
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                return Ok({
                                    let function = cx.lookup(scope, "handleFailedResponse")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "authClient")?);
                                        argv.push(cx.lookup(scope, "idxResponse")?);
                                        argv.push(cx.lookup(scope, "options")?);
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                });
                            }
                        }
                        return Ok({
                            let function = cx.lookup(scope, "getGenericNextStep")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "authClient")?);
                                argv.push(cx.lookup(scope, "idxResponse")?);
                                argv.push(cx.lookup(scope, "options")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        });
                    }
                }
                if binary("===", cx.lookup(scope, "flow")?, V::string("default"))?.truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        return Ok({
                            let object = cx.object();
                            cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                            object
                        });
                    }
                }
                return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push({let mut text=String::new();text.push_str("\n      No remediation can match current flow, check policy settings in your org.\n      Remediations: [");text.push_str(&{let receiver=cx.lookup(scope,"neededToProceed")?;let method="reduce";let function=cx.resolve_method(&receiver,&method)?;let argv={let mut argv=Vec::new();argv.push(cx.arrow(scope,this.clone(),f66));argv.push(V::string(""));argv};cx.call(function,receiver,argv)?}.text());text.push_str("]\n    ");V::string(text)});argv})?);
            }
        }
        if V::Bool(
            !{
                let receiver = cx.lookup(scope, "remediator")?;
                let method = "canRemediate";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy(),
        )
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "nextStep", {
                    let function = cx.lookup(scope, "getNextStep")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "remediator")?);
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
                return Ok({
                    let object = cx.object();
                    cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                    cx.put(&object, &"nextStep", cx.lookup(scope, "nextStep")?)?;
                    object
                });
            }
        }
        cx.define(scope, "name", {
            let receiver = cx.lookup(scope, "remediator")?;
            let method = "getName";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.define(scope, "data", {
            let receiver = cx.lookup(scope, "remediator")?;
            let method = "getData";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        });
        let _ = cx.assign(
            scope,
            "idxResponse",
            cx.await_value({
                let receiver = cx.lookup(scope, "idxResponse")?;
                let method = "proceed";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "name")?);
                    argv.push(cx.lookup(scope, "data")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })
            .await?,
        )?;
        if binary(
            "===",
            cx.get(&cx.lookup(scope, "idxResponse")?, &"requestDidSucceed")?,
            V::Bool(false),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let function = cx.lookup(scope, "handleFailedResponse")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
            }
        }
        if {
            let function = cx.lookup(scope, "isTerminalResponse")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let object = cx.object();
                    cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                    cx.put(&object, &"terminal", V::Bool(true))?;
                    object
                });
            }
        }
        if {
            let left = cx.lookup(scope, "useGenericRemediator")?;
            if left.truthy() {
                left
            } else {
                cx.lookup(scope, "isStepMode")?
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let function = cx.lookup(scope, "getGenericNextStep")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
            }
        }
        let _ = cx.assign(scope, "values", {
            let receiver = cx.lookup(scope, "remediator")?;
            let method = "getValuesAfterProceed";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        })?;
        let _ = cx.assign(scope, "options", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "options")?)?;
            cx.put(&object, &"step", cx.lookup(scope, "undefined")?)?;
            object
        })?;
        return Ok({
            let function = cx.lookup(scope, "remediate")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv.push(cx.lookup(scope, "values")?);
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f68(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "entry", args.get(0).cloned().unwrap_or(V::Undefined));
    if binary(
        "===",
        V::string(cx.lookup(scope, "entry")?.typeof_()),
        V::string("string"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(binary(
                "!==",
                cx.lookup(scope, "entry")?,
                cx.lookup(scope, "actionName")?,
            )?);
        }
    }
    return Ok(binary(
        "!==",
        cx.get(&cx.lookup(scope, "entry")?, &"name")?,
        cx.lookup(scope, "actionName")?,
    )?);

    Ok(V::Undefined)
}

fn f67(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "actionName",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "actions", {
        let left = cx.get(&cx.lookup(scope, "options")?, &"actions")?;
        if left.truthy() {
            left
        } else {
            {
                let mut values = vec![];
                cx.array(values)
            }
        }
    });
    let _ = cx.assign(scope, "actions", {
        let receiver = cx.lookup(scope, "actions")?;
        let method = "filter";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f68));
            argv
        };
        cx.call(function, receiver, argv)?
    })?;
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "options")?)?;
        cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f69(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "values",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "values")?)?;
        cx.put(&object, &"resend", cx.lookup(scope, "undefined")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f71(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "item", args.get(0).cloned().unwrap_or(V::Undefined));
    if {
        let left = {
            let left = binary(
                "===",
                V::string(cx.lookup(scope, "item")?.typeof_()),
                V::string("string"),
            )?;
            if left.truthy() {
                left
            } else {
                binary(
                    "===",
                    V::string(cx.lookup(scope, "item")?.typeof_()),
                    V::string("number"),
                )?
            }
        };
        if left.truthy() {
            left
        } else {
            binary(
                "===",
                V::string(cx.lookup(scope, "item")?.typeof_()),
                V::string("boolean"),
            )?
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "item")?);
        }
    }
    return Ok({
        let function = cx.lookup(scope, "unwrapFormValue")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "item")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f72(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "key", cx.get(&binding, "0")?);
        cx.define(scope, "value", cx.get(&binding, "1")?);
    }
    let _ = {
        let object = cx.lookup(scope, "res")?;
        let key = cx.lookup(scope, "key")?.text();
        let value = cx.lookup(scope, "value")?;
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f70(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if {
        let receiver = cx.lookup(scope, "Array")?;
        let method = "isArray";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "remediation")?);
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let receiver = cx.lookup(scope, "remediation")?;
                let method = "map";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f71));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
        }
    }
    cx.define(scope, "res", {
        let object = cx.object();
        object
    });
    for item in cx.items(&{
        let receiver = cx.lookup(scope, "Object")?;
        let method = "entries";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "remediation")?);
            argv
        };
        cx.call(function, receiver, argv)?
    })? {
        let scope = cx.scope(Some(scope));
        {
            let binding = item;
            cx.define(scope, "key", cx.get(&binding, "0")?);
            cx.define(scope, "value", cx.get(&binding, "1")?);
        }
        {
            let scope = cx.scope(Some(scope));
            if {
                let left = binary("===", cx.lookup(scope, "value")?, V::Null)?;
                if left.truthy() {
                    left
                } else {
                    binary(
                        "===",
                        V::string(cx.lookup(scope, "value")?.typeof_()),
                        V::string("undefined"),
                    )?
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    continue;
                }
            }
            if binary(
                "===",
                V::string(cx.lookup(scope, "value")?.typeof_()),
                V::string("object"),
            )?
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "formKeys", {
                        let receiver = cx.lookup(scope, "Object")?;
                        let method = "keys";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "value")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                    if {
                        let left = {
                            let left = {
                                let receiver = {
                                    let mut values = vec![];
                                    values.push(V::string("value"));
                                    values.push(V::string("form"));
                                    cx.array(values)
                                };
                                let method = "includes";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "key")?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                            if left.truthy() {
                                binary(
                                    "===",
                                    cx.get(&cx.lookup(scope, "formKeys")?, &"length")?,
                                    V::Number(1f64),
                                )?
                            } else {
                                left
                            }
                        };
                        if left.truthy() {
                            {
                                let receiver = {
                                    let mut values = vec![];
                                    values.push(V::string("value"));
                                    values.push(V::string("form"));
                                    cx.array(values)
                                };
                                let method = "includes";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.get(
                                        &cx.lookup(scope, "formKeys")?,
                                        &V::Number(0f64).text(),
                                    )?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            }
                        } else {
                            left
                        }
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            cx.define(scope, "unwrappedForm", {
                                let function = cx.lookup(scope, "unwrapFormValue")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "value")?);
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            });
                            let _ = {
                                let receiver = {
                                    let receiver = cx.lookup(scope, "Object")?;
                                    let method = "entries";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "unwrappedForm")?);
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                };
                                let method = "forEach";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.arrow(scope, this.clone(), f72));
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                        }
                    } else {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "res")?;
                                let key = cx.lookup(scope, "key")?.text();
                                let value = {
                                    let function = cx.lookup(scope, "unwrapFormValue")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "value")?);
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                };
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                }
            } else {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "res")?;
                        let key = cx.lookup(scope, "key")?.text();
                        let value = cx.lookup(scope, "value")?;
                        cx.put(&object, &key, value)?
                    };
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "res")?);

    Ok(V::Undefined)
}

fn f75(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "r", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(cx.get(&cx.lookup(scope, "r")?, &"name")?);

    Ok(V::Undefined)
}

fn f73(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxRemediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok('chain74: {
        {
            let receiver = cx.get(&cx.lookup(scope, "idxRemediation")?, &"value")?;
            if receiver.nullish() {
                break 'chain74 V::Undefined;
            }
            let method = "map";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f75));
                argv
            };
            cx.call(function, receiver, argv)?
        }
    });

    Ok(V::Undefined)
}

fn f77(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        V::string("authenticator"),
    )?);

    Ok(V::Undefined)
}

fn f76(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let receiver = cx.get(&cx.lookup(scope, "remediation")?, &"value")?;
        let method = "find";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f77));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f80(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "required",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "cur", args.get(1).cloned().unwrap_or(V::Undefined));
    if cx.get(&cx.lookup(scope, "cur")?, &"required")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "required")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "cur")?, &"name")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Ok(cx.lookup(scope, "required")?);

    Ok(V::Undefined)
}

fn f78(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxRemediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok('chain79: {
        {
            let receiver = cx.get(&cx.lookup(scope, "idxRemediation")?, &"value")?;
            if receiver.nullish() {
                break 'chain79 V::Undefined;
            }
            let method = "reduce";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f80));
                argv.push({
                    let mut values = vec![];
                    cx.array(values)
                });
                argv
            };
            cx.call(function, receiver, argv)?
        }
    });

    Ok(V::Undefined)
}

fn f81(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "+",
        {
            let receiver = {
                let receiver = cx.lookup(scope, "str")?;
                let method = "charAt";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(0f64));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "toUpperCase";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        },
        {
            let receiver = cx.lookup(scope, "str")?;
            let method = "substring";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::Number(1f64));
                argv
            };
            cx.call(function, receiver, argv)?
        },
    )?);

    Ok(V::Undefined)
}

fn f83(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "msg", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "===",
        cx.get(&cx.lookup(scope, "msg")?, &"class")?,
        V::string("ERROR"),
    )?);

    Ok(V::Undefined)
}

fn f82(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
        {
            let binding = cx.lookup(scope, "data")?;
            cx.define(scope, "options", cx.get(&binding, &"options")?);
            cx.define(scope, "idxResponse", cx.get(&binding, &"idxResponse")?);
            cx.define(scope, "canceled", cx.get(&binding, &"canceled")?);
            cx.define(scope, "status", cx.get(&binding, &"status")?);
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(
                scope,
                "exchangeCodeForTokens",
                cx.get(&binding, &"exchangeCodeForTokens")?,
            );
        }
        cx.define(scope, "shouldSaveResponse", V::Bool(false));
        cx.define(scope, "shouldClearTransaction", V::Bool(false));
        cx.define(scope, "clearSharedStorage", V::Bool(true));
        cx.define(scope, "interactionCode", V::Undefined);
        cx.define(scope, "tokens", V::Undefined);
        cx.define(scope, "enabledFeatures", V::Undefined);
        cx.define(scope, "availableSteps", V::Undefined);
        cx.define(scope, "messages", V::Undefined);
        cx.define(scope, "terminal", V::Undefined);
        if cx.lookup(scope, "idxResponse")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "shouldSaveResponse",
                    V::Bool(
                        !V::Bool(
                            !{
                                let left = cx
                                    .get(&cx.lookup(scope, "idxResponse")?, &"requestDidSucceed")?;
                                if left.truthy() {
                                    left
                                } else {
                                    cx.get(&cx.lookup(scope, "idxResponse")?, &"stepUp")?
                                }
                            }
                            .truthy(),
                        )
                        .truthy(),
                    ),
                )?;
                let _ = cx.assign(scope, "enabledFeatures", {
                    let function = cx.lookup(scope, "getEnabledFeatures")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                let _ = cx.assign(scope, "availableSteps", {
                    let function = cx.lookup(scope, "getAvailableSteps")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv.push(cx.get(&cx.lookup(scope, "options")?, &"useGenericRemediator")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                let _ = cx.assign(scope, "messages", {
                    let function = cx.lookup(scope, "getMessagesFromResponse")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                let _ = cx.assign(scope, "terminal", {
                    let function = cx.lookup(scope, "isTerminalResponse")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
            }
        }
        if cx.lookup(scope, "terminal")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "status",
                    cx.get(&cx.lookup(scope, "IdxStatus")?, &"TERMINAL")?,
                )?;
                cx.define(
                    scope,
                    "hasActions",
                    binary(
                        ">",
                        cx.get(
                            &{
                                let receiver = cx.lookup(scope, "Object")?;
                                let method = "keys";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(
                                        cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?,
                                    );
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            },
                            &"length",
                        )?,
                        V::Number(0f64),
                    )?,
                );
                cx.define(
                    scope,
                    "hasErrors",
                    V::Bool(
                        !V::Bool(
                            !{
                                let receiver = cx.lookup(scope, "messages")?;
                                let method = "find";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.arrow(scope, this.clone(), f83));
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            }
                            .truthy(),
                        )
                        .truthy(),
                    ),
                );
                cx.define(scope, "isTerminalSuccess", {
                    let left = {
                        let left = V::Bool(!cx.lookup(scope, "hasActions")?.truthy());
                        if left.truthy() {
                            V::Bool(!cx.lookup(scope, "hasErrors")?.truthy())
                        } else {
                            left
                        }
                    };
                    if left.truthy() {
                        binary(
                            "===",
                            cx.get(&cx.lookup(scope, "idxResponse")?, &"requestDidSucceed")?,
                            V::Bool(true),
                        )?
                    } else {
                        left
                    }
                });
                if cx.lookup(scope, "isTerminalSuccess")?.truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "shouldClearTransaction", V::Bool(true))?;
                    }
                } else {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(
                            scope,
                            "shouldSaveResponse",
                            V::Bool(!V::Bool(!cx.lookup(scope, "hasActions")?.truthy()).truthy()),
                        )?;
                    }
                }
                let _ = cx.assign(scope, "clearSharedStorage", V::Bool(false))?;
            }
        } else {
            if cx.lookup(scope, "canceled")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = cx.assign(
                        scope,
                        "status",
                        cx.get(&cx.lookup(scope, "IdxStatus")?, &"CANCELED")?,
                    )?;
                    let _ = cx.assign(scope, "shouldClearTransaction", V::Bool(true))?;
                }
            } else {
                if 'chain84: {
                    {
                        let object = cx.lookup(scope, "idxResponse")?;
                        if object.nullish() {
                            break 'chain84 V::Undefined;
                        }
                        cx.get(&object, &"interactionCode")?
                    }
                }
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(
                            scope,
                            "interactionCode",
                            cx.get(&cx.lookup(scope, "idxResponse")?, &"interactionCode")?,
                        )?;
                        if binary(
                            "===",
                            cx.lookup(scope, "exchangeCodeForTokens")?,
                            V::Bool(false),
                        )?
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = cx.assign(
                                    scope,
                                    "status",
                                    cx.get(&cx.lookup(scope, "IdxStatus")?, &"SUCCESS")?,
                                )?;
                                let _ =
                                    cx.assign(scope, "shouldClearTransaction", V::Bool(false))?;
                            }
                        } else {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = cx.assign(
                                    scope,
                                    "tokens",
                                    cx.await_value({
                                        let function = cx.lookup(scope, "getTokens")?;
                                        let argv = {
                                            let mut argv = Vec::new();
                                            argv.push(cx.lookup(scope, "authClient")?);
                                            argv.push(cx.lookup(scope, "data")?);
                                            argv
                                        };
                                        cx.call(function, V::Undefined, argv)?
                                    })
                                    .await?,
                                )?;
                                let _ = cx.assign(
                                    scope,
                                    "status",
                                    cx.get(&cx.lookup(scope, "IdxStatus")?, &"SUCCESS")?,
                                )?;
                                let _ =
                                    cx.assign(scope, "shouldClearTransaction", V::Bool(true))?;
                            }
                        }
                    }
                }
            }
        }
        return Ok({
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "data")?)?;
            cx.put(&object, &"status", cx.lookup(scope, "status")?)?;
            cx.put(
                &object,
                &"interactionCode",
                cx.lookup(scope, "interactionCode")?,
            )?;
            cx.put(&object, &"tokens", cx.lookup(scope, "tokens")?)?;
            cx.put(
                &object,
                &"shouldSaveResponse",
                cx.lookup(scope, "shouldSaveResponse")?,
            )?;
            cx.put(
                &object,
                &"shouldClearTransaction",
                cx.lookup(scope, "shouldClearTransaction")?,
            )?;
            cx.put(
                &object,
                &"clearSharedStorage",
                cx.lookup(scope, "clearSharedStorage")?,
            )?;
            cx.put(
                &object,
                &"enabledFeatures",
                cx.lookup(scope, "enabledFeatures")?,
            )?;
            cx.put(
                &object,
                &"availableSteps",
                cx.lookup(scope, "availableSteps")?,
            )?;
            cx.put(&object, &"messages", cx.lookup(scope, "messages")?)?;
            cx.put(&object, &"terminal", cx.lookup(scope, "terminal")?)?;
            object
        });
        Ok(V::Undefined)
    })
}

fn f85(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
        {
            let binding = cx.lookup(scope, "data")?;
            cx.define(scope, "options", cx.get(&binding, &"options")?);
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(scope, "stateHandle", cx.get(&binding, &"stateHandle")?);
            cx.define(
                scope,
                "withCredentials",
                cx.get(&binding, &"withCredentials")?,
            );
            cx.define(scope, "version", cx.get(&binding, &"version")?);
            cx.define(scope, "state", cx.get(&binding, &"state")?);
            cx.define(scope, "scopes", cx.get(&binding, &"scopes")?);
            cx.define(scope, "recoveryToken", cx.get(&binding, &"recoveryToken")?);
            cx.define(
                scope,
                "activationToken",
                cx.get(&binding, &"activationToken")?,
            );
            cx.define(scope, "maxAge", cx.get(&binding, &"maxAge")?);
            cx.define(scope, "acrValues", cx.get(&binding, &"acrValues")?);
            cx.define(scope, "nonce", cx.get(&binding, &"nonce")?);
            cx.define(
                scope,
                "useGenericRemediator",
                cx.get(&binding, &"useGenericRemediator")?,
            );
        }
        cx.define(scope, "idxResponse", V::Undefined);
        cx.define(scope, "meta", {
            let function = cx.lookup(scope, "getSavedTransactionMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
                    cx.put(
                        &object,
                        &"recoveryToken",
                        cx.lookup(scope, "recoveryToken")?,
                    )?;
                    cx.put(
                        &object,
                        &"activationToken",
                        cx.lookup(scope, "activationToken")?,
                    )?;
                    object
                });
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        if cx.lookup(scope, "stateHandle")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "idxResponse",
                    cx.await_value({
                        let function = cx.lookup(scope, "introspect")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "authClient")?);
                            argv.push({
                                let object = cx.object();
                                cx.put(
                                    &object,
                                    &"withCredentials",
                                    cx.lookup(scope, "withCredentials")?,
                                )?;
                                cx.put(&object, &"version", cx.lookup(scope, "version")?)?;
                                cx.put(&object, &"stateHandle", cx.lookup(scope, "stateHandle")?)?;
                                cx.put(
                                    &object,
                                    &"useGenericRemediator",
                                    cx.lookup(scope, "useGenericRemediator")?,
                                )?;
                                object
                            });
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    })
                    .await?,
                )?;
            }
        } else {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "interactionHandle", 'chain86: {
                    {
                        let object = cx.lookup(scope, "meta")?;
                        if object.nullish() {
                            break 'chain86 V::Undefined;
                        }
                        cx.get(&object, &"interactionHandle")?
                    }
                });
                if V::Bool(!cx.lookup(scope, "interactionHandle")?.truthy()).truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let receiver =
                                cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
                            let method = "clear";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        };
                        cx.define(
                            scope,
                            "interactResponse",
                            cx.await_value({
                                let function = cx.lookup(scope, "interact")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "authClient")?);
                                    argv.push({
                                        let object = cx.object();
                                        cx.put(
                                            &object,
                                            &"withCredentials",
                                            cx.lookup(scope, "withCredentials")?,
                                        )?;
                                        cx.put(&object, &"state", cx.lookup(scope, "state")?)?;
                                        cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
                                        cx.put(
                                            &object,
                                            &"activationToken",
                                            cx.lookup(scope, "activationToken")?,
                                        )?;
                                        cx.put(
                                            &object,
                                            &"recoveryToken",
                                            cx.lookup(scope, "recoveryToken")?,
                                        )?;
                                        cx.put(&object, &"maxAge", cx.lookup(scope, "maxAge")?)?;
                                        cx.put(
                                            &object,
                                            &"acrValues",
                                            cx.lookup(scope, "acrValues")?,
                                        )?;
                                        cx.put(&object, &"nonce", cx.lookup(scope, "nonce")?)?;
                                        object
                                    });
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            })
                            .await?,
                        );
                        let _ = cx.assign(
                            scope,
                            "interactionHandle",
                            cx.get(&cx.lookup(scope, "interactResponse")?, &"interactionHandle")?,
                        )?;
                        let _ = cx.assign(
                            scope,
                            "meta",
                            cx.get(&cx.lookup(scope, "interactResponse")?, &"meta")?,
                        )?;
                    }
                }
                let _ = cx.assign(
                    scope,
                    "idxResponse",
                    cx.await_value({
                        let function = cx.lookup(scope, "introspect")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "authClient")?);
                            argv.push({
                                let object = cx.object();
                                cx.put(
                                    &object,
                                    &"withCredentials",
                                    cx.lookup(scope, "withCredentials")?,
                                )?;
                                cx.put(&object, &"version", cx.lookup(scope, "version")?)?;
                                cx.put(
                                    &object,
                                    &"interactionHandle",
                                    cx.lookup(scope, "interactionHandle")?,
                                )?;
                                cx.put(
                                    &object,
                                    &"useGenericRemediator",
                                    cx.lookup(scope, "useGenericRemediator")?,
                                )?;
                                object
                            });
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    })
                    .await?,
                )?;
            }
        }
        return Ok({
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "data")?)?;
            cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
            cx.put(&object, &"meta", cx.lookup(scope, "meta")?)?;
            object
        });
        Ok(V::Undefined)
    })
}

fn f87(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
        {
            let binding = cx.lookup(scope, "data")?;
            cx.define(scope, "idxResponse", cx.get(&binding, &"idxResponse")?);
            cx.define(scope, "options", cx.get(&binding, &"options")?);
            cx.define(scope, "values", cx.get(&binding, &"values")?);
        }
        {
            let binding = cx.lookup(scope, "options")?;
            cx.define(scope, "autoRemediate", cx.get(&binding, &"autoRemediate")?);
            cx.define(scope, "remediators", cx.get(&binding, &"remediators")?);
            cx.define(scope, "actions", cx.get(&binding, &"actions")?);
            cx.define(scope, "flow", cx.get(&binding, &"flow")?);
            cx.define(scope, "step", cx.get(&binding, &"step")?);
            cx.define(
                scope,
                "useGenericRemediator",
                cx.get(&binding, &"useGenericRemediator")?,
            );
            cx.define(
                scope,
                "enableLegacyMode",
                cx.get(&binding, &"enableLegacyMode")?,
            );
        }
        cx.define(scope, "shouldRemediate", {
            let left = binary("!==", cx.lookup(scope, "autoRemediate")?, V::Bool(false))?;
            if left.truthy() {
                {
                    let left = {
                        let left = cx.lookup(scope, "remediators")?;
                        if left.truthy() {
                            left
                        } else {
                            cx.lookup(scope, "actions")?
                        }
                    };
                    if left.truthy() {
                        left
                    } else {
                        cx.lookup(scope, "step")?
                    }
                }
            } else {
                left
            }
        });
        if V::Bool(!cx.lookup(scope, "shouldRemediate")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Ok(cx.lookup(scope, "data")?);
            }
        }
        let _ = cx.assign(scope, "values", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "values")?)?;
            cx.put(
                &object,
                &"stateHandle",
                cx.get(
                    &cx.get(&cx.lookup(scope, "idxResponse")?, &"rawIdxState")?,
                    &"stateHandle",
                )?,
            )?;
            object
        })?;
        {
            let binding = cx
                .await_value({
                    let function = cx.lookup(scope, "remediate")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "idxResponse")?);
                        argv.push(cx.lookup(scope, "values")?);
                        argv.push({
                            let object = cx.object();
                            cx.put(&object, &"remediators", cx.lookup(scope, "remediators")?)?;
                            cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
                            cx.put(&object, &"flow", cx.lookup(scope, "flow")?)?;
                            cx.put(&object, &"step", cx.lookup(scope, "step")?)?;
                            cx.put(
                                &object,
                                &"useGenericRemediator",
                                cx.lookup(scope, "useGenericRemediator")?,
                            )?;
                            cx.put(
                                &object,
                                &"enableLegacyMode",
                                cx.lookup(scope, "enableLegacyMode")?,
                            )?;
                            object
                        });
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })
                .await?;
            cx.define(
                scope,
                "idxResponseFromRemediation",
                cx.get(&binding, &"idxResponse")?,
            );
            cx.define(scope, "nextStep", cx.get(&binding, &"nextStep")?);
            cx.define(scope, "canceled", cx.get(&binding, &"canceled")?);
        }
        let _ = cx.assign(
            scope,
            "idxResponse",
            cx.lookup(scope, "idxResponseFromRemediation")?,
        )?;
        return Ok({
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "data")?)?;
            cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
            cx.put(&object, &"nextStep", cx.lookup(scope, "nextStep")?)?;
            cx.put(&object, &"canceled", cx.lookup(scope, "canceled")?)?;
            object
        });
        Ok(V::Undefined)
    })
}

fn f88(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
        {
            let binding = cx.lookup(scope, "data")?;
            cx.define(scope, "meta", cx.get(&binding, &"meta")?);
            cx.define(scope, "idxResponse", cx.get(&binding, &"idxResponse")?);
        }
        {
            let binding = cx.lookup(scope, "idxResponse")?;
            cx.define(
                scope,
                "interactionCode",
                cx.get(&binding, &"interactionCode")?,
            );
        }
        {
            let binding = cx.lookup(scope, "meta")?;
            cx.define(scope, "clientId", cx.get(&binding, &"clientId")?);
            cx.define(scope, "codeVerifier", cx.get(&binding, &"codeVerifier")?);
            cx.define(
                scope,
                "ignoreSignature",
                cx.get(&binding, &"ignoreSignature")?,
            );
            cx.define(scope, "redirectUri", cx.get(&binding, &"redirectUri")?);
            cx.define(scope, "urls", cx.get(&binding, &"urls")?);
            cx.define(scope, "scopes", cx.get(&binding, &"scopes")?);
        }
        cx.define(
            scope,
            "tokenResponse",
            cx.await_value({
                let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"token")?;
                let method = "exchangeCodeForTokens";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push({
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"interactionCode",
                            cx.lookup(scope, "interactionCode")?,
                        )?;
                        cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
                        cx.put(&object, &"codeVerifier", cx.lookup(scope, "codeVerifier")?)?;
                        cx.put(
                            &object,
                            &"ignoreSignature",
                            cx.lookup(scope, "ignoreSignature")?,
                        )?;
                        cx.put(&object, &"redirectUri", cx.lookup(scope, "redirectUri")?)?;
                        cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
                        object
                    });
                    argv.push(cx.lookup(scope, "urls")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })
            .await?,
        );
        return Ok(cx.get(&cx.lookup(scope, "tokenResponse")?, &"tokens")?);
        Ok(V::Undefined)
    })
}

fn f89(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "data", args.get(1).cloned().unwrap_or(V::Undefined));
    {
        let binding = cx.lookup(scope, "data")?;
        cx.define(scope, "options", cx.get(&binding, &"options")?);
    }
    let _ = cx.assign(scope, "options", {
        let object = cx.object();
        cx.spread(
            &object,
            &cx.get(
                &cx.get(&cx.lookup(scope, "authClient")?, &"options")?,
                &"idx",
            )?,
        )?;
        cx.spread(&object, &cx.lookup(scope, "options")?)?;
        object
    })?;
    {
        let binding = cx.lookup(scope, "options")?;
        cx.define(scope, "flow", cx.get(&binding, &"flow")?);
        cx.define(
            scope,
            "withCredentials",
            cx.get(&binding, &"withCredentials")?,
        );
        cx.define(scope, "remediators", cx.get(&binding, &"remediators")?);
        cx.define(scope, "actions", cx.get(&binding, &"actions")?);
    }
    cx.define(
        scope,
        "status",
        cx.get(&cx.lookup(scope, "IdxStatus")?, &"PENDING")?,
    );
    let _ = cx.assign(scope, "flow", {
        let left = {
            let left = cx.lookup(scope, "flow")?;
            if left.truthy() {
                left
            } else {
                'chain90: {
                    {
                        let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                        let method = "getFlow";
                        let function = cx.resolve_method(&receiver, &method)?;
                        if function.nullish() {
                            break 'chain90 V::Undefined;
                        }
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                }
            }
        };
        if left.truthy() {
            left
        } else {
            V::string("default")
        }
    })?;
    if cx.lookup(scope, "flow")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = 'chain91: {
                {
                    let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                    let method = "setFlow";
                    let function = cx.resolve_method(&receiver, &method)?;
                    if function.nullish() {
                        break 'chain91 V::Undefined;
                    }
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "flow")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
            };
            cx.define(scope, "flowSpec", {
                let function = cx.lookup(scope, "getFlowSpecification")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "flow")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            let _ = cx.assign(
                scope,
                "withCredentials",
                if binary(
                    "!==",
                    V::string(cx.lookup(scope, "withCredentials")?.typeof_()),
                    V::string("undefined"),
                )?
                .truthy()
                {
                    cx.lookup(scope, "withCredentials")?
                } else {
                    cx.get(&cx.lookup(scope, "flowSpec")?, &"withCredentials")?
                },
            )?;
            let _ = cx.assign(scope, "remediators", {
                let left = cx.lookup(scope, "remediators")?;
                if left.truthy() {
                    left
                } else {
                    cx.get(&cx.lookup(scope, "flowSpec")?, &"remediators")?
                }
            })?;
            let _ = cx.assign(scope, "actions", {
                let left = cx.lookup(scope, "actions")?;
                if left.truthy() {
                    left
                } else {
                    cx.get(&cx.lookup(scope, "flowSpec")?, &"actions")?
                }
            })?;
        }
    }
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "data")?)?;
        cx.put(&object, &"options", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "options")?)?;
            cx.put(&object, &"flow", cx.lookup(scope, "flow")?)?;
            cx.put(
                &object,
                &"withCredentials",
                cx.lookup(scope, "withCredentials")?,
            )?;
            cx.put(&object, &"remediators", cx.lookup(scope, "remediators")?)?;
            cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
            object
        })?;
        cx.put(&object, &"status", cx.lookup(scope, "status")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f93(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "option",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.delete(
        &cx.lookup(scope, "values")?,
        &cx.lookup(scope, "option")?.text(),
    )?;

    Ok(V::Undefined)
}

fn f92(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "knownOptions", {
        let mut values = vec![];
        values.push(V::string("flow"));
        values.push(V::string("remediators"));
        values.push(V::string("actions"));
        values.push(V::string("withCredentials"));
        values.push(V::string("step"));
        values.push(V::string("useGenericRemediator"));
        values.push(V::string("exchangeCodeForTokens"));
        values.push(V::string("enableLegacyMode"));
        cx.array(values)
    });
    cx.define(scope, "values", {
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "options")?)?;
        object
    });
    let _ = {
        let receiver = cx.lookup(scope, "knownOptions")?;
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f93));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok(cx.lookup(scope, "values")?);

    Ok(V::Undefined)
}

fn f94(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        cx.define(scope, "data", {
            let object = cx.object();
            cx.put(&object, &"options", cx.lookup(scope, "options")?)?;
            cx.put(&object, &"values", {
                let function = cx.lookup(scope, "initializeValues")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "options")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            object
        });
        let _ = cx.assign(scope, "data", {
            let function = cx.lookup(scope, "initializeData")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "data")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        })?;
        let _ = cx.assign(
            scope,
            "data",
            cx.await_value({
                let function = cx.lookup(scope, "getDataFromIntrospect")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "data")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        )?;
        let _ = cx.assign(
            scope,
            "data",
            cx.await_value({
                let function = cx.lookup(scope, "getDataFromRemediate")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "data")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        )?;
        let _ = cx.assign(
            scope,
            "data",
            cx.await_value({
                let function = cx.lookup(scope, "finalizeData")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "data")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        )?;
        {
            let binding = cx.lookup(scope, "data")?;
            cx.define(scope, "idxResponse", cx.get(&binding, &"idxResponse")?);
            cx.define(scope, "meta", cx.get(&binding, &"meta")?);
            cx.define(
                scope,
                "shouldSaveResponse",
                cx.get(&binding, &"shouldSaveResponse")?,
            );
            cx.define(
                scope,
                "shouldClearTransaction",
                cx.get(&binding, &"shouldClearTransaction")?,
            );
            cx.define(
                scope,
                "clearSharedStorage",
                cx.get(&binding, &"clearSharedStorage")?,
            );
            cx.define(scope, "status", cx.get(&binding, &"status")?);
            cx.define(
                scope,
                "enabledFeatures",
                cx.get(&binding, &"enabledFeatures")?,
            );
            cx.define(
                scope,
                "availableSteps",
                cx.get(&binding, &"availableSteps")?,
            );
            cx.define(scope, "tokens", cx.get(&binding, &"tokens")?);
            cx.define(scope, "nextStep", cx.get(&binding, &"nextStep")?);
            cx.define(scope, "messages", cx.get(&binding, &"messages")?);
            cx.define(scope, "error", cx.get(&binding, &"error")?);
            cx.define(
                scope,
                "interactionCode",
                cx.get(&binding, &"interactionCode")?,
            );
        }
        if cx.lookup(scope, "shouldClearTransaction")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let receiver =
                        cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
                    let method = "clear";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let object = cx.object();
                            cx.put(
                                &object,
                                &"clearSharedStorage",
                                cx.lookup(scope, "clearSharedStorage")?,
                            )?;
                            object
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
            }
        } else {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let function = cx.lookup(scope, "saveTransactionMeta")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push({
                            let object = cx.object();
                            cx.spread(&object, &cx.lookup(scope, "meta")?)?;
                            object
                        });
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                if cx.lookup(scope, "shouldSaveResponse")?.truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        {
                            let binding = cx.lookup(scope, "idxResponse")?;
                            cx.define(scope, "rawIdxResponse", cx.get(&binding, &"rawIdxState")?);
                            cx.define(
                                scope,
                                "requestDidSucceed",
                                cx.get(&binding, &"requestDidSucceed")?,
                            );
                        }
                        let _ = {
                            let receiver =
                                cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
                            let method = "saveIdxResponse";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push({
                                    let object = cx.object();
                                    cx.put(
                                        &object,
                                        &"rawIdxResponse",
                                        cx.lookup(scope, "rawIdxResponse")?,
                                    )?;
                                    cx.put(
                                        &object,
                                        &"requestDidSucceed",
                                        cx.lookup(scope, "requestDidSucceed")?,
                                    )?;
                                    cx.put(&object, &"stateHandle", 'chain95: {
                                        {
                                            let object = cx.get(
                                                &cx.lookup(scope, "idxResponse")?,
                                                &"context",
                                            )?;
                                            if object.nullish() {
                                                break 'chain95 V::Undefined;
                                            }
                                            cx.get(&object, &"stateHandle")?
                                        }
                                    })?;
                                    cx.put(&object, &"interactionHandle", 'chain96: {
                                        {
                                            let object = cx.lookup(scope, "meta")?;
                                            if object.nullish() {
                                                break 'chain96 V::Undefined;
                                            }
                                            cx.get(&object, &"interactionHandle")?
                                        }
                                    })?;
                                    object
                                });
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        };
                    }
                }
            }
        }
        {
            let binding = {
                let left = cx.lookup(scope, "idxResponse")?;
                if left.truthy() {
                    left
                } else {
                    {
                        let object = cx.object();
                        object
                    }
                }
            };
            cx.define(scope, "actions", cx.get(&binding, &"actions")?);
            cx.define(scope, "context", cx.get(&binding, &"context")?);
            cx.define(
                scope,
                "neededToProceed",
                cx.get(&binding, &"neededToProceed")?,
            );
            cx.define(scope, "proceed", cx.get(&binding, &"proceed")?);
            cx.define(scope, "rawIdxState", cx.get(&binding, &"rawIdxState")?);
            cx.define(
                scope,
                "requestDidSucceed",
                cx.get(&binding, &"requestDidSucceed")?,
            );
            cx.define(scope, "stepUp", cx.get(&binding, &"stepUp")?);
        }
        return Ok({
            let object = cx.object();
            cx.put(&object, &"status", cx.lookup(scope, "status")?)?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "meta")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"meta", cx.lookup(scope, "meta")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "enabledFeatures")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"enabledFeatures",
                            cx.lookup(scope, "enabledFeatures")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "availableSteps")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"availableSteps",
                            cx.lookup(scope, "availableSteps")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "tokens")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"tokens", cx.lookup(scope, "tokens")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "nextStep")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"nextStep", cx.lookup(scope, "nextStep")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = {
                    let left = cx.lookup(scope, "messages")?;
                    if left.truthy() {
                        cx.get(&cx.lookup(scope, "messages")?, &"length")?
                    } else {
                        left
                    }
                };
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"messages", cx.lookup(scope, "messages")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "error")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"error", cx.lookup(scope, "error")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.lookup(scope, "stepUp")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(&object, &"stepUp", cx.lookup(scope, "stepUp")?)?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.put(
                &object,
                &"interactionCode",
                cx.lookup(scope, "interactionCode")?,
            )?;
            cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
            cx.put(&object, &"context", cx.lookup(scope, "context")?)?;
            cx.put(
                &object,
                &"neededToProceed",
                cx.lookup(scope, "neededToProceed")?,
            )?;
            cx.put(&object, &"proceed", cx.lookup(scope, "proceed")?)?;
            cx.put(&object, &"rawIdxState", cx.lookup(scope, "rawIdxState")?)?;
            cx.put(
                &object,
                &"requestDidSucceed",
                cx.lookup(scope, "requestDidSucceed")?,
            )?;
            object
        });
        Ok(V::Undefined)
    })
}

fn f97(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        let _ = {
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
            let method = "clear";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        };
        if V::Bool(
            !cx.get(&cx.lookup(scope, "options")?, &"enableLegacyMode")?
                .truthy(),
        )
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let object = cx.lookup(scope, "options")?;
                    let key = "autoRemediate";
                    let value = V::Bool(false);
                    cx.put(&object, &key, value)?
                };
            }
        }
        return Ok({
            let function = cx.lookup(scope, "run")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"exchangeCodeForTokens", V::Bool(false))?;
                    cx.spread(&object, &cx.lookup(scope, "options")?)?;
                    object
                });
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f98(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(
            scope,
            "authClient",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "options", value);
        }
        cx.define(
            scope,
            "tokenParams",
            cx.await_value({
                let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"token")?;
                let method = "prepareTokenParams";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "options")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })
            .await?,
        );
        cx.define(scope, "pkceMeta", {
            let function = cx.lookup(scope, "createOAuthMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "tokenParams")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        {
            let binding = {
                let object = cx.object();
                cx.spread(
                    &object,
                    &cx.get(&cx.lookup(scope, "authClient")?, &"options")?,
                )?;
                cx.spread(&object, &cx.lookup(scope, "options")?)?;
                object
            };
            {
                let value = cx.get(&binding, &"flow")?;
                let value = if matches!(value, V::Undefined) {
                    V::string("default")
                } else {
                    value
                };
                cx.define(scope, "flow", value);
            }
            {
                let value = cx.get(&binding, &"withCredentials")?;
                let value = if matches!(value, V::Undefined) {
                    V::Bool(true)
                } else {
                    value
                };
                cx.define(scope, "withCredentials", value);
            }
            {
                let value = cx.get(&binding, &"activationToken")?;
                let value = if matches!(value, V::Undefined) {
                    cx.lookup(scope, "undefined")?
                } else {
                    value
                };
                cx.define(scope, "activationToken", value);
            }
            {
                let value = cx.get(&binding, &"recoveryToken")?;
                let value = if matches!(value, V::Undefined) {
                    cx.lookup(scope, "undefined")?
                } else {
                    value
                };
                cx.define(scope, "recoveryToken", value);
            }
            {
                let value = cx.get(&binding, &"maxAge")?;
                let value = if matches!(value, V::Undefined) {
                    cx.lookup(scope, "undefined")?
                } else {
                    value
                };
                cx.define(scope, "maxAge", value);
            }
            {
                let value = cx.get(&binding, &"acrValues")?;
                let value = if matches!(value, V::Undefined) {
                    cx.lookup(scope, "undefined")?
                } else {
                    value
                };
                cx.define(scope, "acrValues", value);
            }
        }
        cx.define(scope, "meta", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "pkceMeta")?)?;
            cx.put(&object, &"flow", cx.lookup(scope, "flow")?)?;
            cx.put(
                &object,
                &"withCredentials",
                cx.lookup(scope, "withCredentials")?,
            )?;
            cx.put(
                &object,
                &"activationToken",
                cx.lookup(scope, "activationToken")?,
            )?;
            cx.put(
                &object,
                &"recoveryToken",
                cx.lookup(scope, "recoveryToken")?,
            )?;
            cx.put(&object, &"maxAge", cx.lookup(scope, "maxAge")?)?;
            cx.put(&object, &"acrValues", cx.lookup(scope, "acrValues")?)?;
            object
        });
        return Ok(cx.lookup(scope, "meta")?);
        Ok(V::Undefined)
    })
}

fn f99(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.assign(scope, "options", {
        let function = cx.lookup(scope, "removeNils")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    })?;
    let _ = cx.assign(scope, "options", {
        let object = cx.object();
        cx.spread(
            &object,
            &cx.get(&cx.lookup(scope, "authClient")?, &"options")?,
        )?;
        cx.spread(&object, &cx.lookup(scope, "options")?)?;
        object
    })?;
    cx.define(scope, "savedMeta", V::Undefined);
    {
        let mut completion = (|| {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "savedMeta", {
                    let receiver =
                        cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
                    let method = "load";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                })?;
            }
            Ok::<Completion, V>(Completion::Normal)
        })();
        if let Err(error) = completion {
            completion = (|| {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "e", error);
                {
                    let scope = cx.scope(Some(scope));
                }
                Ok::<Completion, V>(Completion::Normal)
            })();
        }
        match completion? {
            Completion::Normal => {}
            Completion::Return(value) => {
                return Ok(value);
            }
            Completion::Break => {
                return Err(V::string("unexpected break"));
            }
            Completion::Continue => {
                return Err(V::string("unexpected continue"));
            }
        }
    }
    if V::Bool(!cx.lookup(scope, "savedMeta")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    if {
        let function = cx.lookup(scope, "isTransactionMetaValid")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "savedMeta")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "savedMeta")?);
        }
    }
    let _ = {
        let function = cx.lookup(scope, "warn")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(binary(
                "+",
                V::string("Saved transaction meta does not match the current configuration. "),
                V::string("This may indicate that two apps are sharing a storage key."),
            )?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    };

    Ok(V::Undefined)
}

fn f100(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "meta", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "keys", {
        let mut values = vec![];
        values.push(V::string("issuer"));
        values.push(V::string("clientId"));
        values.push(V::string("redirectUri"));
        values.push(V::string("state"));
        values.push(V::string("codeChallenge"));
        values.push(V::string("codeChallengeMethod"));
        values.push(V::string("activationToken"));
        values.push(V::string("recoveryToken"));
        cx.array(values)
    });
    if binary(
        "===",
        {
            let function = cx.lookup(scope, "isTransactionMetaValidForOptions")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "meta")?);
                argv.push(cx.lookup(scope, "options")?);
                argv.push(cx.lookup(scope, "keys")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        },
        V::Bool(false),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    {
        let binding = cx.lookup(scope, "options")?;
        cx.define(scope, "flow", cx.get(&binding, &"flow")?);
    }
    if binary(
        "===",
        {
            let function = cx.lookup(scope, "isTransactionMetaValidForFlow")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "meta")?);
                argv.push(cx.lookup(scope, "flow")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        },
        V::Bool(false),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    return Ok(V::Bool(true));

    Ok(V::Undefined)
}

fn f101(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "meta", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "flow", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "shouldValidateFlow", {
        let left = {
            let left = cx.lookup(scope, "flow")?;
            if left.truthy() {
                binary("!==", cx.lookup(scope, "flow")?, V::string("default"))?
            } else {
                left
            }
        };
        if left.truthy() {
            binary("!==", cx.lookup(scope, "flow")?, V::string("proceed"))?
        } else {
            left
        }
    });
    if cx.lookup(scope, "shouldValidateFlow")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            if binary(
                "!==",
                cx.lookup(scope, "flow")?,
                cx.get(&cx.lookup(scope, "meta")?, &"flow")?,
            )?
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(V::Bool(false));
                }
            }
        }
    }
    return Ok(V::Bool(true));

    Ok(V::Undefined)
}

fn f103(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "key", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "value",
        cx.get(
            &cx.lookup(scope, "options")?,
            &cx.lookup(scope, "key")?.text(),
        )?,
    );
    if {
        let left = cx.lookup(scope, "value")?;
        if left.truthy() {
            binary(
                "!==",
                cx.lookup(scope, "value")?,
                cx.get(&cx.lookup(scope, "meta")?, &cx.lookup(scope, "key")?.text())?,
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(true));
        }
    }

    Ok(V::Undefined)
}

fn f102(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "meta", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "keys", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "mismatch", {
        let receiver = cx.lookup(scope, "keys")?;
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f103));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok(V::Bool(!cx.lookup(scope, "mismatch")?.truthy()));

    Ok(V::Undefined)
}

fn f104(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "meta", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = {
        let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"transactionManager")?;
        let method = "save";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "meta")?);
            argv.push({
                let object = cx.object();
                cx.put(&object, &"muteWarning", V::Bool(true))?;
                object
            });
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f105(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = cx.lookup(scope, "obj")?;
        if left.truthy() {
            {
                let left = cx.get(&cx.lookup(scope, "obj")?, &"key")?;
                if left.truthy() {
                    left
                } else {
                    cx.get(&cx.lookup(scope, "obj")?, &"id")?
                }
            }
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f106(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = cx.lookup(scope, "obj")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "obj")?, &"version")?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f108(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "actionName",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let receiver = cx.lookup(scope, "actionName")?;
        let method = "includes";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::string("resend"));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f107(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "keys";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f108));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f110(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary("===", cx.lookup(scope, "name")?, V::string("skip"))?);

    Ok(V::Undefined)
}

fn f109(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let receiver = cx.get(&cx.lookup(scope, "idxResponse")?, &"neededToProceed")?;
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f110));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f112(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "r", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "===",
        cx.get(&cx.lookup(scope, "r")?, &"name")?,
        cx.lookup(scope, "remediationName")?,
    )?);

    Ok(V::Undefined)
}

fn f113(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "res", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "entry", args.get(1).cloned().unwrap_or(V::Undefined));
    {
        let binding = cx.lookup(scope, "entry")?;
        cx.define(scope, "name", cx.get(&binding, &"name")?);
        cx.define(scope, "value", cx.get(&binding, &"value")?);
    }
    if binary("===", cx.lookup(scope, "name")?, V::string("stateHandle"))?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "res")?;
                let key = cx.lookup(scope, "name")?.text();
                let value = cx.lookup(scope, "value")?;
                cx.put(&object, &key, value)?
            };
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "res")?;
                let key = cx.lookup(scope, "name")?.text();
                let value = cx.get(
                    &cx.lookup(scope, "values")?,
                    &cx.lookup(scope, "name")?.text(),
                )?;
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok(cx.lookup(scope, "res")?);

    Ok(V::Undefined)
}

fn f111(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "remediationName",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "values",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "remediations", {
        let left = cx.get(&cx.lookup(scope, "idxResponse")?, &"neededToProceed")?;
        if left.truthy() {
            left
        } else {
            {
                let mut values = vec![];
                cx.array(values)
            }
        }
    });
    cx.define(scope, "remediation", {
        let receiver = cx.lookup(scope, "remediations")?;
        let method = "find";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f112));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if V::Bool(!cx.lookup(scope, "remediation")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let function = cx.lookup(scope, "warn")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push({
                        let mut text = String::new();
                        text.push_str("filterValuesForRemediation: \"");
                        text.push_str(&cx.lookup(scope, "remediationName")?.text());
                        text.push_str("\" did not match any remediations");
                        V::string(text)
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            return Ok(cx.lookup(scope, "values")?);
        }
    }
    cx.define(scope, "valuesForRemediation", {
        let receiver = cx.get(&cx.lookup(scope, "remediation")?, &"value")?;
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f113));
            argv.push({
                let object = cx.object();
                object
            });
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok(cx.lookup(scope, "valuesForRemediation")?);

    Ok(V::Undefined)
}

fn f115(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "map", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "remediatorClass",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    if cx
        .get(&cx.lookup(scope, "remediatorClass")?, &"remediationName")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "map")?;
                let key = cx
                    .get(&cx.lookup(scope, "remediatorClass")?, &"remediationName")?
                    .text();
                let value = cx.lookup(scope, "remediatorClass")?;
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok(cx.lookup(scope, "map")?);

    Ok(V::Undefined)
}

fn f116(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(
            scope,
            "params",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        return Ok({
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
            let method = "proceed";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"actions", {
                        let mut values = vec![];
                        values.push({
                            let object = cx.object();
                            cx.put(&object, &"name", cx.lookup(scope, "name")?)?;
                            cx.put(&object, &"params", cx.lookup(scope, "params")?)?;
                            object
                        });
                        cx.array(values)
                    })?;
                    object
                });
                argv
            };
            cx.call(function, receiver, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f118(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "item", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "!==",
        cx.get(&cx.lookup(scope, "item")?, &"name")?,
        V::string("stateHandle"),
    )?);

    Ok(V::Undefined)
}

fn f114(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "useGenericRemediator",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "res", {
        let mut values = vec![];
        cx.array(values)
    });
    cx.define(scope, "remediatorMap", {
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "values";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "ctx")?, &"remediators")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f115));
            argv.push({
                let object = cx.object();
                object
            });
            argv
        };
        cx.call(function, receiver, argv)?
    });
    for item in cx.items(&cx.get(&cx.lookup(scope, "idxResponse")?, &"neededToProceed")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "remediation", item);
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "T", {
                let function = cx.lookup(scope, "getRemediatorClass")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "remediation")?);
                    argv.push({
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"useGenericRemediator",
                            cx.lookup(scope, "useGenericRemediator")?,
                        )?;
                        cx.put(&object, &"remediators", cx.lookup(scope, "remediatorMap")?)?;
                        object
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            if cx.lookup(scope, "T")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(
                        scope,
                        "remediator",
                        cx.construct(cx.lookup(scope, "T")?, {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediation")?);
                            argv
                        })?,
                    );
                    let _ = {
                        let receiver = cx.lookup(scope, "res")?;
                        let method = "push";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let receiver = cx.lookup(scope, "remediator")?;
                                let method = "getNextStep";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "authClient")?);
                                    argv.push(
                                        cx.get(&cx.lookup(scope, "idxResponse")?, &"context")?,
                                    );
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            });
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
            }
        }
    }
    for item in cx.items(&{
        let receiver = cx.lookup(scope, "Object")?;
        let method = "entries";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let left = cx.get(&cx.lookup(scope, "idxResponse")?, &"actions")?;
                if left.truthy() {
                    left
                } else {
                    {
                        let object = cx.object();
                        object
                    }
                }
            });
            argv
        };
        cx.call(function, receiver, argv)?
    })? {
        let scope = cx.scope(Some(scope));
        {
            let binding = item;
            cx.define(scope, "name", cx.get(&binding, "0")?);
        }
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "stepObj", {
                let object = cx.object();
                cx.put(&object, &"name", cx.lookup(scope, "name")?)?;
                cx.put(
                    &object,
                    &"action",
                    cx.async_arrow(scope, this.clone(), f116),
                )?;
                object
            });
            if {
                let receiver = cx.lookup(scope, "name")?;
                let method = "startsWith";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("currentAuthenticator"));
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    {
                        let binding = {
                            let function = cx.lookup(scope, "split2")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "name")?);
                                argv.push(V::string("-"));
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        };
                        cx.define(scope, "part1", cx.get(&binding, "0")?);
                        cx.define(scope, "part2", cx.get(&binding, "1")?);
                    }
                    cx.define(
                        scope,
                        "actionObj",
                        cx.get(
                            &cx.get(
                                &cx.get(
                                    &cx.get(&cx.lookup(scope, "idxResponse")?, &"rawIdxState")?,
                                    &cx.lookup(scope, "part1")?.text(),
                                )?,
                                &"value",
                            )?,
                            &cx.lookup(scope, "part2")?.text(),
                        )?,
                    );
                    {
                        let binding = cx.lookup(scope, "actionObj")?;
                        cx.define(scope, "href", cx.get(&binding, &"href")?);
                        cx.define(scope, "method", cx.get(&binding, &"method")?);
                        cx.define(scope, "rel", cx.get(&binding, &"rel")?);
                        cx.define(scope, "accepts", cx.get(&binding, &"accepts")?);
                        cx.define(scope, "produces", cx.get(&binding, &"produces")?);
                        cx.define(
                            scope,
                            "rest",
                            cx.rest(
                                &binding,
                                &vec![
                                    "href".to_string(),
                                    "method".to_string(),
                                    "rel".to_string(),
                                    "accepts".to_string(),
                                    "produces".to_string(),
                                ],
                            )?,
                        );
                    }
                    cx.define(scope, "value", 'chain117: {
                        {
                            let receiver = cx.get(&cx.lookup(scope, "actionObj")?, &"value")?;
                            if receiver.nullish() {
                                break 'chain117 V::Undefined;
                            }
                            let method = "filter";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.arrow(scope, this.clone(), f118));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        }
                    });
                    let _ = cx.assign(scope, "stepObj", {
                        let object = cx.object();
                        cx.spread(&object, &cx.lookup(scope, "rest")?)?;
                        cx.spread(&object, &{
                            let left = cx.lookup(scope, "value")?;
                            if left.truthy() {
                                {
                                    let object = cx.object();
                                    cx.put(&object, &"value", cx.lookup(scope, "value")?)?;
                                    object
                                }
                            } else {
                                left
                            }
                        })?;
                        cx.spread(&object, &cx.lookup(scope, "stepObj")?)?;
                        object
                    })?;
                }
            }
            let _ = {
                let receiver = cx.lookup(scope, "res")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "stepObj")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Ok(cx.lookup(scope, "res")?);

    Ok(V::Undefined)
}

fn f120(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        V::string("select-enroll-profile"),
    )?);

    Ok(V::Undefined)
}

fn f121(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        V::string("redirect-idp"),
    )?);

    Ok(V::Undefined)
}

fn f122(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        V::string("unlock-account"),
    )?);

    Ok(V::Undefined)
}

fn f119(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "res", {
        let mut values = vec![];
        cx.array(values)
    });
    {
        let binding = cx.lookup(scope, "idxResponse")?;
        cx.define(scope, "actions", cx.get(&binding, &"actions")?);
        cx.define(
            scope,
            "neededToProceed",
            cx.get(&binding, &"neededToProceed")?,
        );
    }
    if cx
        .get(
            &cx.lookup(scope, "actions")?,
            &V::string("currentAuthenticator-recover").text(),
        )?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "res")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "IdxFeature")?, &"PASSWORD_RECOVERY")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    if {
        let receiver = cx.lookup(scope, "neededToProceed")?;
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f120));
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "res")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "IdxFeature")?, &"REGISTRATION")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    if {
        let receiver = cx.lookup(scope, "neededToProceed")?;
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f121));
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "res")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "IdxFeature")?, &"SOCIAL_IDP")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    if {
        let receiver = cx.lookup(scope, "neededToProceed")?;
        let method = "some";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f122));
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "res")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "IdxFeature")?, &"ACCOUNT_UNLOCK")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Ok(cx.lookup(scope, "res")?);

    Ok(V::Undefined)
}

fn f123(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "oktaAuth",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            V::string("default")
        } else {
            value
        };
        cx.define(scope, "flow", value);
    }
    return Ok({
        let receiver = cx.lookup(scope, "ctx")?;
        let method = "getFlowSpecification";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "oktaAuth")?);
            argv.push(cx.lookup(scope, "flow")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f126(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "option",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = V::Bool(!cx.get(&cx.lookup(scope, "option")?, &"value")?.truthy());
        if left.truthy() {
            left
        } else {
            binary(
                "===",
                V::string(cx.get(&cx.lookup(scope, "option")?, &"value")?.typeof_()),
                V::string("string"),
            )?
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    let _ = cx.assign(scope, "optionValues", {
        let mut values = vec![];
        values.extend(cx.items(&cx.lookup(scope, "optionValues")?)?);
        values.push(cx.get(&cx.lookup(scope, "option")?, &"value")?);
        cx.array(values)
    })?;

    Ok(V::Undefined)
}

fn f125(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "messages",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "value", args.get(1).cloned().unwrap_or(V::Undefined));
    if cx.get(&cx.lookup(scope, "value")?, &"messages")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "messages", {
                let mut values = vec![];
                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                values.extend(cx.items(
                    &cx.get(&cx.get(&cx.lookup(scope, "value")?, &"messages")?, &"value")?,
                )?);
                cx.array(values)
            })?;
        }
    }
    if cx.get(&cx.lookup(scope, "value")?, &"form")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "messagesFromForm", {
                let left = {
                    let function = cx.lookup(scope, "getMessagesFromIdxRemediationValue")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(
                            cx.get(&cx.get(&cx.lookup(scope, "value")?, &"form")?, &"value")?,
                        );
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                if left.truthy() {
                    left
                } else {
                    {
                        let mut values = vec![];
                        cx.array(values)
                    }
                }
            });
            let _ = cx.assign(scope, "messages", {
                let mut values = vec![];
                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                values.extend(cx.items(&cx.lookup(scope, "messagesFromForm")?)?);
                cx.array(values)
            })?;
        }
    }
    if cx.get(&cx.lookup(scope, "value")?, &"options")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "optionValues", {
                let mut values = vec![];
                cx.array(values)
            });
            let _ = {
                let receiver = cx.get(&cx.lookup(scope, "value")?, &"options")?;
                let method = "forEach";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f126));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            cx.define(scope, "messagesFromOptions", {
                let left = {
                    let function = cx.lookup(scope, "getMessagesFromIdxRemediationValue")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "optionValues")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                if left.truthy() {
                    left
                } else {
                    {
                        let mut values = vec![];
                        cx.array(values)
                    }
                }
            });
            let _ = cx.assign(scope, "messages", {
                let mut values = vec![];
                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                values.extend(cx.items(&cx.lookup(scope, "messagesFromOptions")?)?);
                cx.array(values)
            })?;
        }
    }
    return Ok(cx.lookup(scope, "messages")?);

    Ok(V::Undefined)
}

fn f124(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "value", args.get(0).cloned().unwrap_or(V::Undefined));
    if {
        let left = V::Bool(!cx.lookup(scope, "value")?.truthy());
        if left.truthy() {
            left
        } else {
            V::Bool(
                !{
                    let receiver = cx.lookup(scope, "Array")?;
                    let method = "isArray";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "value")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy(),
            )
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    return Ok({
        let receiver = cx.lookup(scope, "value")?;
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f125));
            argv.push({
                let mut values = vec![];
                cx.array(values)
            });
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f129(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "message",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok(cx.lookup(scope, "message")?);

    Ok(V::Undefined)
}

fn f130(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "filtered",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "message",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "key", 'chain131: {
        {
            let object = cx.get(&cx.lookup(scope, "message")?, &"i18n")?;
            if object.nullish() {
                break 'chain131 V::Undefined;
            }
            cx.get(&object, &"key")?
        }
    });
    if {
        let left = {
            let left = cx.lookup(scope, "key")?;
            if left.truthy() {
                cx.get(&cx.lookup(scope, "seen")?, &cx.lookup(scope, "key")?.text())?
            } else {
                left
            }
        };
        if left.truthy() {
            binary(
                "===",
                cx.get(&cx.lookup(scope, "message")?, &"message")?,
                cx.get(
                    &cx.get(&cx.lookup(scope, "seen")?, &cx.lookup(scope, "key")?.text())?,
                    &"message",
                )?,
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "filtered")?);
        }
    }
    let _ = {
        let object = cx.lookup(scope, "seen")?;
        let key = cx.lookup(scope, "key")?.text();
        let value = cx.lookup(scope, "message")?;
        cx.put(&object, &key, value)?
    };
    let _ = cx.assign(scope, "filtered", {
        let mut values = vec![];
        values.extend(cx.items(&cx.lookup(scope, "filtered")?)?);
        values.push(cx.lookup(scope, "message")?);
        cx.array(values)
    })?;
    return Ok(cx.lookup(scope, "filtered")?);

    Ok(V::Undefined)
}

fn f127(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "messages", {
        let mut values = vec![];
        cx.array(values)
    });
    {
        let binding = cx.lookup(scope, "idxResponse")?;
        cx.define(scope, "rawIdxState", cx.get(&binding, &"rawIdxState")?);
        cx.define(
            scope,
            "neededToProceed",
            cx.get(&binding, &"neededToProceed")?,
        );
    }
    cx.define(scope, "globalMessages", 'chain128: {
        {
            let receiver = {
                let object = cx.get(&cx.lookup(scope, "rawIdxState")?, &"messages")?;
                if object.nullish() {
                    break 'chain128 V::Undefined;
                }
                cx.get(&object, &"value")?
            };
            let method = "map";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f129));
                argv
            };
            cx.call(function, receiver, argv)?
        }
    });
    if cx.lookup(scope, "globalMessages")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "messages", {
                let mut values = vec![];
                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                values.extend(cx.items(&cx.lookup(scope, "globalMessages")?)?);
                cx.array(values)
            })?;
        }
    }
    if V::Bool(
        !cx.get(&cx.lookup(scope, "options")?, &"useGenericRemediator")?
            .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            for item in cx.items(&cx.lookup(scope, "neededToProceed")?)? {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "remediation", item);
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "fieldMessages", {
                        let function = cx.lookup(scope, "getMessagesFromIdxRemediationValue")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&cx.lookup(scope, "remediation")?, &"value")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    if cx.lookup(scope, "fieldMessages")?.truthy() {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = cx.assign(scope, "messages", {
                                let mut values = vec![];
                                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                                values.extend(cx.items(&cx.lookup(scope, "fieldMessages")?)?);
                                cx.array(values)
                            })?;
                        }
                    }
                }
            }
        }
    }
    cx.define(scope, "seen", {
        let object = cx.object();
        object
    });
    let _ = cx.assign(scope, "messages", {
        let receiver = cx.lookup(scope, "messages")?;
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f130));
            argv.push({
                let mut values = vec![];
                cx.array(values)
            });
            argv
        };
        cx.call(function, receiver, argv)?
    })?;
    return Ok(cx.lookup(scope, "messages")?);

    Ok(V::Undefined)
}

fn f132(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "remediator",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "nextStep", {
        let receiver = cx.lookup(scope, "remediator")?;
        let method = "getNextStep";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push(cx.get(&cx.lookup(scope, "idxResponse")?, &"context")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "canSkip", {
        let function = cx.lookup(scope, "canSkipFn")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "canResend", {
        let function = cx.lookup(scope, "canResendFn")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "nextStep")?)?;
        cx.spread(&object, &{
            let left = cx.lookup(scope, "canSkip")?;
            if left.truthy() {
                {
                    let object = cx.object();
                    cx.put(&object, &"canSkip", cx.lookup(scope, "canSkip")?)?;
                    object
                }
            } else {
                left
            }
        })?;
        cx.spread(&object, &{
            let left = cx.lookup(scope, "canResend")?;
            if left.truthy() {
                {
                    let object = cx.object();
                    cx.put(&object, &"canResend", cx.lookup(scope, "canResend")?)?;
                    object
                }
            } else {
                left
            }
        })?;
        object
    });

    Ok(V::Undefined)
}

fn f134(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        cx.get(&cx.lookup(scope, "options")?, &"step")?,
    )?);

    Ok(V::Undefined)
}

fn f133(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "values",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "options",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "remediators",
        cx.get(&cx.lookup(scope, "options")?, &"remediators")?,
    );
    cx.define(
        scope,
        "useGenericRemediator",
        cx.get(&cx.lookup(scope, "options")?, &"useGenericRemediator")?,
    );
    {
        let binding = cx.lookup(scope, "idxResponse")?;
        cx.define(
            scope,
            "idxRemediations",
            cx.get(&binding, &"neededToProceed")?,
        );
        cx.define(scope, "context", cx.get(&binding, &"context")?);
    }
    cx.define(scope, "remediator", V::Undefined);
    if cx.get(&cx.lookup(scope, "options")?, &"step")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "remediation", {
                let receiver = cx.lookup(scope, "idxRemediations")?;
                let method = "find";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f134));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            if cx.lookup(scope, "remediation")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "T", {
                        let function = cx.lookup(scope, "getRemediatorClass")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediation")?);
                            argv.push(cx.lookup(scope, "options")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    return Ok(if cx.lookup(scope, "T")?.truthy() {
                        cx.construct(cx.lookup(scope, "T")?, {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediation")?);
                            argv.push(cx.lookup(scope, "values")?);
                            argv.push(cx.lookup(scope, "options")?);
                            argv
                        })?
                    } else {
                        cx.lookup(scope, "undefined")?
                    });
                }
            } else {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let function = cx.lookup(scope, "warn")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let mut text = String::new();
                                text.push_str("step \"");
                                text.push_str(
                                    &cx.get(&cx.lookup(scope, "options")?, &"step")?.text(),
                                );
                                text.push_str("\" did not match any remediations");
                                V::string(text)
                            });
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    };
                    return Ok(V::Undefined);
                }
            }
        }
    }
    cx.define(scope, "remediatorCandidates", {
        let mut values = vec![];
        cx.array(values)
    });
    if cx.lookup(scope, "useGenericRemediator")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "remediatorCandidates")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.construct(cx.lookup(scope, "GenericRemediator")?, {
                        let mut argv = Vec::new();
                        argv.push(cx.get(
                            &cx.lookup(scope, "idxRemediations")?,
                            &V::Number(0f64).text(),
                        )?);
                        argv.push(cx.lookup(scope, "values")?);
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    })?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            for item in cx.items(&cx.lookup(scope, "idxRemediations")?)? {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "remediation", item);
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "isRemeditionInFlow", {
                        let receiver = {
                            let receiver = cx.lookup(scope, "Object")?;
                            let method = "keys";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "remediators")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        };
                        let method = "includes";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&cx.lookup(scope, "remediation")?, &"name")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                    if V::Bool(!cx.lookup(scope, "isRemeditionInFlow")?.truthy()).truthy() {
                        {
                            let scope = cx.scope(Some(scope));
                            continue;
                        }
                    }
                    cx.define(scope, "T", {
                        let function = cx.lookup(scope, "getRemediatorClass")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediation")?);
                            argv.push(cx.lookup(scope, "options")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    let _ = cx.assign(
                        scope,
                        "remediator",
                        cx.construct(cx.lookup(scope, "T")?, {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediation")?);
                            argv.push(cx.lookup(scope, "values")?);
                            argv.push(cx.lookup(scope, "options")?);
                            argv
                        })?,
                    )?;
                    if {
                        let receiver = cx.lookup(scope, "remediator")?;
                        let method = "canRemediate";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "context")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            return Ok(cx.lookup(scope, "remediator")?);
                        }
                    }
                    let _ = {
                        let receiver = cx.lookup(scope, "remediatorCandidates")?;
                        let method = "push";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "remediator")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
            }
        }
    }
    return Ok(cx.get(
        &cx.lookup(scope, "remediatorCandidates")?,
        &V::Number(0f64).text(),
    )?);

    Ok(V::Undefined)
}

fn f135(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    {
        let binding = cx.lookup(scope, "options")?;
        cx.define(
            scope,
            "useGenericRemediator",
            cx.get(&binding, &"useGenericRemediator")?,
        );
        cx.define(scope, "remediators", cx.get(&binding, &"remediators")?);
    }
    if V::Bool(!cx.lookup(scope, "remediation")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "undefined")?);
        }
    }
    if cx.lookup(scope, "useGenericRemediator")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "GenericRemediator")?);
        }
    }
    return Ok(cx.get(
        &cx.lookup(scope, "remediators")?,
        &cx.get(&cx.lookup(scope, "remediation")?, &"name")?.text(),
    )?);

    Ok(V::Undefined)
}

fn f136(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(2).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "terminal", {
        let function = cx.lookup(scope, "isTerminalResponse")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "messages", {
        let function = cx.lookup(scope, "getMessagesFromResponse")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if cx.lookup(scope, "terminal")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let object = cx.object();
                cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                cx.put(&object, &"terminal", cx.lookup(scope, "terminal")?)?;
                cx.put(&object, &"messages", cx.lookup(scope, "messages")?)?;
                object
            });
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "remediator", {
                let function = cx.lookup(scope, "getRemediator")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "idxResponse")?);
                    argv.push({
                        let object = cx.object();
                        object
                    });
                    argv.push(cx.lookup(scope, "options")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            cx.define(scope, "nextStep", {
                let left = cx.lookup(scope, "remediator")?;
                if left.truthy() {
                    {
                        let function = cx.lookup(scope, "getNextStep")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "authClient")?);
                            argv.push(cx.lookup(scope, "remediator")?);
                            argv.push(cx.lookup(scope, "idxResponse")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    }
                } else {
                    left
                }
            });
            return Ok({
                let object = cx.object();
                cx.put(&object, &"idxResponse", cx.lookup(scope, "idxResponse")?)?;
                cx.put(&object, &"messages", cx.lookup(scope, "messages")?)?;
                cx.spread(&object, &{
                    let left = cx.lookup(scope, "nextStep")?;
                    if left.truthy() {
                        {
                            let object = cx.object();
                            cx.put(&object, &"nextStep", cx.lookup(scope, "nextStep")?)?;
                            object
                        }
                    } else {
                        left
                    }
                })?;
                object
            });
        }
    }

    Ok(V::Undefined)
}

fn f137(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let binding = cx.lookup(scope, "idxResponse")?;
        cx.define(
            scope,
            "neededToProceed",
            cx.get(&binding, &"neededToProceed")?,
        );
        cx.define(
            scope,
            "interactionCode",
            cx.get(&binding, &"interactionCode")?,
        );
    }
    return Ok({
        let left = V::Bool(
            !cx.get(&cx.lookup(scope, "neededToProceed")?, &"length")?
                .truthy(),
        );
        if left.truthy() {
            V::Bool(!cx.lookup(scope, "interactionCode")?.truthy())
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f139(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    let _ = {
        let object = this.clone();
        let key = "storageManager";
        let value = cx.get(&cx.lookup(scope, "options")?, &"storageManager")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "enableSharedStorage";
        let value = if binary(
            "===",
            cx.get(&cx.lookup(scope, "options")?, &"enableSharedStorage")?,
            V::Bool(false),
        )?
        .truthy()
        {
            V::Bool(false)
        } else {
            V::Bool(true)
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "saveLastResponse";
        let value = if binary(
            "===",
            cx.get(&cx.lookup(scope, "options")?, &"saveLastResponse")?,
            V::Bool(false),
        )?
        .truthy()
        {
            V::Bool(false)
        } else {
            V::Bool(true)
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "options";
        let value = cx.lookup(scope, "options")?;
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f140(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "transactionStorage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getTransactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "meta", {
        let receiver = cx.lookup(scope, "transactionStorage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = {
        let receiver = cx.lookup(scope, "transactionStorage")?;
        let method = "clearStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    };
    if {
        let left = cx.get(&this.clone(), &"enableSharedStorage")?;
        if left.truthy() {
            binary(
                "!==",
                cx.get(&cx.lookup(scope, "options")?, &"clearSharedStorage")?,
                V::Bool(false),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "state", {
                let left = cx.get(&cx.lookup(scope, "options")?, &"state")?;
                if left.truthy() {
                    left
                } else {
                    'chain141: {
                        {
                            let object = cx.lookup(scope, "meta")?;
                            if object.nullish() {
                                break 'chain141 V::Undefined;
                            }
                            cx.get(&object, &"state")?
                        }
                    }
                }
            });
            if cx.lookup(scope, "state")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let function = cx.lookup(scope, "clearTransactionFromSharedStorage")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&this.clone(), &"storageManager")?);
                            argv.push(cx.lookup(scope, "state")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    };
                }
            }
        }
    }

    Ok(V::Undefined)
}

fn f142(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "meta", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "storage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getTransactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "obj", {
        let receiver = cx.lookup(scope, "storage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if {
        let left = {
            let function = cx.lookup(scope, "isTransactionMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "obj")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            V::Bool(
                !cx.get(&cx.lookup(scope, "options")?, &"muteWarning")?
                    .truthy(),
            )
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let function = cx.lookup(scope, "warn")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("a saved auth transaction exists in storage. This may indicate another auth flow is already in progress."));
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
        }
    }
    let _ = {
        let receiver = cx.lookup(scope, "storage")?;
        let method = "setStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "meta")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };
    if {
        let left = cx.get(&this.clone(), &"enableSharedStorage")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "meta")?, &"state")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let function = cx.lookup(scope, "saveTransactionToSharedStorage")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&this.clone(), &"storageManager")?);
                    argv.push(cx.get(&cx.lookup(scope, "meta")?, &"state")?);
                    argv.push(cx.lookup(scope, "meta")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f143(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    {
        let mut completion = (|| {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "meta", {
                    let receiver = this.clone();
                    let method = "load";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
                return Ok(Completion::Return(V::Bool(
                    !V::Bool(!cx.lookup(scope, "meta")?.truthy()).truthy(),
                )));
            }
            Ok::<Completion, V>(Completion::Normal)
        })();
        if let Err(error) = completion {
            completion = (|| {
                let scope = cx.scope(Some(scope));
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(Completion::Return(V::Bool(false)));
                }
                Ok::<Completion, V>(Completion::Normal)
            })();
        }
        match completion? {
            Completion::Normal => {}
            Completion::Return(value) => {
                return Ok(value);
            }
            Completion::Break => {
                return Err(V::string("unexpected break"));
            }
            Completion::Continue => {
                return Err(V::string("unexpected continue"));
            }
        }
    }

    Ok(V::Undefined)
}

fn f144(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "meta", V::Undefined);
    if {
        let left = cx.get(&this.clone(), &"enableSharedStorage")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "options")?, &"state")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let function = cx.lookup(scope, "pruneSharedStorage")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&this.clone(), &"storageManager")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            let _ = cx.assign(scope, "meta", {
                let function = cx.lookup(scope, "loadTransactionFromSharedStorage")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&this.clone(), &"storageManager")?);
                    argv.push(cx.get(&cx.lookup(scope, "options")?, &"state")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            if {
                let function = cx.lookup(scope, "isTransactionMeta")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "meta")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(cx.lookup(scope, "meta")?);
                }
            }
        }
    }
    cx.define(scope, "storage", {
        let receiver = cx.get(&this.clone(), &"storageManager")?;
        let method = "getTransactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = cx.assign(scope, "meta", {
        let receiver = cx.lookup(scope, "storage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    })?;
    if {
        let function = cx.lookup(scope, "isTransactionMeta")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "meta")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "meta")?);
        }
    }
    return Ok(V::Null);

    Ok(V::Undefined)
}

fn f138(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok({
        let superclass = V::Undefined;
        let scope = cx.scope(Some(scope));
        cx.define(scope, "__super", superclass.clone());
        let constructor = cx.function(scope, f139);
        let prototype = cx.object();
        if !superclass.nullish() {
            let parent = cx.get(&superclass, "prototype")?;
            if !parent.nullish() {
                cx.set_prototype(&prototype, parent)?;
            }
            if matches!(superclass, V::Function(_)) {
                cx.set_prototype(&constructor, superclass)?;
            }
        }
        cx.put(&constructor, "prototype", prototype.clone())?;
        cx.define(scope, "TransactionManager", constructor.clone());
        cx.put(&prototype, &"clear", cx.function(scope, f140))?;
        cx.put(&prototype, &"save", cx.function(scope, f142))?;
        cx.put(&prototype, &"exists", cx.function(scope, f143))?;
        cx.put(&prototype, &"load", cx.function(scope, f144))?;
        constructor
    });

    Ok(V::Undefined)
}

fn f145(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "token", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "jwt", {
        let receiver = cx.lookup(scope, "token")?;
        let method = "split";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::string("."));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "decodedToken", V::Undefined);
    {
        let mut completion = (|| {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "decodedToken", {
                    let object = cx.object();
                    cx.put(&object, &"header", {
                        let receiver = cx.lookup(scope, "JSON")?;
                        let method = "parse";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let function = cx.lookup(scope, "base64UrlToString")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(
                                        cx.get(&cx.lookup(scope, "jwt")?, &V::Number(0f64).text())?,
                                    );
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            });
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    })?;
                    cx.put(&object, &"payload", {
                        let receiver = cx.lookup(scope, "JSON")?;
                        let method = "parse";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let function = cx.lookup(scope, "base64UrlToString")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(
                                        cx.get(&cx.lookup(scope, "jwt")?, &V::Number(1f64).text())?,
                                    );
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            });
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    })?;
                    cx.put(
                        &object,
                        &"signature",
                        cx.get(&cx.lookup(scope, "jwt")?, &V::Number(2f64).text())?,
                    )?;
                    object
                })?;
            }
            Ok::<Completion, V>(Completion::Normal)
        })();
        if let Err(error) = completion {
            completion = (|| {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "e", error);
                {
                    let scope = cx.scope(Some(scope));
                    return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                        let mut argv = Vec::new();
                        argv.push(V::string("Malformed token"));
                        argv
                    })?);
                }
                Ok::<Completion, V>(Completion::Normal)
            })();
        }
        match completion? {
            Completion::Normal => {}
            Completion::Return(value) => {
                return Ok(value);
            }
            Completion::Break => {
                return Err(V::string("unexpected break"));
            }
            Completion::Continue => {
                return Err(V::string("unexpected continue"));
            }
        }
    }
    return Ok(cx.lookup(scope, "decodedToken")?);

    Ok(V::Undefined)
}

fn f146(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = {
            let left = {
                let function = cx.lookup(scope, "isOAuthError")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "obj")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            if left.truthy() {
                left
            } else {
                {
                    let function = cx.lookup(scope, "isWWWAuthError")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "obj")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            }
        };
        if left.truthy() {
            binary(
                "===",
                cx.get(&cx.lookup(scope, "obj")?, &"errorCode")?,
                V::string("use_dpop_nonce"),
            )?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f147(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "params", {
        let function = cx.lookup(scope, "removeNils")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let object = cx.object();
                cx.put(
                    &object,
                    &"client_id",
                    cx.get(&cx.lookup(scope, "options")?, &"clientId")?,
                )?;
                cx.put(
                    &object,
                    &"redirect_uri",
                    cx.get(&cx.lookup(scope, "options")?, &"redirectUri")?,
                )?;
                cx.put(
                    &object,
                    &"grant_type",
                    if cx
                        .get(&cx.lookup(scope, "options")?, &"interactionCode")?
                        .truthy()
                    {
                        V::string("interaction_code")
                    } else {
                        V::string("authorization_code")
                    },
                )?;
                cx.put(
                    &object,
                    &"code_verifier",
                    cx.get(&cx.lookup(scope, "options")?, &"codeVerifier")?,
                )?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if cx
        .get(&cx.lookup(scope, "options")?, &"interactionCode")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "params")?;
                let key = V::string("interaction_code").text();
                let value = cx.get(&cx.lookup(scope, "options")?, &"interactionCode")?;
                cx.put(&object, &key, value)?
            };
        }
    } else {
        if cx
            .get(&cx.lookup(scope, "options")?, &"authorizationCode")?
            .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let object = cx.lookup(scope, "params")?;
                    let key = "code";
                    let value = cx.get(&cx.lookup(scope, "options")?, &"authorizationCode")?;
                    cx.put(&object, &key, value)?
                };
            }
        }
    }
    {
        let binding = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
        cx.define(scope, "clientSecret", cx.get(&binding, &"clientSecret")?);
    }
    if cx.lookup(scope, "clientSecret")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "params")?;
                let key = V::string("client_secret").text();
                let value = cx.lookup(scope, "clientSecret")?;
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok({
        let receiver = {
            let function = cx.lookup(scope, "toQueryString")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "params")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(1f64));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f148(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        {
            let binding = args.get(1).cloned().unwrap_or(V::Undefined);
            cx.define(scope, "url", cx.get(&binding, &"url")?);
            cx.define(scope, "data", cx.get(&binding, &"data")?);
            cx.define(scope, "nonce", cx.get(&binding, &"nonce")?);
            cx.define(scope, "dpopKeyPair", cx.get(&binding, &"dpopKeyPair")?);
        }
        cx.define(scope, "method", V::string("POST"));
        cx.define(scope, "headers", {
            let object = cx.object();
            cx.put(
                &object,
                &"Content-Type",
                V::string("application/x-www-form-urlencoded"),
            )?;
            object
        });
        {
            let mut completion = (async {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(
                        scope,
                        "resp",
                        cx.await_value({
                            let function = cx.lookup(scope, "httpRequest")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "sdk")?);
                                argv.push({
                                    let object = cx.object();
                                    cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
                                    cx.put(&object, &"method", cx.lookup(scope, "method")?)?;
                                    cx.put(&object, &"args", cx.lookup(scope, "data")?)?;
                                    cx.put(&object, &"headers", cx.lookup(scope, "headers")?)?;
                                    object
                                });
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })
                        .await?,
                    );
                    return Ok(Completion::Return(cx.lookup(scope, "resp")?));
                }
                Ok::<Completion, V>(Completion::Normal)
            })
            .await;
            if let Err(error) = completion {
                completion=(async{let scope=cx.scope(Some(scope));cx.define(scope,"err",error);
{let scope=cx.scope(Some(scope));
if {let left={let function=cx.lookup(scope,"isDPoPNonceError")?;let argv={let mut argv=Vec::new();argv.push(cx.lookup(scope,"err")?);argv};cx.call(function,V::Undefined,argv)?};if left.truthy(){V::Bool(!cx.lookup(scope,"nonce")?.truthy())}else{left}}.truthy(){{let scope=cx.scope(Some(scope));
cx.define(scope,"dpopNonce",'chain149:{cx.get(&{let object=cx.get(&cx.lookup(scope,"err")?,&"resp")?;if object.nullish(){break 'chain149 V::Undefined;}cx.get(&object,&"headers")?},&V::string("dpop-nonce").text())?});
if V::Bool(!cx.lookup(scope,"dpopNonce")?.truthy()).truthy(){{let scope=cx.scope(Some(scope));
return Err(cx.construct(cx.lookup(scope,"AuthApiError")?, {let mut argv=Vec::new();argv.push({let object=cx.object();cx.put(&object,&"errorSummary", V::string("No `dpop-nonce` header found when required"))?;object});argv.push({let left=cx.get(&cx.lookup(scope,"err")?,&"resp")?;if left.nullish(){cx.lookup(scope,"undefined")?}else{left}});argv})?);
}
}
return Ok(Completion::Return({let function=cx.lookup(scope,"makeTokenRequest")?;let argv={let mut argv=Vec::new();argv.push(cx.lookup(scope,"sdk")?);argv.push({let object=cx.object();cx.put(&object,&"url", cx.lookup(scope,"url")?)?;cx.put(&object,&"data", cx.lookup(scope,"data")?)?;cx.put(&object,&"dpopKeyPair", cx.lookup(scope,"dpopKeyPair")?)?;cx.put(&object,&"nonce", cx.lookup(scope,"dpopNonce")?)?;object});argv};cx.call(function,V::Undefined,argv)?}));
}
}
return Err(cx.lookup(scope,"err")?);
}
Ok::<Completion,V>(Completion::Normal)}).await;
            }
            match completion? {
                Completion::Normal => {}
                Completion::Return(value) => {
                    return Ok(value);
                }
                Completion::Break => {
                    return Err(V::string("unexpected break"));
                }
                Completion::Continue => {
                    return Err(V::string("unexpected continue"));
                }
            }
        }
        Ok(V::Undefined)
    })
}

fn f151(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, "0")?);
        cx.define(scope, "value", cx.get(&binding, "1")?);
    }
    return Ok(binary(
        "+",
        binary("+", cx.lookup(scope, "name")?, V::string("="))?,
        {
            let function = cx.lookup(scope, "encodeURIComponent")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "value")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        },
    )?);

    Ok(V::Undefined)
}

fn f150(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "options",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "refreshToken",
            args.get(2).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "data", {
            let receiver = {
                let receiver = {
                    let receiver = cx.lookup(scope, "Object")?;
                    let method = "entries";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let object = cx.object();
                            cx.put(
                                &object,
                                &"client_id",
                                cx.get(&cx.lookup(scope, "options")?, &"clientId")?,
                            )?;
                            cx.put(&object, &"grant_type", V::string("refresh_token"))?;
                            cx.put(&object, &"scope", {
                                let receiver =
                                    cx.get(&cx.lookup(scope, "refreshToken")?, &"scopes")?;
                                let method = "join";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(V::string(" "));
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            })?;
                            cx.put(
                                &object,
                                &"refresh_token",
                                cx.get(&cx.lookup(scope, "refreshToken")?, &"refreshToken")?,
                            )?;
                            object
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
                let method = "map";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.function(scope, f151));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "join";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("&"));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.define(
            scope,
            "url",
            cx.get(&cx.lookup(scope, "refreshToken")?, &"tokenUrl")?,
        );
        if {
            let left = cx.get(&cx.lookup(scope, "options")?, &"extraParams")?;
            if left.truthy() {
                binary(
                    ">=",
                    cx.get(
                        &{
                            let receiver = cx.lookup(scope, "Object")?;
                            let method = "keys";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.get(&cx.lookup(scope, "options")?, &"extraParams")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        },
                        &"length",
                    )?,
                    V::Number(1f64),
                )?
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "url",
                    binary("+", cx.lookup(scope, "url")?, {
                        let function = cx.lookup(scope, "toQueryString")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&cx.lookup(scope, "options")?, &"extraParams")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    })?,
                )?;
            }
        }
        cx.define(scope, "params", {
            let object = cx.object();
            cx.put(&object, &"url", cx.lookup(scope, "url")?)?;
            cx.put(&object, &"data", cx.lookup(scope, "data")?)?;
            cx.put(&object, &"dpopKeyPair", 'chain152: {
                {
                    let object = cx.lookup(scope, "options")?;
                    if object.nullish() {
                        break 'chain152 V::Undefined;
                    }
                    cx.get(&object, &"dpopKeyPair")?
                }
            })?;
            object
        });
        return Ok({
            let function = cx.lookup(scope, "makeTokenRequest")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "params")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f153(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "options",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "urls", args.get(2).cloned().unwrap_or(V::Undefined));
        let _ = {
            let function = cx.lookup(scope, "validateOptions")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        cx.define(scope, "data", {
            let function = cx.lookup(scope, "getPostData")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "options")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        cx.define(scope, "params", {
            let object = cx.object();
            cx.put(
                &object,
                &"url",
                cx.get(&cx.lookup(scope, "urls")?, &"tokenUrl")?,
            )?;
            cx.put(&object, &"data", cx.lookup(scope, "data")?)?;
            cx.put(&object, &"dpopKeyPair", 'chain154: {
                {
                    let object = cx.lookup(scope, "options")?;
                    if object.nullish() {
                        break 'chain154 V::Undefined;
                    }
                    cx.get(&object, &"dpopKeyPair")?
                }
            })?;
            object
        });
        return Ok({
            let function = cx.lookup(scope, "makeTokenRequest")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "params")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f155(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "options",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if V::Bool(!cx.get(&cx.lookup(scope, "options")?, &"clientId")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "A clientId must be specified in the OktaAuth constructor to get a token",
                ));
                argv
            })?);
        }
    }
    if V::Bool(
        !cx.get(&cx.lookup(scope, "options")?, &"redirectUri")?
            .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "The redirectUri passed to /authorize must also be passed to /token",
                ));
                argv
            })?);
        }
    }
    if {
        let left = V::Bool(
            !cx.get(&cx.lookup(scope, "options")?, &"authorizationCode")?
                .truthy(),
        );
        if left.truthy() {
            V::Bool(
                !cx.get(&cx.lookup(scope, "options")?, &"interactionCode")?
                    .truthy(),
            )
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "An authorization code (returned from /authorize) must be passed to /token",
                ));
                argv
            })?);
        }
    }
    if V::Bool(
        !cx.get(&cx.lookup(scope, "options")?, &"codeVerifier")?
            .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push(V::string("The \"codeVerifier\" (generated and saved by your app) must be passed to /token"));argv})?);
        }
    }

    Ok(V::Undefined)
}

fn f158(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "res", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "key", {
        let function = cx.lookup(scope, "find")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "res")?, &"keys")?);
            argv.push({
                let object = cx.object();
                cx.put(&object, &"kid", cx.lookup(scope, "kid")?)?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if cx.lookup(scope, "key")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "key")?);
        }
    }
    return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
        let mut argv = Vec::new();
        argv.push(binary(
            "+",
            binary("+", V::string("The key id, "), cx.lookup(scope, "kid")?)?,
            V::string(", was not found in the server's keys"),
        )?);
        argv
    })?);

    Ok(V::Undefined)
}

fn f157(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "wellKnown",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "jwksUri",
        cx.get(
            &cx.lookup(scope, "wellKnown")?,
            &V::string("jwks_uri").text(),
        )?,
    );
    cx.define(scope, "cacheContents", {
        let receiver = cx.lookup(scope, "httpCache")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(
        scope,
        "cachedResponse",
        cx.get(
            &cx.lookup(scope, "cacheContents")?,
            &cx.lookup(scope, "jwksUri")?.text(),
        )?,
    );
    if {
        let left = cx.lookup(scope, "cachedResponse")?;
        if left.truthy() {
            binary(
                "<",
                binary(
                    "/",
                    {
                        let receiver = cx.lookup(scope, "Date")?;
                        let method = "now";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    },
                    V::Number(1000f64),
                )?,
                cx.get(&cx.lookup(scope, "cachedResponse")?, &"expiresAt")?,
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "cachedKey", {
                let function = cx.lookup(scope, "find")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(
                        &cx.get(&cx.lookup(scope, "cachedResponse")?, &"response")?,
                        &"keys",
                    )?);
                    argv.push({
                        let object = cx.object();
                        cx.put(&object, &"kid", cx.lookup(scope, "kid")?)?;
                        object
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            if cx.lookup(scope, "cachedKey")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(cx.lookup(scope, "cachedKey")?);
                }
            }
        }
    }
    let _ = {
        let receiver = cx.lookup(scope, "httpCache")?;
        let method = "clearStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "jwksUri")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok({
        let receiver = {
            let function = cx.lookup(scope, "get")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "jwksUri")?);
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"cacheResponse", V::Bool(true))?;
                    object
                });
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f158));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f156(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "issuer",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "kid", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "httpCache", {
        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"storageManager")?;
        let method = "getHttpCache";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"cookies")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok({
        let receiver = {
            let function = cx.lookup(scope, "getWellKnown")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "issuer")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f157));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f159(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "issuer",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "authServerUri", {
        let left = cx.lookup(scope, "issuer")?;
        if left.truthy() {
            left
        } else {
            cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"issuer")?
        }
    });
    return Ok({
        let function = cx.lookup(scope, "get")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(binary(
                "+",
                cx.lookup(scope, "authServerUri")?,
                V::string("/.well-known/openid-configuration"),
            )?);
            argv.push({
                let object = cx.object();
                cx.put(&object, &"cacheResponse", V::Bool(true))?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f160(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "tokenParams",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "urls", args.get(2).cloned().unwrap_or(V::Undefined));
        let _ = cx.assign(scope, "urls", {
            let left = cx.lookup(scope, "urls")?;
            if left.truthy() {
                left
            } else {
                {
                    let function = cx.lookup(scope, "getOAuthUrls")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv.push(cx.lookup(scope, "tokenParams")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            }
        })?;
        let _ = cx.assign(scope, "tokenParams", {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "assign";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    object
                });
                argv.push({
                    let function = cx.lookup(scope, "getDefaultTokenParams")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
                argv.push({
                    let function = cx.lookup(scope, "clone")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "tokenParams")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                });
                argv
            };
            cx.call(function, receiver, argv)?
        })?;
        {
            let binding = cx.lookup(scope, "tokenParams")?;
            cx.define(
                scope,
                "authorizationCode",
                cx.get(&binding, &"authorizationCode")?,
            );
            cx.define(
                scope,
                "interactionCode",
                cx.get(&binding, &"interactionCode")?,
            );
            cx.define(scope, "codeVerifier", cx.get(&binding, &"codeVerifier")?);
            cx.define(scope, "clientId", cx.get(&binding, &"clientId")?);
            cx.define(scope, "redirectUri", cx.get(&binding, &"redirectUri")?);
            cx.define(scope, "scopes", cx.get(&binding, &"scopes")?);
            cx.define(
                scope,
                "ignoreSignature",
                cx.get(&binding, &"ignoreSignature")?,
            );
            cx.define(scope, "state", cx.get(&binding, &"state")?);
            cx.define(scope, "acrValues", cx.get(&binding, &"acrValues")?);
            cx.define(scope, "dpop", cx.get(&binding, &"dpop")?);
            cx.define(scope, "dpopPairId", cx.get(&binding, &"dpopPairId")?);
            cx.define(scope, "extraParams", cx.get(&binding, &"extraParams")?);
        }
        cx.define(scope, "getTokenOptions", {
            let object = cx.object();
            cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
            cx.put(&object, &"redirectUri", cx.lookup(scope, "redirectUri")?)?;
            cx.put(
                &object,
                &"authorizationCode",
                cx.lookup(scope, "authorizationCode")?,
            )?;
            cx.put(
                &object,
                &"interactionCode",
                cx.lookup(scope, "interactionCode")?,
            )?;
            cx.put(&object, &"codeVerifier", cx.lookup(scope, "codeVerifier")?)?;
            cx.put(&object, &"dpop", V::Bool(false))?;
            object
        });
        cx.define(scope, "responseType", {
            let mut values = vec![];
            values.push(V::string("token"));
            cx.array(values)
        });
        if binary(
            "!==",
            {
                let receiver = cx.lookup(scope, "scopes")?;
                let method = "indexOf";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("openid"));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            V::Number(-V::Number(1f64).number()),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let receiver = cx.lookup(scope, "responseType")?;
                    let method = "push";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::string("id_token"));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
            }
        }
        cx.define(scope, "handleResponseOptions", {
            let object = cx.object();
            cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
            cx.put(&object, &"redirectUri", cx.lookup(scope, "redirectUri")?)?;
            cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
            cx.put(&object, &"responseType", cx.lookup(scope, "responseType")?)?;
            cx.put(
                &object,
                &"ignoreSignature",
                cx.lookup(scope, "ignoreSignature")?,
            )?;
            cx.put(&object, &"acrValues", cx.lookup(scope, "acrValues")?)?;
            cx.put(&object, &"extraParams", cx.lookup(scope, "extraParams")?)?;
            object
        });
        {
            let mut completion = (async {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(
                        scope,
                        "oauthResponse",
                        cx.await_value({
                            let function = cx.lookup(scope, "postToTokenEndpoint")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "sdk")?);
                                argv.push(cx.lookup(scope, "getTokenOptions")?);
                                argv.push(cx.lookup(scope, "urls")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })
                        .await?,
                    );
                    cx.define(
                        scope,
                        "tokenResponse",
                        cx.await_value({
                            let function = cx.lookup(scope, "handleOAuthResponse")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "sdk")?);
                                argv.push(cx.lookup(scope, "handleResponseOptions")?);
                                argv.push(cx.lookup(scope, "oauthResponse")?);
                                argv.push(cx.lookup(scope, "urls")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })
                        .await?,
                    );
                    let _ = {
                        let object = cx.lookup(scope, "tokenResponse")?;
                        let key = "code";
                        let value = cx.lookup(scope, "authorizationCode")?;
                        cx.put(&object, &key, value)?
                    };
                    let _ = {
                        let object = cx.lookup(scope, "tokenResponse")?;
                        let key = "state";
                        let value = cx.lookup(scope, "state")?;
                        cx.put(&object, &key, value)?
                    };
                    return Ok(Completion::Return(cx.lookup(scope, "tokenResponse")?));
                }
                Ok::<Completion, V>(Completion::Normal)
            })
            .await;
            let finalization = (async {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"transactionManager")?;
                        let method = "clear";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
                Ok::<Completion, V>(Completion::Normal)
            })
            .await;
            if !matches!(finalization, Ok(Completion::Normal)) {
                completion = finalization;
            }
            match completion? {
                Completion::Normal => {}
                Completion::Return(value) => {
                    return Ok(value);
                }
                Completion::Break => {
                    return Err(V::string("unexpected break"));
                }
                Completion::Continue => {
                    return Err(V::string("unexpected continue"));
                }
            }
        }
        Ok(V::Undefined)
    })
}

fn f162(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "err", args.get(0).cloned().unwrap_or(V::Undefined));
    if {
        let left = cx.relation(
            "instanceof",
            cx.lookup(scope, "err")?,
            cx.lookup(scope, "WWWAuthError")?,
        )?;
        if left.truthy() {
            V::Bool(true)
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            {
                let binding = cx.lookup(scope, "err")?;
                cx.define(scope, "error", cx.get(&binding, &"error")?);
                cx.define(
                    scope,
                    "errorDescription",
                    cx.get(&binding, &"errorDescription")?,
                );
            }
            return Err(cx.construct(cx.lookup(scope, "OAuthError")?, {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "error")?);
                argv.push(cx.lookup(scope, "errorDescription")?);
                argv
            })?);
        }
    }
    {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "e", cx.lookup(scope, "err")?);
        if {
            let left = cx.relation(
                "instanceof",
                cx.lookup(scope, "err")?,
                cx.lookup(scope, "AuthApiError")?,
            )?;
            if left.truthy() {
                'chain163: {
                    {
                        let object = {
                            let object = cx.lookup(scope, "err")?;
                            if object.nullish() {
                                break 'chain163 V::Undefined;
                            }
                            cx.get(&object, &"meta")?
                        };
                        if object.nullish() {
                            break 'chain163 V::Undefined;
                        }
                        cx.get(&object, &"wwwAuthHeader")?
                    }
                }
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "e", {
                    let receiver = cx.lookup(scope, "WWWAuthError")?;
                    let method = "parseHeader";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.get(
                            &cx.get(&cx.lookup(scope, "err")?, &"meta")?,
                            &"wwwAuthHeader",
                        )?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                })?;
            }
        }
        if cx
            .relation(
                "instanceof",
                cx.lookup(scope, "e")?,
                cx.lookup(scope, "WWWAuthError")?,
            )?
            .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                {
                    let binding = cx.lookup(scope, "e")?;
                    cx.define(scope, "error", cx.get(&binding, &"error")?);
                    cx.define(
                        scope,
                        "errorDescription",
                        cx.get(&binding, &"errorDescription")?,
                    );
                }
                return Err(cx.construct(cx.lookup(scope, "OAuthError")?, {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "error")?);
                    argv.push(cx.lookup(scope, "errorDescription")?);
                    argv
                })?);
            }
        }
    }
    return Err(cx.lookup(scope, "err")?);

    Ok(V::Undefined)
}

fn f164(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "userInfo",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if binary(
        "===",
        cx.get(&cx.lookup(scope, "userInfo")?, &"sub")?,
        cx.get(
            &cx.get(&cx.lookup(scope, "idTokenObject")?, &"claims")?,
            &"sub",
        )?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "userInfo")?);
        }
    }
    return Ok({
        let receiver = cx.lookup(scope, "Promise")?;
        let method = "reject";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "getUserInfo request was rejected due to token mismatch",
                ));
                argv
            })?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f161(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "accessTokenObject",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "idTokenObject",
            args.get(2).cloned().unwrap_or(V::Undefined),
        );
        if V::Bool(!cx.lookup(scope, "accessTokenObject")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "accessTokenObject",
                    cx.get(
                        &cx.await_value({
                            let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"tokenManager")?;
                            let method = "getTokens";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        })
                        .await?,
                        &"accessToken",
                    )?,
                )?;
            }
        }
        if V::Bool(!cx.lookup(scope, "idTokenObject")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "idTokenObject",
                    cx.get(
                        &cx.await_value({
                            let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"tokenManager")?;
                            let method = "getTokens";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        })
                        .await?,
                        &"idToken",
                    )?,
                )?;
            }
        }
        if {
            let left = V::Bool(!cx.lookup(scope, "accessTokenObject")?.truthy());
            if left.truthy() {
                left
            } else {
                V::Bool(
                    !{
                        let function = cx.lookup(scope, "isAccessToken")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "accessTokenObject")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    }
                    .truthy(),
                )
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let receiver = cx.lookup(scope, "Promise")?;
                    let method = "reject";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                            let mut argv = Vec::new();
                            argv.push(V::string("getUserInfo requires an access token object"));
                            argv
                        })?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
            }
        }
        if {
            let left = V::Bool(!cx.lookup(scope, "idTokenObject")?.truthy());
            if left.truthy() {
                left
            } else {
                V::Bool(
                    !{
                        let function = cx.lookup(scope, "isIDToken")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "idTokenObject")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    }
                    .truthy(),
                )
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let receiver = cx.lookup(scope, "Promise")?;
                    let method = "reject";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                            let mut argv = Vec::new();
                            argv.push(V::string("getUserInfo requires an ID token object"));
                            argv
                        })?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
            }
        }
        cx.define(scope, "options", {
            let object = cx.object();
            cx.put(
                &object,
                &"url",
                cx.get(&cx.lookup(scope, "accessTokenObject")?, &"userinfoUrl")?,
            )?;
            cx.put(&object, &"method", V::string("GET"))?;
            cx.put(
                &object,
                &"accessToken",
                cx.get(&cx.lookup(scope, "accessTokenObject")?, &"accessToken")?,
            )?;
            object
        });
        return Ok({
            let receiver = {
                let receiver = {
                    let function = cx.lookup(scope, "httpRequest")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv.push(cx.lookup(scope, "options")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                let method = "then";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f164));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "catch";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.function(scope, f162));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f165(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "tokenParams",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(scope, "res", args.get(2).cloned().unwrap_or(V::Undefined));
        cx.define(scope, "urls", args.get(3).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "pkce",
            binary(
                "!==",
                cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"pkce")?,
                V::Bool(false),
            )?,
        );
        let _ = cx.assign(scope, "tokenParams", {
            let left = cx.lookup(scope, "tokenParams")?;
            if left.truthy() {
                left
            } else {
                {
                    let function = cx.lookup(scope, "getDefaultTokenParams")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            }
        })?;
        let _ = {
            let function = cx.lookup(scope, "validateResponse")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "res")?);
                argv.push(cx.lookup(scope, "tokenParams")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if {
            let left = cx.lookup(scope, "pkce")?;
            if left.truthy() {
                {
                    let left = cx.get(&cx.lookup(scope, "res")?, &"code")?;
                    if left.truthy() {
                        left
                    } else {
                        cx.get(&cx.lookup(scope, "res")?, &"interaction_code")?
                    }
                }
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok({
                    let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"token")?;
                    let method = "exchangeCodeForTokens";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let receiver = cx.lookup(scope, "Object")?;
                            let method = "assign";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push({
                                    let object = cx.object();
                                    object
                                });
                                argv.push(cx.lookup(scope, "tokenParams")?);
                                argv.push({
                                    let object = cx.object();
                                    cx.put(
                                        &object,
                                        &"authorizationCode",
                                        cx.get(&cx.lookup(scope, "res")?, &"code")?,
                                    )?;
                                    cx.put(
                                        &object,
                                        &"interactionCode",
                                        cx.get(&cx.lookup(scope, "res")?, &"interaction_code")?,
                                    )?;
                                    object
                                });
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        });
                        argv.push(cx.lookup(scope, "urls")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
            }
        }
        let _ = cx.assign(scope, "urls", {
            let left = cx.lookup(scope, "urls")?;
            if left.truthy() {
                left
            } else {
                {
                    let function = cx.lookup(scope, "getOAuthUrls")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv.push(cx.lookup(scope, "tokenParams")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            }
        })?;
        cx.define(scope, "responseType", {
            let left = cx.get(&cx.lookup(scope, "tokenParams")?, &"responseType")?;
            if left.truthy() {
                left
            } else {
                {
                    let mut values = vec![];
                    cx.array(values)
                }
            }
        });
        if {
            let left = V::Bool(
                !{
                    let receiver = cx.lookup(scope, "Array")?;
                    let method = "isArray";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "responseType")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy(),
            );
            if left.truthy() {
                binary("!==", cx.lookup(scope, "responseType")?, V::string("none"))?
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "responseType", {
                    let mut values = vec![];
                    values.push(cx.lookup(scope, "responseType")?);
                    cx.array(values)
                })?;
            }
        }
        cx.define(scope, "scopes", V::Undefined);
        if cx.get(&cx.lookup(scope, "res")?, &"scope")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "scopes", {
                    let receiver = cx.get(&cx.lookup(scope, "res")?, &"scope")?;
                    let method = "split";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::string(" "));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                })?;
            }
        } else {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(scope, "scopes", {
                    let function = cx.lookup(scope, "clone")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.get(&cx.lookup(scope, "tokenParams")?, &"scopes")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
            }
        }
        cx.define(scope, "clientId", {
            let left = cx.get(&cx.lookup(scope, "tokenParams")?, &"clientId")?;
            if left.truthy() {
                left
            } else {
                cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"clientId")?
            }
        });
        if cx.get(&cx.lookup(scope, "tokenParams")?, &"dpop")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                {
                    let binding = {
                        let left = 'chain166: {
                            {
                                let object = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
                                if object.nullish() {
                                    break 'chain166 V::Undefined;
                                }
                                cx.get(&object, &"dpopOptions")?
                            }
                        };
                        if left.nullish() {
                            {
                                let object = cx.object();
                                cx.put(&object, &"allowBearerTokens", V::Bool(false))?;
                                object
                            }
                        } else {
                            left
                        }
                    };
                    cx.define(
                        scope,
                        "allowBearerTokens",
                        cx.get(&binding, &"allowBearerTokens")?,
                    );
                }
                if {
                    let left = V::Bool(!cx.lookup(scope, "allowBearerTokens")?.truthy());
                    if left.truthy() {
                        binary(
                            "!==",
                            cx.get(&cx.lookup(scope, "res")?, &"token_type")?,
                            V::string("DPoP"),
                        )?
                    } else {
                        left
                    }
                }
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push(V::string("Unable to parse OAuth flow response: DPoP was configured but \"token_type\" was not DPoP"));argv})?);
                    }
                }
            }
        }
        cx.define(scope, "tokenDict", {
            let object = cx.object();
            object
        });
        cx.define(
            scope,
            "expiresIn",
            cx.get(&cx.lookup(scope, "res")?, &"expires_in")?,
        );
        cx.define(
            scope,
            "tokenType",
            cx.get(&cx.lookup(scope, "res")?, &"token_type")?,
        );
        cx.define(
            scope,
            "accessToken",
            cx.get(&cx.lookup(scope, "res")?, &"access_token")?,
        );
        cx.define(
            scope,
            "idToken",
            cx.get(&cx.lookup(scope, "res")?, &"id_token")?,
        );
        cx.define(
            scope,
            "refreshToken",
            cx.get(&cx.lookup(scope, "res")?, &"refresh_token")?,
        );
        cx.define(scope, "now", {
            let receiver = cx.lookup(scope, "Math")?;
            let method = "floor";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(binary(
                    "/",
                    {
                        let receiver = cx.lookup(scope, "Date")?;
                        let method = "now";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    },
                    V::Number(1000f64),
                )?);
                argv
            };
            cx.call(function, receiver, argv)?
        });
        if cx.lookup(scope, "accessToken")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let object = cx.lookup(scope, "tokenDict")?;
                    let key = "accessToken";
                    let value = {
                        let object = cx.object();
                        cx.put(&object, &"accessToken", cx.lookup(scope, "accessToken")?)?;
                        cx.put(
                            &object,
                            &"expiresAt",
                            binary(
                                "+",
                                {
                                    let function = cx.lookup(scope, "Number")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "expiresIn")?);
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                },
                                cx.lookup(scope, "now")?,
                            )?,
                        )?;
                        cx.put(&object, &"tokenType", cx.lookup(scope, "tokenType")?)?;
                        cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
                        cx.put(
                            &object,
                            &"authorizeUrl",
                            cx.get(&cx.lookup(scope, "urls")?, &"authorizeUrl")?,
                        )?;
                        cx.put(
                            &object,
                            &"userinfoUrl",
                            cx.get(&cx.lookup(scope, "urls")?, &"userinfoUrl")?,
                        )?;
                        object
                    };
                    cx.put(&object, &key, value)?
                };
                if cx
                    .get(
                        &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                        &"decodeAccessTokens",
                    )?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        cx.define(scope, "accessJwt", {
                            let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"token")?;
                            let method = "decode";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "accessToken")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        });
                        let _ = {
                            let object = cx.get(&cx.lookup(scope, "tokenDict")?, &"accessToken")?;
                            let key = "claims";
                            let value = cx.get(&cx.lookup(scope, "accessJwt")?, &"payload")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
                if cx
                    .get(&cx.lookup(scope, "tokenParams")?, &"dpopPairId")?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object = cx.get(&cx.lookup(scope, "tokenDict")?, &"accessToken")?;
                            let key = "dpopPairId";
                            let value = cx.get(&cx.lookup(scope, "tokenParams")?, &"dpopPairId")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
                if cx
                    .get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object = cx.get(&cx.lookup(scope, "tokenDict")?, &"accessToken")?;
                            let key = "extraParams";
                            let value =
                                cx.get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
            }
        }
        if cx.lookup(scope, "refreshToken")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let object = cx.lookup(scope, "tokenDict")?;
                    let key = "refreshToken";
                    let value = {
                        let object = cx.object();
                        cx.put(&object, &"refreshToken", cx.lookup(scope, "refreshToken")?)?;
                        cx.put(
                            &object,
                            &"expiresAt",
                            binary(
                                "+",
                                {
                                    let function = cx.lookup(scope, "Number")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "expiresIn")?);
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                },
                                cx.lookup(scope, "now")?,
                            )?,
                        )?;
                        cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
                        cx.put(
                            &object,
                            &"tokenUrl",
                            cx.get(&cx.lookup(scope, "urls")?, &"tokenUrl")?,
                        )?;
                        cx.put(
                            &object,
                            &"authorizeUrl",
                            cx.get(&cx.lookup(scope, "urls")?, &"authorizeUrl")?,
                        )?;
                        cx.put(
                            &object,
                            &"issuer",
                            cx.get(&cx.lookup(scope, "urls")?, &"issuer")?,
                        )?;
                        object
                    };
                    cx.put(&object, &key, value)?
                };
                if cx
                    .get(&cx.lookup(scope, "tokenParams")?, &"dpopPairId")?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object =
                                cx.get(&cx.lookup(scope, "tokenDict")?, &"refreshToken")?;
                            let key = "dpopPairId";
                            let value = cx.get(&cx.lookup(scope, "tokenParams")?, &"dpopPairId")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
                if cx
                    .get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object =
                                cx.get(&cx.lookup(scope, "tokenDict")?, &"refreshToken")?;
                            let key = "extraParams";
                            let value =
                                cx.get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
            }
        }
        if cx.lookup(scope, "idToken")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "idJwt", {
                    let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"token")?;
                    let method = "decode";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "idToken")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                });
                cx.define(scope, "idTokenObj", {
                    let object = cx.object();
                    cx.put(&object, &"idToken", cx.lookup(scope, "idToken")?)?;
                    cx.put(
                        &object,
                        &"claims",
                        cx.get(&cx.lookup(scope, "idJwt")?, &"payload")?,
                    )?;
                    cx.put(
                        &object,
                        &"expiresAt",
                        binary(
                            "+",
                            binary(
                                "-",
                                cx.get(&cx.get(&cx.lookup(scope, "idJwt")?, &"payload")?, &"exp")?,
                                cx.get(&cx.get(&cx.lookup(scope, "idJwt")?, &"payload")?, &"iat")?,
                            )?,
                            cx.lookup(scope, "now")?,
                        )?,
                    )?;
                    cx.put(&object, &"scopes", cx.lookup(scope, "scopes")?)?;
                    cx.put(
                        &object,
                        &"authorizeUrl",
                        cx.get(&cx.lookup(scope, "urls")?, &"authorizeUrl")?,
                    )?;
                    cx.put(
                        &object,
                        &"issuer",
                        cx.get(&cx.lookup(scope, "urls")?, &"issuer")?,
                    )?;
                    cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
                    object
                });
                if cx
                    .get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?
                    .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object = cx.lookup(scope, "idTokenObj")?;
                            let key = "extraParams";
                            let value =
                                cx.get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
                cx.define(scope, "validationParams", {
                    let object = cx.object();
                    cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
                    cx.put(
                        &object,
                        &"issuer",
                        cx.get(&cx.lookup(scope, "urls")?, &"issuer")?,
                    )?;
                    cx.put(
                        &object,
                        &"nonce",
                        cx.get(&cx.lookup(scope, "tokenParams")?, &"nonce")?,
                    )?;
                    cx.put(&object, &"accessToken", cx.lookup(scope, "accessToken")?)?;
                    cx.put(
                        &object,
                        &"acrValues",
                        cx.get(&cx.lookup(scope, "tokenParams")?, &"acrValues")?,
                    )?;
                    object
                });
                if binary(
                    "!==",
                    cx.get(&cx.lookup(scope, "tokenParams")?, &"ignoreSignature")?,
                    cx.lookup(scope, "undefined")?,
                )?
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = {
                            let object = cx.lookup(scope, "validationParams")?;
                            let key = "ignoreSignature";
                            let value =
                                cx.get(&cx.lookup(scope, "tokenParams")?, &"ignoreSignature")?;
                            cx.put(&object, &key, value)?
                        };
                    }
                }
                let _ = cx
                    .await_value({
                        let function = cx.lookup(scope, "verifyToken")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "sdk")?);
                            argv.push(cx.lookup(scope, "idTokenObj")?);
                            argv.push(cx.lookup(scope, "validationParams")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    })
                    .await?;
                let _ = {
                    let object = cx.lookup(scope, "tokenDict")?;
                    let key = "idToken";
                    let value = cx.lookup(scope, "idTokenObj")?;
                    cx.put(&object, &key, value)?
                };
            }
        }
        if {
            let left = binary(
                "!==",
                {
                    let receiver = cx.lookup(scope, "responseType")?;
                    let method = "indexOf";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::string("token"));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::Number(-V::Number(1f64).number()),
            )?;
            if left.truthy() {
                V::Bool(
                    !cx.get(&cx.lookup(scope, "tokenDict")?, &"accessToken")?
                        .truthy(),
                )
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push(V::string("Unable to parse OAuth flow response: response type \"token\" was requested but \"access_token\" was not returned."));argv})?);
            }
        }
        if {
            let left = binary(
                "!==",
                {
                    let receiver = cx.lookup(scope, "responseType")?;
                    let method = "indexOf";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::string("id_token"));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::Number(-V::Number(1f64).number()),
            )?;
            if left.truthy() {
                V::Bool(
                    !cx.get(&cx.lookup(scope, "tokenDict")?, &"idToken")?
                        .truthy(),
                )
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push(V::string("Unable to parse OAuth flow response: response type \"id_token\" was requested but \"id_token\" was not returned."));argv})?);
            }
        }
        return Ok({
            let object = cx.object();
            cx.put(&object, &"tokens", cx.lookup(scope, "tokenDict")?)?;
            cx.put(
                &object,
                &"state",
                cx.get(&cx.lookup(scope, "res")?, &"state")?,
            )?;
            cx.put(
                &object,
                &"code",
                cx.get(&cx.lookup(scope, "res")?, &"code")?,
            )?;
            cx.put(&object, &"responseType", cx.lookup(scope, "responseType")?)?;
            object
        });
        Ok(V::Undefined)
    })
}

fn f167(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "res", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "oauthParams",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = cx.get(&cx.lookup(scope, "res")?, &V::string("error").text())?;
        if left.truthy() {
            cx.get(
                &cx.lookup(scope, "res")?,
                &V::string("error_description").text(),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "OAuthError")?, {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "res")?, &V::string("error").text())?);
                argv.push(cx.get(
                    &cx.lookup(scope, "res")?,
                    &V::string("error_description").text(),
                )?);
                argv
            })?);
        }
    }
    if binary(
        "!==",
        cx.get(&cx.lookup(scope, "res")?, &"state")?,
        cx.get(&cx.lookup(scope, "oauthParams")?, &"state")?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "OAuth flow response state doesn't match request state",
                ));
                argv
            })?);
        }
    }

    Ok(V::Undefined)
}

fn f168(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "tokenParams",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        cx.define(
            scope,
            "refreshTokenObject",
            args.get(2).cloned().unwrap_or(V::Undefined),
        );
        {
            let binding = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
            cx.define(scope, "clientId", cx.get(&binding, &"clientId")?);
            cx.define(scope, "dpop", cx.get(&binding, &"dpop")?);
        }
        if V::Bool(!cx.lookup(scope, "clientId")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string(
                        "A clientId must be specified in the OktaAuth constructor to renew tokens",
                    ));
                    argv
                })?);
            }
        }
        {
            let mut completion = (async {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "renewTokenParams", {
                        let receiver = cx.lookup(scope, "Object")?;
                        let method = "assign";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let object = cx.object();
                                object
                            });
                            argv.push(cx.lookup(scope, "tokenParams")?);
                            argv.push({
                                let object = cx.object();
                                cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
                                object
                            });
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                    if cx
                        .get(&cx.lookup(scope, "refreshTokenObject")?, &"extraParams")?
                        .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "renewTokenParams")?;
                                let key = "extraParams";
                                let value = cx.get(
                                    &cx.lookup(scope, "refreshTokenObject")?,
                                    &"extraParams",
                                )?;
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                    cx.define(scope, "endpointParams", {
                        let object = cx.object();
                        cx.spread(&object, &cx.lookup(scope, "renewTokenParams")?)?;
                        object
                    });
                    cx.define(
                        scope,
                        "tokenResponse",
                        cx.await_value({
                            let function = cx.lookup(scope, "postRefreshToken")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "sdk")?);
                                argv.push(cx.lookup(scope, "endpointParams")?);
                                argv.push(cx.lookup(scope, "refreshTokenObject")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })
                        .await?,
                    );
                    cx.define(scope, "urls", {
                        let function = cx.lookup(scope, "getOAuthUrls")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "sdk")?);
                            argv.push(cx.lookup(scope, "tokenParams")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    {
                        let binding = cx
                            .await_value({
                                let function = cx.lookup(scope, "handleOAuthResponse")?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "sdk")?);
                                    argv.push(cx.lookup(scope, "renewTokenParams")?);
                                    argv.push(cx.lookup(scope, "tokenResponse")?);
                                    argv.push(cx.lookup(scope, "urls")?);
                                    argv
                                };
                                cx.call(function, V::Undefined, argv)?
                            })
                            .await?;
                        cx.define(scope, "tokens", cx.get(&binding, &"tokens")?);
                    }
                    {
                        let binding = cx.lookup(scope, "tokens")?;
                        cx.define(scope, "refreshToken", cx.get(&binding, &"refreshToken")?);
                    }
                    if {
                        let left = cx.lookup(scope, "refreshToken")?;
                        if left.truthy() {
                            V::Bool(
                                !{
                                    let function = cx.lookup(scope, "isSameRefreshToken")?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "refreshToken")?);
                                        argv.push(cx.lookup(scope, "refreshTokenObject")?);
                                        argv
                                    };
                                    cx.call(function, V::Undefined, argv)?
                                }
                                .truthy(),
                            )
                        } else {
                            left
                        }
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let receiver =
                                    cx.get(&cx.lookup(scope, "sdk")?, &"tokenManager")?;
                                let method = "updateRefreshToken";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "refreshToken")?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                        }
                    }
                    return Ok(Completion::Return(cx.lookup(scope, "tokens")?));
                }
                Ok::<Completion, V>(Completion::Normal)
            })
            .await;
            if let Err(error) = completion {
                completion = (async {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "err", error);
                    {
                        let scope = cx.scope(Some(scope));
                        if {
                            let function = cx.lookup(scope, "isRefreshTokenInvalidError")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "err")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = {
                                    let receiver =
                                        cx.get(&cx.lookup(scope, "sdk")?, &"tokenManager")?;
                                    let method = "removeRefreshToken";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                };
                            }
                        }
                        return Err(cx.lookup(scope, "err")?);
                    }
                    Ok::<Completion, V>(Completion::Normal)
                })
                .await;
            }
            match completion? {
                Completion::Normal => {}
                Completion::Return(value) => {
                    return Ok(value);
                }
                Completion::Break => {
                    return Err(V::string("unexpected break"));
                }
                Completion::Continue => {
                    return Err(V::string("unexpected continue"));
                }
            }
        }
        Ok(V::Undefined)
    })
}

fn f169(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(scope, "token", args.get(1).cloned().unwrap_or(V::Undefined));
        cx.define(scope, "accessToken", V::string(""));
        cx.define(scope, "refreshToken", V::string(""));
        if cx.lookup(scope, "token")?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "accessToken",
                    cx.get(&cx.lookup(scope, "token")?, &"accessToken")?,
                )?;
                let _ = cx.assign(
                    scope,
                    "refreshToken",
                    cx.get(&cx.lookup(scope, "token")?, &"refreshToken")?,
                )?;
            }
        }
        if {
            let left = V::Bool(!cx.lookup(scope, "accessToken")?.truthy());
            if left.truthy() {
                V::Bool(!cx.lookup(scope, "refreshToken")?.truthy())
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string(
                        "A valid access or refresh token object is required",
                    ));
                    argv
                })?);
            }
        }
        cx.define(
            scope,
            "clientId",
            cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"clientId")?,
        );
        cx.define(
            scope,
            "clientSecret",
            cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"clientSecret",
            )?,
        );
        if V::Bool(!cx.lookup(scope, "clientId")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope,"AuthSdkError")?, {let mut argv=Vec::new();argv.push(V::string("A clientId must be specified in the OktaAuth constructor to revoke a token"));argv})?);
            }
        }
        cx.define(
            scope,
            "revokeUrl",
            cx.get(
                &{
                    let function = cx.lookup(scope, "getOAuthUrls")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                },
                &"revokeUrl",
            )?,
        );
        cx.define(scope, "args", {
            let receiver = {
                let function = cx.lookup(scope, "toQueryString")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push({
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"token_type_hint",
                            if cx.lookup(scope, "refreshToken")?.truthy() {
                                V::string("refresh_token")
                            } else {
                                V::string("access_token")
                            },
                        )?;
                        cx.put(&object, &"token", {
                            let left = cx.lookup(scope, "refreshToken")?;
                            if left.truthy() {
                                left
                            } else {
                                cx.lookup(scope, "accessToken")?
                            }
                        })?;
                        object
                    });
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            let method = "slice";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::Number(1f64));
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.define(
            scope,
            "creds",
            if cx.lookup(scope, "clientSecret")?.truthy() {
                {
                    let function = cx.lookup(scope, "btoa")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let mut text = String::new();
                            text.push_str("");
                            text.push_str(&cx.lookup(scope, "clientId")?.text());
                            text.push_str(":");
                            text.push_str(&cx.lookup(scope, "clientSecret")?.text());
                            text.push_str("");
                            V::string(text)
                        });
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            } else {
                {
                    let function = cx.lookup(scope, "btoa")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "clientId")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                }
            },
        );
        return Ok({
            let function = cx.lookup(scope, "post")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "revokeUrl")?);
                argv.push(cx.lookup(scope, "args")?);
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"headers", {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"Content-Type",
                            V::string("application/x-www-form-urlencoded"),
                        )?;
                        cx.put(
                            &object,
                            &"Authorization",
                            binary("+", V::string("Basic "), cx.lookup(scope, "creds")?)?,
                        )?;
                        object
                    })?;
                    object
                });
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f170(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = cx.lookup(scope, "obj")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "obj")?, &"accessToken")?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f171(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = cx.lookup(scope, "obj")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "obj")?, &"idToken")?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f173(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "value", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "!==",
        V::string(cx.lookup(scope, "value")?.typeof_()),
        V::string("string"),
    )?);

    Ok(V::Undefined)
}

fn f172(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(
        !{
            let function = cx.lookup(scope, "isObjectWithProperties")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "obj")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    cx.define(
        scope,
        "isAllStringValues",
        binary(
            "===",
            {
                let receiver = {
                    let receiver = cx.lookup(scope, "Object")?;
                    let method = "values";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "obj")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
                let method = "find";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f173));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            cx.lookup(scope, "undefined")?,
        )?,
    );
    return Ok(cx.lookup(scope, "isAllStringValues")?);

    Ok(V::Undefined)
}

fn f174(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(
        !{
            let function = cx.lookup(scope, "isObjectWithProperties")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "obj")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        }
        .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    return Ok({
        let left = V::Bool(
            !V::Bool(!cx.get(&cx.lookup(scope, "obj")?, &"redirectUri")?.truthy()).truthy(),
        );
        if left.truthy() {
            left
        } else {
            V::Bool(
                !V::Bool(!cx.get(&cx.lookup(scope, "obj")?, &"responseType")?.truthy()).truthy(),
            )
        }
    });

    Ok(V::Undefined)
}

fn f175(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    if {
        let left = {
            let left = V::Bool(!cx.lookup(scope, "obj")?.truthy());
            if left.truthy() {
                left
            } else {
                binary(
                    "!==",
                    V::string(cx.lookup(scope, "obj")?.typeof_()),
                    V::string("object"),
                )?
            }
        };
        if left.truthy() {
            left
        } else {
            binary(
                "===",
                cx.get(
                    &{
                        let receiver = cx.lookup(scope, "Object")?;
                        let method = "values";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "obj")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    },
                    &"length",
                )?,
                V::Number(0f64),
            )?
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    return Ok(V::Bool(true));

    Ok(V::Undefined)
}

fn f176(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    if {
        let left = {
            let function = cx.lookup(scope, "isOAuthTransactionMeta")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "obj")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            {
                let function = cx.lookup(scope, "isCustomAuthTransactionMeta")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "obj")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(true));
        }
    }
    return Ok(V::Bool(false));

    Ok(V::Undefined)
}

fn f177(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let binding = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
        cx.define(scope, "pkce", cx.get(&binding, &"pkce")?);
        cx.define(scope, "clientId", cx.get(&binding, &"clientId")?);
        cx.define(scope, "redirectUri", cx.get(&binding, &"redirectUri")?);
        cx.define(scope, "responseType", cx.get(&binding, &"responseType")?);
        cx.define(scope, "responseMode", cx.get(&binding, &"responseMode")?);
        cx.define(scope, "scopes", cx.get(&binding, &"scopes")?);
        cx.define(scope, "acrValues", cx.get(&binding, &"acrValues")?);
        cx.define(scope, "maxAge", cx.get(&binding, &"maxAge")?);
        cx.define(scope, "state", cx.get(&binding, &"state")?);
        cx.define(
            scope,
            "ignoreSignature",
            cx.get(&binding, &"ignoreSignature")?,
        );
        cx.define(scope, "dpop", cx.get(&binding, &"dpop")?);
    }
    cx.define(scope, "defaultRedirectUri", cx.lookup(scope, "undefined")?);
    return Ok({
        let function = cx.lookup(scope, "removeNils")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push({
                let object = cx.object();
                cx.put(&object, &"pkce", cx.lookup(scope, "pkce")?)?;
                cx.put(&object, &"clientId", cx.lookup(scope, "clientId")?)?;
                cx.put(&object, &"redirectUri", {
                    let left = cx.lookup(scope, "redirectUri")?;
                    if left.truthy() {
                        left
                    } else {
                        cx.lookup(scope, "defaultRedirectUri")?
                    }
                })?;
                cx.put(&object, &"responseType", {
                    let left = cx.lookup(scope, "responseType")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let mut values = vec![];
                            values.push(V::string("token"));
                            values.push(V::string("id_token"));
                            cx.array(values)
                        }
                    }
                })?;
                cx.put(&object, &"responseMode", cx.lookup(scope, "responseMode")?)?;
                cx.put(&object, &"state", {
                    let left = cx.lookup(scope, "state")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let function = cx.lookup(scope, "generateState")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                    }
                })?;
                cx.put(&object, &"nonce", {
                    let function = cx.lookup(scope, "generateNonce")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                cx.put(&object, &"scopes", {
                    let left = cx.lookup(scope, "scopes")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let mut values = vec![];
                            values.push(V::string("openid"));
                            values.push(V::string("email"));
                            cx.array(values)
                        }
                    }
                })?;
                cx.put(&object, &"acrValues", cx.lookup(scope, "acrValues")?)?;
                cx.put(&object, &"maxAge", cx.lookup(scope, "maxAge")?)?;
                cx.put(
                    &object,
                    &"ignoreSignature",
                    cx.lookup(scope, "ignoreSignature")?,
                )?;
                cx.put(&object, &"dpop", V::Bool(false))?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f178(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "error", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = {
            let left = {
                let function = cx.lookup(scope, "isOAuthError")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "error")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            };
            if left.truthy() {
                binary(
                    "===",
                    cx.get(&cx.lookup(scope, "error")?, &"errorCode")?,
                    V::string("invalid_grant"),
                )?
            } else {
                left
            }
        };
        if left.truthy() {
            binary(
                "===",
                cx.get(&cx.lookup(scope, "error")?, &"errorSummary")?,
                V::string("The refresh token is invalid or expired."),
            )?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f179(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok({
        let function = cx.lookup(scope, "genRandomString")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(64f64));
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f180(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok({
        let function = cx.lookup(scope, "genRandomString")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(64f64));
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f181(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "issuer", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"issuer")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"issuer")?
        }
    });
    return Ok(cx.lookup(scope, "issuer")?);

    Ok(V::Undefined)
}

fn f182(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "issuer", {
        let function = cx.lookup(scope, "getIssuer")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(
        scope,
        "baseUrl",
        if binary(
            ">",
            {
                let receiver = cx.lookup(scope, "issuer")?;
                let method = "indexOf";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("/oauth2"));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            V::Number(0f64),
        )?
        .truthy()
        {
            cx.lookup(scope, "issuer")?
        } else {
            binary("+", cx.lookup(scope, "issuer")?, V::string("/oauth2"))?
        },
    );
    return Ok(cx.lookup(scope, "baseUrl")?);

    Ok(V::Undefined)
}

fn f183(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    cx.define(scope, "issuer", {
        let function = cx.lookup(scope, "getIssuer")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(
        scope,
        "domain",
        cx.get(
            &{
                let receiver = cx.lookup(scope, "issuer")?;
                let method = "split";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("/oauth2"));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            &V::Number(0f64).text(),
        )?,
    );
    return Ok(cx.lookup(scope, "domain")?);

    Ok(V::Undefined)
}

fn f184(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "options",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    if binary(
        ">",
        cx.get(&cx.lookup(scope, "arguments")?, &"length")?,
        V::Number(2f64),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "As of version 3.0, \"getOAuthUrls\" takes only a single set of options",
                ));
                argv
            })?);
        }
    }
    let _ = cx.assign(scope, "options", {
        let left = cx.lookup(scope, "options")?;
        if left.truthy() {
            left
        } else {
            {
                let object = cx.object();
                object
            }
        }
    })?;
    cx.define(scope, "authorizeUrl", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"authorizeUrl")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"authorizeUrl",
            )?
        }
    });
    cx.define(scope, "issuer", {
        let function = cx.lookup(scope, "getIssuer")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "userinfoUrl", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"userinfoUrl")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"userinfoUrl",
            )?
        }
    });
    cx.define(scope, "tokenUrl", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"tokenUrl")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"tokenUrl")?
        }
    });
    cx.define(scope, "logoutUrl", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"logoutUrl")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"logoutUrl",
            )?
        }
    });
    cx.define(scope, "revokeUrl", {
        let left = {
            let function = cx.lookup(scope, "removeTrailingSlash")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "options")?, &"revokeUrl")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if left.truthy() {
            left
        } else {
            cx.get(
                &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                &"revokeUrl",
            )?
        }
    });
    cx.define(scope, "baseUrl", {
        let function = cx.lookup(scope, "getOAuthBaseUrl")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "options")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    let _ = cx.assign(scope, "authorizeUrl", {
        let left = cx.lookup(scope, "authorizeUrl")?;
        if left.truthy() {
            left
        } else {
            binary(
                "+",
                cx.lookup(scope, "baseUrl")?,
                V::string("/v1/authorize"),
            )?
        }
    })?;
    let _ = cx.assign(scope, "userinfoUrl", {
        let left = cx.lookup(scope, "userinfoUrl")?;
        if left.truthy() {
            left
        } else {
            binary("+", cx.lookup(scope, "baseUrl")?, V::string("/v1/userinfo"))?
        }
    })?;
    let _ = cx.assign(scope, "tokenUrl", {
        let left = cx.lookup(scope, "tokenUrl")?;
        if left.truthy() {
            left
        } else {
            binary("+", cx.lookup(scope, "baseUrl")?, V::string("/v1/token"))?
        }
    })?;
    let _ = cx.assign(scope, "revokeUrl", {
        let left = cx.lookup(scope, "revokeUrl")?;
        if left.truthy() {
            left
        } else {
            binary("+", cx.lookup(scope, "baseUrl")?, V::string("/v1/revoke"))?
        }
    })?;
    let _ = cx.assign(scope, "logoutUrl", {
        let left = cx.lookup(scope, "logoutUrl")?;
        if left.truthy() {
            left
        } else {
            binary("+", cx.lookup(scope, "baseUrl")?, V::string("/v1/logout"))?
        }
    })?;
    return Ok({
        let object = cx.object();
        cx.put(&object, &"issuer", cx.lookup(scope, "issuer")?)?;
        cx.put(&object, &"authorizeUrl", cx.lookup(scope, "authorizeUrl")?)?;
        cx.put(&object, &"userinfoUrl", cx.lookup(scope, "userinfoUrl")?)?;
        cx.put(&object, &"tokenUrl", cx.lookup(scope, "tokenUrl")?)?;
        cx.put(&object, &"revokeUrl", cx.lookup(scope, "revokeUrl")?)?;
        cx.put(&object, &"logoutUrl", cx.lookup(scope, "logoutUrl")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f185(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "tokenParams",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "issuer",
        cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"issuer")?,
    );
    cx.define(scope, "urls", {
        let function = cx.lookup(scope, "getOAuthUrls")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "sdk")?);
            argv.push(cx.lookup(scope, "tokenParams")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "oauthMeta", {
        let object = cx.object();
        cx.put(&object, &"issuer", cx.lookup(scope, "issuer")?)?;
        cx.put(&object, &"urls", cx.lookup(scope, "urls")?)?;
        cx.put(
            &object,
            &"clientId",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"clientId")?,
        )?;
        cx.put(
            &object,
            &"redirectUri",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"redirectUri")?,
        )?;
        cx.put(
            &object,
            &"responseType",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"responseType")?,
        )?;
        cx.put(
            &object,
            &"responseMode",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"responseMode")?,
        )?;
        cx.put(
            &object,
            &"scopes",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"scopes")?,
        )?;
        cx.put(
            &object,
            &"state",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"state")?,
        )?;
        cx.put(
            &object,
            &"nonce",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"nonce")?,
        )?;
        cx.put(
            &object,
            &"ignoreSignature",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"ignoreSignature")?,
        )?;
        cx.put(
            &object,
            &"acrValues",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"acrValues")?,
        )?;
        cx.put(
            &object,
            &"extraParams",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"extraParams")?,
        )?;
        object
    });
    if binary(
        "===",
        cx.get(&cx.lookup(scope, "tokenParams")?, &"pkce")?,
        V::Bool(false),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "oauthMeta")?);
        }
    }
    cx.define(scope, "pkceMeta", {
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "oauthMeta")?)?;
        cx.put(
            &object,
            &"codeVerifier",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"codeVerifier")?,
        )?;
        cx.put(
            &object,
            &"codeChallengeMethod",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"codeChallengeMethod")?,
        )?;
        cx.put(
            &object,
            &"codeChallenge",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"codeChallenge")?,
        )?;
        object
    });
    return Ok(cx.lookup(scope, "pkceMeta")?);

    Ok(V::Undefined)
}

fn f187(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "arrayBuffer",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "hash", {
        let receiver = cx.get(&cx.lookup(scope, "String")?, &"fromCharCode")?;
        let method = "apply";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Null);
            argv.push(cx.construct(cx.lookup(scope, "Uint8Array")?, {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "arrayBuffer")?);
                argv
            })?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "b64u", {
        let function = cx.lookup(scope, "stringToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "hash")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok(cx.lookup(scope, "b64u")?);

    Ok(V::Undefined)
}

fn f186(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "buffer", {
        let receiver = cx.construct(cx.lookup(scope, "TextEncoder")?, {
            let mut argv = Vec::new();
            argv
        })?;
        let method = "encode";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "str")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok({
        let receiver = {
            let receiver = cx.get(&cx.lookup(scope, "crypto")?, &"subtle")?;
            let method = "digest";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::string("SHA-256"));
                argv.push(cx.lookup(scope, "buffer")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "then";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.function(scope, f187));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f188(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "dec", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = binary("+", V::string("0"), {
            let receiver = cx.lookup(scope, "dec")?;
            let method = "toString";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(V::Number(16f64));
                argv
            };
            cx.call(function, receiver, argv)?
        })?;
        let method = "substr";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(-V::Number(2f64).number()));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f189(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "prefix",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "verifier", {
        let left = cx.lookup(scope, "prefix")?;
        if left.truthy() {
            left
        } else {
            V::string("")
        }
    });
    if binary(
        "<",
        cx.get(&cx.lookup(scope, "verifier")?, &"length")?,
        cx.lookup(scope, "MIN_VERIFIER_LENGTH")?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(
                scope,
                "verifier",
                binary("+", cx.lookup(scope, "verifier")?, {
                    let function = cx.lookup(scope, "getRandomString")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(binary(
                            "-",
                            cx.lookup(scope, "MIN_VERIFIER_LENGTH")?,
                            cx.get(&cx.lookup(scope, "verifier")?, &"length")?,
                        )?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?,
            )?;
        }
    }
    return Ok({
        let receiver = {
            let function = cx.lookup(scope, "encodeURIComponent")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "verifier")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(0f64));
            argv.push(cx.lookup(scope, "MAX_VERIFIER_LENGTH")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f190(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "length",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "a",
        cx.construct(cx.lookup(scope, "Uint8Array")?, {
            let mut argv = Vec::new();
            argv.push({
                let receiver = cx.lookup(scope, "Math")?;
                let method = "ceil";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(binary("/", cx.lookup(scope, "length")?, V::Number(2f64))?);
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            argv
        })?,
    );
    let _ = cx.assign(scope, "a", {
        let receiver = cx.lookup(scope, "crypto")?;
        let method = "getRandomValues";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "a")?);
            argv
        };
        cx.call(function, receiver, argv)?
    })?;
    cx.define(scope, "str", {
        let receiver = {
            let receiver = cx.lookup(scope, "Array")?;
            let method = "from";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "a")?);
                argv.push(cx.lookup(scope, "dec2hex")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "join";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::string(""));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok({
        let receiver = cx.lookup(scope, "str")?;
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(0f64));
            argv.push(cx.lookup(scope, "length")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f191(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(
        !{
            let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"features")?;
            let method = "isPKCESupported";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        }
        .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope,"errorMessage",V::string("PKCE requires a modern browser with encryption support running in a secure context."));
            if V::Bool(
                !{
                    let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"features")?;
                    let method = "isHTTPS";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy(),
            )
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _=cx.assign(scope,"errorMessage", binary("+", cx.lookup(scope,"errorMessage")?, V::string("\nThe current page is not being served with HTTPS protocol. PKCE requires secure HTTPS protocol."))?)?;
                }
            }
            if V::Bool(
                !{
                    let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"features")?;
                    let method = "hasTextEncoder";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy(),
            )
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _=cx.assign(scope,"errorMessage", binary("+", cx.lookup(scope,"errorMessage")?, V::string("\n\"TextEncoder\" is not defined. To use PKCE, you may need to include a polyfill/shim for this browser."))?)?;
                }
            }
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "errorMessage")?);
                argv
            })?);
        }
    }

    Ok(V::Undefined)
}

fn f192(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "tokenParams",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        {
            let binding = cx.lookup(scope, "tokenParams")?;
            cx.define(scope, "codeVerifier", cx.get(&binding, &"codeVerifier")?);
            cx.define(scope, "codeChallenge", cx.get(&binding, &"codeChallenge")?);
            cx.define(
                scope,
                "codeChallengeMethod",
                cx.get(&binding, &"codeChallengeMethod")?,
            );
        }
        let _ = cx.assign(scope, "codeChallenge", {
            let left = cx.lookup(scope, "codeChallenge")?;
            if left.truthy() {
                left
            } else {
                cx.get(
                    &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                    &"codeChallenge",
                )?
            }
        })?;
        if V::Bool(!cx.lookup(scope, "codeChallenge")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                let _ = {
                    let function = cx.lookup(scope, "assertPKCESupport")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                let _ = cx.assign(scope, "codeVerifier", {
                    let left = cx.lookup(scope, "codeVerifier")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let receiver = cx.lookup(scope, "PKCE")?;
                            let method = "generateVerifier";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        }
                    }
                })?;
                let _ = cx.assign(
                    scope,
                    "codeChallenge",
                    cx.await_value({
                        let receiver = cx.lookup(scope, "PKCE")?;
                        let method = "computeChallenge";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "codeVerifier")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    })
                    .await?,
                )?;
            }
        }
        let _ = cx.assign(
            scope,
            "codeChallengeMethod",
            cx.await_value({
                let function = cx.lookup(scope, "validateCodeChallengeMethod")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "sdk")?);
                    argv.push(cx.lookup(scope, "codeChallengeMethod")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        )?;
        let _ = cx.assign(scope, "tokenParams", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "tokenParams")?)?;
            cx.put(&object, &"responseType", V::string("code"))?;
            cx.put(&object, &"codeVerifier", cx.lookup(scope, "codeVerifier")?)?;
            cx.put(
                &object,
                &"codeChallenge",
                cx.lookup(scope, "codeChallenge")?,
            )?;
            cx.put(
                &object,
                &"codeChallengeMethod",
                cx.lookup(scope, "codeChallengeMethod")?,
            )?;
            object
        })?;
        return Ok(cx.lookup(scope, "tokenParams")?);
        Ok(V::Undefined)
    })
}

fn f193(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        {
            let value = args.get(1).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "tokenParams", value);
        }
        cx.define(scope, "defaults", {
            let function = cx.lookup(scope, "getDefaultTokenParams")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        let _ = cx.assign(scope, "tokenParams", {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "defaults")?)?;
            cx.spread(&object, &cx.lookup(scope, "tokenParams")?)?;
            object
        })?;
        if {
            let left = cx.get(&cx.lookup(scope, "tokenParams")?, &"dpop")?;
            if left.truthy() {
                V::Bool(
                    !{
                        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"features")?;
                        let method = "isDPoPSupported";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                    .truthy(),
                )
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string(
                        "DPoP has been configured, but is not supported by browser",
                    ));
                    argv
                })?);
            }
        }
        if binary(
            "===",
            cx.get(&cx.lookup(scope, "tokenParams")?, &"pkce")?,
            V::Bool(false),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok(cx.lookup(scope, "tokenParams")?);
            }
        }
        return Ok({
            let function = cx.lookup(scope, "preparePKCE")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.lookup(scope, "tokenParams")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f194(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "codeChallengeMethod",
            args.get(1).cloned().unwrap_or(V::Undefined),
        );
        let _ = cx.assign(scope, "codeChallengeMethod", {
            let left = {
                let left = cx.lookup(scope, "codeChallengeMethod")?;
                if left.truthy() {
                    left
                } else {
                    cx.get(
                        &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                        &"codeChallengeMethod",
                    )?
                }
            };
            if left.truthy() {
                left
            } else {
                cx.lookup(scope, "DEFAULT_CODE_CHALLENGE_METHOD")?
            }
        })?;
        cx.define(
            scope,
            "wellKnownResponse",
            cx.await_value({
                let function = cx.lookup(scope, "getWellKnown")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "sdk")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        );
        cx.define(scope, "methods", {
            let left = cx.get(
                &cx.lookup(scope, "wellKnownResponse")?,
                &V::string("code_challenge_methods_supported").text(),
            )?;
            if left.truthy() {
                left
            } else {
                {
                    let mut values = vec![];
                    cx.array(values)
                }
            }
        });
        if binary(
            "===",
            {
                let receiver = cx.lookup(scope, "methods")?;
                let method = "indexOf";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "codeChallengeMethod")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            V::Number(-V::Number(1f64).number()),
        )?
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("Invalid code_challenge_method"));
                    argv
                })?);
            }
        }
        return Ok(cx.lookup(scope, "codeChallengeMethod")?);
        Ok(V::Undefined)
    })
}

fn f195(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "a", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "b", args.get(1).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "===",
        cx.get(&cx.lookup(scope, "a")?, &"refreshToken")?,
        cx.get(&cx.lookup(scope, "b")?, &"refreshToken")?,
    )?);

    Ok(V::Undefined)
}

fn f196(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "storageManager",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "state", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "sharedStorage", {
        let receiver = cx.lookup(scope, "storageManager")?;
        let method = "getSharedTansactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "entries", {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = cx.delete(
        &cx.lookup(scope, "entries")?,
        &cx.lookup(scope, "state")?.text(),
    )?;
    let _ = {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "setStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "entries")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f197(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "storageManager",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "state", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "sharedStorage", {
        let receiver = cx.lookup(scope, "storageManager")?;
        let method = "getSharedTansactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "entries", {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(
        scope,
        "entry",
        cx.get(
            &cx.lookup(scope, "entries")?,
            &cx.lookup(scope, "state")?.text(),
        )?,
    );
    if {
        let left = {
            let left = cx.lookup(scope, "entry")?;
            if left.truthy() {
                cx.get(&cx.lookup(scope, "entry")?, &"transaction")?
            } else {
                left
            }
        };
        if left.truthy() {
            {
                let function = cx.lookup(scope, "isTransactionMeta")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "entry")?, &"transaction")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.get(&cx.lookup(scope, "entry")?, &"transaction")?);
        }
    }
    return Ok(V::Null);

    Ok(V::Undefined)
}

fn f199(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "state", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "entry",
        cx.get(
            &cx.lookup(scope, "entries")?,
            &cx.lookup(scope, "state")?.text(),
        )?,
    );
    cx.define(
        scope,
        "age",
        binary(
            "-",
            {
                let receiver = cx.lookup(scope, "Date")?;
                let method = "now";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            cx.get(&cx.lookup(scope, "entry")?, &"dateCreated")?,
        )?,
    );
    if binary(
        ">",
        cx.lookup(scope, "age")?,
        cx.lookup(scope, "MAX_ENTRY_LIFETIME")?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.delete(
                &cx.lookup(scope, "entries")?,
                &cx.lookup(scope, "state")?.text(),
            )?;
        }
    }

    Ok(V::Undefined)
}

fn f198(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "storageManager",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "sharedStorage", {
        let receiver = cx.lookup(scope, "storageManager")?;
        let method = "getSharedTansactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "entries", {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = {
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "keys";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "entries")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f199));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    let _ = {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "setStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "entries")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f200(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "storageManager",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "state", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "meta", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "sharedStorage", {
        let receiver = cx.lookup(scope, "storageManager")?;
        let method = "getSharedTansactionStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "entries", {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "getStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = {
        let object = cx.lookup(scope, "entries")?;
        let key = cx.lookup(scope, "state")?.text();
        let value = {
            let object = cx.object();
            cx.put(&object, &"dateCreated", {
                let receiver = cx.lookup(scope, "Date")?;
                let method = "now";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
            cx.put(&object, &"transaction", cx.lookup(scope, "meta")?)?;
            object
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let receiver = cx.lookup(scope, "sharedStorage")?;
        let method = "setStorage";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "entries")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f201(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "hashOrSearch",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "plus2space", cx.regexp("\\+", "g")?);
    cx.define(scope, "paramSplit", cx.regexp("([^&=]+)=?([^&]*)", "g")?);
    cx.define(scope, "fragment", {
        let left = cx.lookup(scope, "hashOrSearch")?;
        if left.truthy() {
            left
        } else {
            V::string("")
        }
    });
    if {
        let left = binary(
            "===",
            {
                let receiver = cx.lookup(scope, "fragment")?;
                let method = "charAt";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(0f64));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            V::string("#"),
        )?;
        if left.truthy() {
            binary(
                "===",
                {
                    let receiver = cx.lookup(scope, "fragment")?;
                    let method = "charAt";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::Number(1f64));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::string("/"),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "fragment", {
                let receiver = cx.lookup(scope, "fragment")?;
                let method = "substring";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(2f64));
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    }
    if {
        let left = binary(
            "===",
            {
                let receiver = cx.lookup(scope, "fragment")?;
                let method = "charAt";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(0f64));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            V::string("#"),
        )?;
        if left.truthy() {
            left
        } else {
            binary(
                "===",
                {
                    let receiver = cx.lookup(scope, "fragment")?;
                    let method = "charAt";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(V::Number(0f64));
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::string("?"),
            )?
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "fragment", {
                let receiver = cx.lookup(scope, "fragment")?;
                let method = "substring";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(1f64));
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    }
    cx.define(scope, "obj", {
        let object = cx.object();
        object
    });
    cx.define(scope, "param", V::Undefined);
    while V::Bool(true).truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "param", {
                let receiver = cx.lookup(scope, "paramSplit")?;
                let method = "exec";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "fragment")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
            if V::Bool(!cx.lookup(scope, "param")?.truthy()).truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    break;
                }
            }
            cx.define(
                scope,
                "key",
                cx.get(&cx.lookup(scope, "param")?, &V::Number(1f64).text())?,
            );
            cx.define(
                scope,
                "value",
                cx.get(&cx.lookup(scope, "param")?, &V::Number(2f64).text())?,
            );
            if {
                let left = {
                    let left = binary("===", cx.lookup(scope, "key")?, V::string("id_token"))?;
                    if left.truthy() {
                        left
                    } else {
                        binary("===", cx.lookup(scope, "key")?, V::string("access_token"))?
                    }
                };
                if left.truthy() {
                    left
                } else {
                    binary("===", cx.lookup(scope, "key")?, V::string("code"))?
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "obj")?;
                        let key = cx.lookup(scope, "key")?.text();
                        let value = cx.lookup(scope, "value")?;
                        cx.put(&object, &key, value)?
                    };
                }
            } else {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "obj")?;
                        let key = cx.lookup(scope, "key")?.text();
                        let value = {
                            let function = cx.lookup(scope, "decodeURIComponent")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push({
                                    let receiver = cx.lookup(scope, "value")?;
                                    let method = "replace";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.lookup(scope, "plus2space")?);
                                        argv.push(V::string(" "));
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                });
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        };
                        cx.put(&object, &key, value)?
                    };
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "obj")?);

    Ok(V::Undefined)
}

fn f202(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "claims",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "validationParams",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "aud",
        cx.get(&cx.lookup(scope, "validationParams")?, &"clientId")?,
    );
    cx.define(
        scope,
        "iss",
        cx.get(&cx.lookup(scope, "validationParams")?, &"issuer")?,
    );
    cx.define(
        scope,
        "nonce",
        cx.get(&cx.lookup(scope, "validationParams")?, &"nonce")?,
    );
    cx.define(
        scope,
        "acr",
        cx.get(&cx.lookup(scope, "validationParams")?, &"acrValues")?,
    );
    if {
        let left = {
            let left = V::Bool(!cx.lookup(scope, "claims")?.truthy());
            if left.truthy() {
                left
            } else {
                V::Bool(!cx.lookup(scope, "iss")?.truthy())
            }
        };
        if left.truthy() {
            left
        } else {
            V::Bool(!cx.lookup(scope, "aud")?.truthy())
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "The jwt, iss, and aud arguments are all required",
                ));
                argv
            })?);
        }
    }
    if {
        let left = cx.lookup(scope, "nonce")?;
        if left.truthy() {
            binary(
                "!==",
                cx.get(&cx.lookup(scope, "claims")?, &"nonce")?,
                cx.lookup(scope, "nonce")?,
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string(
                    "OAuth flow response nonce doesn't match request nonce",
                ));
                argv
            })?);
        }
    }
    cx.define(scope, "now", {
        let receiver = cx.lookup(scope, "Math")?;
        let method = "floor";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(binary(
                "/",
                {
                    let receiver = cx.lookup(scope, "Date")?;
                    let method = "now";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::Number(1000f64),
            )?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if binary(
        "!==",
        cx.get(&cx.lookup(scope, "claims")?, &"iss")?,
        cx.lookup(scope, "iss")?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(binary(
                    "+",
                    binary(
                        "+",
                        binary(
                            "+",
                            binary(
                                "+",
                                binary(
                                    "+",
                                    V::string("The issuer ["),
                                    cx.get(&cx.lookup(scope, "claims")?, &"iss")?,
                                )?,
                                V::string("] "),
                            )?,
                            V::string("does not match ["),
                        )?,
                        cx.lookup(scope, "iss")?,
                    )?,
                    V::string("]"),
                )?);
                argv
            })?);
        }
    }
    if {
        let left = {
            let left = {
                let receiver = cx.lookup(scope, "Array")?;
                let method = "isArray";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "claims")?, &"aud")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            if left.truthy() {
                binary(
                    "<",
                    {
                        let receiver = cx.get(&cx.lookup(scope, "claims")?, &"aud")?;
                        let method = "indexOf";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "aud")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    },
                    V::Number(0f64),
                )?
            } else {
                left
            }
        };
        if left.truthy() {
            left
        } else {
            {
                let left = V::Bool(
                    !{
                        let receiver = cx.lookup(scope, "Array")?;
                        let method = "isArray";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.get(&cx.lookup(scope, "claims")?, &"aud")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                    .truthy(),
                );
                if left.truthy() {
                    binary(
                        "!==",
                        cx.get(&cx.lookup(scope, "claims")?, &"aud")?,
                        cx.lookup(scope, "aud")?,
                    )?
                } else {
                    left
                }
            }
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(binary(
                    "+",
                    binary(
                        "+",
                        binary(
                            "+",
                            binary(
                                "+",
                                binary(
                                    "+",
                                    V::string("The audience ["),
                                    cx.get(&cx.lookup(scope, "claims")?, &"aud")?,
                                )?,
                                V::string("] "),
                            )?,
                            V::string("does not match ["),
                        )?,
                        cx.lookup(scope, "aud")?,
                    )?,
                    V::string("]"),
                )?);
                argv
            })?);
        }
    }
    if {
        let left = cx.lookup(scope, "acr")?;
        if left.truthy() {
            binary(
                "!==",
                cx.get(&cx.lookup(scope, "claims")?, &"acr")?,
                cx.lookup(scope, "acr")?,
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(binary(
                    "+",
                    binary(
                        "+",
                        binary(
                            "+",
                            binary(
                                "+",
                                binary(
                                    "+",
                                    V::string("The acr ["),
                                    cx.get(&cx.lookup(scope, "claims")?, &"acr")?,
                                )?,
                                V::string("] "),
                            )?,
                            V::string("does not match acr_values ["),
                        )?,
                        cx.lookup(scope, "acr")?,
                    )?,
                    V::string("]"),
                )?);
                argv
            })?);
        }
    }
    if binary(
        ">",
        cx.get(&cx.lookup(scope, "claims")?, &"iat")?,
        cx.get(&cx.lookup(scope, "claims")?, &"exp")?,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                let mut argv = Vec::new();
                argv.push(V::string("The JWT expired before it was issued"));
                argv
            })?);
        }
    }
    if V::Bool(
        !cx.get(
            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
            &"ignoreLifetime",
        )?
        .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            if binary(
                ">",
                binary(
                    "-",
                    cx.lookup(scope, "now")?,
                    cx.get(
                        &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                        &"maxClockSkew",
                    )?,
                )?,
                cx.get(&cx.lookup(scope, "claims")?, &"exp")?,
            )?
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                        let mut argv = Vec::new();
                        argv.push(V::string("The JWT expired and is no longer valid"));
                        argv
                    })?);
                }
            }
            if binary(
                ">",
                cx.get(&cx.lookup(scope, "claims")?, &"iat")?,
                binary(
                    "+",
                    cx.lookup(scope, "now")?,
                    cx.get(
                        &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                        &"maxClockSkew",
                    )?,
                )?,
            )?
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                        let mut argv = Vec::new();
                        argv.push(V::string("The JWT was issued in the future"));
                        argv
                    })?);
                }
            }
        }
    }

    Ok(V::Undefined)
}

fn f203(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
        cx.define(scope, "token", args.get(1).cloned().unwrap_or(V::Undefined));
        cx.define(
            scope,
            "validationParams",
            args.get(2).cloned().unwrap_or(V::Undefined),
        );
        if {
            let left = V::Bool(!cx.lookup(scope, "token")?.truthy());
            if left.truthy() {
                left
            } else {
                V::Bool(!cx.get(&cx.lookup(scope, "token")?, &"idToken")?.truthy())
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("Only idTokens may be verified"));
                    argv
                })?);
            }
        }
        cx.define(scope, "jwt", {
            let function = cx.lookup(scope, "decodeToken")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&cx.lookup(scope, "token")?, &"idToken")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        });
        cx.define(scope, "configuredIssuer", {
            let left = 'chain204: {
                {
                    let object = cx.lookup(scope, "validationParams")?;
                    if object.nullish() {
                        break 'chain204 V::Undefined;
                    }
                    cx.get(&object, &"issuer")?
                }
            };
            if left.truthy() {
                left
            } else {
                cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"issuer")?
            }
        });
        {
            let binding = cx
                .await_value({
                    let function = cx.lookup(scope, "getWellKnown")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "sdk")?);
                        argv.push(cx.lookup(scope, "configuredIssuer")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })
                .await?;
            cx.define(scope, "issuer", cx.get(&binding, &"issuer")?);
        }
        cx.define(scope, "validationOptions", {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "assign";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    cx.put(
                        &object,
                        &"clientId",
                        cx.get(&cx.get(&cx.lookup(scope, "sdk")?, &"options")?, &"clientId")?,
                    )?;
                    cx.put(
                        &object,
                        &"ignoreSignature",
                        cx.get(
                            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
                            &"ignoreSignature",
                        )?,
                    )?;
                    object
                });
                argv.push(cx.lookup(scope, "validationParams")?);
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"issuer", cx.lookup(scope, "issuer")?)?;
                    object
                });
                argv
            };
            cx.call(function, receiver, argv)?
        });
        let _ = {
            let function = cx.lookup(scope, "validateClaims")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "sdk")?);
                argv.push(cx.get(&cx.lookup(scope, "jwt")?, &"payload")?);
                argv.push(cx.lookup(scope, "validationOptions")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        if {
            let left = binary(
                "==",
                cx.get(&cx.lookup(scope, "validationOptions")?, &"ignoreSignature")?,
                V::Bool(true),
            )?;
            if left.truthy() {
                left
            } else {
                V::Bool(
                    !{
                        let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"features")?;
                        let method = "isTokenVerifySupported";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }
                    .truthy(),
                )
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                return Ok(cx.lookup(scope, "token")?);
            }
        }
        cx.define(
            scope,
            "key",
            cx.await_value({
                let function = cx.lookup(scope, "getKey")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "sdk")?);
                    argv.push(cx.get(&cx.lookup(scope, "token")?, &"issuer")?);
                    argv.push(cx.get(&cx.get(&cx.lookup(scope, "jwt")?, &"header")?, &"kid")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })
            .await?,
        );
        cx.define(
            scope,
            "valid",
            cx.await_value({
                let receiver = cx.lookup(scope, "sdkCrypto")?;
                let method = "verifyToken";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "token")?, &"idToken")?);
                    argv.push(cx.lookup(scope, "key")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })
            .await?,
        );
        if V::Bool(!cx.lookup(scope, "valid")?.truthy()).truthy() {
            {
                let scope = cx.scope(Some(scope));
                return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                    let mut argv = Vec::new();
                    argv.push(V::string("The token signature is not valid"));
                    argv
                })?);
            }
        }
        if {
            let left = {
                let left = cx.lookup(scope, "validationParams")?;
                if left.truthy() {
                    cx.get(&cx.lookup(scope, "validationParams")?, &"accessToken")?
                } else {
                    left
                }
            };
            if left.truthy() {
                cx.get(&cx.get(&cx.lookup(scope, "token")?, &"claims")?, &"at_hash")?
            } else {
                left
            }
        }
        .truthy()
        {
            {
                let scope = cx.scope(Some(scope));
                cx.define(
                    scope,
                    "hash",
                    cx.await_value({
                        let receiver = cx.lookup(scope, "sdkCrypto")?;
                        let method = "getOidcHash";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(
                                cx.get(&cx.lookup(scope, "validationParams")?, &"accessToken")?,
                            );
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    })
                    .await?,
                );
                if binary(
                    "!==",
                    cx.lookup(scope, "hash")?,
                    cx.get(&cx.get(&cx.lookup(scope, "token")?, &"claims")?, &"at_hash")?,
                )?
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                            let mut argv = Vec::new();
                            argv.push(V::string("Token hash verification failed"));
                            argv
                        })?);
                    }
                }
            }
        }
        return Ok(cx.lookup(scope, "token")?);
        Ok(V::Undefined)
    })
}

fn f206(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));

    Ok(V::Undefined)
}

fn f207(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));

    Ok(V::Undefined)
}

fn f208(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));

    Ok(V::Undefined)
}

fn f209(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));

    Ok(V::Undefined)
}

fn f205(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "nativeConsole", {
        let function = cx.lookup(scope, "getNativeConsole")?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if {
        let left = cx.lookup(scope, "nativeConsole")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "nativeConsole")?, &"log")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "nativeConsole")?);
        }
    }
    return Ok({
        let object = cx.object();
        cx.put(&object, &"log", cx.function(scope, f206))?;
        cx.put(&object, &"warn", cx.function(scope, f207))?;
        cx.put(&object, &"group", cx.function(scope, f208))?;
        cx.put(&object, &"groupEnd", cx.function(scope, f209))?;
        object
    });

    Ok(V::Undefined)
}

fn f210(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    if binary(
        "!==",
        V::string(cx.lookup(scope, "console")?.typeof_()),
        V::string("undefined"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "console")?);
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "undefined")?);
        }
    }

    Ok(V::Undefined)
}

fn f211(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "text", args.get(0).cloned().unwrap_or(V::Undefined));
    let _ = {
        let receiver = {
            let function = cx.lookup(scope, "getConsole")?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        let method = "warn";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(binary(
                "+",
                V::string("[okta-auth-sdk] WARN: "),
                cx.lookup(scope, "text")?,
            )?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f212(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "path", cx.get(&binding, &"path")?);
        cx.define(scope, "json", cx.get(&binding, &"json")?);
    }
    cx.define(scope, "steps", {
        let mut values = vec![];
        cx.array(values)
    });
    cx.define(scope, "match", V::Undefined);
    while binary(
        "!==",
        cx.assign(scope, "match", {
            let receiver = cx.lookup(scope, "jsonpathRegex")?;
            let method = "exec";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "path")?);
                argv
            };
            cx.call(function, receiver, argv)?
        })?,
        V::Null,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "step", {
                let left = 'chain213: {
                    {
                        let object = {
                            let object = cx.lookup(scope, "match")?;
                            if object.nullish() {
                                break 'chain213 V::Undefined;
                            }
                            cx.get(&object, &"groups")?
                        };
                        if object.nullish() {
                            break 'chain213 V::Undefined;
                        }
                        cx.get(&object, &"step")?
                    }
                };
                if left.nullish() {
                    'chain214: {
                        {
                            let object = {
                                let object = cx.lookup(scope, "match")?;
                                if object.nullish() {
                                    break 'chain214 V::Undefined;
                                }
                                cx.get(&object, &"groups")?
                            };
                            if object.nullish() {
                                break 'chain214 V::Undefined;
                            }
                            cx.get(&object, &"index")?
                        }
                    }
                } else {
                    left
                }
            });
            if cx.lookup(scope, "step")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let receiver = cx.lookup(scope, "steps")?;
                        let method = "push";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "step")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
            }
        }
    }
    if binary(
        "<",
        cx.get(&cx.lookup(scope, "steps")?, &"length")?,
        V::Number(1f64),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "undefined")?);
        }
    }
    cx.define(scope, "lastStep", {
        let receiver = cx.lookup(scope, "steps")?;
        let method = "pop";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "curr", cx.lookup(scope, "json")?);
    for item in cx.items(&cx.lookup(scope, "steps")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "step", item);
        {
            let scope = cx.scope(Some(scope));
            if {
                let receiver = cx.get(
                    &cx.get(&cx.lookup(scope, "Object")?, &"prototype")?,
                    &"hasOwnProperty",
                )?;
                let method = "call";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "curr")?);
                    argv.push(cx.lookup(scope, "step")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    if binary(
                        "!==",
                        V::string(
                            cx.get(
                                &cx.lookup(scope, "curr")?,
                                &cx.lookup(scope, "step")?.text(),
                            )?
                            .typeof_(),
                        ),
                        V::string("object"),
                    )?
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            return Ok(cx.lookup(scope, "undefined")?);
                        }
                    }
                    let _ = cx.assign(
                        scope,
                        "curr",
                        cx.get(
                            &cx.lookup(scope, "curr")?,
                            &cx.lookup(scope, "step")?.text(),
                        )?,
                    )?;
                }
            }
        }
    }
    return Ok(cx.get(
        &cx.lookup(scope, "curr")?,
        &cx.lookup(scope, "lastStep")?.text(),
    )?);

    Ok(V::Undefined)
}

fn f215(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "length",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "randomCharset",
        V::string("abcdefghijklnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
    );
    cx.define(scope, "random", V::string(""));
    {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "c", V::Number(0f64));
        cx.define(
            scope,
            "cl",
            cx.get(&cx.lookup(scope, "randomCharset")?, &"length")?,
        );
        let mut first = true;
        loop {
            if !first {
                let _ = {
                    let old = cx.lookup(scope, "c")?;
                    let updated =
                        cx.assign(scope, "c", binary("+", old.clone(), V::Number(1.0))?)?;
                    updated
                };
            }
            first = false;
            if !binary("<", cx.lookup(scope, "c")?, cx.lookup(scope, "length")?)?.truthy() {
                break;
            }
            {
                let scope = cx.scope(Some(scope));
                let _ = cx.assign(
                    scope,
                    "random",
                    binary(
                        "+",
                        cx.lookup(scope, "random")?,
                        cx.get(
                            &cx.lookup(scope, "randomCharset")?,
                            &{
                                let receiver = cx.lookup(scope, "Math")?;
                                let method = "floor";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(binary(
                                        "*",
                                        {
                                            let receiver = cx.lookup(scope, "Math")?;
                                            let method = "random";
                                            let function = cx.resolve_method(&receiver, &method)?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv
                                            };
                                            cx.call(function, receiver, argv)?
                                        },
                                        cx.lookup(scope, "cl")?,
                                    )?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            }
                            .text(),
                        )?,
                    )?,
                )?;
            }
        }
    }
    return Ok(cx.lookup(scope, "random")?);

    Ok(V::Undefined)
}

fn f216(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "str", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "delim", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "parts", {
        let receiver = cx.lookup(scope, "str")?;
        let method = "split";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "delim")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok({
        let mut values = vec![];
        values.push(cx.get(&cx.lookup(scope, "parts")?, &V::Number(0f64).text())?);
        values.push({
            let receiver = {
                let receiver = cx.lookup(scope, "parts")?;
                let method = "splice";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::Number(1f64));
                    argv.push(cx.get(&cx.lookup(scope, "parts")?, &"length")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "join";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "delim")?);
                argv
            };
            cx.call(function, receiver, argv)?
        });
        cx.array(values)
    });

    Ok(V::Undefined)
}

fn f217(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    if cx.lookup(scope, "obj")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "str", {
                let receiver = cx.lookup(scope, "JSON")?;
                let method = "stringify";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "obj")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            if cx.lookup(scope, "str")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok({
                        let receiver = cx.lookup(scope, "JSON")?;
                        let method = "parse";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "str")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "obj")?);

    Ok(V::Undefined)
}

fn f218(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "collection",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "searchParams",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "c",
        cx.get(&cx.lookup(scope, "collection")?, &"length")?,
    );
    while {
        let old = cx.lookup(scope, "c")?;
        let updated = cx.assign(scope, "c", binary("-", old.clone(), V::Number(1.0))?)?;
        old
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(
                scope,
                "item",
                cx.get(
                    &cx.lookup(scope, "collection")?,
                    &cx.lookup(scope, "c")?.text(),
                )?,
            );
            cx.define(scope, "found", V::Bool(true));
            for (key, _) in cx.entries(&cx.lookup(scope, "searchParams")?)? {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "prop", V::string(key));
                {
                    let scope = cx.scope(Some(scope));
                    if V::Bool(
                        !{
                            let receiver = cx.get(
                                &cx.get(&cx.lookup(scope, "Object")?, &"prototype")?,
                                &"hasOwnProperty",
                            )?;
                            let method = "call";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "searchParams")?);
                                argv.push(cx.lookup(scope, "prop")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        }
                        .truthy(),
                    )
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            continue;
                        }
                    }
                    if binary(
                        "!==",
                        cx.get(
                            &cx.lookup(scope, "item")?,
                            &cx.lookup(scope, "prop")?.text(),
                        )?,
                        cx.get(
                            &cx.lookup(scope, "searchParams")?,
                            &cx.lookup(scope, "prop")?.text(),
                        )?,
                    )?
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = cx.assign(scope, "found", V::Bool(false))?;
                            break;
                        }
                    }
                }
            }
            if cx.lookup(scope, "found")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(cx.lookup(scope, "item")?);
                }
            }
        }
    }

    Ok(V::Undefined)
}

fn f219(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "cleaned", {
        let object = cx.object();
        object
    });
    for (key, _) in cx.entries(&cx.lookup(scope, "obj")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "prop", V::string(key));
        {
            let scope = cx.scope(Some(scope));
            if {
                let receiver = cx.get(
                    &cx.get(&cx.lookup(scope, "Object")?, &"prototype")?,
                    &"hasOwnProperty",
                )?;
                let method = "call";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "obj")?);
                    argv.push(cx.lookup(scope, "prop")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(
                        scope,
                        "value",
                        cx.get(&cx.lookup(scope, "obj")?, &cx.lookup(scope, "prop")?.text())?,
                    );
                    if {
                        let left = binary("!==", cx.lookup(scope, "value")?, V::Null)?;
                        if left.truthy() {
                            binary(
                                "!==",
                                cx.lookup(scope, "value")?,
                                cx.lookup(scope, "undefined")?,
                            )?
                        } else {
                            left
                        }
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "cleaned")?;
                                let key = cx.lookup(scope, "prop")?.text();
                                let value = cx.lookup(scope, "value")?;
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "cleaned")?);

    Ok(V::Undefined)
}

fn f220(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "fn", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let left = V::Bool(!V::Bool(!cx.lookup(scope, "fn")?.truthy()).truthy());
        if left.truthy() {
            binary(
                "===",
                {
                    let receiver = cx.get(
                        &{
                            let object = cx.object();
                            object
                        },
                        &"toString",
                    )?;
                    let method = "call";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "fn")?);
                        argv
                    };
                    cx.call(function, receiver, argv)?
                },
                V::string("[object Function]"),
            )?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f221(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "===",
        {
            let receiver = cx.get(
                &cx.get(&cx.lookup(scope, "Object")?, &"prototype")?,
                &"toString",
            )?;
            let method = "call";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "obj")?);
                argv
            };
            cx.call(function, receiver, argv)?
        },
        V::string("[object String]"),
    )?);

    Ok(V::Undefined)
}

fn f222(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "url", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = cx.regexp("^[a-z][a-z0-9+.-]*:", "i")?;
        let method = "test";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "url")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f223(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "path", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(!cx.lookup(scope, "path")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    cx.define(scope, "trimmed", {
        let receiver = cx.lookup(scope, "path")?;
        let method = "replace";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.regexp("^\\s+|\\s+$", "gm")?);
            argv.push(V::string(""));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    let _ = cx.assign(scope, "trimmed", {
        let receiver = cx.lookup(scope, "trimmed")?;
        let method = "replace";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.regexp("\\/+$", "")?);
            argv.push(V::string(""));
            argv
        };
        cx.call(function, receiver, argv)?
    })?;
    return Ok(cx.lookup(scope, "trimmed")?);

    Ok(V::Undefined)
}

fn f224(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "str", {
        let mut values = vec![];
        cx.array(values)
    });
    if binary("!==", cx.lookup(scope, "obj")?, V::Null)?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            for (key, _) in cx.entries(&cx.lookup(scope, "obj")?)? {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "key", V::string(key));
                {
                    let scope = cx.scope(Some(scope));
                    if {
                        let left = {
                            let left = {
                                let receiver = cx.get(
                                    &cx.get(&cx.lookup(scope, "Object")?, &"prototype")?,
                                    &"hasOwnProperty",
                                )?;
                                let method = "call";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(cx.lookup(scope, "obj")?);
                                    argv.push(cx.lookup(scope, "key")?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                            if left.truthy() {
                                binary(
                                    "!==",
                                    cx.get(
                                        &cx.lookup(scope, "obj")?,
                                        &cx.lookup(scope, "key")?.text(),
                                    )?,
                                    cx.lookup(scope, "undefined")?,
                                )?
                            } else {
                                left
                            }
                        };
                        if left.truthy() {
                            binary(
                                "!==",
                                cx.get(
                                    &cx.lookup(scope, "obj")?,
                                    &cx.lookup(scope, "key")?.text(),
                                )?,
                                V::Null,
                            )?
                        } else {
                            left
                        }
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let receiver = cx.lookup(scope, "str")?;
                                let method = "push";
                                let function = cx.resolve_method(&receiver, &method)?;
                                let argv = {
                                    let mut argv = Vec::new();
                                    argv.push(binary(
                                        "+",
                                        binary("+", cx.lookup(scope, "key")?, V::string("="))?,
                                        {
                                            let function =
                                                cx.lookup(scope, "encodeURIComponent")?;
                                            let argv = {
                                                let mut argv = Vec::new();
                                                argv.push(cx.get(
                                                    &cx.lookup(scope, "obj")?,
                                                    &cx.lookup(scope, "key")?.text(),
                                                )?);
                                                argv
                                            };
                                            cx.call(function, V::Undefined, argv)?
                                        },
                                    )?);
                                    argv
                                };
                                cx.call(function, receiver, argv)?
                            };
                        }
                    }
                }
            }
        }
    }
    if cx.get(&cx.lookup(scope, "str")?, &"length")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(binary("+", V::string("?"), {
                let receiver = cx.lookup(scope, "str")?;
                let method = "join";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("&"));
                    argv
                };
                cx.call(function, receiver, argv)?
            })?);
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::string(""));
        }
    }

    Ok(V::Undefined)
}

fn f225(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "message",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "message")?);
        argv
    })?;
    let _ = {
        let receiver = cx.lookup(scope, "Object")?;
        let method = "setPrototypeOf";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(this.clone());
            argv.push(cx.get(&cx.new_target(&this)?, &"prototype")?);
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f226(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "err", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "xhr", args.get(1).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "meta", args.get(2).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "message",
        cx.get(&cx.lookup(scope, "err")?, &"errorSummary")?,
    );
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "message")?);
        argv
    })?;
    let _ = {
        let object = this.clone();
        let key = "name";
        let value = V::string("AuthApiError");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorSummary";
        let value = cx.get(&cx.lookup(scope, "err")?, &"errorSummary")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorCode";
        let value = cx.get(&cx.lookup(scope, "err")?, &"errorCode")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorLink";
        let value = cx.get(&cx.lookup(scope, "err")?, &"errorLink")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorId";
        let value = cx.get(&cx.lookup(scope, "err")?, &"errorId")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorCauses";
        let value = cx.get(&cx.lookup(scope, "err")?, &"errorCauses")?;
        cx.put(&object, &key, value)?
    };
    if cx.lookup(scope, "xhr")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = this.clone();
                let key = "xhr";
                let value = cx.lookup(scope, "xhr")?;
                cx.put(&object, &key, value)?
            };
        }
    }
    if cx.lookup(scope, "meta")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = this.clone();
                let key = "meta";
                let value = cx.lookup(scope, "meta")?;
                cx.put(&object, &key, value)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f227(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "msg", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "xhr", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "msg")?);
        argv
    })?;
    let _ = {
        let object = this.clone();
        let key = "name";
        let value = V::string("AuthSdkError");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorCode";
        let value = V::string("INTERNAL");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorSummary";
        let value = cx.lookup(scope, "msg")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorLink";
        let value = V::string("INTERNAL");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorId";
        let value = V::string("INTERNAL");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorCauses";
        let value = {
            let mut values = vec![];
            cx.array(values)
        };
        cx.put(&object, &key, value)?
    };
    if cx.lookup(scope, "xhr")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = this.clone();
                let key = "xhr";
                let value = cx.lookup(scope, "xhr")?;
                cx.put(&object, &key, value)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f228(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "errorCode",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "summary",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "resp", args.get(2).cloned().unwrap_or(V::Undefined));
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "summary")?);
        argv
    })?;
    let _ = {
        let object = this.clone();
        let key = "resp";
        let value = V::Null;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "name";
        let value = V::string("OAuthError");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorCode";
        let value = cx.lookup(scope, "errorCode")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "errorSummary";
        let value = cx.lookup(scope, "summary")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "error";
        let value = cx.lookup(scope, "errorCode")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "error_description";
        let value = cx.lookup(scope, "summary")?;
        cx.put(&object, &key, value)?
    };
    if cx.lookup(scope, "resp")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = this.clone();
                let key = "resp";
                let value = cx.lookup(scope, "resp")?;
                cx.put(&object, &key, value)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f229(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "scheme",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "parameters",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "resp", args.get(2).cloned().unwrap_or(V::Undefined));
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push({
            let left = cx.get(&cx.lookup(scope, "parameters")?, &"error")?;
            if left.nullish() {
                cx.get(&cx.lookup(scope, "WWWAuthError")?, &"UNKNOWN_ERROR")?
            } else {
                left
            }
        });
        argv
    })?;
    let _ = {
        let object = this.clone();
        let key = "name";
        let value = V::string("WWWAuthError");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "resp";
        let value = V::Null;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "scheme";
        let value = cx.lookup(scope, "scheme")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "parameters";
        let value = cx.lookup(scope, "parameters")?;
        cx.put(&object, &key, value)?
    };
    if cx.lookup(scope, "resp")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = this.clone();
                let key = "resp";
                let value = cx.lookup(scope, "resp")?;
                cx.put(&object, &key, value)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f230(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&cx.get(&this.clone(), &"parameters")?, &"error")?);

    Ok(V::Undefined)
}

fn f231(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&this.clone(), &"error")?);

    Ok(V::Undefined)
}

fn f232(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&cx.get(&this.clone(), &"parameters")?, &"error_description")?);

    Ok(V::Undefined)
}

fn f233(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&this.clone(), &"error_description")?);

    Ok(V::Undefined)
}

fn f234(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&this.clone(), &"errorDescription")?);

    Ok(V::Undefined)
}

fn f235(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&cx.get(&this.clone(), &"parameters")?, &"realm")?);

    Ok(V::Undefined)
}

fn f236(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "header",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if V::Bool(!cx.lookup(scope, "header")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Null);
        }
    }
    cx.define(scope,"regex",cx.regexp("(?:,|, )?([a-zA-Z0-9!#$%&'*+\\-.^_`|~]+)=(?:\"([a-zA-Z0-9!#$%&'*+\\-.,^_`|~ /:]+)\"|([a-zA-Z0-9!#$%&'*+\\-.^_`|~/:]+))", "g")?);
    cx.define(scope, "firstSpace", {
        let receiver = cx.lookup(scope, "header")?;
        let method = "indexOf";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::string(" "));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "scheme", {
        let receiver = cx.lookup(scope, "header")?;
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(V::Number(0f64));
            argv.push(cx.lookup(scope, "firstSpace")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "remaining", {
        let receiver = cx.lookup(scope, "header")?;
        let method = "slice";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(binary(
                "+",
                cx.lookup(scope, "firstSpace")?,
                V::Number(1f64),
            )?);
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "params", {
        let object = cx.object();
        object
    });
    cx.define(scope, "match", V::Undefined);
    while binary(
        "!==",
        cx.assign(scope, "match", {
            let receiver = cx.lookup(scope, "regex")?;
            let method = "exec";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "remaining")?);
                argv
            };
            cx.call(function, receiver, argv)?
        })?,
        V::Null,
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "params")?;
                let key = cx
                    .get(&cx.lookup(scope, "match")?, &V::Number(1f64).text())?
                    .text();
                let value = {
                    let left = cx.get(&cx.lookup(scope, "match")?, &V::Number(2f64).text())?;
                    if left.nullish() {
                        cx.get(&cx.lookup(scope, "match")?, &V::Number(3f64).text())?
                    } else {
                        left
                    }
                };
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok(cx.construct(cx.lookup(scope, "WWWAuthError")?, {
        let mut argv = Vec::new();
        argv.push(cx.lookup(scope, "scheme")?);
        argv.push(cx.lookup(scope, "params")?);
        argv
    })?);

    Ok(V::Undefined)
}

fn f237(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "headers", value);
    }
    if {
        let function = cx.lookup(scope, "isFunction")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push('chain238: {
                {
                    let object = cx.lookup(scope, "headers")?;
                    if object.nullish() {
                        break 'chain238 V::Undefined;
                    }
                    cx.get(&object, &"get")?
                }
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let receiver = cx.lookup(scope, "headers")?;
                let method = "get";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("WWW-Authenticate"));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
        }
    }
    return Ok({
        let left = cx.get(
            &cx.lookup(scope, "headers")?,
            &V::string("www-authenticate").text(),
        )?;
        if left.nullish() {
            cx.get(
                &cx.lookup(scope, "headers")?,
                &V::string("WWW-Authenticate").text(),
            )?
        } else {
            left
        }
    });

    Ok(V::Undefined)
}

fn f239(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "sdk", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "error", args.get(1).cloned().unwrap_or(V::Undefined));
    if cx
        .relation(
            "instanceof",
            cx.lookup(scope, "error")?,
            cx.lookup(scope, "Error")?,
        )?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.construct(cx.lookup(scope, "AuthApiError")?, {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    cx.put(
                        &object,
                        &"errorSummary",
                        cx.get(&cx.lookup(scope, "error")?, &"message")?,
                    )?;
                    object
                });
                argv
            })?);
        }
    }
    cx.define(scope, "resp", cx.lookup(scope, "error")?);
    cx.define(scope, "err", V::Undefined);
    cx.define(scope, "serverErr", {
        let object = cx.object();
        object
    });
    if {
        let left = cx.get(&cx.lookup(scope, "resp")?, &"responseText")?;
        if left.truthy() {
            {
                let function = cx.lookup(scope, "isString")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "resp")?, &"responseText")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            {
                let mut completion = (|| {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "serverErr", {
                            let receiver = cx.lookup(scope, "JSON")?;
                            let method = "parse";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.get(&cx.lookup(scope, "resp")?, &"responseText")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        })?;
                    }
                    Ok::<Completion, V>(Completion::Normal)
                })();
                if let Err(error) = completion {
                    completion = (|| {
                        let scope = cx.scope(Some(scope));
                        cx.define(scope, "e", error);
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = cx.assign(scope, "serverErr", {
                                let object = cx.object();
                                cx.put(&object, &"errorSummary", V::string("Unknown error"))?;
                                object
                            })?;
                        }
                        Ok::<Completion, V>(Completion::Normal)
                    })();
                }
                match completion? {
                    Completion::Normal => {}
                    Completion::Return(value) => {
                        return Ok(value);
                    }
                    Completion::Break => {
                        return Err(V::string("unexpected break"));
                    }
                    Completion::Continue => {
                        return Err(V::string("unexpected continue"));
                    }
                }
            }
        }
    }
    if binary(
        ">=",
        cx.get(&cx.lookup(scope, "resp")?, &"status")?,
        V::Number(500f64),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "serverErr")?;
                let key = "errorSummary";
                let value = V::string("Unknown error");
                cx.put(&object, &key, value)?
            };
        }
    }
    if cx
        .get(
            &cx.get(&cx.lookup(scope, "sdk")?, &"options")?,
            &"transformErrorXHR",
        )?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "resp", {
                let receiver = cx.get(&cx.lookup(scope, "sdk")?, &"options")?;
                let method = "transformErrorXHR";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push({
                        let function = cx.lookup(scope, "clone")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "resp")?);
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    }
    cx.define(scope, "wwwAuthHeader", {
        let left = {
            let receiver = cx.lookup(scope, "WWWAuthError")?;
            let method = "getWWWAuthenticateHeader";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push('chain240: {
                    {
                        let object = cx.lookup(scope, "resp")?;
                        if object.nullish() {
                            break 'chain240 V::Undefined;
                        }
                        cx.get(&object, &"headers")?
                    }
                });
                argv
            };
            cx.call(function, receiver, argv)?
        };
        if left.nullish() {
            V::string("")
        } else {
            left
        }
    });
    if {
        let left = cx.get(&cx.lookup(scope, "serverErr")?, &"error")?;
        if left.truthy() {
            cx.get(&cx.lookup(scope, "serverErr")?, &"error_description")?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(
                scope,
                "err",
                cx.construct(cx.lookup(scope, "OAuthError")?, {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "serverErr")?, &"error")?);
                    argv.push(cx.get(&cx.lookup(scope, "serverErr")?, &"error_description")?);
                    argv.push(cx.lookup(scope, "resp")?);
                    argv
                })?,
            )?;
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(
                scope,
                "err",
                cx.construct(cx.lookup(scope, "AuthApiError")?, {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "serverErr")?);
                    argv.push(cx.lookup(scope, "resp")?);
                    argv.push({
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"wwwAuthHeader",
                            cx.lookup(scope, "wwwAuthHeader")?,
                        )?;
                        object
                    });
                    argv
                })?,
            )?;
        }
    }
    if {
        let left = {
            let left = cx.lookup(scope, "wwwAuthHeader")?;
            if left.truthy() {
                binary(
                    ">=",
                    'chain241: {
                        {
                            let object = cx.lookup(scope, "resp")?;
                            if object.nullish() {
                                break 'chain241 V::Undefined;
                            }
                            cx.get(&object, &"status")?
                        }
                    },
                    V::Number(400f64),
                )?
            } else {
                left
            }
        };
        if left.truthy() {
            binary(
                "<",
                'chain242: {
                    {
                        let object = cx.lookup(scope, "resp")?;
                        if object.nullish() {
                            break 'chain242 V::Undefined;
                        }
                        cx.get(&object, &"status")?
                    }
                },
                V::Number(500f64),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "wwwAuthErr", {
                let receiver = cx.lookup(scope, "WWWAuthError")?;
                let method = "parseHeader";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "wwwAuthHeader")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            if {
                let left = binary(
                    "===",
                    cx.get(&cx.lookup(scope, "resp")?, &"status")?,
                    V::Number(403f64),
                )?;
                if left.truthy() {
                    binary(
                        "===",
                        'chain243: {
                            {
                                let object = cx.lookup(scope, "wwwAuthErr")?;
                                if object.nullish() {
                                    break 'chain243 V::Undefined;
                                }
                                cx.get(&object, &"error")?
                            }
                        },
                        V::string("insufficient_authentication_context"),
                    )?
                } else {
                    left
                }
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    {
                        let binding = cx.get(&cx.lookup(scope, "wwwAuthErr")?, &"parameters")?;
                        cx.define(scope, "max_age", cx.get(&binding, &"max_age")?);
                        cx.define(scope, "acr_values", cx.get(&binding, &"acr_values")?);
                    }
                    let _ = cx.assign(
                        scope,
                        "err",
                        cx.construct(cx.lookup(scope, "AuthApiError")?, {
                            let mut argv = Vec::new();
                            argv.push({
                                let object = cx.object();
                                cx.put(
                                    &object,
                                    &"errorSummary",
                                    cx.get(&cx.lookup(scope, "wwwAuthErr")?, &"error")?,
                                )?;
                                cx.put(&object, &"errorCauses", {
                                    let mut values = vec![];
                                    values.push({
                                        let object = cx.object();
                                        cx.put(
                                            &object,
                                            &"errorSummary",
                                            cx.get(
                                                &cx.lookup(scope, "wwwAuthErr")?,
                                                &"errorDescription",
                                            )?,
                                        )?;
                                        object
                                    });
                                    cx.array(values)
                                })?;
                                object
                            });
                            argv.push(cx.lookup(scope, "resp")?);
                            argv.push({
                                let object = cx.object();
                                cx.put(
                                    &object,
                                    &"max_age",
                                    V::Number(cx.lookup(scope, "max_age")?.number()),
                                )?;
                                cx.spread(&object, &{
                                    let left = cx.lookup(scope, "acr_values")?;
                                    if left.truthy() {
                                        {
                                            let object = cx.object();
                                            cx.put(
                                                &object,
                                                &"acr_values",
                                                cx.lookup(scope, "acr_values")?,
                                            )?;
                                            object
                                        }
                                    } else {
                                        left
                                    }
                                })?;
                                object
                            });
                            argv
                        })?,
                    )?;
                }
            } else {
                if binary(
                    "===",
                    'chain244: {
                        {
                            let object = cx.lookup(scope, "wwwAuthErr")?;
                            if object.nullish() {
                                break 'chain244 V::Undefined;
                            }
                            cx.get(&object, &"scheme")?
                        }
                    },
                    V::string("DPoP"),
                )?
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "err", cx.lookup(scope, "wwwAuthErr")?)?;
                    }
                }
            }
        }
    }
    return Ok(cx.lookup(scope, "err")?);

    Ok(V::Undefined)
}

fn f245(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "state", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "otp", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.push({
            let mut text = String::new();
            text.push_str("Enter the OTP code in the originating client: ");
            text.push_str(&cx.lookup(scope, "otp")?.text());
            text.push_str("");
            V::string(text)
        });
        argv
    })?;
    let _ = {
        let object = this.clone();
        let key = "name";
        let value = V::string("EmailVerifyCallbackError");
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "state";
        let value = cx.lookup(scope, "state")?;
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "otp";
        let value = cx.lookup(scope, "otp")?;
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f246(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "version",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    'switch247: {
        let discriminant = cx.lookup(scope, "version")?;
        let mut matched = false;
        if matched || okta_native_support::eq(&discriminant, &V::string("1.0.0")) {
            matched = true;
            return Ok(cx.lookup(scope, "v1")?);
        }
        if matched || okta_native_support::eq(&discriminant, &cx.lookup(scope, "undefined")?) {
            matched = true;
        }
        if matched || okta_native_support::eq(&discriminant, &V::Null) {
            matched = true;
            return Err(cx.construct(cx.lookup(scope, "Error")?, {
                let mut argv = Vec::new();
                argv.push(V::string("Api version is required"));
                argv
            })?);
        }
        if matched || true {
            matched = true;
            return Err(cx.construct(cx.lookup(scope, "Error")?, {
                let mut argv = Vec::new();
                argv.push({
                    let mut text = String::new();
                    text.push_str("Unknown api version: ");
                    text.push_str(&cx.lookup(scope, "version")?.text());
                    text.push_str(".  Use an exact semver version.");
                    V::string(text)
                });
                argv
            })?);
        }
    }

    Ok(V::Undefined)
}

fn f248(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "field", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(binary(
        "!==",
        cx.get(&cx.lookup(scope, "field")?, &"mutable")?,
        V::Bool(false),
    )?);

    Ok(V::Undefined)
}

fn f249(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "action",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "defaultParamsForAction", {
        let object = cx.object();
        object
    });
    cx.define(scope, "neededParamsForAction", {
        let mut values = vec![];
        cx.array(values)
    });
    cx.define(scope, "immutableParamsForAction", {
        let object = cx.object();
        object
    });
    if V::Bool(!cx.get(&cx.lookup(scope, "action")?, &"value")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "neededParamsForAction")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "action")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            return Ok({
                let object = cx.object();
                cx.put(
                    &object,
                    &"defaultParamsForAction",
                    cx.lookup(scope, "defaultParamsForAction")?,
                )?;
                cx.put(
                    &object,
                    &"neededParamsForAction",
                    cx.lookup(scope, "neededParamsForAction")?,
                )?;
                cx.put(
                    &object,
                    &"immutableParamsForAction",
                    cx.lookup(scope, "immutableParamsForAction")?,
                )?;
                object
            });
        }
    }
    for item in cx.items(&cx.get(&cx.lookup(scope, "action")?, &"value")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "field", item);
        {
            let scope = cx.scope(Some(scope));
            if {
                let function = cx.lookup(scope, "isFieldMutable")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "field")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let receiver = cx.lookup(scope, "neededParamsForAction")?;
                        let method = "push";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "field")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                    if {
                        let left = cx.get(&cx.lookup(scope, "field")?, &"value")?;
                        if left.nullish() {
                            V::Bool(false)
                        } else {
                            left
                        }
                    }
                    .truthy()
                    {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "defaultParamsForAction")?;
                                let key = cx.get(&cx.lookup(scope, "field")?, &"name")?.text();
                                let value = cx.get(&cx.lookup(scope, "field")?, &"value")?;
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                }
            } else {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "immutableParamsForAction")?;
                        let key = cx.get(&cx.lookup(scope, "field")?, &"name")?.text();
                        let value = {
                            let left = cx.get(&cx.lookup(scope, "field")?, &"value")?;
                            if left.nullish() {
                                V::string("")
                            } else {
                                left
                            }
                        };
                        cx.put(&object, &key, value)?
                    };
                }
            }
        }
    }
    return Ok({
        let object = cx.object();
        cx.put(
            &object,
            &"defaultParamsForAction",
            cx.lookup(scope, "defaultParamsForAction")?,
        )?;
        cx.put(
            &object,
            &"neededParamsForAction",
            cx.lookup(scope, "neededParamsForAction")?,
        )?;
        cx.put(
            &object,
            &"immutableParamsForAction",
            cx.lookup(scope, "immutableParamsForAction")?,
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f250(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "actionList",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    let _ = cx.assign(
        scope,
        "actionList",
        if {
            let receiver = cx.lookup(scope, "Array")?;
            let method = "isArray";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "actionList")?);
                argv
            };
            cx.call(function, receiver, argv)?
        }
        .truthy()
        {
            cx.lookup(scope, "actionList")?
        } else {
            {
                let mut values = vec![];
                values.push(cx.lookup(scope, "actionList")?);
                cx.array(values)
            }
        },
    )?;
    cx.define(scope, "neededParams", {
        let mut values = vec![];
        cx.array(values)
    });
    cx.define(scope, "defaultParams", {
        let object = cx.object();
        object
    });
    cx.define(scope, "immutableParams", {
        let object = cx.object();
        object
    });
    for item in cx.items(&cx.lookup(scope, "actionList")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "action", item);
        {
            let scope = cx.scope(Some(scope));
            {
                let binding = {
                    let function = cx.lookup(scope, "divideSingleActionParamsByMutability")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "action")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                cx.define(
                    scope,
                    "defaultParamsForAction",
                    cx.get(&binding, &"defaultParamsForAction")?,
                );
                cx.define(
                    scope,
                    "neededParamsForAction",
                    cx.get(&binding, &"neededParamsForAction")?,
                );
                cx.define(
                    scope,
                    "immutableParamsForAction",
                    cx.get(&binding, &"immutableParamsForAction")?,
                );
            }
            let _ = {
                let receiver = cx.lookup(scope, "neededParams")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "neededParamsForAction")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let _ = {
                let object = cx.lookup(scope, "defaultParams")?;
                let key = cx.get(&cx.lookup(scope, "action")?, &"name")?.text();
                let value = cx.lookup(scope, "defaultParamsForAction")?;
                cx.put(&object, &key, value)?
            };
            let _ = {
                let object = cx.lookup(scope, "immutableParams")?;
                let key = cx.get(&cx.lookup(scope, "action")?, &"name")?.text();
                let value = cx.lookup(scope, "immutableParamsForAction")?;
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok({
        let object = cx.object();
        cx.put(
            &object,
            &"defaultParams",
            cx.lookup(scope, "defaultParams")?,
        )?;
        cx.put(&object, &"neededParams", cx.lookup(scope, "neededParams")?)?;
        cx.put(
            &object,
            &"immutableParams",
            cx.lookup(scope, "immutableParams")?,
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f252(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(scope, "arguments", cx.array(args.clone()));
        {
            let value = args.get(0).cloned().unwrap_or(V::Undefined);
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "params", value);
        }
        cx.define(scope, "headers", {
            let object = cx.object();
            cx.put(&object, &"Content-Type", V::string("application/json"))?;
            cx.put(&object, &"Accept", {
                let left = cx.get(&cx.lookup(scope, "actionDefinition")?, &"accepts")?;
                if left.truthy() {
                    left
                } else {
                    V::string("application/ion+json")
                }
            })?;
            object
        });
        cx.define(scope, "body", {
            let receiver = cx.lookup(scope, "JSON")?;
            let method = "stringify";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    cx.spread(&object, &cx.lookup(scope, "defaultParamsForAction")?)?;
                    cx.spread(&object, &cx.lookup(scope, "params")?)?;
                    cx.spread(&object, &cx.lookup(scope, "immutableParamsForAction")?)?;
                    object
                });
                argv
            };
            cx.call(function, receiver, argv)?
        });
        {
            let mut completion = (async {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "options", {
                        let object = cx.object();
                        cx.put(&object, &"url", cx.lookup(scope, "target")?)?;
                        cx.put(
                            &object,
                            &"method",
                            cx.get(&cx.lookup(scope, "actionDefinition")?, &"method")?,
                        )?;
                        cx.put(&object, &"headers", cx.lookup(scope, "headers")?)?;
                        cx.put(&object, &"args", cx.lookup(scope, "body")?)?;
                        cx.put(&object, &"withCredentials", {
                            let left = 'chain253: {
                                {
                                    let object = cx.lookup(scope, "toPersist")?;
                                    if object.nullish() {
                                        break 'chain253 V::Undefined;
                                    }
                                    cx.get(&object, &"withCredentials")?
                                }
                            };
                            if left.nullish() {
                                V::Bool(true)
                            } else {
                                left
                            }
                        })?;
                        object
                    });
                    cx.define(scope, "isPolling", {
                        let left = binary(
                            "===",
                            cx.get(&cx.lookup(scope, "actionDefinition")?, &"name")?,
                            V::string("poll"),
                        )?;
                        if left.truthy() {
                            left
                        } else {
                            'chain254: {
                                {
                                    let receiver =
                                        cx.get(&cx.lookup(scope, "actionDefinition")?, &"name")?;
                                    if receiver.nullish() {
                                        break 'chain254 V::Undefined;
                                    }
                                    let method = "endsWith";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(V::string("-poll"));
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                }
                            }
                        }
                    });
                    if cx.lookup(scope, "isPolling")?.truthy() {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "options")?;
                                let key = "pollingIntent";
                                let value = V::Bool(true);
                                cx.put(&object, &key, value)?
                            };
                        }
                    }
                    cx.define(
                        scope,
                        "response",
                        cx.await_value({
                            let function = cx.lookup(scope, "httpRequest")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "authClient")?);
                                argv.push(cx.lookup(scope, "options")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        })
                        .await?,
                    );
                    return Ok(Completion::Return({
                        let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                        let method = "makeIdxResponse";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let object = cx.object();
                                cx.spread(&object, &cx.lookup(scope, "response")?)?;
                                object
                            });
                            argv.push(cx.lookup(scope, "toPersist")?);
                            argv.push(V::Bool(true));
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    }));
                }
                Ok::<Completion, V>(Completion::Normal)
            })
            .await;
            if let Err(error) = completion {
                completion = (async {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "err", error);
                    {
                        let scope = cx.scope(Some(scope));
                        if {
                            let left = V::Bool(
                                !cx.relation(
                                    "instanceof",
                                    cx.lookup(scope, "err")?,
                                    cx.lookup(scope, "AuthApiError")?,
                                )?
                                .truthy(),
                            );
                            if left.truthy() {
                                left
                            } else {
                                V::Bool(
                                    !'chain255: {
                                        {
                                            let object = cx.lookup(scope, "err")?;
                                            if object.nullish() {
                                                break 'chain255 V::Undefined;
                                            }
                                            cx.get(&object, &"xhr")?
                                        }
                                    }
                                    .truthy(),
                                )
                            }
                        }
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                return Err(cx.lookup(scope, "err")?);
                            }
                        }
                        cx.define(
                            scope,
                            "response",
                            cx.get(&cx.lookup(scope, "err")?, &"xhr")?,
                        );
                        cx.define(scope, "payload", {
                            let left = cx.get(&cx.lookup(scope, "response")?, &"responseJSON")?;
                            if left.truthy() {
                                left
                            } else {
                                {
                                    let receiver = cx.lookup(scope, "JSON")?;
                                    let method = "parse";
                                    let function = cx.resolve_method(&receiver, &method)?;
                                    let argv = {
                                        let mut argv = Vec::new();
                                        argv.push(cx.get(
                                            &cx.lookup(scope, "response")?,
                                            &"responseText",
                                        )?);
                                        argv
                                    };
                                    cx.call(function, receiver, argv)?
                                }
                            }
                        });
                        cx.define(scope, "wwwAuthHeader", {
                            let left = cx.get(
                                &cx.get(&cx.lookup(scope, "response")?, &"headers")?,
                                &V::string("WWW-Authenticate").text(),
                            )?;
                            if left.truthy() {
                                left
                            } else {
                                cx.get(
                                    &cx.get(&cx.lookup(scope, "response")?, &"headers")?,
                                    &V::string("www-authenticate").text(),
                                )?
                            }
                        });
                        cx.define(scope, "idxResponse", {
                            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
                            let method = "makeIdxResponse";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push({
                                    let object = cx.object();
                                    cx.spread(&object, &cx.lookup(scope, "payload")?)?;
                                    object
                                });
                                argv.push(cx.lookup(scope, "toPersist")?);
                                argv.push(V::Bool(false));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        });
                        if {
                            let left = binary(
                                "===",
                                cx.get(&cx.lookup(scope, "response")?, &"status")?,
                                V::Number(401f64),
                            )?;
                            if left.truthy() {
                                binary(
                                    "===",
                                    cx.lookup(scope, "wwwAuthHeader")?,
                                    V::string("Oktadevicejwt realm=\"Okta Device\""),
                                )?
                            } else {
                                left
                            }
                        }
                        .truthy()
                        {
                            {
                                let scope = cx.scope(Some(scope));
                                let _ = {
                                    let object = cx.lookup(scope, "idxResponse")?;
                                    let key = "stepUp";
                                    let value = V::Bool(true);
                                    cx.put(&object, &key, value)?
                                };
                            }
                        }
                        return Ok(Completion::Return(cx.lookup(scope, "idxResponse")?));
                    }
                    Ok::<Completion, V>(Completion::Normal)
                })
                .await;
            }
            match completion? {
                Completion::Normal => {}
                Completion::Return(value) => {
                    return Ok(value);
                }
                Completion::Break => {
                    return Err(V::string("unexpected break"));
                }
                Completion::Continue => {
                    return Err(V::string("unexpected continue"));
                }
            }
        }
        Ok(V::Undefined)
    })
}

fn f251(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let binding = args.get(1).cloned().unwrap_or(V::Undefined);
        cx.define(
            scope,
            "actionDefinition",
            cx.get(&binding, &"actionDefinition")?,
        );
        {
            let value = cx.get(&binding, &"defaultParamsForAction")?;
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "defaultParamsForAction", value);
        }
        {
            let value = cx.get(&binding, &"immutableParamsForAction")?;
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "immutableParamsForAction", value);
        }
        {
            let value = cx.get(&binding, &"toPersist")?;
            let value = if matches!(value, V::Undefined) {
                {
                    let object = cx.object();
                    object
                }
            } else {
                value
            };
            cx.define(scope, "toPersist", value);
        }
    }
    cx.define(
        scope,
        "target",
        cx.get(&cx.lookup(scope, "actionDefinition")?, &"href")?,
    );
    return Ok(cx.async_function(scope, f252));

    Ok(V::Undefined)
}

fn f256(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "actionDefinition",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "toPersist",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "generator", cx.lookup(scope, "generateDirectFetch")?);
    {
        let binding = {
            let function = cx.lookup(scope, "divideActionParamsByMutability")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "actionDefinition")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        cx.define(scope, "defaultParams", cx.get(&binding, &"defaultParams")?);
        cx.define(scope, "neededParams", cx.get(&binding, &"neededParams")?);
        cx.define(
            scope,
            "immutableParams",
            cx.get(&binding, &"immutableParams")?,
        );
    }
    cx.define(scope, "action", {
        let function = cx.lookup(scope, "generator")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push({
                let object = cx.object();
                cx.put(
                    &object,
                    &"actionDefinition",
                    cx.lookup(scope, "actionDefinition")?,
                )?;
                cx.put(
                    &object,
                    &"defaultParamsForAction",
                    cx.get(
                        &cx.lookup(scope, "defaultParams")?,
                        &cx.get(&cx.lookup(scope, "actionDefinition")?, &"name")?
                            .text(),
                    )?,
                )?;
                cx.put(
                    &object,
                    &"immutableParamsForAction",
                    cx.get(
                        &cx.lookup(scope, "immutableParams")?,
                        &cx.get(&cx.lookup(scope, "actionDefinition")?, &"name")?
                            .text(),
                    )?,
                )?;
                cx.put(&object, &"toPersist", cx.lookup(scope, "toPersist")?)?;
                object
            });
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    let _ = {
        let object = cx.lookup(scope, "action")?;
        let key = "neededParams";
        let value = cx.lookup(scope, "neededParams")?;
        cx.put(&object, &key, value)?
    };
    return Ok(cx.lookup(scope, "action")?);

    Ok(V::Undefined)
}

fn f258(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "obj", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "remediation",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "obj")?)?;
        cx.put(
            &object,
            &cx.get(&cx.lookup(scope, "remediation")?, &"name")?.text(),
            {
                let function = cx.lookup(scope, "generateIdxAction")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push(cx.lookup(scope, "remediation")?);
                    argv.push(cx.lookup(scope, "toPersist")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            },
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f257(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "remediationValue",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(2).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "toPersist", value);
    }
    return Ok({
        let receiver = cx.lookup(scope, "remediationValue")?;
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f258));
            argv.push({
                let object = cx.object();
                object
            });
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f259(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "remediation",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "toPersist",
        args.get(2).cloned().unwrap_or(V::Undefined),
    );
    if cx.get(&cx.lookup(scope, "remediation")?, &"rel")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "remediationActions", {
                let function = cx.lookup(scope, "generateRemediationFunctions")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authClient")?);
                    argv.push({
                        let mut values = vec![];
                        values.push(cx.lookup(scope, "remediation")?);
                        cx.array(values)
                    });
                    argv.push(cx.lookup(scope, "toPersist")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            cx.define(
                scope,
                "actionFn",
                cx.get(
                    &cx.lookup(scope, "remediationActions")?,
                    &cx.get(&cx.lookup(scope, "remediation")?, &"name")?.text(),
                )?,
            );
            return Ok({
                let object = cx.object();
                cx.spread(&object, &cx.lookup(scope, "remediation")?)?;
                cx.put(&object, &"action", cx.lookup(scope, "actionFn")?)?;
                object
            });
        }
    }
    return Ok(cx.lookup(scope, "remediation")?);

    Ok(V::Undefined)
}

fn f262(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "innerValue",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let function = cx.lookup(scope, "expandRelatesTo")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv.push(cx.lookup(scope, "innerValue")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f261(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "k", args.get(0).cloned().unwrap_or(V::Undefined));
    if binary("===", cx.lookup(scope, "k")?, V::string("relatesTo"))?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(
                scope,
                "query",
                if {
                    let receiver = cx.lookup(scope, "Array")?;
                    let method = "isArray";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(
                            cx.get(&cx.lookup(scope, "value")?, &cx.lookup(scope, "k")?.text())?,
                        );
                        argv
                    };
                    cx.call(function, receiver, argv)?
                }
                .truthy()
                {
                    cx.get(
                        &cx.get(&cx.lookup(scope, "value")?, &cx.lookup(scope, "k")?.text())?,
                        &V::Number(0f64).text(),
                    )?
                } else {
                    cx.get(&cx.lookup(scope, "value")?, &cx.lookup(scope, "k")?.text())?
                },
            );
            if binary(
                "===",
                V::string(cx.lookup(scope, "query")?.typeof_()),
                V::string("string"),
            )?
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    cx.define(scope, "result", {
                        let function = cx.lookup(scope, "jsonpath")?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push({
                                let object = cx.object();
                                cx.put(&object, &"path", cx.lookup(scope, "query")?)?;
                                cx.put(&object, &"json", cx.lookup(scope, "idxResponse")?)?;
                                object
                            });
                            argv
                        };
                        cx.call(function, V::Undefined, argv)?
                    });
                    if cx.lookup(scope, "result")?.truthy() {
                        {
                            let scope = cx.scope(Some(scope));
                            let _ = {
                                let object = cx.lookup(scope, "value")?;
                                let key = cx.lookup(scope, "k")?.text();
                                let value = cx.lookup(scope, "result")?;
                                cx.put(&object, &key, value)?
                            };
                            return Ok(V::Undefined);
                        }
                    } else {
                        {
                            let scope = cx.scope(Some(scope));
                            return Err(cx.construct(cx.lookup(scope, "AuthSdkError")?, {
                                let mut argv = Vec::new();
                                argv.push({
                                    let mut text = String::new();
                                    text.push_str("Cannot resolve relatesTo: ");
                                    text.push_str(&cx.lookup(scope, "query")?.text());
                                    text.push_str("");
                                    V::string(text)
                                });
                                argv
                            })?);
                        }
                    }
                }
            }
        }
    }
    if {
        let receiver = cx.lookup(scope, "Array")?;
        let method = "isArray";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "value")?, &cx.lookup(scope, "k")?.text())?);
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver =
                    cx.get(&cx.lookup(scope, "value")?, &cx.lookup(scope, "k")?.text())?;
                let method = "forEach";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f262));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f260(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "idxResponse",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "value", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = {
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "keys";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "value")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f261));
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f265(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "subField", cx.get(&binding, "0")?);
        cx.define(scope, "value", cx.get(&binding, "1")?);
    }
    if cx.get(&cx.lookup(scope, "value")?, &"rel")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "actions")?;
                let key = {
                    let mut text = String::new();
                    text.push_str("");
                    text.push_str(&cx.lookup(scope, "field")?.text());
                    text.push_str("-");
                    text.push_str(
                        &{
                            let left = cx.get(&cx.lookup(scope, "subField")?, &"name")?;
                            if left.truthy() {
                                left
                            } else {
                                cx.lookup(scope, "subField")?
                            }
                        }
                        .text(),
                    );
                    text.push_str("");
                    V::string(text)
                }
                .text();
                let value = {
                    let function = cx.lookup(scope, "generateIdxAction")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.lookup(scope, "value")?);
                        argv.push(cx.lookup(scope, "toPersist")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                cx.put(&object, &key, value)?
            };
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.get(
                    &cx.get(
                        &cx.lookup(scope, "context")?,
                        &cx.lookup(scope, "field")?.text(),
                    )?,
                    &"value",
                )?;
                let key = cx.lookup(scope, "subField")?.text();
                let value = cx.lookup(scope, "value")?;
                cx.put(&object, &key, value)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f264(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "field", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "fieldIsObject", {
        let left = binary(
            "===",
            V::string(
                cx.get(
                    &cx.lookup(scope, "idxResponse")?,
                    &cx.lookup(scope, "field")?.text(),
                )?
                .typeof_(),
            ),
            V::string("object"),
        )?;
        if left.truthy() {
            V::Bool(
                !V::Bool(
                    !cx.get(
                        &cx.lookup(scope, "idxResponse")?,
                        &cx.lookup(scope, "field")?.text(),
                    )?
                    .truthy(),
                )
                .truthy(),
            )
        } else {
            left
        }
    });
    if V::Bool(!cx.lookup(scope, "fieldIsObject")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "context")?;
                let key = cx.lookup(scope, "field")?.text();
                let value = cx.get(
                    &cx.lookup(scope, "idxResponse")?,
                    &cx.lookup(scope, "field")?.text(),
                )?;
                cx.put(&object, &key, value)?
            };
            return Ok(V::Undefined);
        }
    }
    if cx
        .get(
            &cx.get(
                &cx.lookup(scope, "idxResponse")?,
                &cx.lookup(scope, "field")?.text(),
            )?,
            &"rel",
        )?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "actions")?;
                let key = cx
                    .get(
                        &cx.get(
                            &cx.lookup(scope, "idxResponse")?,
                            &cx.lookup(scope, "field")?.text(),
                        )?,
                        &"name",
                    )?
                    .text();
                let value = {
                    let function = cx.lookup(scope, "generateIdxAction")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.lookup(scope, "authClient")?);
                        argv.push(cx.get(
                            &cx.lookup(scope, "idxResponse")?,
                            &cx.lookup(scope, "field")?.text(),
                        )?);
                        argv.push(cx.lookup(scope, "toPersist")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                };
                cx.put(&object, &key, value)?
            };
            return Ok(V::Undefined);
        }
    }
    {
        let binding = cx.get(
            &cx.lookup(scope, "idxResponse")?,
            &cx.lookup(scope, "field")?.text(),
        )?;
        cx.define(scope, "fieldValue", cx.get(&binding, &"value")?);
        cx.define(scope, "type", cx.get(&binding, &"type")?);
        cx.define(
            scope,
            "info",
            cx.rest(&binding, &vec!["value".to_string(), "type".to_string()])?,
        );
    }
    let _ = {
        let object = cx.lookup(scope, "context")?;
        let key = cx.lookup(scope, "field")?.text();
        let value = {
            let object = cx.object();
            cx.put(&object, &"type", cx.lookup(scope, "type")?)?;
            cx.spread(&object, &cx.lookup(scope, "info")?)?;
            object
        };
        cx.put(&object, &key, value)?
    };
    if binary("!==", cx.lookup(scope, "type")?, V::string("object"))?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.get(
                    &cx.lookup(scope, "context")?,
                    &cx.lookup(scope, "field")?.text(),
                )?;
                let key = "value";
                let value = cx.lookup(scope, "fieldValue")?;
                cx.put(&object, &key, value)?
            };
            return Ok(V::Undefined);
        }
    }
    let _ = {
        let object = cx.get(
            &cx.lookup(scope, "context")?,
            &cx.lookup(scope, "field")?.text(),
        )?;
        let key = "value";
        let value = {
            let object = cx.object();
            object
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "entries";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "fieldValue")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f265));
            argv
        };
        cx.call(function, receiver, argv)?
    };

    Ok(V::Undefined)
}

fn f266(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "field", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(V::Bool(
        !cx.get(
            &cx.lookup(scope, "SKIP_FIELDS")?,
            &cx.lookup(scope, "field")?.text(),
        )?
        .truthy(),
    ));

    Ok(V::Undefined)
}

fn f263(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(2).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "toPersist", value);
    }
    cx.define(scope, "actions", {
        let object = cx.object();
        object
    });
    cx.define(scope, "context", {
        let object = cx.object();
        object
    });
    let _ = {
        let receiver = {
            let receiver = {
                let receiver = cx.lookup(scope, "Object")?;
                let method = "keys";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "idxResponse")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "filter";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f266));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f264));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok({
        let object = cx.object();
        cx.put(&object, &"context", cx.lookup(scope, "context")?)?;
        cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f269(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = {
            let left = binary(
                "===",
                cx.get(&cx.lookup(scope, "remediation")?, &"name")?,
                V::string("launch-authenticator"),
            )?;
            if left.truthy() {
                binary(
                    "===",
                    'chain270: {
                        {
                            let object = {
                                let object = cx.lookup(scope, "remediation")?;
                                if object.nullish() {
                                    break 'chain270 V::Undefined;
                                }
                                cx.get(&object, &"relatesTo")?
                            };
                            if object.nullish() {
                                break 'chain270 V::Undefined;
                            }
                            cx.get(&object, &V::Number(0f64).text())?
                        }
                    },
                    V::string("authenticatorChallenge"),
                )?
            } else {
                left
            }
        };
        if left.truthy() {
            V::Bool(
                !'chain271: {
                    {
                        let object = cx.lookup(scope, "idxResponse")?;
                        if object.nullish() {
                            break 'chain271 V::Undefined;
                        }
                        cx.get(&object, &"authenticatorChallenge")?
                    }
                }
                .truthy(),
            )
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.delete(&cx.lookup(scope, "remediation")?, &"relatesTo")?;
            return Ok(V::Undefined);
        }
    }
    return Ok({
        let function = cx.lookup(scope, "expandRelatesTo")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "idxResponse")?);
            argv.push(cx.lookup(scope, "remediation")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f272(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let function = cx.lookup(scope, "convertRemediationAction")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authClient")?);
            argv.push(cx.lookup(scope, "remediation")?);
            argv.push(cx.lookup(scope, "toPersist")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f267(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "idxResponse",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(2).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "toPersist", value);
    }
    cx.define(scope, "remediationData", {
        let left = 'chain268: {
            {
                let object = cx.get(&cx.lookup(scope, "idxResponse")?, &"remediation")?;
                if object.nullish() {
                    break 'chain268 V::Undefined;
                }
                cx.get(&object, &"value")?
            }
        };
        if left.truthy() {
            left
        } else {
            {
                let mut values = vec![];
                cx.array(values)
            }
        }
    });
    let _ = {
        let receiver = cx.lookup(scope, "remediationData")?;
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f269));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    cx.define(scope, "remediations", {
        let receiver = cx.lookup(scope, "remediationData")?;
        let method = "map";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f272));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    {
        let binding = {
            let function = cx.lookup(scope, "parseNonRemediations")?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "authClient")?);
                argv.push(cx.lookup(scope, "idxResponse")?);
                argv.push(cx.lookup(scope, "toPersist")?);
                argv
            };
            cx.call(function, V::Undefined, argv)?
        };
        cx.define(scope, "context", cx.get(&binding, &"context")?);
        cx.define(scope, "actions", cx.get(&binding, &"actions")?);
    }
    return Ok({
        let object = cx.object();
        cx.put(&object, &"remediations", cx.lookup(scope, "remediations")?)?;
        cx.put(&object, &"context", cx.lookup(scope, "context")?)?;
        cx.put(&object, &"actions", cx.lookup(scope, "actions")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f273(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "values", value);
    }
    {
        let value = args.get(2).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let object = cx.object();
                object
            }
        } else {
            value
        };
        cx.define(scope, "options", value);
    }
    let _ = {
        let object = this.clone();
        let key = "values";
        let value = {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "values")?)?;
            object
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = this.clone();
        let key = "options";
        let value = {
            let object = cx.object();
            cx.spread(&object, &cx.lookup(scope, "options")?)?;
            object
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let receiver = this.clone();
        let method = "formatAuthenticators";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    };
    let _ = {
        let object = this.clone();
        let key = "remediation";
        let value = cx.lookup(scope, "remediation")?;
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f275(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "authenticator",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let function = cx.lookup(scope, "formatAuthenticator")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authenticator")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f276(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "existing",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let function = cx.lookup(scope, "compareAuthenticators")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "authenticator")?);
            argv.push(cx.lookup(scope, "existing")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });

    Ok(V::Undefined)
}

fn f277(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "acc", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(
        scope,
        "authenticator",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    if {
        let left = binary(
            "===",
            V::string(cx.lookup(scope, "authenticator")?.typeof_()),
            V::string("object"),
        )?;
        if left.truthy() {
            binary(
                ">",
                cx.get(
                    &{
                        let receiver = cx.lookup(scope, "Object")?;
                        let method = "keys";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "authenticator")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    },
                    &"length",
                )?,
                V::Number(1f64),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "acc")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authenticator")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }
    return Ok(cx.lookup(scope, "acc")?);

    Ok(V::Undefined)
}

fn f274(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    let _ = {
        let object = cx.get(&this.clone(), &"values")?;
        let key = "authenticators";
        let value = {
            let left = cx.get(&cx.get(&this.clone(), &"values")?, &"authenticators")?;
            if left.truthy() {
                left
            } else {
                {
                    let mut values = vec![];
                    cx.array(values)
                }
            }
        };
        cx.put(&object, &key, value)?
    };
    let _ = {
        let object = cx.get(&this.clone(), &"values")?;
        let key = "authenticators";
        let value = {
            let receiver = cx.get(&cx.get(&this.clone(), &"values")?, &"authenticators")?;
            let method = "map";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f275));
                argv
            };
            cx.call(function, receiver, argv)?
        };
        cx.put(&object, &key, value)?
    };
    if cx
        .get(&cx.get(&this.clone(), &"values")?, &"authenticator")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "authenticator", {
                let function = cx.lookup(scope, "formatAuthenticator")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.get(&this.clone(), &"values")?, &"authenticator")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            cx.define(scope, "hasAuthenticatorInList", {
                let receiver = cx.get(&cx.get(&this.clone(), &"values")?, &"authenticators")?;
                let method = "some";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f276));
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            if V::Bool(!cx.lookup(scope, "hasAuthenticatorInList")?.truthy()).truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let receiver =
                            cx.get(&cx.get(&this.clone(), &"values")?, &"authenticators")?;
                        let method = "push";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.lookup(scope, "authenticator")?);
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    };
                }
            }
        }
    }
    let _ = {
        let object = cx.get(&this.clone(), &"values")?;
        let key = "authenticatorsData";
        let value = {
            let receiver = cx.get(&cx.get(&this.clone(), &"values")?, &"authenticators")?;
            let method = "reduce";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f277));
                argv.push({
                    let left = cx.get(&cx.get(&this.clone(), &"values")?, &"authenticatorsData")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let mut values = vec![];
                            cx.array(values)
                        }
                    }
                });
                argv
            };
            cx.call(function, receiver, argv)?
        };
        cx.put(&object, &key, value)?
    };

    Ok(V::Undefined)
}

fn f278(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok(cx.get(&cx.get(&this.clone(), &"remediation")?, &"name")?);

    Ok(V::Undefined)
}

fn f280(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "key", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(V::Bool(
        !{
            let receiver = this.clone();
            let method = "hasData";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "key")?);
                argv
            };
            cx.call(function, receiver, argv)?
        }
        .truthy(),
    ));

    Ok(V::Undefined)
}

fn f279(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "context",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "required", {
        let function = cx.lookup(scope, "getRequiredValues")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&this.clone(), &"remediation")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "needed", {
        let receiver = cx.lookup(scope, "required")?;
        let method = "find";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f280));
            argv
        };
        cx.call(function, receiver, argv)?
    });
    if cx.lookup(scope, "needed")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    return Ok(V::Bool(true));

    Ok(V::Undefined)
}

fn f282(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "data", args.get(0).cloned().unwrap_or(V::Undefined));
    cx.define(scope, "key", args.get(1).cloned().unwrap_or(V::Undefined));
    let _ = {
        let object = cx.lookup(scope, "data")?;
        let key = cx.lookup(scope, "key")?.text();
        let value = {
            let receiver = this.clone();
            let method = "getData";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "key")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        cx.put(&object, &key, value)?
    };
    return Ok(cx.lookup(scope, "data")?);

    Ok(V::Undefined)
}

fn f283(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        cx.lookup(scope, "key")?,
    )?);

    Ok(V::Undefined)
}

fn f281(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "key", args.get(0).cloned().unwrap_or(V::Undefined));
    if V::Bool(!cx.lookup(scope, "key")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "allValues", {
                let function = cx.lookup(scope, "getAllValues")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&this.clone(), &"remediation")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            });
            cx.define(scope, "res", {
                let receiver = cx.lookup(scope, "allValues")?;
                let method = "reduce";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f282));
                    argv.push({
                        let object = cx.object();
                        object
                    });
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            return Ok(cx.lookup(scope, "res")?);
        }
    }
    if binary(
        "===",
        V::string(
            cx.get(
                &this.clone(),
                &{
                    let mut text = String::new();
                    text.push_str("map");
                    text.push_str(
                        &{
                            let function = cx.lookup(scope, "titleCase")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "key")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                        .text(),
                    );
                    text.push_str("");
                    V::string(text)
                }
                .text(),
            )?
            .typeof_(),
        ),
        V::string("function"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "val", {
                let receiver = this.clone();
                let method = {
                    let mut text = String::new();
                    text.push_str("map");
                    text.push_str(
                        &{
                            let function = cx.lookup(scope, "titleCase")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "key")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                        .text(),
                    );
                    text.push_str("");
                    V::string(text)
                }
                .text();
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push({
                        let receiver = cx.get(&cx.get(&this.clone(), &"remediation")?, &"value")?;
                        let method = "find";
                        let function = cx.resolve_method(&receiver, &method)?;
                        let argv = {
                            let mut argv = Vec::new();
                            argv.push(cx.arrow(scope, this.clone(), f283));
                            argv
                        };
                        cx.call(function, receiver, argv)?
                    });
                    argv
                };
                cx.call(function, receiver, argv)?
            });
            if cx.lookup(scope, "val")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    return Ok(cx.lookup(scope, "val")?);
                }
            }
        }
    }
    if {
        let left = cx.get(&this.clone(), &"map")?;
        if left.truthy() {
            cx.get(
                &cx.get(&this.clone(), &"map")?,
                &cx.lookup(scope, "key")?.text(),
            )?
        } else {
            left
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(
                scope,
                "entry",
                cx.get(
                    &cx.get(&this.clone(), &"map")?,
                    &cx.lookup(scope, "key")?.text(),
                )?,
            );
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "i", V::Number(0f64));
                let mut first = true;
                loop {
                    if !first {
                        let _ = {
                            let old = cx.lookup(scope, "i")?;
                            let updated =
                                cx.assign(scope, "i", binary("+", old.clone(), V::Number(1.0))?)?;
                            old
                        };
                    }
                    first = false;
                    if !binary(
                        "<",
                        cx.lookup(scope, "i")?,
                        cx.get(&cx.lookup(scope, "entry")?, &"length")?,
                    )?
                    .truthy()
                    {
                        break;
                    }
                    {
                        let scope = cx.scope(Some(scope));
                        cx.define(
                            scope,
                            "val",
                            cx.get(
                                &cx.get(&this.clone(), &"values")?,
                                &cx.get(
                                    &cx.lookup(scope, "entry")?,
                                    &cx.lookup(scope, "i")?.text(),
                                )?
                                .text(),
                            )?,
                        );
                        if cx.lookup(scope, "val")?.truthy() {
                            {
                                let scope = cx.scope(Some(scope));
                                return Ok(cx.lookup(scope, "val")?);
                            }
                        }
                    }
                }
            }
        }
    }
    return Ok(cx.get(
        &cx.get(&this.clone(), &"values")?,
        &cx.lookup(scope, "key")?.text(),
    )?);

    Ok(V::Undefined)
}

fn f284(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "key", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok(V::Bool(
        !V::Bool(
            !{
                let receiver = this.clone();
                let method = "getData";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "key")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy(),
        )
        .truthy(),
    ));

    Ok(V::Undefined)
}

fn f285(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "_authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "_context",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "name", {
        let receiver = this.clone();
        let method = "getName";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "inputs", {
        let receiver = this.clone();
        let method = "getInputs";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "authenticator", {
        let receiver = this.clone();
        let method = "getAuthenticator";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "type", 'chain286: {
        {
            let object = cx.lookup(scope, "authenticator")?;
            if object.nullish() {
                break 'chain286 V::Undefined;
            }
            cx.get(&object, &"type")?
        }
    });
    return Ok({
        let object = cx.object();
        cx.put(&object, &"name", cx.lookup(scope, "name")?)?;
        cx.put(&object, &"inputs", cx.lookup(scope, "inputs")?)?;
        cx.spread(&object, &{
            let left = cx.lookup(scope, "type")?;
            if left.truthy() {
                {
                    let object = cx.object();
                    cx.put(&object, &"type", cx.lookup(scope, "type")?)?;
                    object
                }
            } else {
                left
            }
        })?;
        cx.spread(&object, &{
            let left = cx.lookup(scope, "authenticator")?;
            if left.truthy() {
                {
                    let object = cx.object();
                    cx.put(
                        &object,
                        &"authenticator",
                        cx.lookup(scope, "authenticator")?,
                    )?;
                    object
                }
            } else {
                left
            }
        })?;
        object
    });

    Ok(V::Undefined)
}

fn f289(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "name", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = {
            let receiver = cx.lookup(scope, "Object")?;
            let method = "keys";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.get(&this.clone(), &"values")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "includes";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "name")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f290(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "i", args.get(0).cloned().unwrap_or(V::Undefined));
    return Ok({
        let receiver = cx.lookup(scope, "inputs")?;
        let method = "push";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "i")?);
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f288(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "inputFromRemediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "input", V::Undefined);
    {
        let binding = cx.lookup(scope, "inputFromRemediation")?;
        cx.define(scope, "name", cx.get(&binding, &"name")?);
        cx.define(scope, "type", cx.get(&binding, &"type")?);
        cx.define(scope, "visible", cx.get(&binding, &"visible")?);
        cx.define(scope, "messages", cx.get(&binding, &"messages")?);
    }
    if binary("===", cx.lookup(scope, "visible")?, V::Bool(false))?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    if binary(
        "===",
        V::string(
            cx.get(
                &this.clone(),
                &{
                    let mut text = String::new();
                    text.push_str("getInput");
                    text.push_str(
                        &{
                            let function = cx.lookup(scope, "titleCase")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "name")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                        .text(),
                    );
                    text.push_str("");
                    V::string(text)
                }
                .text(),
            )?
            .typeof_(),
        ),
        V::string("function"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "input", {
                let receiver = this.clone();
                let method = {
                    let mut text = String::new();
                    text.push_str("getInput");
                    text.push_str(
                        &{
                            let function = cx.lookup(scope, "titleCase")?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "name")?);
                                argv
                            };
                            cx.call(function, V::Undefined, argv)?
                        }
                        .text(),
                    );
                    text.push_str("");
                    V::string(text)
                }
                .text();
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "inputFromRemediation")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            })?;
        }
    } else {
        if binary("!==", cx.lookup(scope, "type")?, V::string("object"))?.truthy() {
            {
                let scope = cx.scope(Some(scope));
                cx.define(scope, "alias", V::Undefined);
                cx.define(scope, "aliases", {
                    let left = if cx.get(&this.clone(), &"map")?.truthy() {
                        cx.get(
                            &cx.get(&this.clone(), &"map")?,
                            &cx.lookup(scope, "name")?.text(),
                        )?
                    } else {
                        V::Null
                    };
                    if left.truthy() {
                        left
                    } else {
                        {
                            let mut values = vec![];
                            cx.array(values)
                        }
                    }
                });
                if binary(
                    "===",
                    cx.get(&cx.lookup(scope, "aliases")?, &"length")?,
                    V::Number(1f64),
                )?
                .truthy()
                {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(
                            scope,
                            "alias",
                            cx.get(&cx.lookup(scope, "aliases")?, &V::Number(0f64).text())?,
                        )?;
                    }
                } else {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "alias", {
                            let receiver = cx.lookup(scope, "aliases")?;
                            let method = "find";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.arrow(scope, this.clone(), f289));
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        })?;
                    }
                }
                if cx.lookup(scope, "alias")?.truthy() {
                    {
                        let scope = cx.scope(Some(scope));
                        let _ = cx.assign(scope, "input", {
                            let object = cx.object();
                            cx.spread(&object, &cx.lookup(scope, "inputFromRemediation")?)?;
                            cx.put(&object, &"name", cx.lookup(scope, "alias")?)?;
                            object
                        })?;
                    }
                }
            }
        }
    }
    if V::Bool(!cx.lookup(scope, "input")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "input", cx.lookup(scope, "inputFromRemediation")?)?;
        }
    }
    if {
        let receiver = cx.lookup(scope, "Array")?;
        let method = "isArray";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.lookup(scope, "input")?);
            argv
        };
        cx.call(function, receiver, argv)?
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let receiver = cx.lookup(scope, "input")?;
                let method = "forEach";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f290));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    } else {
        {
            let scope = cx.scope(Some(scope));
            if cx.lookup(scope, "messages")?.truthy() {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "input")?;
                        let key = "messages";
                        let value = cx.lookup(scope, "messages")?;
                        cx.put(&object, &key, value)?
                    };
                }
            }
            let _ = {
                let receiver = cx.lookup(scope, "inputs")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "input")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f287(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "inputs", {
        let mut values = vec![];
        cx.array(values)
    });
    cx.define(scope, "inputsFromRemediation", {
        let left = cx.get(&cx.get(&this.clone(), &"remediation")?, &"value")?;
        if left.truthy() {
            left
        } else {
            {
                let mut values = vec![];
                cx.array(values)
            }
        }
    });
    let _ = {
        let receiver = cx.lookup(scope, "inputsFromRemediation")?;
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f288));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok(cx.lookup(scope, "inputs")?);

    Ok(V::Undefined)
}

fn f293(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "messages",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "field", args.get(1).cloned().unwrap_or(V::Undefined));
    if cx.get(&cx.lookup(scope, "field")?, &"messages")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.assign(scope, "messages", {
                let mut values = vec![];
                values.extend(cx.items(&cx.lookup(scope, "messages")?)?);
                values.extend(cx.items(
                    &cx.get(&cx.get(&cx.lookup(scope, "field")?, &"messages")?, &"value")?,
                )?);
                cx.array(values)
            })?;
        }
    }
    return Ok(cx.lookup(scope, "messages")?);

    Ok(V::Undefined)
}

fn f291(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "remediation",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if V::Bool(
        !cx.get(&cx.lookup(scope, "remediation")?, &"value")?
            .truthy(),
    )
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    return Ok('chain292: {
        {
            let receiver = {
                let object = {
                    let object = cx.get(
                        &cx.get(&cx.lookup(scope, "remediation")?, &"value")?,
                        &V::Number(0f64).text(),
                    )?;
                    if object.nullish() {
                        break 'chain292 V::Undefined;
                    }
                    cx.get(&object, &"form")?
                };
                if object.nullish() {
                    break 'chain292 V::Undefined;
                }
                cx.get(&object, &"value")?
            };
            let method = "reduce";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.arrow(scope, this.clone(), f293));
                argv.push({
                    let mut values = vec![];
                    cx.array(values)
                });
                argv
            };
            cx.call(function, receiver, argv)?
        }
    });

    Ok(V::Undefined)
}

fn f294(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "inputsFromRemediation", {
        let left = cx.get(&cx.get(&this.clone(), &"remediation")?, &"value")?;
        if left.truthy() {
            left
        } else {
            {
                let mut values = vec![];
                cx.array(values)
            }
        }
    });
    cx.define(scope, "inputsFromRemediator", {
        let receiver = this.clone();
        let method = "getInputs";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "inputs", {
        let mut values = vec![];
        values.extend(cx.items(&cx.lookup(scope, "inputsFromRemediation")?)?);
        values.extend(cx.items(&cx.lookup(scope, "inputsFromRemediator")?)?);
        cx.array(values)
    });
    for item in cx.items(&cx.lookup(scope, "inputs")?)? {
        let scope = cx.scope(Some(scope));
        cx.define(scope, "input", item);
        {
            let scope = cx.scope(Some(scope));
            let _ = cx.delete(
                &cx.get(&this.clone(), &"values")?,
                &cx.get(&cx.lookup(scope, "input")?, &"name")?.text(),
            )?;
        }
    }
    return Ok(cx.get(&this.clone(), &"values")?);

    Ok(V::Undefined)
}

fn f297(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary("===", cx.lookup(scope, "name")?, V::string("id"))?);

    Ok(V::Undefined)
}

fn f299(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "===",
        cx.lookup(scope, "name")?,
        V::string("enrollmentId"),
    )?);

    Ok(V::Undefined)
}

fn f295(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "relatesTo", 'chain296: {
        {
            let object = cx.get(&cx.get(&this.clone(), &"remediation")?, &"relatesTo")?;
            if object.nullish() {
                break 'chain296 V::Undefined;
            }
            cx.get(&object, &"value")?
        }
    });
    if V::Bool(!cx.lookup(scope, "relatesTo")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Undefined);
        }
    }
    cx.define(scope, "authenticatorFromRemediation", {
        let function = cx.lookup(scope, "getAuthenticatorFromRemediation")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&this.clone(), &"remediation")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    if V::Bool(!cx.lookup(scope, "authenticatorFromRemediation")?.truthy()).truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok(cx.lookup(scope, "relatesTo")?);
        }
    }
    cx.define(
        scope,
        "id",
        cx.get(
            &{
                let receiver = cx.get(
                    &cx.get(&cx.lookup(scope, "authenticatorFromRemediation")?, &"form")?,
                    &"value",
                )?;
                let method = "find";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f297));
                    argv
                };
                cx.call(function, receiver, argv)?
            },
            &"value",
        )?,
    );
    cx.define(scope, "enrollmentId", 'chain298: {
        {
            let object = {
                let receiver = cx.get(
                    &cx.get(&cx.lookup(scope, "authenticatorFromRemediation")?, &"form")?,
                    &"value",
                )?;
                let method = "find";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f299));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            if object.nullish() {
                break 'chain298 V::Undefined;
            }
            cx.get(&object, &"value")?
        }
    });
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.lookup(scope, "relatesTo")?)?;
        cx.put(&object, &"id", cx.lookup(scope, "id")?)?;
        cx.put(&object, &"enrollmentId", cx.lookup(scope, "enrollmentId")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f300(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "args",
        cx.array(args.iter().skip(0).cloned().collect()),
    );
    let _ = cx.call(cx.lookup(scope, "__super")?, this.clone(), {
        let mut argv = Vec::new();
        argv.extend(cx.items(&cx.lookup(scope, "args")?)?);
        argv
    })?;

    Ok(V::Undefined)
}

fn f301(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    if binary(
        "!==",
        V::string(
            cx.get(&cx.get(&this.clone(), &"remediation")?, &"action")?
                .typeof_(),
        ),
        V::string("function"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(false));
        }
    }
    if {
        let left = binary(
            "===",
            cx.get(&cx.get(&this.clone(), &"remediation")?, &"name")?,
            V::string("poll"),
        )?;
        if left.truthy() {
            left
        } else {
            {
                let receiver = cx.get(&cx.get(&this.clone(), &"remediation")?, &"name")?;
                let method = "endsWith";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(V::string("-poll"));
                    argv
                };
                cx.call(function, receiver, argv)?
            }
        }
    }
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(true));
        }
    }
    if cx
        .get(&cx.get(&this.clone(), &"options")?, &"step")?
        .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            return Ok(V::Bool(true));
        }
    }
    return Ok(V::Bool(false));

    Ok(V::Undefined)
}

fn f303(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "acc", args.get(0).cloned().unwrap_or(V::Undefined));
    {
        let binding = args.get(1).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    let _ = {
        let object = cx.lookup(scope, "acc")?;
        let key = cx.lookup(scope, "name")?.text();
        let value = cx.get(
            &cx.get(&this.clone(), &"values")?,
            &cx.lookup(scope, "name")?.text(),
        )?;
        cx.put(&object, &key, value)?
    };
    return Ok(cx.lookup(scope, "acc")?);

    Ok(V::Undefined)
}

fn f302(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(scope, "data", {
        let receiver = {
            let receiver = this.clone();
            let method = "getInputs";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "reduce";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f303));
            argv.push({
                let object = cx.object();
                object
            });
            argv
        };
        cx.call(function, receiver, argv)?
    });
    return Ok(cx.lookup(scope, "data")?);

    Ok(V::Undefined)
}

fn f305(engine: Engine, parent: usize, this: V, args: Vec<V>) -> okta_native_support::JFuture {
    Box::pin(async move {
        let cx = &engine;
        let scope = cx.scope(Some(parent));
        cx.define(
            scope,
            "params",
            args.get(0).cloned().unwrap_or(V::Undefined),
        );
        return Ok({
            let receiver = cx.get(&cx.lookup(scope, "authClient")?, &"idx")?;
            let method = "proceed";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push({
                    let object = cx.object();
                    cx.put(&object, &"step", cx.lookup(scope, "name")?)?;
                    cx.spread(&object, &cx.lookup(scope, "params")?)?;
                    object
                });
                argv
            };
            cx.call(function, receiver, argv)?
        });
        Ok(V::Undefined)
    })
}

fn f304(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "authClient",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "_context",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    cx.define(scope, "name", {
        let receiver = this.clone();
        let method = "getName";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    cx.define(scope, "inputs", {
        let receiver = this.clone();
        let method = "getInputs";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv
        };
        cx.call(function, receiver, argv)?
    });
    {
        let binding = cx.get(&this.clone(), &"remediation")?;
        cx.define(scope, "href", cx.get(&binding, &"href")?);
        cx.define(scope, "method", cx.get(&binding, &"method")?);
        cx.define(scope, "rel", cx.get(&binding, &"rel")?);
        cx.define(scope, "accepts", cx.get(&binding, &"accepts")?);
        cx.define(scope, "produces", cx.get(&binding, &"produces")?);
        cx.define(scope, "value", cx.get(&binding, &"value")?);
        cx.define(scope, "action", cx.get(&binding, &"action")?);
        cx.define(
            scope,
            "rest",
            cx.rest(
                &binding,
                &vec![
                    "href".to_string(),
                    "method".to_string(),
                    "rel".to_string(),
                    "accepts".to_string(),
                    "produces".to_string(),
                    "value".to_string(),
                    "action".to_string(),
                ],
            )?,
        );
    }
    if cx.lookup(scope, "action")?.truthy() {
        {
            let scope = cx.scope(Some(scope));
            return Ok({
                let object = cx.object();
                cx.spread(&object, &cx.lookup(scope, "rest")?)?;
                cx.spread(&object, &{
                    let left = V::Bool(
                        !V::Bool(!cx.get(&cx.lookup(scope, "inputs")?, &"length")?.truthy())
                            .truthy(),
                    );
                    if left.truthy() {
                        {
                            let object = cx.object();
                            cx.put(&object, &"inputs", cx.lookup(scope, "inputs")?)?;
                            object
                        }
                    } else {
                        left
                    }
                })?;
                cx.put(
                    &object,
                    &"action",
                    cx.async_arrow(scope, this.clone(), f305),
                )?;
                object
            });
        }
    }
    return Ok({
        let object = cx.object();
        cx.spread(&object, &cx.get(&this.clone(), &"remediation")?)?;
        object
    });

    Ok(V::Undefined)
}

fn f307(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "input", args.get(0).cloned().unwrap_or(V::Undefined));
    let _ = {
        let object = cx.lookup(scope, "input")?;
        let key = "type";
        let value = {
            let left = cx.get(&cx.lookup(scope, "input")?, &"type")?;
            if left.truthy() {
                left
            } else {
                V::string("string")
            }
        };
        cx.put(&object, &key, value)?
    };
    return Ok(cx.lookup(scope, "input")?);

    Ok(V::Undefined)
}

fn f308(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let binding = args.get(0).cloned().unwrap_or(V::Undefined);
        cx.define(scope, "name", cx.get(&binding, &"name")?);
    }
    return Ok(binary(
        "!==",
        cx.lookup(scope, "name")?,
        V::string("stateHandle"),
    )?);

    Ok(V::Undefined)
}

fn f306(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    return Ok({
        let receiver = {
            let receiver = {
                let receiver = {
                    let left = cx.get(&cx.get(&this.clone(), &"remediation")?, &"value")?;
                    if left.truthy() {
                        left
                    } else {
                        {
                            let mut values = vec![];
                            cx.array(values)
                        }
                    }
                };
                let method = "filter";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.arrow(scope, this.clone(), f308));
                    argv
                };
                cx.call(function, receiver, argv)?
            };
            let method = "map";
            let function = cx.resolve_method(&receiver, &method)?;
            let argv = {
                let mut argv = Vec::new();
                argv.push(cx.lookup(scope, "unwrapFormValue")?);
                argv
            };
            cx.call(function, receiver, argv)?
        };
        let method = "map";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f307));
            argv
        };
        cx.call(function, receiver, argv)?
    });

    Ok(V::Undefined)
}

fn f309(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(scope, "arguments", cx.array(args.clone()));
    cx.define(
        scope,
        "_oktaAuth",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    {
        let value = args.get(1).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            V::string("default")
        } else {
            value
        };
        cx.define(scope, "_flow", value);
    }
    return Ok({
        let object = cx.object();
        cx.put(&object, &"remediators", {
            let object = cx.object();
            object
        })?;
        object
    });

    Ok(V::Undefined)
}

fn f311(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "enrollement",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    if binary(
        "===",
        cx.get(&cx.lookup(scope, "enrollement")?, &"key")?,
        V::string("webauthn"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            cx.define(scope, "credential", {
                let object = cx.object();
                cx.put(&object, &"type", V::string("public-key"))?;
                cx.put(&object, &"id", {
                    let function = cx.lookup(scope, "base64UrlToBuffer")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.get(&cx.lookup(scope, "enrollement")?, &"credentialId")?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                object
            });
            cx.define(scope, "transports", {
                let left = cx.get(&cx.lookup(scope, "enrollement")?, &"transports")?;
                if left.nullish() {
                    'chain312: {
                        {
                            let object = cx.get(&cx.lookup(scope, "enrollement")?, &"profile")?;
                            if object.nullish() {
                                break 'chain312 V::Undefined;
                            }
                            cx.get(&object, &"transports")?
                        }
                    }
                } else {
                    left
                }
            });
            if {
                let receiver = cx.lookup(scope, "Array")?;
                let method = "isArray";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "transports")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            }
            .truthy()
            {
                {
                    let scope = cx.scope(Some(scope));
                    let _ = {
                        let object = cx.lookup(scope, "credential")?;
                        let key = "transports";
                        let value = cx.lookup(scope, "transports")?;
                        cx.put(&object, &key, value)?
                    };
                }
            }
            let _ = {
                let receiver = cx.lookup(scope, "credentials")?;
                let method = "push";
                let function = cx.resolve_method(&receiver, &method)?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "credential")?);
                    argv
                };
                cx.call(function, receiver, argv)?
            };
        }
    }

    Ok(V::Undefined)
}

fn f310(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    {
        let value = args.get(0).cloned().unwrap_or(V::Undefined);
        let value = if matches!(value, V::Undefined) {
            {
                let mut values = vec![];
                cx.array(values)
            }
        } else {
            value
        };
        cx.define(scope, "authenticatorEnrollments", value);
    }
    cx.define(scope, "credentials", {
        let mut values = vec![];
        cx.array(values)
    });
    let _ = {
        let receiver = cx.lookup(scope, "authenticatorEnrollments")?;
        let method = "forEach";
        let function = cx.resolve_method(&receiver, &method)?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.arrow(scope, this.clone(), f311));
            argv
        };
        cx.call(function, receiver, argv)?
    };
    return Ok(cx.lookup(scope, "credentials")?);

    Ok(V::Undefined)
}

fn f313(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "activationData",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "authenticatorEnrollments",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let object = cx.object();
        cx.put(&object, &"publicKey", {
            let object = cx.object();
            cx.put(
                &object,
                &"rp",
                cx.get(&cx.lookup(scope, "activationData")?, &"rp")?,
            )?;
            cx.put(&object, &"user", {
                let object = cx.object();
                cx.put(&object, &"id", {
                    let function = cx.lookup(scope, "base64UrlToBuffer")?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push(cx.get(
                            &cx.get(&cx.lookup(scope, "activationData")?, &"user")?,
                            &"id",
                        )?);
                        argv
                    };
                    cx.call(function, V::Undefined, argv)?
                })?;
                cx.put(
                    &object,
                    &"name",
                    cx.get(
                        &cx.get(&cx.lookup(scope, "activationData")?, &"user")?,
                        &"name",
                    )?,
                )?;
                cx.put(
                    &object,
                    &"displayName",
                    cx.get(
                        &cx.get(&cx.lookup(scope, "activationData")?, &"user")?,
                        &"displayName",
                    )?,
                )?;
                object
            })?;
            cx.put(&object, &"challenge", {
                let function = cx.lookup(scope, "base64UrlToBuffer")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "activationData")?, &"challenge")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            cx.put(
                &object,
                &"pubKeyCredParams",
                cx.get(&cx.lookup(scope, "activationData")?, &"pubKeyCredParams")?,
            )?;
            cx.put(
                &object,
                &"attestation",
                cx.get(&cx.lookup(scope, "activationData")?, &"attestation")?,
            )?;
            cx.put(
                &object,
                &"authenticatorSelection",
                cx.get(
                    &cx.lookup(scope, "activationData")?,
                    &"authenticatorSelection",
                )?,
            )?;
            cx.put(&object, &"excludeCredentials", {
                let function = cx.lookup(scope, "getEnrolledCredentials")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authenticatorEnrollments")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            cx.spread(&object, &{
                let left = cx.get(&cx.lookup(scope, "activationData")?, &"hints")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"hints",
                            cx.get(&cx.lookup(scope, "activationData")?, &"hints")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            object
        })?;
        object
    });

    Ok(V::Undefined)
}

fn f314(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "challengeData",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "authenticatorEnrollments",
        args.get(1).cloned().unwrap_or(V::Undefined),
    );
    return Ok({
        let object = cx.object();
        cx.put(&object, &"publicKey", {
            let object = cx.object();
            cx.put(&object, &"challenge", {
                let function = cx.lookup(scope, "base64UrlToBuffer")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.get(&cx.lookup(scope, "challengeData")?, &"challenge")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            cx.put(
                &object,
                &"userVerification",
                cx.get(&cx.lookup(scope, "challengeData")?, &"userVerification")?,
            )?;
            cx.put(&object, &"allowCredentials", {
                let function = cx.lookup(scope, "getEnrolledCredentials")?;
                let argv = {
                    let mut argv = Vec::new();
                    argv.push(cx.lookup(scope, "authenticatorEnrollments")?);
                    argv
                };
                cx.call(function, V::Undefined, argv)?
            })?;
            cx.spread(&object, &{
                let left = cx.get(&cx.lookup(scope, "challengeData")?, &"rpId")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"rpId",
                            cx.get(&cx.lookup(scope, "challengeData")?, &"rpId")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            cx.spread(&object, &{
                let left = cx.get(&cx.lookup(scope, "challengeData")?, &"hints")?;
                if left.truthy() {
                    {
                        let object = cx.object();
                        cx.put(
                            &object,
                            &"hints",
                            cx.get(&cx.lookup(scope, "challengeData")?, &"hints")?,
                        )?;
                        object
                    }
                } else {
                    left
                }
            })?;
            object
        })?;
        object
    });

    Ok(V::Undefined)
}

fn f315(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "credential",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "response",
        cx.get(&cx.lookup(scope, "credential")?, &"response")?,
    );
    cx.define(
        scope,
        "id",
        cx.get(&cx.lookup(scope, "credential")?, &"id")?,
    );
    cx.define(scope, "clientData", {
        let function = cx.lookup(scope, "bufferToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "response")?, &"clientDataJSON")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "authenticatorData", {
        let function = cx.lookup(scope, "bufferToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "response")?, &"authenticatorData")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "signatureData", {
        let function = cx.lookup(scope, "bufferToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "response")?, &"signature")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    return Ok({
        let object = cx.object();
        cx.put(&object, &"id", cx.lookup(scope, "id")?)?;
        cx.put(&object, &"clientData", cx.lookup(scope, "clientData")?)?;
        cx.put(
            &object,
            &"authenticatorData",
            cx.lookup(scope, "authenticatorData")?,
        )?;
        cx.put(
            &object,
            &"signatureData",
            cx.lookup(scope, "signatureData")?,
        )?;
        object
    });

    Ok(V::Undefined)
}

fn f316(cx: &Engine, parent: usize, this: V, args: Vec<V>) -> JResult {
    let scope = cx.scope(Some(parent));
    cx.define(
        scope,
        "credential",
        args.get(0).cloned().unwrap_or(V::Undefined),
    );
    cx.define(
        scope,
        "response",
        cx.get(&cx.lookup(scope, "credential")?, &"response")?,
    );
    cx.define(
        scope,
        "id",
        cx.get(&cx.lookup(scope, "credential")?, &"id")?,
    );
    cx.define(scope, "clientData", {
        let function = cx.lookup(scope, "bufferToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "response")?, &"clientDataJSON")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(scope, "attestation", {
        let function = cx.lookup(scope, "bufferToBase64Url")?;
        let argv = {
            let mut argv = Vec::new();
            argv.push(cx.get(&cx.lookup(scope, "response")?, &"attestationObject")?);
            argv
        };
        cx.call(function, V::Undefined, argv)?
    });
    cx.define(
        scope,
        "getTransportsFn",
        cx.get(&cx.lookup(scope, "response")?, &"getTransports")?,
    );
    cx.define(scope, "result", {
        let object = cx.object();
        cx.put(&object, &"id", cx.lookup(scope, "id")?)?;
        cx.put(&object, &"clientData", cx.lookup(scope, "clientData")?)?;
        cx.put(&object, &"attestation", cx.lookup(scope, "attestation")?)?;
        object
    });
    if binary(
        "===",
        V::string(cx.lookup(scope, "getTransportsFn")?.typeof_()),
        V::string("function"),
    )?
    .truthy()
    {
        {
            let scope = cx.scope(Some(scope));
            let _ = {
                let object = cx.lookup(scope, "result")?;
                let key = "transports";
                let value = {
                    let receiver = cx.lookup(scope, "JSON")?;
                    let method = "stringify";
                    let function = cx.resolve_method(&receiver, &method)?;
                    let argv = {
                        let mut argv = Vec::new();
                        argv.push({
                            let receiver = cx.lookup(scope, "getTransportsFn")?;
                            let method = "call";
                            let function = cx.resolve_method(&receiver, &method)?;
                            let argv = {
                                let mut argv = Vec::new();
                                argv.push(cx.lookup(scope, "response")?);
                                argv
                            };
                            cx.call(function, receiver, argv)?
                        });
                        argv
                    };
                    cx.call(function, receiver, argv)?
                };
                cx.put(&object, &key, value)?
            };
        }
    }
    return Ok(cx.lookup(scope, "result")?);

    Ok(V::Undefined)
}

pub fn initialize(cx: &Engine) -> Result<std::collections::BTreeMap<String, V>, V> {
    let this = V::Undefined;
    let scopes: Vec<_> = (0..132).map(|_| cx.scope(None)).collect();
    cx.link(scopes[1], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[3],
        "stringToBase64Url",
        scopes[1],
        "stringToBase64Url",
    );
    cx.link(scopes[5], "base64UrlDecode", scopes[1], "base64UrlDecode");
    cx.link(scopes[5], "clone", scopes[129], "clone");
    cx.link(scopes[5], "stringToBuffer", scopes[1], "stringToBuffer");
    cx.link(scopes[6], "CustomError", scopes[8], "default");
    cx.link(scopes[7], "CustomError", scopes[8], "default");
    cx.link(scopes[9], "CustomError", scopes[8], "default");
    cx.link(scopes[10], "CustomError", scopes[8], "default");
    cx.link(scopes[10], "isFunction", scopes[130], "isFunction");
    cx.link(scopes[11], "AuthApiError", scopes[6], "default");
    cx.link(scopes[11], "OAuthError", scopes[9], "default");
    cx.link(scopes[11], "WWWAuthError", scopes[10], "default");
    cx.link(scopes[13], "default", scopes[13], "fetchRequest");
    cx.link(scopes[19], "AuthApiError", scopes[6], "default");
    cx.link(
        scopes[19],
        "DEFAULT_CACHE_DURATION",
        scopes[0],
        "DEFAULT_CACHE_DURATION",
    );
    cx.link(scopes[19], "OAuthError", scopes[9], "default");
    cx.link(
        scopes[19],
        "STATE_TOKEN_KEY_NAME",
        scopes[0],
        "STATE_TOKEN_KEY_NAME",
    );
    cx.link(scopes[19], "WWWAuthError", scopes[10], "default");
    cx.link(scopes[19], "clone", scopes[129], "clone");
    cx.link(scopes[19], "isAbsoluteUrl", scopes[131], "isAbsoluteUrl");
    cx.link(scopes[19], "isString", scopes[130], "isString");
    cx.link(scopes[19], "removeNils", scopes[129], "removeNils");
    cx.link(
        scopes[21],
        "createTransactionManager",
        scopes[73],
        "createTransactionManager",
    );
    cx.link(
        scopes[21],
        "isRawIdxResponse",
        scopes[65],
        "isRawIdxResponse",
    );
    cx.link(scopes[22], "isAuthenticator", scopes[64], "isAuthenticator");
    cx.link(scopes[23], "CustomError", scopes[8], "default");
    cx.link(
        scopes[23],
        "urlParamsToObject",
        scopes[120],
        "urlParamsToObject",
    );
    cx.link(scopes[24], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[24], "OAuthError", scopes[9], "default");
    cx.link(scopes[25], "IDX_API_VERSION", scopes[0], "IDX_API_VERSION");
    cx.link(scopes[25], "v1", scopes[30], "default");
    cx.link(scopes[27], "AuthApiError", scopes[6], "default");
    cx.link(
        scopes[27],
        "divideActionParamsByMutability",
        scopes[26],
        "divideActionParamsByMutability",
    );
    cx.link(scopes[27], "httpRequest", scopes[19], "httpRequest");
    cx.link(scopes[28], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[28],
        "generateIdxAction",
        scopes[27],
        "generateIdxAction",
    );
    cx.link(
        scopes[28],
        "generateRemediationFunctions",
        scopes[31],
        "generateRemediationFunctions",
    );
    cx.link(scopes[28], "jsonpath", scopes[127], "jsonpath");
    cx.link(
        scopes[29],
        "parseIdxResponse",
        scopes[28],
        "parseIdxResponse",
    );
    cx.link(scopes[30], "makeIdxState", scopes[29], "makeIdxState");
    cx.link(
        scopes[31],
        "generateIdxAction",
        scopes[27],
        "generateIdxAction",
    );
    cx.link(
        scopes[32],
        "createTransactionMeta",
        scopes[63],
        "createTransactionMeta",
    );
    cx.link(
        scopes[32],
        "getOAuthBaseUrl",
        scopes[113],
        "getOAuthBaseUrl",
    );
    cx.link(
        scopes[32],
        "getSavedTransactionMeta",
        scopes[63],
        "getSavedTransactionMeta",
    );
    cx.link(scopes[32], "httpRequest", scopes[19], "httpRequest");
    cx.link(scopes[32], "removeNils", scopes[129], "removeNils");
    cx.link(
        scopes[32],
        "saveTransactionMeta",
        scopes[63],
        "saveTransactionMeta",
    );
    cx.link(scopes[33], "IDX_API_VERSION", scopes[0], "IDX_API_VERSION");
    cx.link(scopes[33], "getOAuthDomain", scopes[113], "getOAuthDomain");
    cx.link(scopes[33], "httpRequest", scopes[19], "httpRequest");
    cx.link(scopes[33], "isAuthApiError", scopes[11], "isAuthApiError");
    cx.link(
        scopes[33],
        "isRawIdxResponse",
        scopes[65],
        "isRawIdxResponse",
    );
    cx.link(scopes[33], "makeIdxState", scopes[25], "makeIdxState");
    cx.link(
        scopes[33],
        "validateVersionConfig",
        scopes[25],
        "validateVersionConfig",
    );
    cx.link(scopes[34], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[34],
        "getSavedTransactionMeta",
        scopes[63],
        "getSavedTransactionMeta",
    );
    cx.link(scopes[34], "run", scopes[61], "run");
    cx.link(scopes[35], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[35],
        "GenericRemediator",
        scopes[45],
        "GenericRemediator",
    );
    cx.link(
        scopes[35],
        "filterValuesForRemediation",
        scopes[70],
        "filterValuesForRemediation",
    );
    cx.link(scopes[35], "getNextStep", scopes[70], "getNextStep");
    cx.link(scopes[35], "getRemediator", scopes[70], "getRemediator");
    cx.link(
        scopes[35],
        "handleFailedResponse",
        scopes[70],
        "handleFailedResponse",
    );
    cx.link(
        scopes[35],
        "isTerminalResponse",
        scopes[70],
        "isTerminalResponse",
    );
    cx.link(
        scopes[38],
        "compareAuthenticators",
        scopes[22],
        "compareAuthenticators",
    );
    cx.link(
        scopes[38],
        "formatAuthenticator",
        scopes[22],
        "formatAuthenticator",
    );
    cx.link(scopes[38], "getAllValues", scopes[60], "getAllValues");
    cx.link(
        scopes[38],
        "getAuthenticatorFromRemediation",
        scopes[60],
        "getAuthenticatorFromRemediation",
    );
    cx.link(
        scopes[38],
        "getRequiredValues",
        scopes[60],
        "getRequiredValues",
    );
    cx.link(scopes[38], "titleCase", scopes[60], "titleCase");
    cx.link(scopes[45], "Remediator", scopes[38], "Remediator");
    cx.link(scopes[45], "unwrapFormValue", scopes[47], "unwrapFormValue");
    cx.link(scopes[61], "IdxStatus", scopes[64], "IdxStatus");
    cx.link(
        scopes[61],
        "getAvailableSteps",
        scopes[70],
        "getAvailableSteps",
    );
    cx.link(
        scopes[61],
        "getEnabledFeatures",
        scopes[70],
        "getEnabledFeatures",
    );
    cx.link(
        scopes[61],
        "getFlowSpecification",
        scopes[70],
        "getFlowSpecification",
    );
    cx.link(
        scopes[61],
        "getMessagesFromResponse",
        scopes[70],
        "getMessagesFromResponse",
    );
    cx.link(
        scopes[61],
        "getSavedTransactionMeta",
        scopes[63],
        "getSavedTransactionMeta",
    );
    cx.link(scopes[61], "interact", scopes[32], "interact");
    cx.link(scopes[61], "introspect", scopes[33], "introspect");
    cx.link(
        scopes[61],
        "isTerminalResponse",
        scopes[70],
        "isTerminalResponse",
    );
    cx.link(scopes[61], "remediate", scopes[35], "remediate");
    cx.link(
        scopes[61],
        "saveTransactionMeta",
        scopes[63],
        "saveTransactionMeta",
    );
    cx.link(scopes[62], "run", scopes[61], "run");
    cx.link(
        scopes[63],
        "createOAuthMeta",
        scopes[114],
        "createOAuthMeta",
    );
    cx.link(scopes[63], "removeNils", scopes[129], "removeNils");
    cx.link(scopes[63], "warn", scopes[125], "warn");
    cx.link(
        scopes[70],
        "GenericRemediator",
        scopes[45],
        "GenericRemediator",
    );
    cx.link(scopes[70], "IdxFeature", scopes[64], "IdxFeature");
    cx.link(scopes[70], "split2", scopes[128], "split2");
    cx.link(scopes[70], "warn", scopes[125], "warn");
    cx.link(
        scopes[71],
        "base64UrlToBuffer",
        scopes[1],
        "base64UrlToBuffer",
    );
    cx.link(
        scopes[71],
        "bufferToBase64Url",
        scopes[1],
        "bufferToBase64Url",
    );
    cx.link(
        scopes[73],
        "clearTransactionFromSharedStorage",
        scopes[119],
        "clearTransactionFromSharedStorage",
    );
    cx.link(
        scopes[73],
        "isTransactionMeta",
        scopes[96],
        "isTransactionMeta",
    );
    cx.link(
        scopes[73],
        "loadTransactionFromSharedStorage",
        scopes[119],
        "loadTransactionFromSharedStorage",
    );
    cx.link(
        scopes[73],
        "pruneSharedStorage",
        scopes[119],
        "pruneSharedStorage",
    );
    cx.link(
        scopes[73],
        "saveTransactionToSharedStorage",
        scopes[119],
        "saveTransactionToSharedStorage",
    );
    cx.link(scopes[73], "warn", scopes[125], "warn");
    cx.link(scopes[74], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[74],
        "base64UrlToString",
        scopes[1],
        "base64UrlToString",
    );
    cx.link(scopes[75], "isOAuthError", scopes[11], "isOAuthError");
    cx.link(scopes[75], "isWWWAuthError", scopes[11], "isWWWAuthError");
    cx.link(scopes[78], "AuthApiError", scopes[6], "default");
    cx.link(scopes[78], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[78], "httpRequest", scopes[19], "httpRequest");
    cx.link(
        scopes[78],
        "isDPoPNonceError",
        scopes[75],
        "isDPoPNonceError",
    );
    cx.link(scopes[78], "removeNils", scopes[129], "removeNils");
    cx.link(scopes[78], "toQueryString", scopes[131], "toQueryString");
    cx.link(scopes[79], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[79], "find", scopes[129], "find");
    cx.link(scopes[79], "get", scopes[19], "get");
    cx.link(scopes[80], "clone", scopes[129], "clone");
    cx.link(
        scopes[80],
        "getDefaultTokenParams",
        scopes[108],
        "getDefaultTokenParams",
    );
    cx.link(scopes[80], "getOAuthUrls", scopes[113], "getOAuthUrls");
    cx.link(
        scopes[80],
        "handleOAuthResponse",
        scopes[85],
        "handleOAuthResponse",
    );
    cx.link(
        scopes[80],
        "postToTokenEndpoint",
        scopes[78],
        "postToTokenEndpoint",
    );
    cx.link(scopes[84], "AuthApiError", scopes[6], "default");
    cx.link(scopes[84], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[84], "OAuthError", scopes[9], "default");
    cx.link(scopes[84], "WWWAuthError", scopes[10], "default");
    cx.link(scopes[84], "httpRequest", scopes[19], "httpRequest");
    cx.link(scopes[84], "isAccessToken", scopes[94], "isAccessToken");
    cx.link(scopes[84], "isIDToken", scopes[94], "isIDToken");
    cx.link(scopes[85], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[85], "OAuthError", scopes[9], "default");
    cx.link(scopes[85], "clone", scopes[129], "clone");
    cx.link(
        scopes[85],
        "getDefaultTokenParams",
        scopes[108],
        "getDefaultTokenParams",
    );
    cx.link(scopes[85], "getOAuthUrls", scopes[113], "getOAuthUrls");
    cx.link(scopes[85], "verifyToken", scopes[123], "verifyToken");
    cx.link(scopes[90], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[90], "getOAuthUrls", scopes[113], "getOAuthUrls");
    cx.link(
        scopes[90],
        "handleOAuthResponse",
        scopes[85],
        "handleOAuthResponse",
    );
    cx.link(
        scopes[90],
        "isRefreshTokenInvalidError",
        scopes[110],
        "isRefreshTokenInvalidError",
    );
    cx.link(
        scopes[90],
        "isSameRefreshToken",
        scopes[118],
        "isSameRefreshToken",
    );
    cx.link(
        scopes[90],
        "postRefreshToken",
        scopes[78],
        "postRefreshToken",
    );
    cx.link(scopes[91], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[91], "getOAuthUrls", scopes[113], "getOAuthUrls");
    cx.link(scopes[91], "post", scopes[19], "post");
    cx.link(scopes[91], "toQueryString", scopes[131], "toQueryString");
    cx.link(scopes[108], "generateNonce", scopes[113], "generateNonce");
    cx.link(scopes[108], "generateState", scopes[113], "generateState");
    cx.link(scopes[108], "removeNils", scopes[129], "removeNils");
    cx.link(scopes[110], "isOAuthError", scopes[11], "isOAuthError");
    cx.link(scopes[113], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[113],
        "genRandomString",
        scopes[128],
        "genRandomString",
    );
    cx.link(
        scopes[113],
        "removeTrailingSlash",
        scopes[131],
        "removeTrailingSlash",
    );
    cx.link(scopes[114], "getOAuthUrls", scopes[113], "getOAuthUrls");
    cx.link(
        scopes[115],
        "DEFAULT_CODE_CHALLENGE_METHOD",
        scopes[0],
        "DEFAULT_CODE_CHALLENGE_METHOD",
    );
    cx.link(
        scopes[115],
        "MAX_VERIFIER_LENGTH",
        scopes[0],
        "MAX_VERIFIER_LENGTH",
    );
    cx.link(
        scopes[115],
        "MIN_VERIFIER_LENGTH",
        scopes[0],
        "MIN_VERIFIER_LENGTH",
    );
    cx.link(
        scopes[115],
        "stringToBase64Url",
        scopes[1],
        "stringToBase64Url",
    );
    cx.link(scopes[117], "AuthSdkError", scopes[7], "default");
    cx.link(
        scopes[117],
        "DEFAULT_CODE_CHALLENGE_METHOD",
        scopes[0],
        "DEFAULT_CODE_CHALLENGE_METHOD",
    );
    cx.link(scopes[117], "PKCE", scopes[115], "default");
    cx.link(
        scopes[117],
        "getDefaultTokenParams",
        scopes[108],
        "getDefaultTokenParams",
    );
    cx.link(scopes[117], "getWellKnown", scopes[79], "getWellKnown");
    cx.link(
        scopes[119],
        "isTransactionMeta",
        scopes[96],
        "isTransactionMeta",
    );
    cx.link(scopes[121], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[123], "AuthSdkError", scopes[7], "default");
    cx.link(scopes[123], "decodeToken", scopes[74], "decodeToken");
    cx.link(scopes[123], "getKey", scopes[79], "getKey");
    cx.link(scopes[123], "getWellKnown", scopes[79], "getWellKnown");
    cx.link(scopes[123], "validateClaims", scopes[121], "validateClaims");
    {
        let scope = scopes[1];
        cx.define(scope, "base64ToBase64Url", cx.function(scope, f0));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "base64UrlDecode", cx.function(scope, f1));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "base64UrlToBase64", cx.function(scope, f2));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "base64UrlToBuffer", cx.function(scope, f3));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "base64UrlToString", cx.function(scope, f5));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "bufferToBase64Url", cx.function(scope, f7));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "stringToBase64Url", cx.function(scope, f9));
    }
    {
        let scope = scopes[1];
        cx.define(scope, "stringToBuffer", cx.function(scope, f10));
    }
    {
        let scope = scopes[3];
        cx.define(scope, "getOidcHash", cx.function(scope, f11));
    }
    {
        let scope = scopes[5];
        cx.define(scope, "verifyToken", cx.function(scope, f13));
    }
    {
        let scope = scopes[11];
        cx.define(scope, "isAuthApiError", cx.function(scope, f15));
    }
    {
        let scope = scopes[11];
        cx.define(scope, "isOAuthError", cx.function(scope, f16));
    }
    {
        let scope = scopes[11];
        cx.define(scope, "isWWWAuthError", cx.function(scope, f17));
    }
    {
        let scope = scopes[13];
        cx.define(scope, "fetchRequest", cx.function(scope, f18));
    }
    {
        let scope = scopes[13];
        cx.define(scope, "formatResult", cx.function(scope, f24));
    }
    {
        let scope = scopes[13];
        cx.define(scope, "readData", cx.function(scope, f25));
    }
    {
        let scope = scopes[19];
        cx.define(scope, "get", cx.function(scope, f27));
    }
    {
        let scope = scopes[19];
        cx.define(scope, "httpRequest", cx.function(scope, f28));
    }
    {
        let scope = scopes[19];
        cx.define(scope, "post", cx.function(scope, f32));
    }
    {
        let scope = scopes[21];
        cx.define(
            scope,
            "createIdxTransactionManager",
            cx.function(scope, f33),
        );
    }
    {
        let scope = scopes[22];
        cx.define(scope, "compareAuthenticators", cx.function(scope, f40));
    }
    {
        let scope = scopes[22];
        cx.define(scope, "formatAuthenticator", cx.function(scope, f41));
    }
    {
        let scope = scopes[23];
        cx.define(
            scope,
            "handleEmailVerifyCallback",
            cx.async_function(scope, f42),
        );
    }
    {
        let scope = scopes[23];
        cx.define(scope, "isEmailVerifyCallback", cx.function(scope, f43));
    }
    {
        let scope = scopes[23];
        cx.define(scope, "parseEmailVerifyCallback", cx.function(scope, f44));
    }
    {
        let scope = scopes[24];
        cx.define(
            scope,
            "handleInteractionCodeRedirect",
            cx.async_function(scope, f45),
        );
    }
    {
        let scope = scopes[25];
        cx.define(scope, "makeIdxState", cx.function(scope, f46));
    }
    {
        let scope = scopes[25];
        cx.define(scope, "validateVersionConfig", cx.function(scope, f48));
    }
    {
        let scope = scopes[29];
        cx.define(scope, "makeIdxState", cx.function(scope, f49));
    }
    {
        let scope = scopes[32];
        cx.define(scope, "getResponse", cx.function(scope, f54));
    }
    {
        let scope = scopes[32];
        cx.define(scope, "interact", cx.async_function(scope, f55));
    }
    {
        let scope = scopes[33];
        cx.define(scope, "introspect", cx.async_function(scope, f57));
    }
    {
        let scope = scopes[34];
        cx.define(scope, "canProceed", cx.function(scope, f58));
    }
    {
        let scope = scopes[34];
        cx.define(scope, "proceed", cx.async_function(scope, f59));
    }
    {
        let scope = scopes[35];
        cx.define(scope, "getActionFromValues", cx.function(scope, f61));
    }
    {
        let scope = scopes[35];
        cx.define(scope, "getGenericNextStep", cx.function(scope, f63));
    }
    {
        let scope = scopes[35];
        cx.define(scope, "remediate", cx.async_function(scope, f64));
    }
    {
        let scope = scopes[35];
        cx.define(scope, "removeActionFromOptions", cx.function(scope, f67));
    }
    {
        let scope = scopes[35];
        cx.define(scope, "removeActionFromValues", cx.function(scope, f69));
    }
    {
        let scope = scopes[47];
        cx.define(scope, "unwrapFormValue", cx.function(scope, f70));
    }
    {
        let scope = scopes[60];
        cx.define(scope, "getAllValues", cx.function(scope, f73));
    }
    {
        let scope = scopes[60];
        cx.define(
            scope,
            "getAuthenticatorFromRemediation",
            cx.function(scope, f76),
        );
    }
    {
        let scope = scopes[60];
        cx.define(scope, "getRequiredValues", cx.function(scope, f78));
    }
    {
        let scope = scopes[60];
        cx.define(scope, "titleCase", cx.function(scope, f81));
    }
    {
        let scope = scopes[61];
        cx.define(scope, "finalizeData", cx.async_function(scope, f82));
    }
    {
        let scope = scopes[61];
        cx.define(
            scope,
            "getDataFromIntrospect",
            cx.async_function(scope, f85),
        );
    }
    {
        let scope = scopes[61];
        cx.define(scope, "getDataFromRemediate", cx.async_function(scope, f87));
    }
    {
        let scope = scopes[61];
        cx.define(scope, "getTokens", cx.async_function(scope, f88));
    }
    {
        let scope = scopes[61];
        cx.define(scope, "initializeData", cx.function(scope, f89));
    }
    {
        let scope = scopes[61];
        cx.define(scope, "initializeValues", cx.function(scope, f92));
    }
    {
        let scope = scopes[61];
        cx.define(scope, "run", cx.async_function(scope, f94));
    }
    {
        let scope = scopes[62];
        cx.define(scope, "startTransaction", cx.async_function(scope, f97));
    }
    {
        let scope = scopes[63];
        cx.define(
            scope,
            "createTransactionMeta",
            cx.async_function(scope, f98),
        );
    }
    {
        let scope = scopes[63];
        cx.define(scope, "getSavedTransactionMeta", cx.function(scope, f99));
    }
    {
        let scope = scopes[63];
        cx.define(scope, "isTransactionMetaValid", cx.function(scope, f100));
    }
    {
        let scope = scopes[63];
        cx.define(
            scope,
            "isTransactionMetaValidForFlow",
            cx.function(scope, f101),
        );
    }
    {
        let scope = scopes[63];
        cx.define(
            scope,
            "isTransactionMetaValidForOptions",
            cx.function(scope, f102),
        );
    }
    {
        let scope = scopes[63];
        cx.define(scope, "saveTransactionMeta", cx.function(scope, f104));
    }
    {
        let scope = scopes[64];
        cx.define(scope, "isAuthenticator", cx.function(scope, f105));
    }
    {
        let scope = scopes[65];
        cx.define(scope, "isRawIdxResponse", cx.function(scope, f106));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "canResendFn", cx.function(scope, f107));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "canSkipFn", cx.function(scope, f109));
    }
    {
        let scope = scopes[70];
        cx.define(
            scope,
            "filterValuesForRemediation",
            cx.function(scope, f111),
        );
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getAvailableSteps", cx.function(scope, f114));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getEnabledFeatures", cx.function(scope, f119));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getFlowSpecification", cx.function(scope, f123));
    }
    {
        let scope = scopes[70];
        cx.define(
            scope,
            "getMessagesFromIdxRemediationValue",
            cx.function(scope, f124),
        );
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getMessagesFromResponse", cx.function(scope, f127));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getNextStep", cx.function(scope, f132));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getRemediator", cx.function(scope, f133));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "getRemediatorClass", cx.function(scope, f135));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "handleFailedResponse", cx.function(scope, f136));
    }
    {
        let scope = scopes[70];
        cx.define(scope, "isTerminalResponse", cx.function(scope, f137));
    }
    {
        let scope = scopes[73];
        cx.define(scope, "createTransactionManager", cx.function(scope, f138));
    }
    {
        let scope = scopes[74];
        cx.define(scope, "decodeToken", cx.function(scope, f145));
    }
    {
        let scope = scopes[75];
        cx.define(scope, "isDPoPNonceError", cx.function(scope, f146));
    }
    {
        let scope = scopes[78];
        cx.define(scope, "getPostData", cx.function(scope, f147));
    }
    {
        let scope = scopes[78];
        cx.define(scope, "makeTokenRequest", cx.async_function(scope, f148));
    }
    {
        let scope = scopes[78];
        cx.define(scope, "postRefreshToken", cx.async_function(scope, f150));
    }
    {
        let scope = scopes[78];
        cx.define(scope, "postToTokenEndpoint", cx.async_function(scope, f153));
    }
    {
        let scope = scopes[78];
        cx.define(scope, "validateOptions", cx.function(scope, f155));
    }
    {
        let scope = scopes[79];
        cx.define(scope, "getKey", cx.function(scope, f156));
    }
    {
        let scope = scopes[79];
        cx.define(scope, "getWellKnown", cx.function(scope, f159));
    }
    {
        let scope = scopes[80];
        cx.define(
            scope,
            "exchangeCodeForTokens",
            cx.async_function(scope, f160),
        );
    }
    {
        let scope = scopes[84];
        cx.define(scope, "getUserInfo", cx.async_function(scope, f161));
    }
    {
        let scope = scopes[85];
        cx.define(scope, "handleOAuthResponse", cx.async_function(scope, f165));
    }
    {
        let scope = scopes[85];
        cx.define(scope, "validateResponse", cx.function(scope, f167));
    }
    {
        let scope = scopes[90];
        cx.define(
            scope,
            "renewTokensWithRefresh",
            cx.async_function(scope, f168),
        );
    }
    {
        let scope = scopes[91];
        cx.define(scope, "revokeToken", cx.async_function(scope, f169));
    }
    {
        let scope = scopes[94];
        cx.define(scope, "isAccessToken", cx.function(scope, f170));
    }
    {
        let scope = scopes[94];
        cx.define(scope, "isIDToken", cx.function(scope, f171));
    }
    {
        let scope = scopes[96];
        cx.define(
            scope,
            "isCustomAuthTransactionMeta",
            cx.function(scope, f172),
        );
    }
    {
        let scope = scopes[96];
        cx.define(scope, "isOAuthTransactionMeta", cx.function(scope, f174));
    }
    {
        let scope = scopes[96];
        cx.define(scope, "isObjectWithProperties", cx.function(scope, f175));
    }
    {
        let scope = scopes[96];
        cx.define(scope, "isTransactionMeta", cx.function(scope, f176));
    }
    {
        let scope = scopes[108];
        cx.define(scope, "getDefaultTokenParams", cx.function(scope, f177));
    }
    {
        let scope = scopes[110];
        cx.define(
            scope,
            "isRefreshTokenInvalidError",
            cx.function(scope, f178),
        );
    }
    {
        let scope = scopes[113];
        cx.define(scope, "generateNonce", cx.function(scope, f179));
    }
    {
        let scope = scopes[113];
        cx.define(scope, "generateState", cx.function(scope, f180));
    }
    {
        let scope = scopes[113];
        cx.define(scope, "getIssuer", cx.function(scope, f181));
    }
    {
        let scope = scopes[113];
        cx.define(scope, "getOAuthBaseUrl", cx.function(scope, f182));
    }
    {
        let scope = scopes[113];
        cx.define(scope, "getOAuthDomain", cx.function(scope, f183));
    }
    {
        let scope = scopes[113];
        cx.define(scope, "getOAuthUrls", cx.function(scope, f184));
    }
    {
        let scope = scopes[114];
        cx.define(scope, "createOAuthMeta", cx.function(scope, f185));
    }
    {
        let scope = scopes[115];
        cx.define(scope, "computeChallenge", cx.function(scope, f186));
    }
    {
        let scope = scopes[115];
        cx.define(scope, "dec2hex", cx.function(scope, f188));
    }
    {
        let scope = scopes[115];
        cx.define(scope, "generateVerifier", cx.function(scope, f189));
    }
    {
        let scope = scopes[115];
        cx.define(scope, "getRandomString", cx.function(scope, f190));
    }
    {
        let scope = scopes[117];
        cx.define(scope, "assertPKCESupport", cx.function(scope, f191));
    }
    {
        let scope = scopes[117];
        cx.define(scope, "preparePKCE", cx.async_function(scope, f192));
    }
    {
        let scope = scopes[117];
        cx.define(scope, "prepareTokenParams", cx.async_function(scope, f193));
    }
    {
        let scope = scopes[117];
        cx.define(
            scope,
            "validateCodeChallengeMethod",
            cx.async_function(scope, f194),
        );
    }
    {
        let scope = scopes[118];
        cx.define(scope, "isSameRefreshToken", cx.function(scope, f195));
    }
    {
        let scope = scopes[119];
        cx.define(
            scope,
            "clearTransactionFromSharedStorage",
            cx.function(scope, f196),
        );
    }
    {
        let scope = scopes[119];
        cx.define(
            scope,
            "loadTransactionFromSharedStorage",
            cx.function(scope, f197),
        );
    }
    {
        let scope = scopes[119];
        cx.define(scope, "pruneSharedStorage", cx.function(scope, f198));
    }
    {
        let scope = scopes[119];
        cx.define(
            scope,
            "saveTransactionToSharedStorage",
            cx.function(scope, f200),
        );
    }
    {
        let scope = scopes[120];
        cx.define(scope, "urlParamsToObject", cx.function(scope, f201));
    }
    {
        let scope = scopes[121];
        cx.define(scope, "validateClaims", cx.function(scope, f202));
    }
    {
        let scope = scopes[123];
        cx.define(scope, "verifyToken", cx.async_function(scope, f203));
    }
    {
        let scope = scopes[125];
        cx.define(scope, "getConsole", cx.function(scope, f205));
    }
    {
        let scope = scopes[125];
        cx.define(scope, "getNativeConsole", cx.function(scope, f210));
    }
    {
        let scope = scopes[125];
        cx.define(scope, "warn", cx.function(scope, f211));
    }
    {
        let scope = scopes[127];
        cx.define(scope, "jsonpath", cx.function(scope, f212));
    }
    {
        let scope = scopes[128];
        cx.define(scope, "genRandomString", cx.function(scope, f215));
    }
    {
        let scope = scopes[128];
        cx.define(scope, "split2", cx.function(scope, f216));
    }
    {
        let scope = scopes[129];
        cx.define(scope, "clone", cx.function(scope, f217));
    }
    {
        let scope = scopes[129];
        cx.define(scope, "find", cx.function(scope, f218));
    }
    {
        let scope = scopes[129];
        cx.define(scope, "removeNils", cx.function(scope, f219));
    }
    {
        let scope = scopes[130];
        cx.define(scope, "isFunction", cx.function(scope, f220));
    }
    {
        let scope = scopes[130];
        cx.define(scope, "isString", cx.function(scope, f221));
    }
    {
        let scope = scopes[131];
        cx.define(scope, "isAbsoluteUrl", cx.function(scope, f222));
    }
    {
        let scope = scopes[131];
        cx.define(scope, "removeTrailingSlash", cx.function(scope, f223));
    }
    {
        let scope = scopes[131];
        cx.define(scope, "toQueryString", cx.function(scope, f224));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "DEFAULT_CACHE_DURATION", V::Number(86400f64));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "DEFAULT_CODE_CHALLENGE_METHOD", V::string("S256"));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "IDX_API_VERSION", V::string("1.0.0"));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "MAX_VERIFIER_LENGTH", V::Number(128f64));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "MIN_VERIFIER_LENGTH", V::Number(43f64));
    }
    {
        let scope = scopes[0];
        cx.define(scope, "STATE_TOKEN_KEY_NAME", V::string("oktaStateToken"));
    }
    {
        let scope = scopes[8];
        cx.define(scope, "default", {
            let superclass = cx.lookup(scope, "Error")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f225);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "CustomError", constructor.clone());
            constructor
        });
    }
    {
        let scope = scopes[6];
        cx.define(scope, "default", {
            let superclass = cx.lookup(scope, "CustomError")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f226);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "AuthApiError", constructor.clone());
            constructor
        });
    }
    {
        let scope = scopes[7];
        cx.define(scope, "default", {
            let superclass = cx.lookup(scope, "CustomError")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f227);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "AuthSdkError", constructor.clone());
            constructor
        });
    }
    {
        let scope = scopes[9];
        cx.define(scope, "default", {
            let superclass = cx.lookup(scope, "CustomError")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f228);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "OAuthError", constructor.clone());
            constructor
        });
    }
    {
        let scope = scopes[10];
        cx.define(scope, "default", {
            let superclass = cx.lookup(scope, "CustomError")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f229);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "WWWAuthError", constructor.clone());
            cx.getter(&prototype, &"error", cx.function(scope, f230))?;
            cx.getter(&prototype, &"errorCode", cx.function(scope, f231))?;
            cx.getter(&prototype, &"error_description", cx.function(scope, f232))?;
            cx.getter(&prototype, &"errorDescription", cx.function(scope, f233))?;
            cx.getter(&prototype, &"errorSummary", cx.function(scope, f234))?;
            cx.getter(&prototype, &"realm", cx.function(scope, f235))?;
            cx.put(&constructor, &"parseHeader", cx.function(scope, f236))?;
            cx.put(
                &constructor,
                &"getWWWAuthenticateHeader",
                cx.function(scope, f237),
            )?;
            {
                let this = constructor.clone();
                cx.put(
                    &constructor,
                    &"UNKNOWN_ERROR",
                    V::string("UNKNOWN_WWW_AUTH_ERROR"),
                )?;
            }
            constructor
        });
    }
    {
        let scope = scopes[13];
        cx.define(
            scope,
            "appJsonContentTypeRegex",
            cx.regexp("application\\/\\w*\\+?json", "")?,
        );
    }
    {
        let scope = scopes[19];
        cx.define(scope, "formatError", cx.arrow(scope, this.clone(), f239));
    }
    {
        let scope = scopes[23];
        cx.define(scope, "EmailVerifyCallbackError", {
            let superclass = cx.lookup(scope, "CustomError")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f245);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "EmailVerifyCallbackError", constructor.clone());
            constructor
        });
    }
    {
        let scope = scopes[30];
        cx.define(scope, "default", {
            let object = cx.object();
            cx.put(&object, &"makeIdxState", cx.lookup(scope, "makeIdxState")?)?;
            object
        });
    }
    {
        let scope = scopes[25];
        cx.define(scope, "parsersForVersion", cx.function(scope, f246));
    }
    {
        let scope = scopes[26];
        cx.define(scope, "isFieldMutable", cx.function(scope, f248));
    }
    {
        let scope = scopes[26];
        cx.define(
            scope,
            "divideSingleActionParamsByMutability",
            cx.function(scope, f249),
        );
    }
    {
        let scope = scopes[26];
        cx.define(
            scope,
            "divideActionParamsByMutability",
            cx.function(scope, f250),
        );
    }
    {
        let scope = scopes[27];
        cx.define(scope, "generateDirectFetch", cx.function(scope, f251));
    }
    {
        let scope = scopes[27];
        cx.define(scope, "generateIdxAction", cx.function(scope, f256));
    }
    {
        let scope = scopes[28];
        cx.define(scope, "SKIP_FIELDS", {
            let object = cx.object();
            cx.put(&object, &"remediation", V::Bool(true))?;
            cx.put(&object, &"context", V::Bool(true))?;
            object
        });
    }
    {
        let scope = scopes[31];
        cx.define(
            scope,
            "generateRemediationFunctions",
            cx.function(scope, f257),
        );
    }
    {
        let scope = scopes[28];
        cx.define(
            scope,
            "convertRemediationAction",
            cx.arrow(scope, this.clone(), f259),
        );
    }
    {
        let scope = scopes[28];
        cx.define(
            scope,
            "expandRelatesTo",
            cx.arrow(scope, this.clone(), f260),
        );
    }
    {
        let scope = scopes[28];
        cx.define(scope, "parseNonRemediations", cx.function(scope, f263));
    }
    {
        let scope = scopes[28];
        cx.define(scope, "parseIdxResponse", cx.function(scope, f267));
    }
    {
        let scope = scopes[38];
        cx.define(scope, "Remediator", {
            let superclass = V::Undefined;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f273);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "Remediator", constructor.clone());
            cx.put(
                &prototype,
                &"formatAuthenticators",
                cx.function(scope, f274),
            )?;
            cx.put(&prototype, &"getName", cx.function(scope, f278))?;
            cx.put(&prototype, &"canRemediate", cx.function(scope, f279))?;
            cx.put(&prototype, &"getData", cx.function(scope, f281))?;
            cx.put(&prototype, &"hasData", cx.function(scope, f284))?;
            cx.put(&prototype, &"getNextStep", cx.function(scope, f285))?;
            cx.put(&prototype, &"getInputs", cx.function(scope, f287))?;
            cx.put(&constructor, &"getMessages", cx.function(scope, f291))?;
            cx.put(
                &prototype,
                &"getValuesAfterProceed",
                cx.function(scope, f294),
            )?;
            cx.put(&prototype, &"getAuthenticator", cx.function(scope, f295))?;
            constructor
        });
    }
    {
        let scope = scopes[45];
        cx.define(scope, "GenericRemediator", {
            let superclass = cx.lookup(scope, "Remediator")?;
            let scope = cx.scope(Some(scope));
            cx.define(scope, "__super", superclass.clone());
            let constructor = cx.function(scope, f300);
            let prototype = cx.object();
            if !superclass.nullish() {
                let parent = cx.get(&superclass, "prototype")?;
                if !parent.nullish() {
                    cx.set_prototype(&prototype, parent)?;
                }
                if matches!(superclass, V::Function(_)) {
                    cx.set_prototype(&constructor, superclass)?;
                }
            }
            cx.put(&constructor, "prototype", prototype.clone())?;
            cx.define(scope, "GenericRemediator", constructor.clone());
            cx.put(&prototype, &"canRemediate", cx.function(scope, f301))?;
            cx.put(&prototype, &"getData", cx.function(scope, f302))?;
            cx.put(&prototype, &"getNextStep", cx.function(scope, f304))?;
            cx.put(&prototype, &"getInputs", cx.function(scope, f306))?;
            constructor
        });
    }
    {
        let scope = scopes[64];
        cx.define(scope, "IdxFeature", {
            let enumeration = cx.object();
            {
                let value = V::string("recover-password");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("PASSWORD_RECOVERY"))?;
                }
                cx.put(&enumeration, "PASSWORD_RECOVERY", value)?;
            }
            {
                let value = V::string("enroll-profile");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("REGISTRATION"))?;
                }
                cx.put(&enumeration, "REGISTRATION", value)?;
            }
            {
                let value = V::string("redirect-idp");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("SOCIAL_IDP"))?;
                }
                cx.put(&enumeration, "SOCIAL_IDP", value)?;
            }
            {
                let value = V::string("unlock-account");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("ACCOUNT_UNLOCK"))?;
                }
                cx.put(&enumeration, "ACCOUNT_UNLOCK", value)?;
            }
            enumeration
        });
    }
    {
        let scope = scopes[64];
        cx.define(scope, "IdxStatus", {
            let enumeration = cx.object();
            {
                let value = V::string("SUCCESS");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("SUCCESS"))?;
                }
                cx.put(&enumeration, "SUCCESS", value)?;
            }
            {
                let value = V::string("PENDING");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("PENDING"))?;
                }
                cx.put(&enumeration, "PENDING", value)?;
            }
            {
                let value = V::string("FAILURE");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("FAILURE"))?;
                }
                cx.put(&enumeration, "FAILURE", value)?;
            }
            {
                let value = V::string("TERMINAL");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("TERMINAL"))?;
                }
                cx.put(&enumeration, "TERMINAL", value)?;
            }
            {
                let value = V::string("CANCELED");
                if matches!(value, V::Number(_)) {
                    cx.put(&enumeration, &value.text(), V::string("CANCELED"))?;
                }
                cx.put(&enumeration, "CANCELED", value)?;
            }
            enumeration
        });
    }
    {
        let scope = scopes[70];
        cx.define(scope, "ctx", {
            let object = cx.object();
            cx.put(&object, &"remediators", {
                let object = cx.object();
                object
            })?;
            cx.put(&object, &"getFlowSpecification", cx.function(scope, f309))?;
            object
        });
    }
    {
        let scope = scopes[71];
        cx.define(
            scope,
            "getEnrolledCredentials",
            cx.arrow(scope, this.clone(), f310),
        );
    }
    {
        let scope = scopes[71];
        cx.define(
            scope,
            "buildCredentialCreationOptions",
            cx.arrow(scope, this.clone(), f313),
        );
    }
    {
        let scope = scopes[71];
        cx.define(
            scope,
            "buildCredentialRequestOptions",
            cx.arrow(scope, this.clone(), f314),
        );
    }
    {
        let scope = scopes[71];
        cx.define(scope, "getAssertion", cx.arrow(scope, this.clone(), f315));
    }
    {
        let scope = scopes[71];
        cx.define(scope, "getAttestation", cx.arrow(scope, this.clone(), f316));
    }
    {
        let scope = scopes[115];
        cx.define(scope, "default", {
            let object = cx.object();
            cx.put(
                &object,
                &"DEFAULT_CODE_CHALLENGE_METHOD",
                cx.lookup(scope, "DEFAULT_CODE_CHALLENGE_METHOD")?,
            )?;
            cx.put(
                &object,
                &"generateVerifier",
                cx.lookup(scope, "generateVerifier")?,
            )?;
            cx.put(
                &object,
                &"computeChallenge",
                cx.lookup(scope, "computeChallenge")?,
            )?;
            object
        });
    }
    {
        let scope = scopes[119];
        cx.define(
            scope,
            "MAX_ENTRY_LIFETIME",
            binary(
                "*",
                binary("*", V::Number(30f64), V::Number(60f64))?,
                V::Number(1000f64),
            )?,
        );
    }
    {
        let scope = scopes[127];
        cx.define(
            scope,
            "jsonpathRegex",
            cx.regexp("\\$?(?<step>\\w+)|(?:\\[(?<index>\\d+)\\])", "g")?,
        );
    }
    {
        let namespace = cx.object();
        cx.put(
            &namespace,
            "getOidcHash",
            cx.lookup(scopes[3], "getOidcHash")?,
        )?;
        cx.put(
            &namespace,
            "verifyToken",
            cx.lookup(scopes[5], "verifyToken")?,
        )?;
        cx.define(scopes[123], "sdkCrypto", namespace);
    }
    let mut exports = std::collections::BTreeMap::new();
    exports.insert(
        "divideActionParamsByMutability".to_string(),
        cx.lookup(scopes[26], "divideActionParamsByMutability")?,
    );
    exports.insert(
        "unwrapFormValue".to_string(),
        cx.lookup(scopes[47], "unwrapFormValue")?,
    );
    exports.insert(
        "startTransaction".to_string(),
        cx.lookup(scopes[62], "startTransaction")?,
    );
    exports.insert("proceed".to_string(), cx.lookup(scopes[34], "proceed")?);
    exports.insert(
        "introspect".to_string(),
        cx.lookup(scopes[33], "introspect")?,
    );
    exports.insert(
        "makeIdxStateV1".to_string(),
        cx.lookup(scopes[29], "makeIdxState")?,
    );
    exports.insert(
        "createTransactionMeta".to_string(),
        cx.lookup(scopes[63], "createTransactionMeta")?,
    );
    exports.insert(
        "getSavedTransactionMeta".to_string(),
        cx.lookup(scopes[63], "getSavedTransactionMeta")?,
    );
    exports.insert(
        "buildCredentialCreationOptions".to_string(),
        cx.lookup(scopes[71], "buildCredentialCreationOptions")?,
    );
    exports.insert(
        "buildCredentialRequestOptions".to_string(),
        cx.lookup(scopes[71], "buildCredentialRequestOptions")?,
    );
    exports.insert(
        "getAttestation".to_string(),
        cx.lookup(scopes[71], "getAttestation")?,
    );
    exports.insert(
        "getAssertion".to_string(),
        cx.lookup(scopes[71], "getAssertion")?,
    );
    exports.insert(
        "exchangeCodeForTokens".to_string(),
        cx.lookup(scopes[80], "exchangeCodeForTokens")?,
    );
    exports.insert(
        "verifyToken".to_string(),
        cx.lookup(scopes[123], "verifyToken")?,
    );
    exports.insert(
        "renewTokensWithRefresh".to_string(),
        cx.lookup(scopes[90], "renewTokensWithRefresh")?,
    );
    exports.insert(
        "revokeToken".to_string(),
        cx.lookup(scopes[91], "revokeToken")?,
    );
    exports.insert(
        "getUserInfo".to_string(),
        cx.lookup(scopes[84], "getUserInfo")?,
    );
    exports.insert(
        "handleInteractionCodeRedirect".to_string(),
        cx.lookup(scopes[24], "handleInteractionCodeRedirect")?,
    );
    exports.insert(
        "parseEmailVerifyCallback".to_string(),
        cx.lookup(scopes[23], "parseEmailVerifyCallback")?,
    );
    exports.insert("jsonpath".to_string(), cx.lookup(scopes[127], "jsonpath")?);
    exports.insert(
        "getOidcHash".to_string(),
        cx.lookup(scopes[3], "getOidcHash")?,
    );
    exports.insert(
        "verifySignature".to_string(),
        cx.lookup(scopes[5], "verifyToken")?,
    );
    exports.insert(
        "base64UrlToString".to_string(),
        cx.lookup(scopes[1], "base64UrlToString")?,
    );
    exports.insert(
        "validateClaims".to_string(),
        cx.lookup(scopes[121], "validateClaims")?,
    );
    exports.insert(
        "toQueryString".to_string(),
        cx.lookup(scopes[131], "toQueryString")?,
    );
    exports.insert(
        "getPostData".to_string(),
        cx.lookup(scopes[78], "getPostData")?,
    );
    exports.insert(
        "prepareTokenParams".to_string(),
        cx.lookup(scopes[117], "prepareTokenParams")?,
    );
    exports.insert(
        "fetchRequest".to_string(),
        cx.lookup(scopes[13], "default")?,
    );
    exports.insert(
        "makeIdxState".to_string(),
        cx.lookup(scopes[25], "makeIdxState")?,
    );
    exports.insert(
        "createIdxTransactionManager".to_string(),
        cx.lookup(scopes[21], "createIdxTransactionManager")?,
    );
    exports.insert(
        "decodeToken".to_string(),
        cx.lookup(scopes[74], "decodeToken")?,
    );
    exports.insert("getTokens".to_string(), cx.lookup(scopes[61], "getTokens")?);
    exports.insert(
        "getOAuthUrls".to_string(),
        cx.lookup(scopes[113], "getOAuthUrls")?,
    );
    exports.insert(
        "isEmailVerifyCallback".to_string(),
        cx.lookup(scopes[23], "isEmailVerifyCallback")?,
    );
    exports.insert(
        "handleEmailVerifyCallback".to_string(),
        cx.lookup(scopes[23], "handleEmailVerifyCallback")?,
    );
    exports.insert(
        "canProceed".to_string(),
        cx.lookup(scopes[34], "canProceed")?,
    );
    Ok(exports)
}
