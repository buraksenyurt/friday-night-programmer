struct Part {
    id: u32,
    name: String,
    stock_level: i32,
}

impl Part {
    fn change_stock_level(&mut self, change: i32) -> () {
        self.stock_level += change;
    }
}

// CASE02: Nesne içeriğine nesneye ait metot ile değiştirme
fn main() {
    let mut part = Part {
        id: 1,
        name: String::from("Widget"),
        stock_level: 10,
    };

    println!("Initial stock level of {}: {}", part.name, part.stock_level);
    part.change_stock_level(5);
    println!("Stock level after adding 5: {}", part.stock_level);
}

// // CASE01: Geriye yeni bir nesne döndürerek state değiştirme
// fn main() {
//     let mut part = Part {
//         id: 1,
//         name: String::from("Widget"),
//         stock_level: 10,
//     };

//     println!("Initial stock level of {}: {}", part.name, part.stock_level);
//     part = change_stock_level(part, 5);
//     println!("Stock level after adding 5: {}", part.stock_level);
// }

// fn change_stock_level(part: Part, change: i32) -> Part {
//     Part {
//         stock_level: part.stock_level + change,
//         ..part
//     }
// }


// //CASE00: Referans yoluyla state değiştirme
// fn main() {
//     let mut part = Part {
//         id: 1,
//         name: String::from("Widget"),
//         stock_level: 10,
//     };

//     println!("Initial stock level of {}: {}", part.name, part.stock_level);
//     change_stock_level(&mut part, 5);
//     println!("Stock level after adding 5: {}", part.stock_level);
// }

// fn change_stock_level(part: &mut Part, change: i32) {
//     part.stock_level += change;
// }

