use async_openai::{Client, config::OpenAIConfig, types::chat::CreateChatCompletionResponse};
use colored::Colorize;
use entity::*;
use eyre::{Result, WrapErr};
use std::io::{Write, stdin, stdout};

mod entity;

pub async fn run(model: String, api_key: String, api_base: String) -> Result<()> {
    let mut context = LLMContext::new(model.clone());
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(api_base);
    let client = Client::with_config(config);

    println!(
        "{}",
        format!("AI Agent ready — model: {model}").cyan().bold()
    );
    println!(
        "{}",
        "Type your message and press Enter. Ctrl+C to quit.".dimmed()
    );
    println!();

    loop {
        let user_input = get_prompt()?;
        let user_message = Message::new_user(user_input);

        println!("{} {}", "You:".green().bold(), user_message.content.green());

        context.messages.push(user_message);

        print!("{}", "Thinking...".yellow().dimmed());
        stdout().flush()?;

        let llm_response = send(&context, &client).await?;

        print!("\r{}\r", " ".repeat(12));

        if !llm_response.message.content.is_empty() {
            println!(
                "{} {}",
                "Assistant:".blue().bold(),
                llm_response.message.content.white()
            );
        }

        let usage_str = format!(
            "tokens — prompt: {}, completion: {}, total: {}",
            llm_response.stats.prompt_tokens,
            llm_response.stats.completion_tokens,
            llm_response.stats.total_tokens
        );
        println!("{}\n", usage_str.dimmed());
        context.messages.push(llm_response.message);
    }
}

pub fn get_prompt() -> Result<String> {
    let mut prompt = String::new();
    print!("{} ", "›".green().bold());
    stdout().flush()?;
    stdin().read_line(&mut prompt)?;
    Ok(prompt.trim().to_string())
}

pub async fn send(context: &LLMContext, client: &Client<OpenAIConfig>) -> Result<LLMResponse> {
    let response: CreateChatCompletionResponse = client
        .chat()
        .create_byot(context)
        .await
        .context("Sending request to llm")?;
    let choice = response.choices[0].clone();
    let message = Message::from(choice.message);
    let (prompt_tokens, completion_tokens, total_tokens) = response
        .usage
        .map(|u| (u.prompt_tokens, u.completion_tokens, u.total_tokens))
        .unwrap_or((0, 0, 0));

    Ok(LLMResponse {
        message,
        stats: Stats {
            prompt_tokens,
            completion_tokens,
            total_tokens,
        },
    })
}
