use crate::sdk::controls::Control;
use crate::sdk::dom::{Element, Node};
use crate::hashmap;
use std::fmt::Display;

#[allow(dead_code)]
pub enum ActionMethod {
    Post,
    Put,
    Get,
    Delete,
    Patch,
}
impl Display for ActionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Post => {
                write!(f, "POST")
            }
            Self::Put => {
                write!(f, "PUT")
            }
            Self::Get => {
                write!(f, "GET")
            }
            Self::Delete => {
                write!(f, "DELETE")
            }
            Self::Patch => {
                write!(f, "PATCH")
            }
        }
    }
}
#[allow(dead_code)]
pub struct Field {
    pub name: String,
    pub control: Box<dyn Control>,
}

pub struct Form {
    pub fields: Vec<Field>,
    pub action: String,
    pub method: ActionMethod,
}

impl Form {
    pub fn render(&self) -> Node {
        let children = self
            .fields
            .iter()
            .map(|binding| binding.control.render())
            .collect();

        Node::Element(Element {
            tag: "form".into(),
            attributes: hashmap! {
                "action".into() => self.action.clone(),
                "method".into() => self.method.to_string(),
                "class".into() => "p-4".into(),
            },
            children,
        })
    }
}
