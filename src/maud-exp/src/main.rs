mod case_intro;
mod case_loops;
mod case_partial;

use std::fs::File;
use std::io::Write;

fn main() {
    // let content = case_one::create();
    // let content = case_loops::create();
    let content = case_partial::create();
    println!("{}", content);

    let mut index_file = File::create("index.html").expect("Unable to create file");
    index_file
        .write_all(content.as_bytes())
        .expect("Unable to write data");
}
