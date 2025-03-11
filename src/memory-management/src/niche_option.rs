use std::mem::transmute;
use std::num::NonZero;
use std::num::NonZeroU32;

#[allow(dead_code)]
pub fn run() {
    println!("Baştan söyleyelim...");
    println!("u32 {} byte yer tutar", size_of::<u32>());
    println!(
        "Option<u32> ise {} byte yer tutar. Diğer 4 byte None içindir.",
        size_of::<Option<u32>>()
    );
    println!("NonZero32 {} byte yer tutar", size_of::<NonZeroU32>());
    println!(
        "Option<NonZero32> ise yine {} byte yer tutar. Zira 0, None olarak ifade edilir.",
        size_of::<Option<NonZeroU32>>()
    );

    let nan = give_me_a_none();
    match nan {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u32 = unsafe { transmute(nan) };
    println!("NonZeroU32 için None : {transmuted:b}");

    let nan = give_me_another_none();
    match nan {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u64 = unsafe { transmute(nan) };
    println!("U32 için None : {transmuted:b}");

    let number = give_me_a_number();
    match number {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u32 = unsafe { transmute(number) };
    println!("NonZero için Number 23 : {transmuted:b}");

    let number = give_me_another_number();
    match number {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u64 = unsafe { transmute(number) };
    println!("U32 için Number 23 : {transmuted:b}");

    println!("&u32 türü için de {} byte yer ayrılır.", size_of::<&u32>());
    println!(
        "ve Option<&u32> içinde {} byte söz konusudur.",
        size_of::<Option<&u32>>()
    );
}

fn give_me_a_none() -> Option<NonZeroU32> {
    NonZero::new(0)
    // None
}

fn give_me_another_none() -> Option<u32> {
    None
}

fn give_me_a_number() -> Option<NonZeroU32> {
    NonZero::new(23)
}

fn give_me_another_number() -> Option<u32> {
    Some(23)
}
