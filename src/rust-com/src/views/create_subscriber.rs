use crate::sdk::binding::{ActionMethod, Field, Form};
use crate::sdk::controls::{Button, Label, PasswordBox, Textbox};

pub fn create() -> Form {
    Form {
        action: "/create-new-user".into(),
        method: ActionMethod::Post,
        fields: vec![
            Field {
                name: "lblUsername".into(),
                control: Box::new(Label::new("Username", "txtUsername", "label-control")),
            },
            Field {
                name: "username".into(),
                control: Box::new(Textbox::new("txtUsername", "form-control mb-2")),
            },
            Field {
                name: "lblEmail".into(),
                control: Box::new(Label::new("Email", "txtEmail", "label-control")),
            },
            Field {
                name: "email".into(),
                control: Box::new(Textbox::new("txtEmail", "form-control mb-2")),
            },
            Field {
                name: "lblPassword".into(),
                control: Box::new(Label::new("Password", "txtPassword", "label-control")),
            },
            Field {
                name: "password".into(),
                control: Box::new(PasswordBox::new("txtPassword", "form-control mb-3")),
            },
            Field {
                name: "submit".into(),
                control: Box::new(Button::new("Save", "btn btn-primary")),
            },
        ],
    }
}
