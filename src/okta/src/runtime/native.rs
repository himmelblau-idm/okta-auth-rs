//! Platform primitives used by translated source. Protocol logic stays generated.
use super::{Engine, JResult, V};
use base64::{
    alphabet,
    engine::{
        general_purpose::{GeneralPurpose, GeneralPurposeConfig},
        DecodePaddingMode,
    },
    Engine as _,
};
use ring::{
    digest,
    rand::{SecureRandom, SystemRandom},
    signature,
};

pub(crate) fn global(name: &str) -> Option<V> {
    Some(V::Builtin(match name {
        "global" => "global",
        "crypto" => "crypto",
        "TextEncoder" => "TextEncoder",
        "Uint8Array" => "Uint8Array",
        "Date" => "Date",
        "URL" => "URL",
        "encodeURIComponent" => "encodeURIComponent",
        "decodeURIComponent" => "decodeURIComponent",
        "escape" => "escape",
        "atob" => "atob",
        "btoa" => "btoa",
        "console" => return Some(V::Undefined),
        _ => return None,
    }))
}
fn base64_decoder() -> GeneralPurpose {
    GeneralPurpose::new(
        &alphabet::STANDARD,
        GeneralPurposeConfig::new()
            .with_decode_padding_mode(DecodePaddingMode::Indifferent)
            .with_decode_allow_trailing_bits(true),
    )
}
impl Engine {
    pub fn bytes(&self, v: &V) -> Result<Vec<u8>, V> {
        self.items(v)?
            .into_iter()
            .map(|v| {
                let n = v.number();
                if !n.is_finite() {
                    return Err(V::error("invalid byte"));
                }
                Ok(n as u8)
            })
            .collect()
    }
    fn byte_array(&self, bytes: &[u8]) -> V {
        self.array(bytes.iter().map(|b| V::Number(*b as f64)).collect())
    }
    pub(crate) fn native_call(&self, name: &str, this: V, args: Vec<V>) -> JResult {
        let a = args.first().cloned().unwrap_or(V::Undefined);
        match name {
            "fetch" => self.fetch(args),
            "Array.slice" => self.method(this, "slice", args),
            "String.fromCharCode" => self.native_method("String", "fromCharCode", args),
            "Uint8Array" => {
                if let V::Number(length) = a {
                    if length < 0.0 || length > 16_777_216.0 || length.fract() != 0.0 {
                        return Err(V::error("invalid byte buffer length"));
                    }
                    Ok(self.array(vec![V::Number(0.0); length as usize]))
                } else if matches!(a, V::Undefined) {
                    Ok(self.array(vec![]))
                } else {
                    Ok(self.byte_array(&self.bytes(&a)?))
                }
            }
            "TextEncoder" => Ok(V::Builtin("TextEncoderInstance")),
            "URL" => {
                fn get(cx: &Engine, scope: usize, _this: V, args: Vec<V>) -> JResult {
                    let value = cx.get(
                        &cx.lookup(scope, "values")?,
                        &args.first().unwrap_or(&V::Undefined).text(),
                    )?;
                    Ok(if matches!(value, V::Undefined) {
                        V::Null
                    } else {
                        value
                    })
                }
                let text = a.text();
                let url = if let Some(base) = args.get(1) {
                    url::Url::parse(&base.text()).and_then(|u| u.join(&text))
                } else {
                    url::Url::parse(&text)
                }
                .map_err(|_| V::error("invalid URL"))?;
                let out = self.object();
                for (k, v) in [
                    ("href", url.as_str()),
                    ("origin", &url.origin().ascii_serialization()),
                    ("pathname", url.path()),
                    ("hostname", url.host_str().unwrap_or("")),
                    ("protocol", &(url.scheme().to_string() + ":")),
                    (
                        "search",
                        &url.query().map(|q| format!("?{q}")).unwrap_or_default(),
                    ),
                    (
                        "hash",
                        &url.fragment().map(|q| format!("#{q}")).unwrap_or_default(),
                    ),
                ] {
                    self.put(&out, k, V::string(v))?;
                }
                let values = self.object();
                for (key, value) in url.query_pairs() {
                    if matches!(self.get(&values, &key)?, V::Undefined) {
                        self.put(&values, &key, V::string(value))?;
                    }
                }
                let scope = self.scope(None);
                self.define(scope, "values", values);
                let params = self.object();
                self.put(&params, "get", self.function(scope, get))?;
                self.put(&out, "searchParams", params)?;
                Ok(out)
            }
            "atob" => {
                let encoded: String = a
                    .text()
                    .chars()
                    .filter(|c| !matches!(c, '\t' | '\n' | '\r' | ' ' | '\u{000C}'))
                    .collect();
                let bytes = base64_decoder()
                    .decode(encoded)
                    .map_err(|_| V::error("invalid base64"))?;
                Ok(V::string(
                    bytes.into_iter().map(char::from).collect::<String>(),
                ))
            }
            "btoa" => {
                let bytes = a
                    .text()
                    .encode_utf16()
                    .map(|c| u8::try_from(c).map_err(|_| V::error("btoa expects Latin-1")))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(V::string(
                    base64::engine::general_purpose::STANDARD.encode(bytes),
                ))
            }
            "encodeURIComponent" => {
                let mut out = String::new();
                for b in a.text().bytes() {
                    if b.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&b) {
                        out.push(b as char);
                    } else {
                        out.push_str(&format!("%{b:02X}"));
                    }
                }
                Ok(V::string(out))
            }
            "decodeURIComponent" => {
                let text = a.text();
                let mut bytes = vec![];
                let mut i = 0;
                while i < text.len() {
                    if text.as_bytes()[i] == b'%' {
                        let digits = text
                            .get(i + 1..i + 3)
                            .ok_or_else(|| V::error("invalid percent encoding"))?;
                        bytes.push(
                            u8::from_str_radix(digits, 16)
                                .map_err(|_| V::error("invalid percent encoding"))?,
                        );
                        i += 3;
                    } else {
                        bytes.push(text.as_bytes()[i]);
                        i += 1;
                    }
                }
                Ok(V::string(
                    String::from_utf8(bytes).map_err(|_| V::error("invalid UTF-8 encoding"))?,
                ))
            }
            "escape" => {
                let mut out = String::new();
                for c in a.text().encode_utf16() {
                    if c < 128 && (c as u8).is_ascii_alphanumeric()
                        || c < 128 && b"@*_+-./".contains(&(c as u8))
                    {
                        out.push(char::from_u32(c as u32).unwrap());
                    } else if c < 256 {
                        out.push_str(&format!("%{c:02X}"));
                    } else {
                        out.push_str(&format!("%u{c:04X}"));
                    }
                }
                Ok(V::string(out))
            }
            _ => Err(V::error(format!("unsupported native function {name}"))),
        }
    }
    pub(crate) fn native_method(&self, object: &str, key: &str, args: Vec<V>) -> JResult {
        let a = args.first().cloned().unwrap_or(V::Undefined);
        match (object, key) {
            ("Date", "now") => Ok(V::Number(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| V::error("clock precedes epoch"))?
                    .as_secs_f64()
                    * 1000.0,
            )),
            ("Math", "ceil") => Ok(V::Number(a.number().ceil())),
            ("crypto", "getRandomValues") => {
                let length = self.items(&a)?.len();
                if length > 65536 {
                    return Err(V::error("random request exceeds platform limit"));
                }
                let mut bytes = vec![0; length];
                SystemRandom::new()
                    .fill(&mut bytes)
                    .map_err(|_| V::error("random source failed"))?;
                for (i, b) in bytes.iter().enumerate() {
                    self.put(&a, &i.to_string(), V::Number(*b as f64))?;
                }
                Ok(a)
            }
            ("Array", "from") => {
                let values = match &a {
                    V::String(s) => s.chars().map(|c| V::string(c.to_string())).collect(),
                    _ => self.items(&a)?,
                };
                let mut out = vec![];
                for (i, v) in values.into_iter().enumerate() {
                    out.push(if let Some(f) = args.get(1) {
                        self.call(f.clone(), V::Undefined, vec![v, V::Number(i as f64)])?
                    } else {
                        v
                    });
                }
                Ok(self.array(out))
            }
            ("Math", "random") => {
                let mut bytes = [0u8; 8];
                SystemRandom::new()
                    .fill(&mut bytes)
                    .map_err(|_| V::error("random source failed"))?;
                Ok(V::Number(
                    (u64::from_le_bytes(bytes) >> 11) as f64 / 9_007_199_254_740_992.0,
                ))
            }
            ("TextEncoderInstance", "encode") => {
                let text = if matches!(a, V::Undefined) {
                    String::new()
                } else {
                    a.text()
                };
                Ok(self.byte_array(text.as_bytes()))
            }
            ("Uint8Array", "from") => {
                let items = match &a {
                    V::String(s) => s.chars().map(|c| V::string(c.to_string())).collect(),
                    _ => self.items(&a)?,
                };
                let mut out = vec![];
                for (i, item) in items.into_iter().enumerate() {
                    let item = if let Some(f) = args.get(1) {
                        self.call(f.clone(), V::Undefined, vec![item, V::Number(i as f64)])?
                    } else {
                        item
                    };
                    out.push(V::Number((item.number() as i64).rem_euclid(256) as f64));
                }
                Ok(self.array(out))
            }
            ("String", "fromCharCode") => {
                let units: Vec<_> = args
                    .iter()
                    .map(|v| (v.number() as i64).rem_euclid(65536) as u16)
                    .collect();
                Ok(V::string(
                    String::from_utf16(&units)
                        .map_err(|_| V::error("unpaired UTF-16 surrogate"))?,
                ))
            }
            ("crypto.subtle", "digest") => {
                if a.text() != "SHA-256" {
                    return Err(V::error("unsupported digest algorithm"));
                }
                let value = self.byte_array(
                    digest::digest(
                        &digest::SHA256,
                        &self.bytes(args.get(1).unwrap_or(&V::Undefined))?,
                    )
                    .as_ref(),
                );
                Ok(self.promise(Box::pin(async move { Ok(value) })))
            }
            ("crypto.subtle", "importKey") => {
                if a.text() != "jwk" {
                    return Err(V::error("unsupported key encoding"));
                }
                let jwk = args.get(1).cloned().unwrap_or(V::Undefined);
                let algorithm = args.get(2).cloned().unwrap_or(V::Undefined);
                if self.get(&jwk, "kty")?.text() != "RSA"
                    || self.get(&algorithm, "name")?.text() != "RSASSA-PKCS1-v1_5"
                    || self.get(&self.get(&algorithm, "hash")?, "name")?.text() != "SHA-256"
                {
                    return Err(V::error("unsupported verification key"));
                }
                let alg = self.get(&jwk, "alg")?;
                if !alg.nullish() && alg.text() != "RS256" {
                    return Err(V::error("incompatible JWK algorithm"));
                }
                Ok(self.promise(Box::pin(async move { Ok(jwk) })))
            }
            ("crypto.subtle", "verify") => {
                if self.get(&a, "name")?.text() != "RSASSA-PKCS1-v1_5"
                    || self.get(&self.get(&a, "hash")?, "name")?.text() != "SHA-256"
                {
                    return Err(V::error("unsupported signature algorithm"));
                }
                let jwk = args.get(1).cloned().unwrap_or(V::Undefined);
                let decode = |name| {
                    base64::engine::general_purpose::URL_SAFE_NO_PAD
                        .decode(self.get(&jwk, name)?.text())
                        .map_err(|_| V::error("invalid JWK component"))
                };
                let n = decode("n")?;
                let e = decode("e")?;
                let signature = self.bytes(args.get(2).unwrap_or(&V::Undefined))?;
                let data = self.bytes(args.get(3).unwrap_or(&V::Undefined))?;
                let valid = signature::RsaPublicKeyComponents { n, e }
                    .verify(&signature::RSA_PKCS1_2048_8192_SHA256, &data, &signature)
                    .is_ok();
                Ok(self.promise(Box::pin(async move { Ok(V::Bool(valid)) })))
            }
            _ => Err(V::error(format!(
                "unsupported native operation {object}.{key}"
            ))),
        }
    }
}
