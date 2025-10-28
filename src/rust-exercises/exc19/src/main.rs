use std::fs::File;
use std::io;
use std::num;

fn main() {
    let result = search("WARNING");
    match result {
        Ok(content) => println!("Search successful: {}", content),
        Err(e) => match e {
            AppError::Io(err) => eprintln!("I/O Error: {}", err),
            AppError::Parse(err) => eprintln!("Parse Error: {}", err),
            AppError::Auth(msg) => eprintln!("Authentication Error: {}", msg),
            AppError::NotFound(msg) => eprintln!("Not Found Error: {}", msg),
        },
    }

    let num_str = "32a14";
    if let Err(e) = parse_number(num_str) {
        match e {
            AppError::Parse(err) => eprintln!("Failed to parse number: {}", err),
            _ => eprintln!("An unexpected error occurred"),
        }
    }

    // into kullanımı ile de türler arası dönüşüm yapılabilir
    // Burada io::Error türündeki hata AppError türüne dönüştürülmektedir
    let io_error = io::Error::new(io::ErrorKind::Other, "an I/O error occurred");
    let app_error: AppError = io_error.into();
    match app_error {
        AppError::Io(err) => eprintln!("Converted I/O Error: {}", err),
        _ => eprintln!("An unexpected error occurred"),
    }
}

fn parse_number(s: &str) -> Result<i32, AppError> {
    /*
    parse fonksiyonu str türündeki bir veriyi i32 türüne dönüştürmeye çalışır.
    Eğer dönüşüm başarılı olursa, Ok(num) döner aksi durumda ParseIntError türünde bir hata oluşur.
    From trait implemente edildiği için bu hata AppError türüne otomatikman dönüştürülür.
    */
    let num: i32 = s.parse()?;
    Ok(num)
}

fn search(query: &str) -> Result<String, AppError> {
    /*
    Bu fonksiyon belirtilen dosyayı açar ve içeriğinde query parametresi ile gelen veriyi arar.
    Eğer dosya açılamazsa io::Error türünde bir hata oluşur ve bu hata AppError türüne dönüştürülür.
    Zira, From trait implemente edilmiştir.
    */
    let f = File::open("games.dat")?;
    println!("File opened successfully: {:?}", f);
    println!("Searching for query: {}", query);
    Ok(String::from("Content found"))
}

#[allow(dead_code)]
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Auth(String),
    NotFound(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError::Parse(error)
    }
}
