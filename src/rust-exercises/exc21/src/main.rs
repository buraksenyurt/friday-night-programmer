use rand::Rng;

fn main() {
    // 10 adet rastgele sayı üretimi (map ile birlikte kullanım)
    let mut rng = rand::rng();
    let numbers: Vec<i32> = (0..10).map(|_| rng.random_range(1..101)).collect();
    println!("Random 10 numbers: {:?}", numbers);

    // Rastgele üretilmiş olan sayılardan çift olanların filtrelenmesi (filter ile birlikte kullanım)
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // Bir sayı dizisindeki asal sayıların listesi ve toplam sayısı (filter ile birlikte kullanım)
    let numbers: Vec<i32> = (0..20).map(|_| rng.random_range(1..101)).collect();
    let primes: Vec<i32> = numbers.into_iter().filter(|&x| is_prime(x)).collect();
    println!("Prime numbers: {:?}", primes);
    println!("Count of prime numbers: {}", primes.len());

    // 8 adet güçleri 2 ile 5 arasında değişen AIPlayer nesneleri oluşturulması
    let ai_players: Vec<AIPlayer> = (0..8)
        .map(|i| AIPlayer {
            name: format!("AI_Player_{}", i + 1),
            power: rng.random_range(2..6),
        })
        .collect();

    // Bu oyunculardan gücü 4'ten büyük olanların rastgele bir lokasyona atanması
    let strong_ai_locations: Vec<(AIPlayer, Location)> = ai_players
        .into_iter()
        .filter(|player| player.power > 4)
        .map(|player| {
            let location = Location {
                x: rng.random_range(0.0..100.0),
                y: rng.random_range(0.0..100.0),
            };
            (player, location)
        })
        .collect();

    println!("Strong AI Players and their Locations:");
    strong_ai_locations.iter().for_each(|(player, location)| {
        println!(
            "{} (Power: {}) is at Location ({:.2}, {:.2})",
            player.name, player.power, location.x, location.y
        );
    });
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..=((num as f64).sqrt() as i32) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
#[derive(Debug)]
struct AIPlayer {
    name: String,
    power: u16,
}

#[derive(Debug)]
struct Location {
    x: f64,
    y: f64,
}
