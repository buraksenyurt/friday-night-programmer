---
layout: ../../layouts/MarkdownPostLayout.astro
title: Structs in Rust
pubDate: 2026-05-12
description: This article explains the concept of structs in Rust, including how to define and use them, as well as the differences between named and tuple structs.
author: Burak Selim Şenyurt
image:
    url: "https://docs.astro.build/assets/arc.webp"
    alt: "The Astro logo on a dark background with a purple gradient arc."
tags: ['rust', 'structs', 'programming']
---
Let 's run this code in your local machine.

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    println!("{} is {} years old.", person1.name, person1.age);
    println!("{} is {} years old.", person2.name, person2.age);
}
```

That's it! You have successfully defined and used structs in Rust. In this example, we created a `Person` struct with two fields: `name` and `age`. We then created two instances of the `Person` struct and printed their information to the console.
