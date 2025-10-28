/*
    Bir oyundaki oyuncu ve takım sayılarına ait istatistikleri tutan Stats isimli bir veri yapımız var.
    Bu veri yapısındaki player_count alanı Raw Pointer türündendir ve bu türler
    Rust'ta Send ve Sync trait'lerini otomatik olarak implemente etmezler.

    Dolayısıyla bu veri yapısını farklı thread'ler arasında paylaşmaya çalıştığımızda,
    "*mut 32 cannot be sent between threads safely" hatasını alırız. Bu hatayı çözmek için
    Stats yapısına manuel olarak Send trait'ini implemente etmemiz gerekir.

    Şu anki senaryoda sync trait'ine de ihtiyacımız yok gibi görünebilir zira derleme zamanında
    hata alınmaz. Yine de raw pointer'lar thread-safe olmadığından dolayı Sync trait'ini açıkça
    ekleyerek kodun güvenliğini artırabiliriz.
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let game_stats = Arc::new(Mutex::new(Stats {
        player_count: Box::into_raw(Box::new(0)),
    }));

    let mut handlers = vec![];

    for _ in 0..10 {
        let stats_clone = Arc::clone(&game_stats);
        let handle = std::thread::spawn(move || {
            let mut stats = stats_clone.lock().unwrap();
            unsafe {
                *stats.player_count += 1;
            }
            thread::sleep(std::time::Duration::from_millis(10));
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Player Count: {}", unsafe {
        *game_stats.lock().unwrap().player_count
    });
}

#[allow(dead_code)]
#[derive(Debug)]
struct Stats {
    player_count: *mut u32,
}

unsafe impl Send for Stats {}
unsafe impl Sync for Stats {}
