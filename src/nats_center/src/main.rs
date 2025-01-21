use rand::Rng;
use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct Stock<'a> {
    symbol: &'a str,
    buy_price: f64,
    sell_price: f64,
}

fn get_stocks<'a>() -> Vec<Stock<'a>> {
    let symbols = vec!["AAPL", "AMZN", "GOGL", "MSFT", "NVID"];
    let mut rng = rand::thread_rng();

    symbols
        .into_iter()
        .map(|symbol| Stock {
            symbol,
            buy_price: rng.gen_range(50.0..100.0),
            sell_price: rng.gen_range(50.0..100.0),
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {

    let nats_conn = nats::connect("127.0.0.1:4222")?;
    let subject = "stocks.update";

    loop {
        let stocks = get_stocks();
        let message = serde_json::to_string(&stocks)?;

        nats_conn.publish(subject, &message)?;
        println!("Published stock prices: {}\n", message);

        thread::sleep(Duration::from_secs(10));
    }
}
