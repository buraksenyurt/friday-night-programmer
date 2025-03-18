// Import the reqwest library
use reqwest::Client;

#[derive(Clone)]
// Extend this struct with the feature you will need for your application
pub struct ApplicationState {
    // This will be available to all your route handlers
    pub fetch: Client,
}

pub fn main() -> ApplicationState {
    env_logger::init();

    let fetch = Client::new();
    return ApplicationState { fetch };
}
