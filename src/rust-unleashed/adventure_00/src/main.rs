#![allow(dead_code)]

struct Identity {
    value: u32,
}

impl Drop for Identity {
    fn drop(&mut self) {
        println!("Buckle your seat belt, dorothy, because kansas is going bye-bye");
    }
}
fn main() {
    // case_one();
    case_two();
    println!("End of the app");
}

fn case_one() {
    let _id = &Identity { value: 1001 };
    println!("Scope is closing...");
}

fn case_two() {
    _ = &Identity { value: 1001 };
    println!("Scope is closing...");
}
