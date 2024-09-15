## Tutorial 14 - Enums and Pattern Matching

### The Book

Chapter 6: Enums and Pattern Matching

### Summary

- Enums are a way to define a type by enumerating its possible variants.
- Enums can have associated data.
- Enums can have methods.
- Enums can be used in pattern matching.

### Notes

Option is an enum that is defined in the standard library. Option is used to represent a value that may or may not be present. Option is used to represent a value that may or may not be present. Option is used to represent a value that may or may not be present.

e.g.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching.

e.g.

```rust
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        // if 3 is the value, print "three"
        Some(3) => println!("three"),
    }
}
```
