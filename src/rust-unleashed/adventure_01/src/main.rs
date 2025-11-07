#![allow(dead_code)]

// const MAX_LEVEL: u32; // 'free constant item without body' error

struct BackgroundColor {
    name: &'static str,
    id: u32,
}

impl Drop for BackgroundColor {
    fn drop(&mut self) {
        println!("Dropping constant. State: {},{}", self.name, self.id);
    }
}

const BACKGROUND_COLOR: BackgroundColor = BackgroundColor {
    name: "Lightsaber",
    id: 1,
};

fn main() {
    let value = &mut BACKGROUND_COLOR;
    value.name = "Black pearl";
    value.id = 2;
    println!("Value Name: {} and ID: {}", value.name, value.id);

    BACKGROUND_COLOR.name = "Red wine";
    BACKGROUND_COLOR.id = 2;
    println!(
        "Background Color Name: {} and Id {}",
        BACKGROUND_COLOR.name, BACKGROUND_COLOR.id
    );
}
