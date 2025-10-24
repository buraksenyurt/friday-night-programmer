# Rust Kodlama İdmanları

Bu dokümanda rust bilgilerimizi tazelemek için çeşitli kaynaklardan derlediğim örneklere yer verilmektedir.

## Başlangıç Seviyesi

### Örnek 1: Unwrap/Expect Tuzaklarından Kaçınmak

Rust'ın güçlü yönlerinden birisi Option< T > ve Result<T, E> tipleri ile hata yönetimidir. Bazen özellikle development safhasındayken unwrap ve expect kullanarak ilerleyebiliriz zira match veya if let kullanmak kodu uzatabilir. Ancak bu yöntem production kodunda ciddi problemlere yol açabilir. Bir sistemin açılırken kritik bir yapılandırma dosyasını okumaya çalıştığını düşünelim. Dosyanın bulunamamsı veya okuma sırasında bir hata alınması halinde programın paniklemesi yerine kullanıcıya anlamlı bir hata mesajı döndürmek veya izlenebilir, tedbir alınabilir bir makine logu bırakmak daha sağlıklı olacaktır.

```rust
use std::fs;

// Kötü pratik: unwrap ve expect kullanımı
#[allow(dead_code)]
fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

// İyi pratik: Hata yönetimi ile dosya okuma
fn read_file_safely(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() {
    // let content = read_file("appSettings.json");
    // println!("{}", content);

    match read_file_safely("appSettings.json") {
        Ok(content) => println!("{}", content),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("Dosya bulunamadı: {}", e);
            } else {
                println!("Dosya okunurken bir hata oluştu: {}", e);
            }
        }
    }

    println!("Paniksiz günler dilerim!");
}
```

### Örnek 2: Gereksiz clone Çağrılarından Kaçınmak

Rust sahiplik *(ownership)* modelinde özellikle *Vector*, *String* gibi heap bellek bölgesinde değerlendirilen veri yapıları kapsamlar *(scopes)* arasında taşınırken varsayılan olarak sahipliğin aktarımı söz konusudur. Eğer veri yapısı taşındığı fonksiyonda bir değişikliğe, başka bir deyişle mutasyona uğramayacaksa tüm veri yapısını klonlayarak göndermek yerine referans ile göndermek daha performanslı ve bellek dostu bir yaklaşımdır. Söz gelimi büyük bir sayı listesinin vektör veri yapısında ele alındığını düşünelim. Bu sayı kümesinin matematiksel bir analiz fonksiyonu işleten bir metot tarafından da kullanıldığını varsayalım. Analizi yapan fonksiyon veriyi değiştirmeyeceği için tüm vektörün klonlanması yerine referans ile gönderilmesi daha doğru olacaktır.

```rust
// Kötü pratik: ownership alan fonksiyon kullanımı
#[allow(dead_code)]
fn calculate_bad(data: Vec<i32>) -> i32 {
    let sum: i32 = data.iter().sum();
    sum / (data.len() as i32)
}

// Tercih edilen pratik: referans ile veri geçme
fn calculate(data: &[i32]) -> i32 {
    let sum: i32 = data.iter().sum();
    sum / (data.len() as i32)
}

fn main() {
    /*
     Aşağıdaki kullanım value moved here hatası verir çünkü calculate fonksiyonu ownership'i alır ve data'yı kullanır.

     Sık yapılan çözümlerden birisi vektörü klonlamaktır ancak bu performans açısından maliyetlidir.
     Eğer veri değişmeyecekse, ownership almak yerine referans ile geçmek daha iyidir.

     error[E0382]: borrow of moved value: `numbers`
    --> exc01\src\main.rs:11:22
    |
    7  |     let numbers = vec![10, 20, 30, 40, 50];
    |         ------- move occurs because `numbers` has type `Vec<i32>`, which does not implement the `Copy` trait
    8  |     let result = calculate(numbers);
    |                            ------- value moved here
    ...
    11 |     println!("{:?}", numbers);
    |                      ^^^^^^^ value borrowed here after move
    |
    note: consider changing this parameter type in function `calculate` to borrow instead if owning the value isn't necessary
    --> exc01\src\main.rs:1:20
    |
    1  | fn calculate(data: Vec<i32>) -> i32 {
    |    ---------       ^^^^^^^^ this parameter takes ownership of the value
    |    |
    |    in this function
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
    |
    8  |     let result = calculate(numbers.clone());
    |                                   ++++++++

    */
    let numbers = vec![10, 20, 30, 40, 50];

    // Bad practice: ownership alan fonksiyon kullanımı
    // // let result = calculate_bad(numbers);
    // let result = calculate_bad(numbers.clone()); // Performans maliyeti var
    // println!("Sonuç: {}", result);

    // println!("{:?}", numbers);

    // Good practice: referans ile veri geçme
    let result = calculate(&numbers);
    println!("Sonuç: {}", result);
    println!("{:?}", numbers);
}
```

### Örnek 3: Mutasyon Kapsamını Sınırlamak

