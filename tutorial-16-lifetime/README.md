## Tutorial 16 - Lifetimes

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
* [Example](#example)

<!-- vim-markdown-toc -->

### The Book

Chapter 10: Generic Types, Traits, and Lifetimes

### Summary

- Lifetimes are a way to ensure that references in Rust are always valid.
- Lifetimes are a type system feature that allows you to specify the relationship between the lifetimes of references in your code.
- Lifetimes are a way to prevent dangling references, which can cause undefined behavior and memory safety issues.
- Lifetimes are a way to ensure that references are always valid for the duration of their lifetime.

### Notes

Lifetimes in Rust are a way of ensuring that references are valid as long as they are needed and no longer. They prevent dangling references, which can lead to undefined behavior. Lifetimes are denoted using an apostrophe followed by a name, like `'a`.

Lifetimes are a powerful feature in Rust that help ensure memory safety by preventing dangling references. They can be complex at first, but they are essential for writing safe and efficient Rust code.

Using a lifetime annotation in a function signature helps the Rust compiler (borrow checker) understand the relationship between the lifetimes of the parameters and return values. This information is crucial for the compiler to ensure that references are always valid (identify dangling references).

>[!NOTE]
> A reference lifetime should be shorter than the data it points to

Here's a brief overview:

1. **Basic Concept**: Lifetimes are annotations that tell the Rust compiler how long references should be valid. They are used to prevent dangling references.

2. **Syntax**: Lifetimes are specified using an apostrophe followed by a name, e.g., `'a`.

3. **Function Signatures**: When you have functions that take references as parameters, you might need to specify lifetimes to indicate how the lifetimes of the parameters and return values relate to each other.

4. **Lifetime Elision**: In many cases, Rust can infer lifetimes, so you don't need to specify them explicitly. This is known as lifetime elision.

5. **Structs with References**: When structs contain references, you need to specify lifetimes to ensure the references within the struct are valid for as long as the struct is.

### Example

Here's a simple example to illustrate lifetimes in a function:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In this example:

- `'a` is a lifetime parameter.
- `&'a str` means a reference to a string slice that lives at least as long as `'a`.
- The function `longest` takes two string slices that must live at least as long as `'a` and returns a string slice that also lives at least as long as `'a`.

Static lifetime `'static` is a special lifetime that indicates that a reference can live for the entire duration of the program. It is used for string literals, which are stored in the program's binary and are always available.

```rust
fn main() {
    let s: &'static str = "Hello, world!";
}
```
