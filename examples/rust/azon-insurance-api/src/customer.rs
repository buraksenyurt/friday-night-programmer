use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Customer {
    pub customer_id:i32,
    pub full_name:String,
    pub max_policy_limit:f64,
    pub policy_start_date:String,
    pub policy_end_date:String,
}