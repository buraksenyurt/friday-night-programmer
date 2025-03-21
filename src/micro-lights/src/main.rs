#![deny(unsafe_code)]
#![no_main] // klasik rust giriş fonksiyonuna bağlı çalışmayacak
#![no_std] // rust standart kütüphanesi kullanılmayacak

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;

#[entry] // entry point olduğunu belirtir
fn start() -> ! { // ! dönüyor. Bu fonksiyonun hiç sonlanmayacağını belirtmekte
    let mut board = Board::take().unwrap(); // Board'un sahipliğini alıyoruz
    let _ = board.display_pins.col3.set_low();
    let mut row3 = board.display_pins.row3;
    let mut timer = Timer::new(board.TIMER0);

    loop {
        let _ = row3.set_low(); // Işığı kapatıyor
        timer.delay_ms(1_500); // 1.5 saniye gecikme
        let _ = row3.set_high(); // Işığı açıyor
        timer.delay_ms(1_500); // 1.5 saniye gecikme
    }
}

//use core::panic::PanicInfo;
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
