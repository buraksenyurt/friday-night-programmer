#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

pub fn setup_log() {
    dotenvy::dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());
    unsafe {
        std::env::set_var("RUST_LOG", &log_level);
    }
    env_logger::init();
}

pub fn wrap_html(content: String) -> String {
    format!(
        r#"<!DOCTYPE html>
<html><head>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
</head><body class="container mt-5">
{content}
</body></html>"#
    )
}
