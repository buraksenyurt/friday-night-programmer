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
            let mut attrs_vec: Vec<_> = e.attributes.iter().collect();
            attrs_vec.sort_by_key(|(k, _)| *k);

            let attrs = attrs_vec
                .into_iter()
                .map(|(k, v)| format!(r#"{k}="{v}""#))
                .collect::<Vec<_>>()
                .join(" ");

            let children = e.children.iter().map(render).collect::<Vec<_>>().join("");

            format!("<{tag} {attrs}>{children}</{tag}>", tag = e.tag)
        }
    }
}