Rust programlama dilinde değişkenler varsayılan olarak immutable *(değiştirilemez)* olarak tanımlanır. Değişkenin değerini değiştirmek istediğimizde `mut` anahtar kelimesi ile değişkeni mutable *(değiştirilebilir)* olarak tanımlamamız gerekir. Mutasyonu mümkün olan en dar kapsamda kullanmak kod okunurluğu ve güvenliğini artıran bir pratiktir. Örneğin bileşik faiz hesaplaması yapan bir muhasebe fonksiyonunda döngü içinde güncellenen belli değişkenler olduğunu düşünelim. Bu değişkenler sadece döngü içinde güncellenir ve ihtiyaç duyduğu ara değerler değiştirilemez *(immutable)* olarak tanımlanıp kullanılabilir. Aşağıdaki örnekte bu prensibi uygulayan bir bileşik faiz hesaplama fonksiyonu yer almaktadır.

```rust
fn calculate_compound_interest(principal: f64, annual_rate: f64, years: u32) -> f64 {
    let mut current_amount = principal;
    let mut total_interest = 0.0;

    for year in 1..=years {
        let yearly_interest = current_amount * annual_rate / 100.0;
        current_amount += yearly_interest;
        total_interest += yearly_interest;
        
        println!("Year {}: Interest earned: {:.2}, Total amount: {:.2}", 
                 year, yearly_interest, current_amount);
    }

    total_interest
}

fn main() {
    let principal = 1000.0;
    let annual_rate = 4.5;
    let years = 3;

    let total_interest = calculate_compound_interest(principal, annual_rate, years);
    let final_amount = principal + total_interest;
    
    println!("\nSummary:");
    println!("Principal amount: {:.2}", principal);
    println!("Annual interest rate: {:.1}%", annual_rate);
    println!("Time period: {} years", years);
    println!("Total compound interest earned: {:.2}", total_interest);
    println!("Final amount: {:.2}", final_amount);
}
```

### Örnek 4: Dangling Referanslardan Kaçınmak

Rust'ın güçlü sahiplik *(ownership)* ve borçlanma *(borrowing)* modeli, dangling *(Sarkmış)* referansların oluşmasını derleme zamanında engeller. Dangling referanslar, bir değişkenin kapsamı dışına çıktıktan sonra ona erişmeye çalıştığımızda ortaya çıkar ve bu durum bellek güvenliği sorunlarına yol açabilir. Rust, bu tür hataların oluşmasını önlemek için katı kurallar uygular. *Borrow Checker* prensiplerine göre bir referansın atıfta bulunduğu değerden daha uzun yaşaması mümkün değildir. Dangling *(Sarkmış)* referanslar genelde bir fonksiyonun local bir değere referans döndürmeye çalışması sırasında ortaya çıkan kritik bir bellek güvenliği hatasıdır.

N sayıda cümleyi literal string olarak tutan bir dizideki en uzun cümleyi bulmaya çalışan bir fonksiyon yazdığımızı düşünelim. En uzun cümleyi referans olarak döndürmeye çalışırsak, fonksiyonun kapsamı sona erdiğinde taşınan dizinin bellekten silinmesiyle birlikte döndürdüğümüz referansın geçersiz hale gelmesi söz konusu olur ve sorunu çözmek için karmaşık lifetime annotasyonları kullanmamız gerekir. Bunun yerine en uzun cümleyi sahiplenen bir String olarak döndürmek daha doğru bir yaklaşımdır.

```rust
// // Kötü pratik: Dangling referans sorunu oluşması ve lifetime kullanma gerekliliği
// fn find_longest_sentence_badly(lines: &[&str]) -> &str {
//     let mut longest: &str = "";
//     for &line in lines {
//         if line.len() > longest.len() {
//             longest = line;
//         }
//     }
//     longest
// }

// Doğru pratik: String döndürme
fn find_longest_sentence_safely(lines: &[&str]) -> String {
    let mut longest = String::new();
    for line in lines {
        if line.len() > longest.len() {
            longest = line.to_string();
        }
    }
    longest
}

fn main() {
    let lines = vec![
        "Rust is a systems programming language.",
        "It is designed for performance and safety.",
        "Ownership and borrowing are key concepts in Rust.",
    ];

    /*
    Bu fonksiyon dangling referans hatasına neden olur ve ayrıca derleme zamanında 'expected named lifetime parameter' hatası verir.
    Sorunu çözmek için fonksiyon imzasına yaşam süresi parametreleri eklemek gerekir.
    Bunun yerine en uzun cümleyi String olarak döndürmek daha güvenlidir.

    error[E0106]: missing lifetime specifier
    --> exc03\src\main.rs:2:51
    |
    2 | fn find_longest_sentence_badly(lines: &[&str]) -> &str {
    |                                       -------     ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of `lines`'s 2 lifetimes it is borrowed from
    help: consider introducing a named lifetime parameter
    |
    2 | fn find_longest_sentence_badly<'a>(lines: &'a [&'a str]) -> &'a str {
    |                               ++++         ++   ++           ++

    For more information about this error, try `rustc --explain E0106`.

    */

    // let longest_sentence = find_longest_sentence_badly(&lines);
    // println!("En uzun cümle (kötü pratik): {}", longest_sentence);

    let longest_sentence = find_longest_sentence_safely(&lines);
    println!("En uzun cümle (iyi pratik): {}", longest_sentence);
}
```

## Orta Seviye

> Yakında eklenecek

## İleri Seviye

> Yakında eklenecek
