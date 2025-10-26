/*
    Log bırakmak için makro kullanmak yerine fonksiyon kullanmak kodun okunurluğunu daha da basitleştirir.
    Bir makroda genellikle expression ve çeşitli regex patternler kullanılır. Bu da kodun anlaşılmasını zorlaştırabilir.
    Özellikle basit işlemler için makro kullanmak yerine fonksiyon kullanmak çok daha kolaydır.
*/
macro_rules! log {
    ($msg:expr, $level:expr) => {
        println!("[{}]: {}", $level, $msg);
    };
}

/// Basit bir log fonksiyonu. 
/// Mesajı, log seviyesini alır ve formatlı bir şekilde ekrana basar.
///
/// # Arguments
/// * `message` - Log mesajı.
/// * `level` - Log seviyesi (örneğin: "INFO", "WARN", "ERROR").
fn log(message: &str, level: &str) {
    println!("[{}]: {}", level, message);
}

fn main() {
    log!("This is a warning message.", "WARN");

    log("This is an info message.", "INFO");
    log("This is an error message.", "ERROR");
}
