use crate::sdk::dom::render;
use crate::models::subscriber::Subscriber;
use crate::sdk::utils::wrap_html;
use axum::response::Html;
use log::warn;
use std::collections::HashMap;
use crate::views::create_subscriber::create;

pub async fn handle_create_user_view() -> Html<String> {
    let form = create();
    Html(wrap_html(render(&form.render())))
}

pub async fn handle_create_user_post(
    axum::Form(data): axum::Form<HashMap<String, String>>,
) -> Html<String> {
    let subscriber = Subscriber::from_input(&data);
    warn!("user: {:?}", subscriber);
    Html(wrap_html(format!(
        "<p>Created {} with {}</p>",
        subscriber.username, subscriber.email
    )))
}
