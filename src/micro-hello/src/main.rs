#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;

#[entry]
fn start() -> ! {
    let mut board = Board::take().unwrap();
    // Tüm Led'ler kapalı
    let clear = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // Belli ledler açık ve H harfi şeklinde yanıyor.
    let H = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ];
    // Matrisleri kullanabilecek bir Display nesnesi
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    loop {
        // 1 saniye boyunca clear matrisine göre led'ler konumlanacak (Hepsi Kapalı)
        display.show(&mut timer, clear, 1000);
        // 1 saniye boyunca H matrisindeki Led'ler yanacak
        display.show(&mut timer, H, 1000);
        // ve bu böyle sürüp gidecek (Blinky effect)
    }
}