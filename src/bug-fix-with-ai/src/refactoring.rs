pub fn calculate_price(price: f32, tax: f32) -> f32 {
    let result = price + (price * tax/100.0);
    println!("Price: {}, Tax: {}, Total: {}", price, tax, result);
    return result;
}