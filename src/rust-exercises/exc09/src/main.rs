fn main() {
    let log_data = vec![
        String::from("INFO: Application started"),
        String::from("ERROR: Failed to load configuration"),
        String::from("INFO: User logged in"),
        String::from("ERROR: Database connection lost"),
    ];

    println!("--- Lazy Evaluation Results ---");
    let error_logs = get_error_logs_lazy(&log_data);
    error_logs.iter().for_each(|log| println!("{}", log));

    println!("--- Eager Evaluation Results ---");
    let error_logs = get_error_logs_eager(&log_data);
    error_logs.iter().for_each(|log| println!("{}", log));
}

/// Basit bir log analiz fonksiyonu (Lazy Evaluation ile)
/// Log verisi alır ve "ERROR" içeren satırları döner
///
/// # Arguments
///
/// * `log_data` - Log verisi içeren String vektörü
///
/// # Returns
///
/// * `impl Iterator<Item=String>` - "ERROR" içeren log satırlarını üreten iterator
fn get_error_logs_lazy(log_data: &[String]) -> Vec<String> {
    /*
        Bu yaklaşımda Lazy Evaluation kullanılmaktadır.
        Log verisi üzerinde bir iterator oluşturulur ve
        "ERROR" içeren satırlar filtrelenir.
        Bu sayede gereksiz yere tüm veriyi işlemekten kaçınılır.
    */
    log_data
        .into_iter()
        .filter(|line| line.contains("ERROR"))
        .map(|line| {
            let columns = line.split(": ").collect::<Vec<&str>>();
            format!(
                "Critical Error Found: {}",
                columns.last().unwrap_or(&"Unknown Error")
            )
        })
        .collect()
}

/// Basit bir log analiz fonksiyonu (Eager Evaluation ile)
/// Log verisi alır ve "ERROR" içeren satırları döner
///
/// # Arguments
///
/// * `log_data` - Log verisi içeren String vektörü
///
/// # Returns
///
/// * `Vec<String>` - "ERROR" içeren log satırlarını içeren vektör
fn get_error_logs_eager(log_data: &[String]) -> Vec<String> {
    /*
        Bu yaklaşımda Eager Evaluation kullanılmaktadır.
        Tüm log verisi işlenir ve "ERROR" içeren satırlar
        hemen döndürülür.
    */
    let mut error_logs = Vec::new();
    for line in log_data {
        if line.contains("ERROR") {
            let columns: Vec<&str> = line.split(": ").collect();
            let formatted_log = format!(
                "Critical Error Found: {}",
                columns.last().unwrap_or(&"Unknown Error")
            );
            error_logs.push(formatted_log);
        }
    }
    error_logs
}
