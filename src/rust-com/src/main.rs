use axum::Router;
use axum::response::Html;
use axum::routing::get;
use log::{info, warn};
use std::collections::HashMap;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    setup_log();

    let app = Router::new().route("/", get(index_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1903").await?;
    info!("Server is listening on: 0.0.0.0:1903");
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_log() {
    dotenvy::dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());
    unsafe {
        std::env::set_var("RUST_LOG", &log_level);
    }
    env_logger::init();
}

async fn index_handler() -> Html<String> {
    warn!("Index Handler call");
    let button = Button {
        label: "Save".to_string(),
        class: "btn btn-primary".to_string(),
    };
    let node = button.render();
    Html(render(&node))
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Text(String),
    Element(Element),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

pub trait Component {
    fn render(&self) -> Node;
}
#[derive(Debug)]
pub struct Button {
    pub label: String,
    pub class: String,
}

impl Component for Button {
    fn render(&self) -> Node {
        Node::Element(Element {
            tag: "button".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), self.class.clone());
                attrs
            },
            children: vec![Node::Text(self.label.clone())],
        })
    }
}

pub fn render(node: &Node) -> String {
    match node {
        Node::Text(text) => text.clone(),
        Node::Element(e) => {
            let attrs = e
                .attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ");
            let children = e
                .children
                .iter()
                .map(|child| render(child))
                .collect::<Vec<String>>()
                .join("");

            format!(
                "<{tag} {attrs}>{children}</{tag}>",
                tag = e.tag,
                attrs = attrs,
                children = children
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_test() {
        let button = Button {
            label: "Save".to_string(),
            class: "btn btn-primary".to_string(), // Bootstrap buton
        };
        let node = button.render();
        let actual = render(&node);
        let expected = "<button class=\"btn btn-primary\">Save</button>";
        assert_eq!(actual, expected);
    }
}
