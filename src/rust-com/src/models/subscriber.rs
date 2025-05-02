use std::collections::HashMap;

#[derive(Debug)]
pub struct Subscriber {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Subscriber {
    pub fn from_input(input: &HashMap<String, String>) -> Self {
        Self {
            username: input.get("txtUsername").cloned().unwrap_or_default(),
            email: input.get("txtEmail").cloned().unwrap_or_default(),
            password: input.get("txtPassword").cloned().unwrap_or_default(),
        }
    }
}
