## Tutorial 18 - Testing

## Table of Contents

<!-- vim-markdown-toc GFM -->

    * [The Book](#the-book)
    * [Summary](#summary)
* [Notes](#notes)
    * [Example](#example)
    * [Running Tests](#running-tests)
        * [Example](#example-1)

<!-- vim-markdown-toc -->

### The Book

Chapter 11: Writing Automated Tests

### Summary

- Writing tests is an essential part of software development.
- Tests help ensure that your code works as expected and catches bugs early.
- Rust has a built-in testing framework that makes it easy to write and run tests.
- You can write unit tests, integration tests, and documentation tests in Rust.

## Notes

In Rust, testing is built into the language and the standard library. Here's a step-by-step explanation:

1. **Test Functions**: Functions are considered tests if they are annotated with the `#[test]` attribute.
2. **Test Module**: Tests are usually placed in a module named `tests` which is annotated with `#[cfg(test)]`. This ensures that the test code is only compiled when running tests.
3. **Running Tests**: Tests can be run using the `cargo test` command.
4. **Assertions**: Rust provides several macros for assertions, such as `assert!`, `assert_eq!`, and `assert_ne!`, to check conditions within test functions.

### Example

Here's a simple example of a Rust module with tests:

```rust
// src/lib.rs

// Function to be tested
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Tests module
#[cfg(test)]
mod tests {
    // Import the outer module
    use super::*;

    // Test function
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

### Running Tests

To run the tests, use the following command in the terminal:

```sh
cargo test
```

This will compile the code in test mode and run all the test functions. The output will show which tests passed and which failed.

If we want to show the console output of the tests, we can use the `-- --show-output` flag:

```sh
cargo test -- --show-output
```

If we want to only run a specific test function, we run `cargo test` followed by the test function name:

```sh
cargo test <fn name>
```

If we want to run tests per module, we can use the following:

```sh
cargo test <mod>::
```

For integration tests, we can create a `tests` directory in the project root and place test files inside it. These tests will be treated as separate executables and can test the public API of the crate.

#### Example

Here's an example of an integration test file:

```rust
// tests/integration_test.rs

use my_crate::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(2, 3), 5);
}
```

Every file in the `tests` directory is treated as a separate crate, so we need to import the functions we want to test.

To run integration tests, use the following command:

```sh
cargo test --test integration_test
```
