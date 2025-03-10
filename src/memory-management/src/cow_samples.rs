use std::borrow::Cow;

pub fn run() {
    let user_one = "Super Mario";
    let player_two = "Ready Player One";
    let length = 16;

    println!("{}", padding_end(user_one, length));
    println!("{}", padding_end(player_two, length));

    let data = "
        ```
        let value = 10;
        ```
    ";

    println!("{}", remove_ellipsis_dots(data));
}

fn padding_end<'a>(input: &'a str, target_len: usize) -> Cow<'a, str> {
    if input.len() < target_len {
        // Yeni string oluşturur ve target_len'e göre belli sayıda _ karakteri ekler
        Cow::Owned(format!("{:_<width$}", input, width = target_len))
    } else {
        // Yeterli uzunlukta olduğu için kopyalamadan orijinal referansı döndürür
        Cow::Borrowed(input)
    }
}

// fn remove_ellipsis_dots(input: &str) -> String {
//     input.to_string().replace('`', "")
// }

fn remove_ellipsis_dots(input: &str) -> Cow<str> {
    if input.contains('`') {
        Cow::Owned(input.to_string().replace('`', ""))
    } else {
        Cow::Borrowed(input)
    }
}
