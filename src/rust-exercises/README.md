# Rust Kodlama İdmanları

Bu dokümanda rust bilgilerimizi tazelemek için çeşitli kaynaklardan derlediğim örneklere yer verilmektedir.

## Başlangıç Seviyesi

### Unwrap/Expect Tuzaklarından Kaçınmak (exc00)

Rust'ın güçlü yönlerinden birisi **Option< T >** ve **Result<T, E>** tipleri ile hata yönetimidir. Bazen özellikle geliştirme safhasındayken **unwrap** ve **expect** kullanarak ilerleyebiliriz zira **match** veya **if let** kullanmak kodu uzatabilir. Ancak bu yaklaşım üretim kodunda ciddi problemlere yol açabilir.

Örneğin bir sistemin açılırken kritik bir yapılandırma dosyasını okumaya çalıştığını düşünelim. Dosyanın bulunamaması veya okuma sırasında bir hata alınması halinde programın paniklemesi yerine kullanıcıya anlamlı bir hata mesajı döndürmek veya izlenebilir, tedbir alınabilir bir makine logu bırakmak daha sağlıklı olacaktır.

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

### Gereksiz clone Çağrılarından Kaçınmak (exc01)

Rust sahiplik *(ownership)* modelinde özellikle **Vector**, **String** gibi heap bellek bölgesinde değerlendirilen veri yapıları kapsamlar *(scopes)* arasında taşınırken varsayılan olarak sahipliğin aktarımı söz konusudur. Eğer veri yapısı taşındığı fonksiyonda bir değişikliğe, başka bir deyişle mutasyona uğramayacaksa tüm veri yapısını klonlayarak göndermek yerine referans ile göndermek daha performanslı ve bellek dostu bir yaklaşımdır. Söz gelimi büyük bir sayı listesinin vektör veri yapısında ele alındığını düşünelim. Bu sayı kümesinin matematiksel bir analiz fonksiyonu işleten bir metot tarafından da kullanıldığını varsayalım. Analizi yapan fonksiyon veriyi değiştirmeyeceği için tüm vektörün klonlanması yerine referans ile gönderilmesi daha doğru olacaktır.

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

### Mutasyon Kapsamını Sınırlamak (exc02)

Rust programlama dilinde değişkenler varsayılan olarak **immutable** *(değiştirilemez)* olarak tanımlanır. Değişkenin değerini değiştirmek istediğimizde `mut` anahtar kelimesi ile değişkeni **mutable** *(değiştirilebilir)* olarak tanımlamamız gerekir. Mutasyonu mümkün olan en dar kapsamda kullanmak kod okunurluğu ve güvenliğini artıran bir pratiktir. Örneğin bileşik faiz hesaplaması yapan bir muhasebe fonksiyonunda döngü içinde güncellenen belli değişkenler olduğunu düşünelim. Bu değişkenler sadece döngü içinde güncellenir ve ihtiyaç duyduğu ara değerler değiştirilemez *(immutable)* olarak tanımlanıp kullanılabilir. Aşağıdaki örnekte bu prensibi uygulayan bir bileşik faiz hesaplama fonksiyonu yer almaktadır.

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

### Dangling Referanslardan Kaçınmak (exc03)

Rust'ın güçlü sahiplik *(ownership)* ve borçlanma *(borrowing)* modeli, dangling *(Sarkmış)* referansların oluşmasını derleme zamanında engeller. Dangling referanslar, bir değişkenin kapsamı dışına çıktıktan sonra ona erişmeye çalıştığımızda ortaya çıkar ve bu durum bellek güvenliği sorunlarına yol açabilir. Rust, bu tür hataların oluşmasını önlemek için katı kurallar uygular. **Borrow Checker** prensiplerine göre bir referansın atıfta bulunduğu değerden daha uzun yaşaması mümkün değildir. Dangling *(Sarkmış)* referanslar genelde bir fonksiyonun local bir değere referans döndürmeye çalışması sırasında ortaya çıkan kritik bir bellek güvenliği hatasıdır.

**N** sayıda cümleyi **literal string** olarak tutan bir dizideki en uzun cümleyi bulmaya çalışan bir fonksiyon yazdığımızı düşünelim. En uzun cümleyi referans olarak döndürmeye çalışırsak, fonksiyonun kapsamı sona erdiğinde taşınan dizinin bellekten silinmesiyle birlikte döndürdüğümüz referansın geçersiz hale gelmesi söz konusu olur ve sorunu çözmek için karmaşık lifetime annotasyonları kullanmamız gerekir. Bunun yerine en uzun cümleyi sahiplenen bir String olarak döndürmek daha doğru bir yaklaşımdır.

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

