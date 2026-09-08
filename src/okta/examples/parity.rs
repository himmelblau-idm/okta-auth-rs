//! JSON-lines bridge for differential tests. Never use with real credentials.
use okta_native_support::{Engine, V};
use std::io::{self, BufRead};
#[tokio::main(flavor = "current_thread")]
async fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("stdin");
        let request: serde_json::Value = serde_json::from_str(&line).expect("JSON");
        let cx = Engine::new();
        let result = async {
            let exports = okta::generated::initialize(&cx).map_err(|e| {
                eprintln!("initialization: {}", e.text());
                e
            })?;
            let f = exports
                .get(request["function"].as_str().unwrap())
                .cloned()
                .ok_or_else(|| V::string("unknown function"))?;
            let args = request["args"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| cx.from_json(v))
                .collect();
            let result = cx.execute(f, V::Undefined, args).await?;
            cx.to_json(&result)
        }
        .await;
        cx.cancel_pending();
        println!(
            "{}",
            match result {
                Ok(v) => serde_json::json!({"ok": v}),
                Err(error) => {
                    let name = cx
                        .get(&error, "name")
                        .ok()
                        .filter(|v| !v.nullish())
                        .map(|v| v.text())
                        .unwrap_or_else(|| "Error".into());
                    let message = cx
                        .get(&error, "message")
                        .ok()
                        .filter(|v| !v.nullish())
                        .map(|v| v.text())
                        .unwrap_or_else(|| error.text());
                    serde_json::json!({"error": {"name":name,"message":message}})
                }
            }
        );
    }
}
