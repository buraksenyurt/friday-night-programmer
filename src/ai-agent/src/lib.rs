use eyre::Result;
use std::io::{stdin, stdout, Write};
use serde::{Deserialize, Serialize};
use async_openai::config::OpenAIConfig;
use async_openai::Client;

pub async fn run(model: String,api_key:String,api_base:String) -> Result<()> {
    let mut context = Context::new(model);
    let config = OpenAIConfig {
        api_key,
        api_base,
        ..Default::default()
    };
    let client = Client::with_config(config);
    loop{
        let user_input = get_prompt()?;
        dbg!("User input: {}", user_input);
    }
}

pub fn get_prompt() -> Result<String> {
    let mut prompt = String::new();
    print!("> ");
    stdout().flush()?;
    stdin().read_line(&mut prompt)?;
    Ok(prompt.trim().to_string())
}

pub async fn call_llm(context: &Context,client : &Client) -> Result<String> {

    Ok("This is a response from the LLM.".to_string())
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Context {
    pub model: String,
    pub messages: Vec<Message>,
}

impl Context {
    pub fn new(model: String) -> Self {
        let system_message = Message::new_system("You are a helpful assistant who provides accurate and concise information.".to_string());
        Self { model, messages: vec![system_message] }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new_system(content: String) -> Self {
        Self { role: Role::System, content }
    }
    pub fn new_user(content: String) -> Self {
        Self { role: Role::User, content }
    }
    pub fn new_assistant(content: String) -> Self {
        Self { role: Role::Assistant, content }
    }
}