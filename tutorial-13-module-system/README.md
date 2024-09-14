## Tutorial 13 - Module System

### The Book

Chapter 10.2: Modules as a Privacy Boundary

### Summary

- Modules can be used to control the visibility of items.
- Modules can be used to organize code.
- Modules can be used to define a public API.
- Modules can be used to hide implementation details.
- Modules can be used to define a hierarchy of modules.

### Notes

A binary crate is a crate that produces an executable. A library crate is a crate that produces a library. A crate is a package of Rust code.

In `/bin` directory, we can have multiple binary crates. In `/lib` directory, we can have multiple library crates.

Use `cargo new --lib mylib` to create a library crate. Use `cargo new --bin mybin` to create a binary crate.

A module is a way to organize code. Modules can be nested. Modules can be used to control the visibility of items. Modules can be used to define a public API. Modules can be used to hide implementation details.

The notation `mod foo;` is used to load the contents of `foo.rs` or `foo/mod.rs`. The notation `mod foo;` is used to define and create a module named `foo`.

Module tree:

```console
crate
├── front_of_house
│   ├── hosting
│   │   ├── add_to_waitlist
│   │   ├── seat_at_table
│   ├── serving
│   │   ├── take_order
│   │   ├── serve_order
│   │   ├── take_payment
```

The `use` keyword in Rust is used to bring a module into scope. The `use` keyword in Rust is used to bring a module into scope. The `use` keyword in Rust is used to bring a module into scope. The `use` keyword in Rust is used to bring a module into scope.

e.g

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

In this example, `HashMap` is brought into scope using the `use` keyword.

For export and import use cases, if we want to separate the child modules in its own files, follow the rules below:

- If a module is defined in a file, the file should be named the same as the module.
- If a module is defined in a directory, the directory should contain a file named `mod.rs`.
