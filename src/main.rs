use std::process;

use anyhow::{Context, Result};
use lambda_runtime::handler_fn;
use serde_json::{json, Value};

async fn lambda_handler(event: Value, _: lambda_runtime::Context) -> Result<Value> {
    let name = event["name"].as_str().context("name does not exist.")?;
    Ok(json!({ "message": format!("Hello {}!", name) }))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let func = handler_fn(lambda_handler);
    if let Err(err) = lambda_runtime::run(func).await {
        log::error!("{:?}", err);
        process::exit(1);
    }
}
