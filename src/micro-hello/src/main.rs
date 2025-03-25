#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod letter_pipe;

use crate::letter_pipe::{Letter, get_letter};
use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;

#[entry]
fn start() -> ! {
    let board = Board::take().unwrap();

    // Matrisleri kullanabilecek bir Display nesnesi
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    const WAIT: u32 = 500;
    loop {
        // 250 milisaniye boyunca clear matrisine göre led'ler konumlanacak (Hepsi Kapalı)
        display.show(&mut timer, get_letter(Letter::Clear), WAIT);
        // 250 saniye boyunca R matrisindeki Led'ler yanacak
        display.show(&mut timer, get_letter(Letter::R), WAIT);
        display.show(&mut timer, get_letter(Letter::U), WAIT);
        display.show(&mut timer, get_letter(Letter::S), WAIT);
        display.show(&mut timer, get_letter(Letter::T), WAIT);

        // ve bu böyle sürüp gidecek (Blinky effect)
    }
}
