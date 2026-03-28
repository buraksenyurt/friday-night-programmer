use async_openai::{Client, config::OpenAIConfig, types::chat::CreateChatCompletionResponse};
use entity::*;
use eyre::{Result, WrapErr};
use std::io::{Write, stdin, stdout};

mod entity;

pub async fn run(model: String, api_key: String, api_base: String) -> Result<()> {
    let mut context = LLMContext::new(model);
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(api_base);
    let client = Client::with_config(config);
    loop {
        let user_input = get_prompt()?;
        let user_message = Message::new_user(user_input);

        println!("{user_message}");

        context.messages.push(user_message);

        let llm_response = send(&context, &client).await?;

        if !llm_response.content.is_empty() {
            println!("{llm_response}");
        }

        // context.messages.push(llm_response.clone());
    }
}

pub fn get_prompt() -> Result<String> {
    let mut prompt = String::new();
    print!("> ");
    stdout().flush()?;
    stdin().read_line(&mut prompt)?;
    Ok(prompt.trim().to_string())
}

pub async fn send(context: &LLMContext, client: &Client<OpenAIConfig>) -> Result<Message> {
    let response: CreateChatCompletionResponse = client
        .chat()
        .create_byot(context)
        .await
        .context("Sending request to llm")?;
    let choice = response.choices[0].clone();
    let message = Message::from(choice.message);

    Ok(message)
}
