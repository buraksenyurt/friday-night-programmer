use rand::rngs::SmallRng;
use rand::RngExt;
use rayon::prelude::*;

fn main() {
    let total_iterations = 1_000_000_000;

    for _ in 0..10 {
        let start_time = std::time::Instant::now();
        let pi_estimate = calculate_pi(total_iterations);
        let elapsed = start_time.elapsed();
        println!("Estimated Pi: {} in {:?}", pi_estimate, elapsed);
    }
}

fn calculate_pi(total_iterations: u64) -> f64 {
    let chunk_size = 4; // SIMD için 4'lü gruplar halinde işlem yapacağız
                        // çünkü AVX2 256-bit genişliğinde ve 4 adet f64 (64-bit) değeri aynı anda işleyebilir.
    let loop_count = total_iterations / chunk_size;

    let in_circle: u64 = (0..loop_count)
        .into_par_iter()
        .map_init(
            || rand::make_rng::<SmallRng>(),
            |rng, _| {
                let x_values: [f64; 4] = rng.random(); // random'un güzel yanlarından birisi. Tek çağrıda arka arkaya 4 sayı üretip diziye atar.
                let y_values: [f64; 4] = rng.random();

                get_circle_value(&x_values, &y_values)
            },
        )
        .sum();

    4.0 * (in_circle as f64) / (total_iterations as f64)
}

// x_values ve y_values elemanları 4 boyutlu dizi olduğunda derleyici bunların kesinlikle sabit boyutlu olduğunu bilecek.
// Buna göre kod doğrudan AVX2 SIMD komutuna çevrilebilir.

#[inline(always)]
// Bunu eklediğimiz için derleyici bu fonksiyonu çağırmak yerine doğrudan kodun içerisine gömer.
pub fn get_circle_value(x_values: &[f64; 4], y_values: &[f64; 4]) -> u64 {
    let mut count = 0;

    for i in 0..4 {
        if x_values[i] * x_values[i] + y_values[i] * y_values[i] <= 1.0 {
            count += 1;
        }
    }

    count
}
