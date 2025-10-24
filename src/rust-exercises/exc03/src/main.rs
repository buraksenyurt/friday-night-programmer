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
