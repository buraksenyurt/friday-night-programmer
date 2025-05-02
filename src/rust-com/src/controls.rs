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

impl Button {
    pub fn new(label: &str, class: &str) -> Self {
        Button {
            label: label.into(),
            class: class.to_string(),
        }
    }
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

impl Label {
    pub fn new(text: &str, bounded: &str, class: &str) -> Self {
        Label {
            text: text.into(),
            bounded: bounded.into(),
            class: class.into(),
        }
    }
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

impl Textbox {
    pub fn new(name: &str, class: &str) -> Self {
        Textbox {
            id: name.into(),
            name: name.into(),
            value: None,
            class: class.into(),
        }
    }
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

impl PasswordBox {
    pub fn new(name: &str, class: &str) -> Self {
        PasswordBox {
            id: name.into(),
            name: name.into(),
            class: class.into(),
        }
    }
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
