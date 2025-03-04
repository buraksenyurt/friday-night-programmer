pub fn run(){
    let motto = String::from("AI Programming with Rust");
    do_something(motto);
    println!("{}", motto);
}

fn do_something(message:String) {
    println!("Incoming message : {}", message);
}
