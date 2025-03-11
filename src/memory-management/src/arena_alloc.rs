use bumpalo::Bump;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
#[allow(dead_code)]
struct Position {
    id: u32,
    x_value: u32,
    y_value: u32,
    z_value: u32,
}

static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0); // Threads-Safe olarak veriyi kitlemeden (lock-free) değişikliğe izin vermek için AtomicUsize kullanıldı

impl Drop for Position {
    fn drop(&mut self) {
        println!("{} nolu position için Drop çağrısı", self.id);
        DROPPED_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

#[allow(dead_code)]
pub fn run() {
    let bump = Bump::new();

    { // Kasıtlı olarak scope açıldı
        let player_one = bump.alloc(Position {
            id: 1,
            x_value: 10,
            y_value: 20,
            z_value: 0,
        });
        let player_two = bump.alloc(Position {
            id: 2,
            x_value: 15,
            y_value: 5,
            z_value: 30,
        });
        let john_doe = bump.alloc(Position {
            id: 3,
            x_value: 3,
            y_value: 5,
            z_value: 8,
        });

        println!("Player One Adresi {:p}", player_one);
        println!("Player Two Adresi {:p}", player_two);
        println!("John Doe Adresi {:p}", john_doe);

        let player_one_addr = player_one as *const _ as usize;
        let player_two_addr = player_two as *const _ as usize;
        let john_doe_addr = john_doe as *const _ as usize;

        println!(
            "Gerçek Player Two - Player One adres farkı: {} byte",
            address_diff(player_two_addr, player_one_addr)
        );
        println!(
            "Gerçek John Doe - Player Two adres farkı: {} byte",
            address_diff(john_doe_addr, player_two_addr)
        );
    } // Scope dışındayız ama nesne drop'ları çalışmaz Area Allocation sebebiyle

    println!(
        "Dropped Position nesne sayısı {}",
        DROPPED_COUNT.load(Ordering::SeqCst)
    );

    // Arena burada scope'dan çıkarken içindeki tüm Player nesneleri de tek seferde düşürülecektir
}

fn address_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
