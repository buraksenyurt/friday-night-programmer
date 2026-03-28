use eyre::Result;
use ai_agent::run;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    let model = env::var("LLM_MODEL").unwrap_or_else(|_| "qwen/qwen3-14b".to_string());
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_base = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "http://127.0.0.1:1234".to_string());

    run(model, api_key, api_base).await?;

    Ok(())
}