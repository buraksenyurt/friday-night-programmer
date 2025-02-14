fn main() {
    // let some_data = 1;
    // let mut buffer = [0; 10]; // 10 elemanlı bir dizi oluşturuluyor

    // for i in 0..=11 {
    //     buffer[i] = 5; // Dizi sınırlarını aşma girişimi, Rust derleyicisi tarafından yakalanır
    // }

    // println!("some_data = {}", some_data);

    let mut buffer = [0u8; 8]; // 8 byte uzunluğunda bir dizi

    buffer[10] = 1; // Rust buradaki sorunu derleme zamanında fark ederek hata verir.

    match buffer.get_mut(10) {
        Some(byte) => *byte = 1,
        None => println!("Kapasite sınırı aşılmaya çalışıldı"),
    };

    println!("Buffer: {:?}", buffer);
}