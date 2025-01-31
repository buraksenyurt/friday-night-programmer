use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationResponse<T> {
    is_success: bool,
    message: String,
    error: Option<String>,
    data: Option<T>,
}

impl<T> OperationResponse<T> {
    pub fn new(is_success: bool, message: &str, error: Option<&str>, data: Option<T>) -> Self {
        OperationResponse {
            is_success,
            message: message.to_string(),
            error: error.map(|e| e.to_string()),
            data,
        }
    }
}
