use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Stock<'a> {
    symbol: &'a str,
    buy_price: f64,
    sell_price: f64,
}

fn main() -> Result<(), Box<dyn Error>> {

    let nats_conn = nats::connect("127.0.0.1:4222")?;
    let subject = "stocks.update";
    let subscription = nats_conn.subscribe(subject)?;

    println!("Listening for stock updates on '{}'", subject);

    for msg in subscription.messages() {
        let stocks: Vec<Stock> = serde_json::from_slice(&msg.data)?;
        println!("Received stock updates: {:#?}\n", stocks);
    }

    Ok(())
}