use crate::dom::render;
use crate::models::subscriber::Subscriber;
use crate::utils::wrap_html;
use crate::views::*;
use axum::response::Html;
use log::warn;
use std::collections::HashMap;

pub async fn handle_create_user_view() -> Html<String> {
    let form = create();
    Html(wrap_html(render(&form.render())))
}

pub async fn handle_create_user_post(
    axum::Form(data): axum::Form<HashMap<String, String>>,
) -> Html<String> {
    let user = Subscriber::from_input(&data);
    warn!("user: {:?}", user);
    Html(wrap_html(format!(
        "<p>Created {} with {}</p>",
        user.username, user.email
    )))
}
