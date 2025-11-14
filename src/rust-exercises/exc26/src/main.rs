use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

static COUNTER: Mutex<u32> = Mutex::new(0);
static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
const NUMBER_OF_ITERATIONS: u32 = 10_000_000;

fn main() {
    println!("Calculating with Mutex:");
    calculate_with_mutex();

    println!("\nCalculating with Atomic:");
    calculate_with_atomic();
}

fn calculate_with_mutex() {
    let mut threads = vec![];
    let time_start = Instant::now();

    for _ in 0..10 {
        let t = thread::spawn(|| {
            for _ in 0..NUMBER_OF_ITERATIONS {
                let mut num = COUNTER.lock().unwrap();
                *num += 1;
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    let duration = time_start.elapsed();
    println!("Final counter value: {}", *COUNTER.lock().unwrap());
    println!("Time elapsed: {:?}", duration);
}

fn calculate_with_atomic() {
    let mut threads = vec![];
    let time_start = Instant::now();

    for _ in 0..10 {
        let t = thread::spawn(|| {
            for _ in 0..NUMBER_OF_ITERATIONS {
                ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    let duration = time_start.elapsed();
    println!(
        "Final atomic counter value: {}",
        ATOMIC_COUNTER.load(Ordering::SeqCst)
    );
    println!("Time elapsed: {:?}", duration);
}
