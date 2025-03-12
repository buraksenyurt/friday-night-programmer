pub fn main() {
    let numbers: Vec<u32> = (0..=10).collect();

    let total_sum_1: u32 = numbers.iter().map(|x| x + 1).sum();

    let mut total_sum_2: u32 = 0;
    for x in &numbers {
        total_sum_2 += x + 1;
    }

    println!("Iterator fonksiyonları üzerinden toplam : {}", total_sum_1);
    println!("Klasik for döngüsü ile toplam : {}", total_sum_2);
}
