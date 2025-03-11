use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
#[allow(dead_code)]
struct Velocity {
    id: u32,
    value: u32,
    direction: i32,
}

static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0); // Threads-Safe olarak veriyi kitlemeden (lock-free) değişikliğe izin vermek için AtomicUsize kullanıldı

impl Drop for Velocity {
    fn drop(&mut self) {
        println!("{} nolu Velocity nesnesi için Drop çağrısı", self.id);
        DROPPED_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

#[allow(dead_code)]
pub fn run() {
    { // Kasıtlı olarak Scope açıldı
        let _p1 = Box::new(Velocity {
            id: 1,
            value: 10,
            direction: 1,
        });
        let _p2 = Box::new(Velocity {
            id: 2,
            value: 20,
            direction: -1,
        });
        let _p3 = Box::new(Velocity {
            id: 3,
            value: 50,
            direction: 1,
        });
    } // Drop'lar çalışır

    println!(
        "Dropped Velocity nesne sayısı {}",
        DROPPED_COUNT.load(Ordering::SeqCst)
    );
}
