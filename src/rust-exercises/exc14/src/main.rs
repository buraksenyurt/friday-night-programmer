use anyhow::{Context, Result};
use std::io;
use std::num::ParseIntError;

fn main() {
    match run() {
        Ok(_) => println!("All operations completed successfully."),
        Err(e) => {
            // Burada oluşan tüm hataları ve context bilgilerini yazdırabiliriz
            let mut source = e.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("  {}. {}", level, err);
                source = err.source();
                level += 1;
            }

            // İstersek bir anyhow::Error içindeki spesifik hata türlerine de erişebiliriz
            // Bunu, downcast_ref fonksiyonu ile sağlayabiliriz.
            if let Some(io_err) = e.downcast_ref::<io::Error>() {
                println!("IO Error details: {:?}", io_err.kind());
            }

            // Örneğin detaya gelen hata ParseIntError ise,
            if let Some(parse_err) = e.downcast_ref::<ParseIntError>() {
                println!("Parse Error details: {}", parse_err);
            }
        }
    }
}

// Bu fonksiyonda farklı senaryoları test ediyoruz
// Her adımda context ekleyerek hataların nerede oluştuğunu daha iyi anlamak mümkün.
// Kod tabanı geniş uygulamalarda bu yaklaşım hata ayıklamayı kolaylaştırır.
fn run() -> Result<()> {
    // Senaryoları tek tek açarak deneyebiliriz.
    add_product(1001, "ElCi Laptop", 999.99)
        .with_context(|| "Failed in scenario 1 - product not found")?;

    add_product(1003, "AyFone Smartphone", -399.99)
        .with_context(|| "Failed in scenario 3 - negative price test")?;

    add_product(9999, "Mouse Optical", 100.45)
        .with_context(|| "Failed in scenario 4 - database error test")?;

    Ok(())
}

// business modülünde ürün ekleme fonksiyonu
// anyhow ile context ekleme örneği
fn add_product(id: u32, name: &str, price: f64) -> Result<()> {
    validate_product(id, name, price)
        .with_context(|| format!("Product validation failed for ID: {}", id))?;
    write(&Product::new(id, name, price))
        .with_context(|| format!("Database operation failed for product: {}", name))?;

    Ok(())
}

// business modülünde çağrılan bir ürün doğrulama fonksiyonu
fn validate_product(id: u32, name: &str, price: f64) -> Result<()> {
    if id == 0 {
        return Err(anyhow::anyhow!("Product ID cannot be zero"));
    }

    if name.is_empty() {
        return Err(anyhow::anyhow!("Product name cannot be empty"));
    }

    if name.len() > 50 {
        return Err(anyhow::anyhow!(
            "Product name too long: {} characters (max: 50)",
            name.len()
        ));
    }

    if price < 0.0 {
        return Err(anyhow::anyhow!(
            "Product price cannot be negative: ${:.2}",
            price
        ));
    }

    if price > 10000.0 {
        return Err(anyhow::anyhow!(
            "Product price too high: ${:.2} (max: $10000.00)",
            price
        ));
    }

    Ok(())
}

// db modülünde bir ürün yazma fonksiyonu
// En alt katman - io::Error döndürüyor, anyhow yukarıdaki katmanlarda kullanılıyor
fn write(product: &Product) -> io::Result<()> {
    // Sadece database bağlantı hatasını simüle etmek için
    if product.id == 9999 {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "Database connection failed",
        ));
    }

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl Product {
    fn new(id: u32, name: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
        }
    }
}
