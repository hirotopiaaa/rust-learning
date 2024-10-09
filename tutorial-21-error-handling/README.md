## Tutorial 21 - Error Handling

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Result and Option Types](#result-and-option-types)
    * [Unwrapping and Expecting](#unwrapping-and-expecting)
    * [? Operator](#-operator)
    * [Custom Error Types](#custom-error-types)
    * [Error Propagation](#error-propagation)

<!-- vim-markdown-toc -->

### The Book

Chapter 9: Error Handling

### Summary

Error handling in Rust is a crucial aspect of the language that ensures robustness and reliability. Rust uses a combination of enums and the type system to manage errors in a way that’s expressive and encourages the developer to handle errors explicitly.

Rust’s error handling emphasizes safety and explicitness, ensuring that errors are not ignored and are dealt with properly. The combination of `enums`, the `? operator`, and `custom error types` allows developers to write robust, clear, and maintainable code.

### Notes

#### Result and Option Types

Rust has two primary enums for error handling:

- `Result<T, E>`: Used when an operation can return either success `(Ok(T))` or an error `(Err(E))`. It’s the most common way to handle errors.
  - T represents the success type.
  - E represents the error type.

Example:

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
}
```

- `Option<T>`: Used when a value might be present `(Some(T))` or absent `(None)`. It is commonly used when you might not have an error but just an absence of a value.

Example:

```rust
fn get_user_name(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Kevin"))
    } else {
        None
    }
}
```

#### Unwrapping and Expecting

Sometimes, you want to directly extract the value from a Result or Option, knowing the result should be valid:

- `unwrap()`: Extracts the value, but panics if there’s an error or None.
- `expect(msg)`: Similar to `unwrap()` but allows a custom panic message.

```rust
let value = divide(10.0, 2.0).unwrap(); // Works fine if no error, panics otherwise
let value = divide(10.0, 0.0).expect("Failed to divide"); // Custom error message on panic
```

#### ? Operator

The `?` operator is a shorthand for propagating errors. It simplifies code by returning the error if the Result is Err, or extracting the value if it’s Ok.

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn compute() -> Result<f64, String> {
    let result = divide(4.0, 2.0)?; // Propagates error if any
    Ok(result + 2.0)
}
```

#### Custom Error Types

Rust allows creating custom error types for more complex scenarios. This involves implementing the `std::fmt::Debug` and `std::fmt::Display` traits for custom error messages, and possibly the `std::error::Error` trait for compatibility.

```rust
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

fn example_function() -> Result<(), MyError> {
    Err(MyError { message: String::from("An error occurred") })
}
```

#### Error Propagation

Error propagation is the process of passing errors up the call stack to be handled by the caller. Rust’s `?` operator is a concise way to propagate errors, but you can also use `match` or `if let` to handle errors explicitly.

The best practice is to handle errors at the point where they can be handled effectively. This ensures that errors are not ignored and are dealt with appropriately. If the error cannot be handled at a particular level, it should be propagated up the call stack.
