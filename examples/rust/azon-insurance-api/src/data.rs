use chrono::NaiveDate;
use rand::Rng;
use crate::customer::Customer;

fn get_customer_names() -> Vec<String> {
    vec![
        "Micky Maus".to_string(),
        "Betty Rubble".to_string(),
        "Donny Duck".to_string(),
        "Pinkie Panther".to_string(),
        "Fred Flinestone".to_string(),
        "Scooby Doo".to_string(),
        "Road Runner".to_string(),
        "Goffy Goof".to_string(),
        "Snoopee Dog".to_string(),
        "Tommy Cat".to_string(),
        "Jerry Mouse".to_string(),
        "Popeye Sailor".to_string(),
        "Garfield Cat".to_string(),
        "Bugs Bunner".to_string(),
        "Daffee Duck".to_string(),
        "Elmer Fudd".to_string(),
        "Wile E. Cyote".to_string(),
        "Barney Ruble".to_string(),
        "Shaggy Rogers".to_string(),
        "Wilma Flintstone".to_string(),
    ]
}

fn generate_random_policy_dates() -> (String, String) {
    let mut rng = rand::thread_rng();

    let start_year = 2024;
    let start_month = rng.gen_range(1..=12);
    let start_day = rng.gen_range(1..=28);
    let policy_start_date = NaiveDate::from_ymd_opt(start_year, start_month, start_day).unwrap().to_string();

    let end_year = start_year + 2;
    let end_month = rng.gen_range(1..=12);
    let end_day = rng.gen_range(1..=28);
    let policy_end_date = NaiveDate::from_ymd_opt(end_year, end_month, end_day).unwrap().to_string();

    (policy_start_date, policy_end_date)
}

fn generate_random_policy_limit() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10_000.0..125_000.0)
}

pub fn generate_customers() -> Vec<Customer> {
    let names = get_customer_names();
    let mut customers = Vec::new();

    for (i, name) in names.iter().enumerate() {
        let (policy_start_date, policy_end_date) =
            generate_random_policy_dates();

        customers.push(Customer {
            full_name: name.clone(),
            customer_id: i as i32 + 1,
            max_policy_limit: generate_random_policy_limit(),
            policy_start_date,
            policy_end_date
        });
    }

    customers
}