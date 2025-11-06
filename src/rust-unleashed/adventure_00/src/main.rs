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
    {
        let _id = Identity { value: 1001 };
        println!("Scope is closing...");
    }
    println!("End of the app");
}
