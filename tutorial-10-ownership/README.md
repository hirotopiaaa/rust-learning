## Tutorial 10 - Ownership

### The Book

Chapter 4.1: What is Ownership?

### Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    {
        // let s: &str = "hello"; // s is not valid here, it’s not yet declared
        let s: String = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid
}
```

Source URL: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

### References Rules

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