### Public API'lerde Kapsamlı Dokümantasyon Kullanmak (exc04)

Rust'ın güçlü yanlarından birisi zengin dokümantasyon desteğidir. Özellikle public API'ler geliştirirken kapsamlı dokümantasyon kullanmak, kullanıcıların fonksiyonların nasıl kullanılacağını ve ne işe yaradığını anlamalarına yardımcı olur. **pub** erişim belirleyicisi ile işaretlenmiş tüm enstrümanlarda zengin dokümantasyon yorumları kullanmak gerekir.

```rust
/// Verilen bir fonksiyonun türevini yaklaşık olarak hesaplar.
///
/// # Argümanlar
/// * `f` - Türevini almak istediğimiz fonksiyon.
/// * `x` - Türevini hesaplamak istediğimiz nokta.
/// * `h` - Küçük bir değer, türev hesaplamasında kullanılır (varsayılan: 1e-7).
/// # Dönüş Değeri
/// * `f` fonksiyonunun `x` noktasındaki yaklaşık türevi.
pub fn derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Verilen bir fonksiyonun belirli bir aralıktaki integralini yaklaşık olarak hesaplar.
///
/// # Argümanlar
/// * `f` - İntegralini almak istediğimiz fonksiyon.
/// * `a` - İntegral başlangıç noktası.
/// * `b` - İntegral bitiş noktası.
/// * `n` - İntegral hesaplamasında kullanılacak dikdörtgen sayısı (varsayılan: 1000).
/// # Dönüş Değeri
/// * `f` fonksiyonunun `[a, b]` aralığındaki yaklaşık integrali.
pub fn integral<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let width = (b - a) / (n as f64);
    let mut total_area = 0.0;

    for i in 0..n {
        let x = a + (i as f64 + 0.5) * width;
        total_area += f(x) * width;
    }

    total_area
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_derivative() {
        let f = |x: f64| x.powi(2);
        let deriv_at_3 = derivative(f, 3.0, 1e-7);
        assert!((deriv_at_3 - 6.0).abs() < 1e-5);
    }

    #[test]
    fn test_integral() {
        let f = |x: f64| x;
        let integral_result = integral(f, 0.0, 1.0, 1000);
        assert!((integral_result - 0.5).abs() < 1e-5);
    }
}
```

ve modül içinde aşağıdaki gibi ilerlenebilir.

```rust
//! # Calculus Modülü
//!
//! Bu modül, temel matematiksel işlemleri gerçekleştiren fonksiyonlar içerir.
//! Örnek olarak, türev ve integral hesaplamaları için fonksiyonlar sağlar.
//!
//! # Örnekler
//! ```rust
//! mod calculus;
//!
//! use calculus::{derivative, integral};
//! fn main() {
//!   let f = |x: f64| x.powi(2);
//!   let deriv_at_3 = derivative(f, 3.0, 1e-7);
//!   println!("f'(3) yaklaşık olarak: {}", deriv_at_3); // Yaklaşık 6.0
//!   let integral_result = integral(f, 0.0, 1.0, 1000);
//!   println!("∫f(x)dx from 0 to 1 yaklaşık olarak: {}", integral_result); // Yaklaşık 0.3333
//! }
//! ```

pub mod calculus;
```

## Orta Seviye

### Composition Over Inheritance ile Daha Modüler Tasarım (exc05)

Rust nesne yönelimli programlama paradigmalarını tam olarak destekler mi desteklemez mi veya buna ihtiyacı var mıdır bilinmez ancak **Composition over Inheritance** prensibi daha çok ön plana çıkar. Hatta birçok **ECS** tabanlı oyun motorunda bu prensip temel alınarak tasarım yapılır. Bir nesnenin davranışlarını ve özelliklerini başka nesnelerden miras almak yerine, o nesnenin ihtiyaç duyduğu özellikleri ve davranışları başka nesnelerden bileşenler *(components)* aracılığıyla alınması tercih edilmelidir. Bu yaklaşım, kodun daha esnek, yeniden kullanılabilir ve test edilebilir olmasını sağlar.

Bir yazılım sistemindeki kullanıcıları temsil edecek bir yapı geliştirmeye çalıştığımızı düşünelim. Kullanıcı ile ilgili tüm bilgileri tek bir **God Object** içinde toplamak yerine, kullanıcıya ait farklı özellikleri ve davranışları ayrı bileşenler olarak tanımlayıp, kullanıcı yapısına bu bileşenleri ekleyerek oluşturmak daha esnek bir tasarım sağlar.

```rust
fn main() {
    let personal_info = PersonalInfo::new("John".to_string(), "Doe".to_string(), 25);
    let contact_info = ContactInfo::new("john.doe@nowhere.com".to_string());
    let activity_status = ActivityStatus::new(true, 120120044543);
    let gaming_info = GamingInfo::new(7);

    let user = User::new(personal_info, contact_info, activity_status, gaming_info);

    println!("User: {}", user.get_full_name());
    println!("Email: {}", user.get_email());
    println!("Active: {}", user.is_active());
    println!("Level: {}", user.get_level());

    let mut mutable_user = user;
    mutable_user.set_active(true);
    mutable_user.level_up();

    println!("New level: {}", mutable_user.get_level());
}

