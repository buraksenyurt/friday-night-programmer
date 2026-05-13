use framework_macros::{TableName, greetings, invoice_code};
fn main() {
    // function-like macro kullanım örnekleri

    greetings!("Input...");
    let invoice = invoice_code!("VEH", "BG", 1001);
    println!("Created invoice number: {}", invoice);

    // derive-macro kullanım örnekleri
    println!("Game table name is {}", Game::table_name());
}

#[allow(dead_code)]
#[derive(TableName)]
struct Game {
    id: u32,
    title: String,
}
