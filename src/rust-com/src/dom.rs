use std::collections::HashMap;

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