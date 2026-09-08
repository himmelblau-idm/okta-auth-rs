use okta::{
    AuthInput, AuthOptions, AuthStatus, ClientConfig, FormInput, PublicClientApplication, StepKind,
};
use std::io::{self, Write};

fn line(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    Ok(value.trim().to_string())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let issuer = args
        .next()
        .ok_or("usage: okta-mfa ISSUER CLIENT_ID REDIRECT_URI")?;
    let client_id = args.next().ok_or("missing CLIENT_ID")?;
    let redirect = args.next().ok_or("missing REDIRECT_URI")?;
    let client = PublicClientApplication::new(ClientConfig::new(issuer, client_id, redirect))?;
    let mut flow = client.initiate_auth_flow(AuthOptions::default()).await?;
    loop {
        if let Some(messages) = flow.messages() {
            println!("messages: {}", serde_json::to_string_pretty(messages)?);
        }
        match flow.status() {
            AuthStatus::Success => {
                if let Some(tokens) = flow.tokens() {
                    println!(
                        "authentication succeeded\nclaims: {}",
                        serde_json::to_string_pretty(
                            tokens.claims().unwrap_or(&serde_json::Value::Null)
                        )?
                    );
                } else {
                    println!("authentication succeeded");
                }
                return Ok(());
            }
            AuthStatus::Terminal | AuthStatus::Canceled => {
                return Err("authentication ended without tokens".into())
            }
            AuthStatus::Uncertain => {
                println!("request outcome is uncertain; introspecting current server state");
                client.introspect_auth_flow(&mut flow).await?;
                continue;
            }
            AuthStatus::Pending => {}
        }
        for (index, step) in flow.steps().iter().enumerate() {
            println!(
                "[{index}] {} ({:?})\n{}",
                step.name(),
                step.kind,
                serde_json::to_string_pretty(step.schema())?
            );
        }
        let index: usize = line("select step: ")?.parse()?;
        let step = flow.steps().get(index).ok_or("invalid step")?.clone();
        match step.kind {
            StepKind::Poll => {
                client
                    .continue_auth_flow(&mut flow, step.id(), AuthInput::Poll)
                    .await?
            }
            StepKind::Form => {
                let values = serde_json::from_str(&line("form values as JSON object: ")?)?;
                client
                    .continue_auth_flow(
                        &mut flow,
                        step.id(),
                        AuthInput::Form(FormInput::new(values)?),
                    )
                    .await?;
            }
            StepKind::Redirect | StepKind::External => {
                println!(
                    "caller-managed handoff required: {}",
                    serde_json::to_string_pretty(step.schema())?
                );
                let continuation = client.export_auth_flow(&flow)?;
                println!("continuation contains sensitive material ({} bytes); store or seal it before leaving this process",continuation.as_bytes().len());
                return Ok(());
            }
        }
    }
}
