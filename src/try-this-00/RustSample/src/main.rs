fn main() {
    let mut part = Part {
        id: 1,
        name: String::from("Widget"),
        stock_level: 10,
    };

    println!("Initial stock level of {}: {}", part.name, part.stock_level);
    change_stock_level(&mut part, 5);
    println!("Stock level after adding 5: {}", part.stock_level);
}

fn change_stock_level(part: &mut Part, change: i32) {
    part.stock_level += change;
}

struct Part {
    id: u32,
    name: String,
    stock_level: i32,
}