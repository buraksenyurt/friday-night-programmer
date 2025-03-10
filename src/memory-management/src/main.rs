use std::borrow::Cow;

fn main() {
    let user_one = "john doe";
    let player_two = "ready player one";
    let length = 16;

    println!("{}", padding_end(user_one, length));
    println!("{}", padding_end(player_two, length));
}

// Cow Örneği
fn padding_end<'a>(input: &'a str, target_len: usize) -> Cow<'a, str> {
    if input.len() < target_len {
        // Yeni string oluşturur ve target_len'e göre belli sayıda _ karakteri ekler
        Cow::Owned(format!("{:_<width$}", input, width = target_len))
    } else {
        // Yeterli uzunlukta olduğu için kopyalamadan orijinal referansı döndürür
        Cow::Borrowed(input)
    }
}