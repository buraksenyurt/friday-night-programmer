use crate::dom::*;
use crate::hashmap;

pub trait Control {
    fn render(&self) -> Node;
}

#[derive(Debug)]
pub struct Button {
    pub label: String,
    pub class: String,
}

impl Control for Button {
    fn render(&self) -> Node {
        Node::Element(Element {
            tag: "button".to_string(),
            attributes: hashmap! {
                "class".into() => self.class.clone()
            },
            children: vec![Node::Text(self.label.clone())],
        })
    }
}

#[derive(Debug)]
pub struct Label {
    pub text: String,
    pub bounded: String,
    pub class: String,
}

impl Control for Label {
    fn render(&self) -> Node {
        Node::Element(Element {
            tag: "label".into(),
            attributes: hashmap! {
                "for".into() => self.bounded.clone(),
                "class".into() => self.class.clone(),
            },
            children: vec![Node::Text(self.text.clone())],
        })
    }
}

#[derive(Debug)]
pub struct Textbox {
    pub id: String,
    pub name: String,
    pub value: Option<String>,
    pub class: String,
}

impl Control for Textbox {
    fn render(&self) -> Node {
        Node::Element(Element {
            tag: "input".into(),
            attributes: hashmap! {
                "name".into() => self.name.clone(),
                "value".into() => if let Some(val) = &self.value {
                    val.clone()
                } else {
                    "".to_string()
                },
                "class".into() => self.class.clone(),
                "id".into() => self.id.clone(),
            },
            children: vec![],
        })
    }
}

#[derive(Debug)]
pub struct PasswordBox {
    pub name: String,
    pub id: String,
    pub class: String,
}

impl Control for PasswordBox {
    fn render(&self) -> Node {
        Node::Element(Element {
            tag: "input".into(),
            attributes: hashmap! {
                "type".into() => "password".into(),
                "name".into() => self.name.clone(),
                "id".into() => self.id.clone(),
                "class".into() => self.class.clone(),
            },
            children: vec![],
        })
    }
}
