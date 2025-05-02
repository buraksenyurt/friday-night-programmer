use crate::bindables::{ActionMethod, Field, Form};
use crate::controls::Button;
use crate::controls::*;
use crate::dom::render;
use axum::response::Html;
use log::warn;

pub async fn index_handler() -> Html<String> {
    warn!("Index Handler call");
    let form = Form {
        action: "/create-new-user".into(),
        method: ActionMethod::Post,
        fields: vec![
            Field {
                name: "lblUsername".into(),
                control: Box::new(Label::new("Username", "txtUsername", "label-control")),
            },
            Field {
                name: "username".into(),
                control: Box::new(Textbox::new("txtUsername", "form-control")),
            },
            Field {
                name: "lblEmail".into(),
                control: Box::new(Label::new("Email", "txtEmail", "label-control")),
            },
            Field {
                name: "email".into(),
                control: Box::new(Textbox::new("txtEmail", "form-control")),
            },
            Field {
                name: "lblPassword".into(),
                control: Box::new(Label::new("Password", "txtPassword", "label-control")),
            },
            Field {
                name: "password".into(),
                control: Box::new(PasswordBox::new("txtPassword", "form-control")),
            },
            Field {
                name: "submit".into(),
                control: Box::new(Button::new("Save", "btn btn-primary")),
            },
        ],
    };
    let create_user_form = form.render();
    Html(render(&create_user_form))
}
