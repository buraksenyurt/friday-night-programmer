use crate::dom::Node;

pub trait Control {
    fn render(&self) -> Node;
}