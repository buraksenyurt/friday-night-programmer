use std::thread;

// // CASE02: Bu senaryoda Arc'ı tek başına Mutex olmadan kullanırsak.
// use std::sync::Arc;

// fn main(){
//     let calculation_result = Arc::new(0.0_f64);

//     let calc_res_clone_1 = Arc::clone(&calculation_result);
//     let handle_1 = thread::spawn(move || {
//         for i in 1..=100 {
//             *calc_res_clone_1 += (i as f64).sqrt();
//             thread::sleep(std::time::Duration::from_millis(50));
//         }
//     });

//     let calc_res_clone_2 = Arc::clone(&calculation_result);
//     let handle_2 = thread::spawn(move || {
//         for i in 1..=100 {
//             *calc_res_clone_2 += (i as f64 + 1.0).ln();
//             thread::sleep(std::time::Duration::from_millis(50));
//         }
//     });

//     handle_1.join().unwrap();
//     handle_2.join().unwrap();

//     println!("Calculation result {}", *calculation_result);
// }


// CASE01: Arc kullanarak ortak veriyi thread'lere paylaştırma.
use std::sync::Arc;

fn main(){
    let calculation_result = Arc::new(std::sync::Mutex::new(0.0_f64));

    let calc_res_clone_1 = Arc::clone(&calculation_result);
    let handle_1 = thread::spawn(move || {
        for i in 1..=100 {
            let mut result = calc_res_clone_1.lock().unwrap();
            *result += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let calc_res_clone_2 = Arc::clone(&calculation_result);
    let handle_2 = thread::spawn(move || {
        for i in 1..=100 {
            let mut result = calc_res_clone_2.lock().unwrap();
            *result += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", *calculation_result.lock().unwrap());
}

// // CASE00: calculation_result hep 0 kalıyor.
// fn main() {
//     let mut calculation_result: f64 = 0.0;

//     let handle_1 = std::thread::spawn(move || {
//         for i in 1..=100 {
//             calculation_result += (i as f64).sqrt();
//             thread::sleep(std::time::Duration::from_millis(50));
//             println!("Intermediate result from Handle 1 {}", calculation_result);
//         }
//     });

//     let handle_2 = std::thread::spawn(move || {
//         for i in 1..=100 {
//             calculation_result += (i as f64).ln();
//             thread::sleep(std::time::Duration::from_millis(50));
//             println!("Intermediate result from Handle 2 {}", calculation_result);
//         }
//     });

//     handle_1.join().unwrap();
//     handle_2.join().unwrap();

//     println!("Calculation result {}", calculation_result);
// }
