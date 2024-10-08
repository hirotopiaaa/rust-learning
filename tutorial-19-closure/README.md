## Tutorial 19 - Closure

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)

<!-- vim-markdown-toc -->

### The Book

Chapter 13: Closures: Anonymous Functions that Can Capture Their Environment

### Summary

In Rust, closures are similar to functions but have additional capabilities, like capturing variables from their environment. They are anonymous functions that can be stored in variables or passed as arguments to other functions.

- Closures are similar to functions but can capture variables from the environment.
- They have flexible parameter and return types, often allowing type inference.
- Rust decides how to capture variables (by borrowing, mutably borrowing, or taking ownership) based on the closure's usage.
- Closures can be used as function arguments and returned from functions.

### Notes

In Rust, a closure is an anonymous function that can capture variables from its surrounding scope. Closures are similar to functions but have some differences:

1. **Type Inference**: Closures can infer the types of their parameters and return values.
2. **Environment Capture**: Closures can capture variables from their enclosing scope by reference, by mutable reference, or by value.
3. **Flexibility**: Closures can be stored in variables, passed as arguments to functions, and returned from functions.

Here's a simple example of a closure in Rust:

```rust
fn main() {
    let x = 10;
    let add_x = |y| y + x; // Closure capturing `x` from the environment
    println!("{}", add_x(5)); // Output: 15
}
```

In this example, the closure `add_x` captures the variable `x` from its surrounding scope and adds it to its parameter `y`.

For ownership and borrowing rules, closures can capture variables by reference, by mutable reference, or by value. The `move` keyword can be used to force a closure to take ownership of the variables it captures. We often use the `move` keyword when we want to move the closure to another thread or store it in a data structure.

---

Closures as Function Parameters

Closures can be passed as arguments to functions. Rust uses traits to determine how a closure should be used:

- Fn: For closures that only borrow from the environment.
- FnMut: For closures that modify the environment.
- FnOnce: For closures that take ownership of the environment.

Here's a function that takes a closure as an argument:

```rust
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

let name = String::from("World");
let greet = || println!("Hello, {}!", name);
apply(greet);
```

---

Returning Closures

Closures can also be returned from functions, but the syntax is a bit tricky since Rust needs to know the exact type of the closure. Typically, this involves using the impl Trait syntax:

```rust
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let adder = create_adder(5);
println!("{}", adder(3)); // Output: 8
```
