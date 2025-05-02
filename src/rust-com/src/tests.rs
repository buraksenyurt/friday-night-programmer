#[cfg(test)]
mod tests {
    use crate::controls::Button;
    use crate::controls::*;
    use crate::dom::*;

    #[test]
    fn button_render_test() {
        let button = Button {
            label: "Save".to_string(),
            class: "btn btn-primary".to_string(),
        };
        let node = button.render();
        let actual = render(&node);
        let expected = "<button class=\"btn btn-primary\">Save</button>";
        assert_eq!(actual, expected);
    }

    #[test]
    fn label_render_test() {
        let label = Label {
            bounded: "FirstNameText".to_string(),
            text: "First Name".to_string(),
            class: "form-label".to_string(),
        };
        let node = label.render();
        let actual = render(&node);
        let expected = "<label class=\"form-label\" for=\"FirstNameText\">First Name</label>";
        assert_eq!(actual, expected);
    }

    #[test]
    fn textbox_render_test() {
        let text_box = Textbox {
            id: "FirstNameTextbox".to_string(),
            name: "FirstNameTextbox".to_string(),
            value: None,
            class: "form-control".to_string(),
        };
        let node = text_box.render();
        let actual = render(&node);
        let expected = "<input class=\"form-control\" id=\"FirstNameTextbox\" name=\"FirstNameTextbox\" value=\"\"></input>";
        assert_eq!(actual, expected);
    }

    #[test]
    fn password_box_render_test() {
        let pass_box = PasswordBox {
            id: "UserPasswordBox".to_string(),
            name: "UserPasswordBox".to_string(),
            class: "form-control".to_string(),
        };
        let node = pass_box.render();
        let actual = render(&node);
        let expected = "<input class=\"form-control\" id=\"UserPasswordBox\" name=\"UserPasswordBox\" type=\"password\"></input>";
        assert_eq!(actual, expected);
    }
}
