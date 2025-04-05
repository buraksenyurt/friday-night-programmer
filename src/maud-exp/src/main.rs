mod case_one;
mod case_two;

use std::fs::File;
use std::io::Write;

fn main() {
    // let content = case_one::create();
    let content = case_two::create();
    println!("{}", content);

    let mut index_file = File::create("index.html").expect("Unable to create file");
    index_file
        .write_all(content.as_bytes())
        .expect("Unable to write data");
}
