use std::collections::HashMap;
use crate::controls::control::Control;
use crate::dom::{Element, Node};

#[derive(Debug)]
pub struct Button {
    pub label: String,
    pub class: String,
}

impl Control for Button {
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