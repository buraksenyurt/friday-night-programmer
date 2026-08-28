---
layout: ../../layouts/MarkdownPostLayout.astro
title: Variables in Rust
author: Burak Selim Şenyurt
description: This article explains the concept of variables in Rust, including how to declare and use them, as well as the differences between mutable and immutable variables.
image:
    url: "https://docs.astro.build/assets/arc.webp"
    alt: "The Astro logo on a dark background with a purple gradient arc."
pubDate: 2026-08-28
tags: ['rust', 'variables', 'programming']
---
Run this code in your local machine.

```rust
fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y += 5; // Modify the value of y
    println!("The modified value of y is: {}", y);
}
```

In this example, we declare an immutable variable `x` and a mutable variable `y`. The immutable variable cannot be changed after its initial assignment, while the mutable variable can be modified.

> To be continued
