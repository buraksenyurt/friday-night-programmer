use framework_macros::{TableName, greetings, invoice_code, invoice_code_safe};

fn main() {
    // function-like macro kullanım örnekleri

    greetings!("Input...");
    let invoice = invoice_code!("VEH", "BG", 1001);
    println!("Created invoice number: {}", invoice);

    // invoice_code!("VEH", get_type("VIP,User"), 1000); // Derleme zamanı hatası

    let invoice_safe = invoice_code_safe!("VEH", get_type("MBR,User"), 10046);
    println!("Created invoice number with safe macro: {}", invoice_safe);

    // derive-macro kullanım örnekleri
    println!("Game table name is {}", Game::table_name());
}

#[allow(dead_code)]
#[derive(TableName)]
struct Game {
    id: u32,
    title: String,
}

fn get_type(types: &str) -> String {
    types.split(',').next().unwrap_or("").to_string()
}
