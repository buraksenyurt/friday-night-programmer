use std::marker::PhantomData;

struct FirstPersonShooter;
struct RealTimeStrategy;
struct RolePlayingGame;

struct Identity<T> {
    value: u64,
    marker: PhantomData<T>,
}

pub fn run() {
    let fps_id = Identity::<FirstPersonShooter> {
        value: 1001,
        marker: PhantomData,
    };

    let rts_id = Identity::<RealTimeStrategy> {
        value: 1002,
        marker: PhantomData,
    };

    let rpg_id = Identity::<RolePlayingGame> {
        value: 1003,
        marker: PhantomData,
    };

    let number = 42u64;
    println!(
        "Number(u64): {} and the size of number is {}",
        number,
        std::mem::size_of_val(&number)
    );

    println!(
        "FPS ID: {} and the size of struct is {}",
        fps_id.value,
        std::mem::size_of_val(&fps_id)
    );
    println!(
        "RTS ID: {} and the size of struct is {}",
        rts_id.value,
        std::mem::size_of_val(&rts_id)
    );
    println!(
        "RPG ID: {} and the size of struct is {}",
        rpg_id.value,
        std::mem::size_of_val(&rpg_id)
    );
}
