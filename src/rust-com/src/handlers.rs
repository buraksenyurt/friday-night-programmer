use crate::controls::Button;
use crate::controls::*;
use crate::dom::render;
use axum::response::Html;
use log::warn;

pub async fn index_handler() -> Html<String> {
    warn!("Index Handler call");
    let button = Button {
        label: "Save".to_string(),
        class: "btn btn-primary".to_string(),
    };
    let node = button.render();
    Html(render(&node))
}
