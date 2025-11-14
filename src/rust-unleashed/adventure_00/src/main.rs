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
    case_when_to_use_underscore();
}

fn case_one() {
    let _id = &Identity { value: 1001 };
    println!("Scope is closing...");
}

fn case_two() {
    _ = &Identity { value: 1001 };
    println!("Scope is closing...");
}

fn case_when_to_use_underscore() {
    let in_future_value = 23; // Warning: "unused variable: `in_future_value`"

    // Example of ignoring specific variables in a pattern
    let sections = (100, 200, 300, 400);
    let (_, second, _, fourth) = sections; // No warning for unused variables
    println!("Second section: {}, Fourth section: {}", second, fourth);

    // Example of ignoring remaining elements in a pattern
    let top_scorers = ["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace"];
    match top_scorers {
        [first, second, third, ..] => {
            println!("Top three scorers are: {}, {}, {}", first, second, third);
        }
    }

    // Example of does not bind a variable
    for _ in 0..3 {
        println!("It's a great day for Rust programming!");
    }

    // Example of ignoring a function return value
    fn compute_value() -> i32 {
        42
    }
    _ = compute_value(); // No warning for unused return value

    // Example of always match a value
    let score = Some(85);
    match score {
        Some(_) => println!("You have a score!"), // Variable not bound
        None => println!("No score available."),
    }
}
