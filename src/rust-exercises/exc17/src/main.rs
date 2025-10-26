fn main() {
    let api_paths = vec![
        String::from("/api/v1/users"),
        String::from("/api/v1/orders"),
        String::from("/api/v1/products"),
    ];

    for path in api_paths {
        // // Bad Practice
        // route_request_owned(path.clone());

        // Good Practice
        route_request(&path);
    }
}

// Bad Practice: Kopya üzerinden işlem yapmak
#[allow(dead_code)]
fn route_request_owned(path: String) {
    match path.as_str() {
        "/api/v1/users" => println!("Routing to Users API"),
        "/api/v1/orders" => println!("Routing to Orders API"),
        "/api/v1/products" => println!("Routing to Products API"),
        _ => println!("404 Not Found"),
    }
}

// Good Practice: Referans üzerinden işlem yapmak
fn route_request(path: &str) {
    match path {
        "/api/v1/users" => println!("Routing to Users API"),
        "/api/v1/orders" => println!("Routing to Orders API"),
        "/api/v1/products" => println!("Routing to Products API"),
        _ => println!("404 Not Found"),
    }
}
