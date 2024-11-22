## Tutorial 30 - Advanced Types

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Newtype Pattern](#newtype-pattern)
    * [Type Alias](#type-alias)
    * [Never Type](#never-type)
    * [Dynamically Sized Types](#dynamically-sized-types)

<!-- vim-markdown-toc -->

### The Book

Chapter 19 - Advanced Types

### Summary

Rust provides powerful and flexible advanced type features to express complex relationships between data and functionality. These features enhance type safety, expressiveness, and compile-time correctness.

### Notes

#### Newtype Pattern

The newtype pattern is a way to create a new type that is distinct from its original type. This pattern is useful for creating types that have the same representation as another type but are considered to be different types for the type system.

The newtype pattern involves creating a wrapper type to encapsulate another type, often used for stronger typing.

```rust
struct Millimeters(u32);
struct Meters(u32);

fn add_lengths(a: Millimeters, b: Meters) -> Millimeters {
    Millimeters(a.0 + b.0 * 1000)
}
```

#### Type Alias

A type alias is a way to give a new name to an existing type. Type aliases are useful when you want to reduce boilerplate by creating a new name for an existing type.

Type aliases provide a way to simplify complex type names.

```rust
type Result<T> = std::result::Result<T, std::io::Error>;

fn read_file() -> Result<String> {
    Ok("File content".to_string())
}
```

#### Never Type

The never type `!` is a special type in Rust that is used for functions that never return. The never type is useful for functions that always panic or exit the program.

The `!` type represents computations that never complete, such as a function that panics or loops infinitely.

```rust
fn never_returns() -> ! {
    panic!("This function never returns!");
}
```

#### Dynamically Sized Types

Dynamically sized types (DSTs) are types that have a size that is not known at compile time. DSTs are useful for working with dynamically sized data, such as slices and trait objects.

In Rust, most types are statically sized, meaning their size is known at compile time. However, some types do not have a fixed size known at compile time. These are called Dynamically Sized Types (DSTs) or unsized types. They are a fundamental part of Rust's type system, enabling powerful abstractions like slices and trait objects.

Characteristics of DSTs:

Size Unknown at Compile Time:

- For statically sized types (e.g., i32, String), the size in memory is known during compilation.
- For DSTs, their size can only be determined at runtime.

Examples:

- Slices: [T] (e.g., [i32], str).
- Trait Objects: dyn Trait.

Pointer Requirements:

- You cannot directly work with DSTs without some kind of indirection, like a pointer (e.g., &, Box, or Rc).
- This is because Rust needs to store additional metadata (like the size of the data or the vtable for a trait object) alongside the reference.

Example (Slices ([T])):

A slice is a view into a contiguous block of memory. Its size depends on the number of elements in the slice, which is not known at compile time.

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

let array = [1, 2, 3, 4];
let slice = &array[1..3]; // slice: &[i32]
println!("{}", sum(slice)); // Output: 5
```

Example (Trait Objects (dyn Trait)):

A trait object (dyn Trait) enables dynamic dispatch, meaning the method implementation is determined at runtime. The size of the concrete type implementing the trait is unknown at compile time.

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing Circle");
    }
}

fn draw_object(obj: &dyn Draw) {
    obj.draw();
}

let circle = Circle;
draw_object(&circle);
```

Example (Strings (str)):

```rust
fn print_slice(s: &str) {
    println!("{}", s);
}

let greeting = "Hello, world!";
print_slice(&greeting[0..5]); // Output: Hello
```
