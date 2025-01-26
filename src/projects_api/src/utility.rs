use chrono::Utc;

pub fn parse_datetime(value: &str) -> chrono::DateTime<Utc> {
    value.parse().unwrap_or_else(|_| Utc::now())
}

pub fn format_datetime(value: &chrono::DateTime<Utc>) -> String {
    value.to_rfc3339()
}
