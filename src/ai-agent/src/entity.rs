use async_openai::types::chat::ChatCompletionResponseMessage;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl From<async_openai::types::chat::Role> for Role {
    fn from(value: async_openai::types::chat::Role) -> Self {
        match value {
            async_openai::types::chat::Role::System => Self::System,
            async_openai::types::chat::Role::User => Self::User,
            async_openai::types::chat::Role::Assistant => Self::Assistant,
            _ => unimplemented!(),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::User => "User",
            Role::Assistant => "Assistant",
            Role::System => "System",
        };
        write!(f, "{}", role_str)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new_system(content: String) -> Self {
        Self {
            role: Role::System,
            content,
        }
    }
    pub fn new_user(content: String) -> Self {
        Self {
            role: Role::User,
            content,
        }
    }
    pub fn new_assistant(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.role, self.content)
    }
}

impl From<ChatCompletionResponseMessage> for Message {
    fn from(value: ChatCompletionResponseMessage) -> Self {
        let role = Role::from(value.role);
        let content = value.content.unwrap_or_default();

        Self { role, content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug)]
pub struct LLMResponse {
    pub message: Message,
    pub stats: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMContext {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub enable_thinking: bool,
}

impl LLMContext {
    pub fn new(model: String) -> Self {
        let system_message = Message::new_system(
            "You are a helpful assistant who provides accurate and concise information."
                .to_string(),
        );
        Self {
            model,
            messages: vec![system_message],
            max_tokens: 2048,
            enable_thinking: false,
        }
    }
}
