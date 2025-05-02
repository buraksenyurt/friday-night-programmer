use crate::controls::control::Control;
use axum::response::Html;
use log::warn;
use crate::controls::Button;
use crate::dom::render;

pub async fn index_handler() -> Html<String> {
    warn!("Index Handler call");
    let button = Button {
        label: "Save".to_string(),
        class: "btn btn-primary".to_string(),
    };
    let node = button.render();
    Html(render(&node))
}