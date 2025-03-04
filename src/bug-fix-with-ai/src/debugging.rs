use std::fs::File;
use std::io::{Read, BufReader};

fn read_values_from_file(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    
    reader.read_to_string(&mut content).unwrap();
    
    content.split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn run() {
    let values = read_values_from_file("there_is_no_spoon.data");
    let sum: i32 = values.iter().sum();
    println!("Total: {}", sum);
}
