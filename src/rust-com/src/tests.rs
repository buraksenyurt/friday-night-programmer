#[cfg(test)]
mod tests {
    use crate::sdk::binding::*;
    use crate::sdk::controls::Button;
    use crate::sdk::controls::*;
    use crate::sdk::dom::*;

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

    #[test]
    fn bindable_form_render_test() {
        let form = Form {
            action: "/list-products".into(),
            method: ActionMethod::Post,
            fields: vec![
                Field {
                    name: "title".into(),
                    control: Box::new(Label::new("Product Name", "", "form-control")),
                },
                Field {
                    name: "productName".into(),
                    control: Box::new(Textbox::new("txtProductName", "form-control")),
                },
                Field {
                    name: "submit".into(),
                    control: Box::new(Button::new("Search", "btn btn-secondary")),
                },
            ],
        };
        let node = form.render();
        let actual = render(&node);
        let expected = "<form action=\"/list-products\" class=\"p-4\" method=\"POST\"><label class=\"form-control\" for=\"\">Product Name</label><input class=\"form-control\" id=\"txtProductName\" name=\"txtProductName\" value=\"\"></input><button class=\"btn btn-secondary\">Search</button></form>";
        assert_eq!(actual, expected);
    }
}
