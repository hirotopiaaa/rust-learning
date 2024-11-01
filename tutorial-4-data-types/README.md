# Tutorial 4 - Data Types

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [Notes](#notes)
    * [Strings](#strings)

<!-- vim-markdown-toc -->

### Notes

#### Strings

In Rust, `String` and `str` are two distinct types for handling textual data, each serving specific purposes and requirements:

1. **`String`**:

   - A dynamically allocated, growable string. It owns its data, meaning it has a defined heap memory ownership.
   - You can modify the content of a `String`, add characters, or change its size as needed.
   - A `String` is often created from a `str` slice by calling methods like `to_string()` or `String::from()`.
   - Example:
     ```rust
     let mut s = String::from("Hello");
     s.push_str(", world!");
     ```

2. **`str`**:
   - A fixed-size string slice, commonly seen as `&str`, which is a reference to a segment of a string.
   - `str` itself doesn’t own its data, so it’s usually borrowed as an immutable slice (`&str`) or mutable slice (`&mut str`).
   - It's not directly modifiable, making it more lightweight and efficient for read-only access to string data.
   - Example:
     ```rust
     let greeting: &str = "Hello, world!";
     ```

In practice:

- Use `String` when you need to own and modify string data.
- Use `&str` when you need a lightweight reference to string data, often for parameters in functions.
