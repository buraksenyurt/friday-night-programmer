use framework_macros::*;

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
    println!("Player table name is {}", Player::table_name()); // quote, syn kullanılan versiyon

    let game = Game {
        id: 1,
        title: "Example Game".to_string(),
    };
    println!("Game created event: {}", game.created_event());

    let product_dto = ProductDto {
        title : "".to_string(),
        sku : "PRD-1234".to_string(),
        list_price: 4.99
    };

    match product_dto.validate() {
        Ok(_)=> println!("Valid"),
        Err(e) => println!("Error: {e}"),
    }
}

// Her bir domain modeli için otomatik event tanımı üreten bir derive macro senaryosu
trait CreatedEvent {
    fn created_event(&self) -> String;
}

#[allow(dead_code)]
#[derive(TableName, CreatedEvent)]
struct Game {
    id: u32,
    title: String,
}

fn get_type(types: &str) -> String {
    types.split(',').next().unwrap_or("").to_string()
}

#[allow(dead_code)]
#[derive(TableNameSafe)]
struct Player {
    id: u32,
    name: String,
    point: f32,
}

#[derive(Validator)]
#[allow(dead_code)]
struct ProductDto {
    title: String,
    sku: String,
    list_price: f32
}