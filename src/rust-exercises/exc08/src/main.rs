use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Global paylaşımlı değişken
    // Arc ile çoklu sahiplik
    // Mutext kilitleme ile değiştirebilir erişim imkanı
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    let thread_count = 4;

    for i in 0..thread_count {
        let counter_clone = Arc::clone(&counter); // Referansları say

        let thread = thread::spawn(move || {
            println!("Thread {} starting", i);

            // Mutext ile kilitlenir ve MutexGuard alınır.
            // Diğer erişmeye çalışanlara müsaade edilmez
            let mut value = counter_clone.lock().unwrap();
            *value += 1;

            thread::sleep(Duration::from_millis(100));
        });
        threads.push(thread);
    }

    // Tüm iş parçacıklarının bitmesini bekleyelim
    for t in threads {
        t.join().unwrap();
    }

    println!(
        "Current total request count is {}",
        *counter.lock().unwrap()
    );
}
