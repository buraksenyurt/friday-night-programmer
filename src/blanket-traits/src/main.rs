fn main() {
    let message = String::from("Blanket Traits!");
    println!("{}", message.colorized_print("36"));

    let slice = "This is a &str slice";
    println!("{}", slice.colorized_print("35"));

    let number = 2024;
    println!("{}", number.colorized_print("33"));

    let numbers = vec![1, 2, 3, 4, 5];
    println!("{}", numbers.colorized_print("32"));
}

trait ColorizedPrint {
    fn colorized_print(&self, color_code: &str) -> String;
}

impl<T> ColorizedPrint for T
where
    T: std::fmt::Debug,
{
    fn colorized_print(&self, color_code: &str) -> String {
        format!("\x1b[{}m{:?}\x1b[0m", color_code, self)
    }
}

// impl ColorizedPrint for String {
//     fn colorized_print(&self, color_code: &str) -> String {
//         format!("\x1b[{}m{}\x1b[0m", color_code, self)
//     }
// }

// impl ColorizedPrint for &str {
//     fn colorized_print(&self, color_code: &str) -> String {
//         format!("\x1b[{}m{}\x1b[0m", color_code, self)
//     }
// }
