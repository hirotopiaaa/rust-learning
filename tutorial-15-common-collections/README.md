## Tutorial 15 - Common Collections

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Vectors](#vectors)
        * [Creating a Vector](#creating-a-vector)
        * [Using the `vec!` Macro](#using-the-vec-macro)
        * [Adding Elements](#adding-elements)
        * [Accessing Elements](#accessing-elements)
        * [Iterating Over a Vector](#iterating-over-a-vector)
        * [Modifying Elements](#modifying-elements)
    * [Strings](#strings)
        * [Creating a String](#creating-a-string)
        * [Using the `to_string` Method](#using-the-to_string-method)
        * [Using the `String::from` Function](#using-the-stringfrom-function)
        * [Appending to a String](#appending-to-a-string)
        * [Appending a Single Character](#appending-a-single-character)
        * [Concatenating Strings](#concatenating-strings)
        * [Using the `format!` Macro](#using-the-format-macro)
        * [Iterating Over a String](#iterating-over-a-string)
        * [Slicing a String](#slicing-a-string)
    * [Hash Maps](#hash-maps)
        * [Steps to use a HashMap in Rust:](#steps-to-use-a-hashmap-in-rust)
        * [Example Code](#example-code)

<!-- vim-markdown-toc -->

### The Book

Chapter 8: Common Collections

### Summary

- Vectors are used to store a variable number of values.
- Strings are used to store text.
- Hash maps are used to store key-value pairs.

### Notes

#### Vectors

In Rust, a vector is a dynamic array that can grow or shrink in size. It is part of the standard library and is defined in the `std::vec` module. Vectors are useful when you need a collection that can change size. They are implemented as a contiguous growable array type, which means they store elements next to each other in memory, providing efficient access and modification.

Here is a brief overview of how to use vectors in Rust:

##### Creating a Vector

```rust
let mut v: Vec<i32> = Vec::new();
```

##### Using the `vec!` Macro

```rust
let v = vec![1, 2, 3];
```

##### Adding Elements

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
```

##### Accessing Elements

```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
```

##### Iterating Over a Vector

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

##### Modifying Elements

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Vectors are a fundamental part of Rust's collection types and are widely used due to their flexibility and performance.

---

#### Strings

Strings are another common collection type in Rust that is used to store text data. Strings are implemented as a collection of bytes, and they are UTF-8 encoded by default. Rust's `String` type is a growable, mutable, owned, UTF-8 encoded string type that is part of the standard library.

Here is a brief overview of how to use strings in Rust:

In Rust, a `String` is a growable, mutable, owned, UTF-8 encoded string type. It is part of the standard library and is defined in the `std::string` module. `String` is used when you need to store and manipulate text data that can change in size.

Here is a brief overview of how to use `String` in Rust:

##### Creating a String

```rust
let mut s = String::new();
```

##### Using the `to_string` Method

```rust
let data = "initial contents";
let s = data.to_string();
```

##### Using the `String::from` Function

```rust
let s = String::from("initial contents");
```

##### Appending to a String

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

##### Appending a Single Character

```rust
let mut s = String::from("lo");
s.push('l');
```

##### Concatenating Strings

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
```

##### Using the `format!` Macro

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```

##### Iterating Over a String

```rust
for c in "hello".chars() {
    println!("{}", c);
}
```

##### Slicing a String

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Strings in Rust are powerful and flexible, allowing for efficient manipulation of text data. They are essential for many applications, from simple text processing to complex data handling.

---

#### Hash Maps

A hash map is a data structure that stores key-value pairs. It allows for fast retrieval, insertion, and deletion of values based on their keys. In Rust, the `HashMap` is provided by the standard library and is part of the `std::collections` module.

##### Steps to use a HashMap in Rust:

1. **Import the HashMap module**: You need to bring the `HashMap` into scope.
2. **Create a HashMap**: Instantiate a new `HashMap`.
3. **Insert key-value pairs**: Use the `insert` method to add entries.
4. **Access values**: Use the `get` method to retrieve values.
5. **Iterate over entries**: Use a for loop to iterate over key-value pairs.
6. **Remove entries**: Use the `remove` method to delete entries.

##### Example Code

```rust
// Import the HashMap module
use std::collections::HashMap;

fn main() {
    // Create a new HashMap
    let mut scores = HashMap::new();

    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access values
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("The score for {} is {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Remove a key-value pair
    scores.remove(&String::from("Blue"));

    // Check the updated HashMap
    println!("Updated scores: {:?}", scores);
}
```

Explanation

1. **Importing**: `use std::collections::HashMap;` brings the `HashMap` type into scope.
2. **Creating**: `let mut scores = HashMap::new();` creates a mutable `HashMap`.
3. **Inserting**: `scores.insert(String::from("Blue"), 10);` adds a key-value pair to the map.
4. **Accessing**: `scores.get(&team_name)` retrieves the value associated with `team_name`.
5. **Iterating**: `for (key, value) in &scores` loops through all key-value pairs.
6. **Removing**: `scores.remove(&String::from("Blue"));` removes the entry for the key "Blue".

This example demonstrates the basic operations you can perform with a `HashMap` in Rust.