// Bad Practice: God Object - Tüm bilgileri tek bir struct'ta toplamak
#[allow(dead_code)]
struct BadUser {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    is_active: bool,
    last_activity_timestamp: u64,
    level: u8,
}

// Good Practice: Composition over Inheritance - Farklı sorumlulukları ayrı bileşenlerde tutmak
#[derive(Debug, Clone)]
struct PersonalInfo {
    first_name: String,
    last_name: String,
    age: u8,
}

impl PersonalInfo {
    fn new(first_name: String, last_name: String, age: u8) -> Self {
        Self {
            first_name,
            last_name,
            age,
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

#[derive(Debug, Clone)]
struct ContactInfo {
    email: String,
}

impl ContactInfo {
    fn new(email: String) -> Self {
        Self { email }
    }

    fn get_email(&self) -> &str {
        &self.email
    }

    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

#[derive(Debug, Clone)]
struct ActivityStatus {
    is_active: bool,
    last_activity_timestamp: u64,
}

impl ActivityStatus {
    fn new(is_active: bool, last_activity_timestamp: u64) -> Self {
        Self {
            is_active,
            last_activity_timestamp,
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, active: bool) {
        self.is_active = active;
        if active {
            self.last_activity_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    fn get_last_activity(&self) -> u64 {
        self.last_activity_timestamp
    }
}

#[derive(Debug, Clone)]
struct GamingInfo {
    level: u8,
}

impl GamingInfo {
    fn new(level: u8) -> Self {
        Self { level }
    }

    fn get_level(&self) -> u8 {
        self.level
    }

    fn level_up(&mut self) {
        if self.level < u8::MAX {
            self.level += 1;
        }
    }

    fn set_level(&mut self, level: u8) {
        self.level = level;
    }
}

#[derive(Debug, Clone)]
struct User {
    personal_info: PersonalInfo,
    contact_info: ContactInfo,
    activity_status: ActivityStatus,
    gaming_info: GamingInfo,
}

#[allow(dead_code)]
impl User {
    fn new(
        personal_info: PersonalInfo,
        contact_info: ContactInfo,
        activity_status: ActivityStatus,
        gaming_info: GamingInfo,
    ) -> Self {
        Self {
            personal_info,
            contact_info,
            activity_status,
            gaming_info,
        }
    }

    fn get_full_name(&self) -> String {
        self.personal_info.get_full_name()
    }

    fn get_age(&self) -> u8 {
        self.personal_info.get_age()
    }

    fn get_email(&self) -> &str {
        self.contact_info.get_email()
    }

    fn update_email(&mut self, new_email: String) {
        self.contact_info.update_email(new_email);
    }

    fn is_active(&self) -> bool {
        self.activity_status.is_active()
    }

    fn set_active(&mut self, active: bool) {
        self.activity_status.set_active(active);
    }

    fn get_last_activity(&self) -> u64 {
        self.activity_status.get_last_activity()
    }

    fn get_level(&self) -> u8 {
        self.gaming_info.get_level()
    }

    fn level_up(&mut self) {
        self.gaming_info.level_up();
    }

    fn set_level(&mut self, level: u8) {
        self.gaming_info.set_level(level);
    }

    fn get_user_summary(&self) -> String {
        format!(
            "User: {} ({}), Email: {}, Active: {}, Level: {}",
            self.get_full_name(),
            self.get_age(),
            self.get_email(),
            self.is_active(),
            self.get_level()
        )
    }
}
```

### Daha Kapsamlı Test Senaryoları Yazmak (exc06)

Kodun kalitesini ve doğruluğunu artırmak için kapsamlı test senaryoları yazmak önemlidir. Burada normal durumlar dışında uç vakalar *(edge cases)* ve hata senaryolarını da kapsayan testler yazılması önemlidir. Söz gelimi bir sosyal sigorta güvenlik numarasının doğruluğunu kontrol eden bir fonksiyon geliştirdiğimizi düşünelim. Bu fonksiyon için sadece geçerli numaraları değil, aynı zamanda hatalı formatları, eksik karakterleri ve diğer olası hata durumlarını da test etmeliyiz.

```rust
pub fn validate_social_security_number(ssn: &str) -> bool {
    // Basit bir doğrulama: SSN 9 haneli olmalı ve sadece rakamlardan oluşmalı
    let is_nine_digits = ssn.len() == 9;
    let all_digits = ssn.chars().all(|c| c.is_digit(10));
    is_nine_digits && all_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    // Normal durum testi
    #[test]
    fn test_valid_ssn() {
        assert!(validate_social_security_number("123456789"));
    }

    // Edge case testleri
    #[test]
    fn test_empty_or_whitespace_ssn() {
        assert!(!validate_social_security_number("")); // Boş string
        assert!(!validate_social_security_number("   ")); // Sadece boşluk
    }
    
    #[test]
    fn test_too_long_or_short_ssn() {
        assert!(!validate_social_security_number("123456789012345")); // Çok uzun
        assert!(!validate_social_security_number("12345")); // Çok kısa
    }

    // Hata Senaryosu/Negatif testleri
    #[test]
    fn test_invalid_format_ssn() {
        assert!(!validate_social_security_number("123-45-6789")); // Yanlış format
        assert!(!validate_social_security_number("12345678A")); // Harf içeriyor
        assert!(!validate_social_security_number("12 3456789")); // Boşluk içeriyor
    }

    #[test]
    fn test_right_length_but_wrong_characters_ssn() {
        assert!(!validate_social_security_number("12345A789")); // 8 haneli
    }
}
```

## İleri Seviye

### Unsafe Kodları Soyutlamalar ile Sarmak (exc07)

Derleyicinin bellek güvenliğini garantiye alamadığı durumlarda **unsafe** kod blokları kullanılır. Ancak **unsafe** kodların doğrudan kullanımı, bellek güvenliği sorunlarına da yol açabilir. Bu nedenle **unsafe** kodları güvenli soyutlamalar *(safe abstractions)* ile sarmak ideal yaklaşımlardan birisidir.

Örneğin bir sayı dizisini referans olarak kullanırken ödünç alma kurallarını atlayarak herhangi bir noktasından ikiye bölmek istediğimizi düşünelim. 101 elemanlı bir sayı dizisini 16ncı indisinden itibaren iki ayrı parça halinde değiştirilebilir referans olarak ele almak istiyoruz. Normalde rust aynı anda aynı veriye iki farklı değiştirilebilir referans vermeye izin vermez. **unsafe** çağrılabileceğini bildiğimiz bir fonksiyona göz yumup bu kuralı atlayarak geliştirme yapabiliriz. İşte burada unsafe kodu güvenli bir soyutlama ile sarmak önemlidir.

```rust
use std::slice;

fn main() {
    let mut numbers = vec![1, 4, 6, 1, 6, 2, 4, 6, 7, 9, 123, 7, 1, 7];

    // numbers dizisi 3. indexten ikiye bölünüyor
    let (left_slice, right_slice) = split_array_from(&mut numbers, 3);

    println!("Left slice values: {:?}", left_slice);
    println!("Right slice values: {:?}", right_slice);

    // left_slice dilimindeki ilk elemanı değiştiriyoruz
    // bu değişiklik orijinal numbers dizisini de etkileyecektir
    left_slice[0] = 345;
    println!("After changed the left slice: {:?}", numbers);
}

/// Bu fonksiyon, verilen `values` dilimini `index` konumunda ikiye böler
/// ve iki ayrı dilim olarak döner.
///
/// # Güvenlik Notu
///
/// Bu fonksiyon unsafe kod kullanır, bu nedenle dikkatli olunmalıdır.
///
/// # Parametreler
///
/// - `values`: Bölünecek olan tamsayı dilimi.
/// - `index`: Bölme işleminin gerçekleşeceği konum.
///
/// # Dönüş Değeri
/// İki ayrı tamsayı dilimi olarak döner.
fn split_array_from(values: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // ptr değişkeni, values diliminin başlangıç adresini tutan bir işaretçidir(pointer).
    let ptr = values.as_mut_ptr();

    /*
        from_raw_parts_mut fonksiyonu unsafe türdendir ve bu nedenle
        unsafe kod bloğu içerisinde çalıştırılması gerekir.
    */
    unsafe {
        // ptr ile tutulan adresten başlayarak index uzunluğunda bir dilim oluşturur.
        let left = slice::from_raw_parts_mut(ptr, index);
        // index noktasından başlayarak len - index uzunluğunda bir dilim oluşturur.
        let right = slice::from_raw_parts_mut(ptr.add(index), len - index);
        (left, right)
    }
}
```
