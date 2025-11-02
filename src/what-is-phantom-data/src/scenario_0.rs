struct SomeData;

pub fn run() {
    let _data = SomeData;
    println!("Size of SomeData: {}", std::mem::size_of_val(&_data));
}
