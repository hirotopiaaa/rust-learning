## Tutorial 27 - Patterns and Matching

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [What Are Patterns](#what-are-patterns)
    * [Match Expressions](#match-expressions)
    * [Pattern Types](#pattern-types)
    * [Advanced Pattern Matching Techniques](#advanced-pattern-matching-techniques)
    * [Irrefutable vs. Refutable Patterns](#irrefutable-vs-refutable-patterns)

<!-- vim-markdown-toc -->

### The Book

Chapter 18: Patterns and Matching

### Summary

Using patterns effectively in Rust can simplify your code and reduce boilerplate, especially when dealing with enums, structs, and other complex data types. This flexibility in destructuring makes Rust’s pattern matching a key feature for writing clear and concise code.

### Notes

In Rust, **patterns** and **matching** are powerful features that allow you to destructure and handle data flexibly. They are used extensively in Rust for control flow, particularly with enums and complex data types, making code more readable and expressive. Here's a breakdown:

#### What Are Patterns

Patterns in Rust are special forms used to match against values. They can be used in a variety of places, such as:

- `match` expressions
- `if let` and `while let` expressions
- `for` loops
- `let` statements

Patterns allow you to destructure complex data types, like tuples, enums, structs, and even references, by specifying what the data should look like. They provide a way to selectively access parts of the data and handle each possibility.

#### Match Expressions

A `match` expression is the most common way to use patterns. It enables you to compare a value against a series of patterns and execute code based on which pattern matches.

Here's the syntax for a `match` expression:

```rust
match value {
    Pattern1 => { /* code to execute */ },
    Pattern2 => { /* code to execute */ },
    _ => { /* code for default case */ }
}
```

- **Example with Enums**: Enums are commonly matched in Rust. Suppose you have an enum to represent network status:

  ```rust
  enum NetworkStatus {
      Connected(u32),
      Disconnected,
      Pending,
  }

  fn get_status_message(status: NetworkStatus) -> &'static str {
      match status {
          NetworkStatus::Connected(id) => "Connected to server",
          NetworkStatus::Disconnected => "Disconnected from server",
          NetworkStatus::Pending => "Connection pending...",
      }
  }
  ```

  In this example:

  - `NetworkStatus::Connected(id)` destructures the variant and captures the associated `id`.
  - `NetworkStatus::Disconnected` and `NetworkStatus::Pending` match the variants without additional data.

#### Pattern Types

Rust patterns come in different forms, depending on what you’re matching. Here are some common pattern types:

- **Literal Patterns**: Match literal values.

  ```rust
  let x = 5;
  match x {
      1 => println!("One"),
      5 => println!("Five"),
      _ => println!("Something else"),
  }
  ```

- **Tuple Patterns**: Match tuples by structure.

  ```rust
  let coordinates = (3, 5);
  match coordinates {
      (0, 0) => println!("Origin"),
      (x, y) => println!("Point is at ({}, {})", x, y),
  }
  ```

- **Struct Patterns**: Destructure structs.

  ```rust
  struct Point { x: i32, y: i32 }

  let p = Point { x: 5, y: 10 };
  match p {
      Point { x: 0, y: 0 } => println!("Origin"),
      Point { x, y } => println!("Point at ({}, {})", x, y),
  }
  ```

- **Enum Patterns**: Destructure enums, as shown in the previous example with `NetworkStatus`.

- **Range Patterns**: Match a range of values.

  ```rust
  let age = 18;
  match age {
      0..=12 => println!("Child"),
      13..=19 => println!("Teenager"),
      _ => println!("Adult"),
  }
  ```

- **Reference Patterns**: Match and destructure through references using `&` or `ref`.

#### Advanced Pattern Matching Techniques

- **`if let` and `while let`**: These expressions simplify cases where you only care about one pattern.

  ```rust
  let some_option = Some(5);
  if let Some(value) = some_option {
      println!("Got a value: {}", value);
  }
  ```

- **Pattern Guards**: Add conditions to patterns with `if` clauses.

  ```rust
  let num = Some(4);
  match num {
      Some(x) if x < 5 => println!("Less than 5"),
      Some(x) => println!("Got {}", x),
      None => println!("No value"),
  }
  ```

- **Bindings**: Use `@` to bind a name to a matched value.

  ```rust
  let point = Point { x: 10, y: 20 };
  match point {
      Point { x: 0..=5, y } => println!("Point in range"),
      Point { x: 10..=15, y: val @ 20 } => println!("Point at x: 10-15 and y = {}", val),
      _ => println!("Somewhere else"),
  }
  ```

#### Irrefutable vs. Refutable Patterns

- **Irrefutable patterns**: Patterns that will always match, such as a plain `let` statement (`let x = 5;`).
- **Refutable patterns**: Patterns that might not match, such as in `match`, `if let`, or `while let`.
