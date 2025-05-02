#[cfg(test)]
mod tests {
    use crate::controls::Button;
    use crate::controls::control::Control;
    use crate::dom::*;
    use super::*;

    #[test]
    fn button_render_test() {
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