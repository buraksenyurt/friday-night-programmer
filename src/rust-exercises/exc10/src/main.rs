fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    match find_min_max(&numbers) {
        Some((min, max)) => {
            println!("Minimum: {}, Maximum: {}", min, max);
        }
        None => {
            println!("Empty slice provided.");
        }
    }

    let chars = vec!['y', 'c', 'm', 'e', 'q', 'l', 'x', 'k'];
    match find_min_max(&chars) {
        Some((min, max)) => {
            println!("Minimum: {}, Maximum: {}", min, max);
        }
        None => {
            println!("Empty slice provided.");
        }
    }

    let towers = vec![
        Tower { height: 150 },
        Tower { height: 200 },
        Tower { height: 175 },
    ];
    match find_min_max(&towers) {
        Some((min, max)) => {
            println!(
                "Minimum Tower Height: {}, Maximum Tower Height: {}",
                min.height, max.height
            );
        }
        None => {
            println!("Empty slice provided.");
        }
    }
}

/// Verilen bir slice içindeki minimum ve maksimum değerleri bulan fonksiyon.
/// Eğer slice boşsa None döner, aksi takdirde Some((min, max)) döner.
///
/// # Arguments
/// * `values` - Karşılaştırılacak değerlerin bulunduğu slice.
///
/// # Returns
/// * `Option<(T, T)>` - Minimum ve maksimum değerleri içeren bir tuple veya None.
///
/// # Constraints
/// * `T: Ord + Copy` - T türü karşılaştırılabilir ve kopyalanabilir olmalıdır.
fn find_min_max<T: Ord + Copy>(values: &[T]) -> Option<(T, T)> {
    if values.is_empty() {
        return None;
    }

    let mut min = values[0];
    let mut max = values[0];

    for &value in values.iter() {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }

    Some((min, max))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Tower {
    height: u32,
}

impl PartialOrd for Tower {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tower {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}
