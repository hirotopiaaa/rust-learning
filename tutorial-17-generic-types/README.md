## Tutorial 17 - Generic Types

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
* [Example](#example)
    * [Generic Function](#generic-function)
    * [Generic Struct](#generic-struct)
    * [Generic Enum](#generic-enum)
    * [Traits and Bounds](#traits-and-bounds)

<!-- vim-markdown-toc -->

### The Book

Chapter 10: Generic Types, Traits, and Lifetimes

### Summary

- Generic types are a way to define functions, structs, enums, and methods that work with any type.
- Generic types are a way to write code that is flexible and reusable.
- Generic types are a way to write code that is type-safe and efficient.
- Generic types are a way to write code that is easy to understand and maintain.

### Notes

Generics in Rust allow you to write flexible and reusable code. They enable you to define functions, structs, enums, and methods that can operate on different data types without sacrificing type safety. Here's a step-by-step explanation:

1. **Generic Functions**: You can define functions that take parameters of any type.
2. **Generic Structs**: Structs can store data of any type.
3. **Generic Enums**: Enums can hold variants of any type.
4. **Traits and Bounds**: You can specify that a generic type must implement certain traits.

### Example

#### Generic Function

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

#### Generic Struct

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

#### Generic Enum

```rust
enum Option<T> {
    Some(T),
    None,
}
```

#### Traits and Bounds

```rust
fn print<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}
```

Generics ensure that your code is both flexible and type-safe, allowing for more abstract and reusable components.
