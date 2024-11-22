## Tutorial 31 - Advanced Function and Closures

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Function as First-Class Citizens](#function-as-first-class-citizens)
    * [Closures](#closures)
    * [Closure Traits](#closure-traits)
    * [Using Fn, FnMut, and FnOnce as Trait Bounds](#using-fn-fnmut-and-fnonce-as-trait-bounds)
    * [Returning Closures](#returning-closures)
    * [Closures and Iterators](#closures-and-iterators)
    * [Performance Considerations](#performance-considerations)

<!-- vim-markdown-toc -->

### The Book

Chapter 18: [Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)

### Summary

Rust allows for expressive and powerful functional programming techniques. This includes advanced usage of functions and closures, traits like `Fn`, `FnMut`, and `FnOnce`, and techniques for creating flexible and reusable code.

Rust’s advanced functions and closures offer a flexible way to handle functional programming tasks:

- Functions as arguments/returns: Use fn pointers.
- Closures: Capture environment variables with Fn, FnMut, or FnOnce.
- Returning closures: Use impl Fn or Box<dyn Fn> for unknown sizes.
- Common use cases: Closures are integral to iterators and async programming.

### Notes

#### Function as First-Class Citizens

Rust treats functions as first-class citizens. This means you can pass functions as arguments, return them from other functions, and store them in variables.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let result = do_twice(add_one, 5);
    println!("Result: {}", result); // Result: 12
}
```

- `fn(i32) -> i32`: The type of a function pointer.
- Functions passed this way must have a concrete type known at compile time.

#### Closures

Closures in Rust are anonymous functions you can save in variables or pass as arguments. Unlike functions, they can capture variables from their environment.

```rust
let closure = |x| x + 1;
```

Capturing from the Environment

Closures capture variables from their scope in one of three ways:

- By Borrowing (`&T`)
- By Mutable Borrowing (`&mut T`)
- By Taking Ownership (`T`)

```rust
let x = 4;
let equal_to_x = |z| z == x; // Captures `x` by borrowing
println!("{}", equal_to_x(4)); // true
```

#### Closure Traits

Closures implement one or more of these traits:

- `Fn`: Captures by immutable reference.
- `FnMut`: Captures by mutable reference.
- `FnOnce`: Captures by taking ownership.

Trait Hierarchy

- A closure can only implement one of these traits.
- `Fn` is the least restrictive, and `FnOnce` is the most restrictive.

Example: Moving Ownership

```rust
let x = String::from("hello");
let consume = move || println!("Captured: {}", x); // `x` is moved
consume();
// `x` is no longer accessible here
```

#### Using Fn, FnMut, and FnOnce as Trait Bounds

To accept closures in a function, use trait bounds:

```rust
fn call_with_one<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    closure(1)
}

fn main() {
    let double = |x| x * 2;
    println!("{}", call_with_one(double)); // 2
}
```

#### Returning Closures

Closures have unknown size at compile time due to being dynamically allocated. To return closures:

- Use `impl Fn` (for Rust 1.26+).
- Use a `Box<dyn Fn>` if heap allocation is acceptable.

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let closure = returns_closure();
    println!("{}", closure(2)); // 3
}
```

#### Closures and Iterators

Closures are commonly used with iterators for data processing.

```rust
let numbers = vec![1, 2, 3, 4];
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
println!("{:?}", doubled); // [2, 4, 6, 8]
```

#### Performance Considerations

- Rust's zero-cost abstractions mean closures are generally efficient.
- Rust can optimize closures to inline code or use them without heap allocation.
