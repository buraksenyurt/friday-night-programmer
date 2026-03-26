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
    let in_circle: u64 = (0..total_iterations)
        .into_par_iter()
        .map_init(
            || rand::make_rng::<SmallRng>(),
            |rng, _| {
                let (x, y): (f64, f64) = rng.random();

                if x * x + y * y <= 1.0 {
                    1_u64
                } else {
                    0_u64
                }
            },
        )
        .sum();

    4.0 * (in_circle as f64) / (total_iterations as f64)
}
