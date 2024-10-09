## Tutorial 22 - Publish Crate

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Documentation Comments](#documentation-comments)
    * [Release Profile](#release-profile)
    * [Publishing a Crate](#publishing-a-crate)

<!-- vim-markdown-toc -->

### The Book

Chapter 14: Cargo and Crates.io

### Summary

### Notes

#### Documentation Comments

Rust allows you to write documentation comments using `///` or `//!` to document your code. These comments are used to generate documentation for your project using `cargo doc`.

- Use `///` to document items like functions, structs, enums, and modules
- Use `//!` to document the containing item (e.g., the crate or module)

Build the project with `cargo doc` to generate documentation:

```sh
cargo doc
```

Tips:

- Use `pub` and `use` statements to make items public and accessible in the documentation:

#### Release Profile

When you build your Rust project, you can specify different profiles for different purposes. The two main profiles are `dev` and `release`.

- **dev**: This profile is used for development and debugging. It has fewer optimizations and is faster to compile.
- **release**: This profile is used for building the final optimized version of your project. It has more optimizations enabled and produces faster and smaller binaries.

You can build your project with the `--release` flag to use the release profile:

```sh
cargo build --release
```

This will compile your project with optimizations enabled, resulting in a faster and smaller binary.

#### Publishing a Crate

Once you have a crate that you want to share with others, you can publish it to [crates.io](https://crates.io/), the official Rust package registry. Here are the steps to publish a crate:

1. **Create an Account**: You need to create an account on crates.io to publish crates. You can do this by registering on the website.
2. **Update Cargo.toml**: Make sure your `Cargo.toml` file has the correct information, including the name, version, and description of your crate.
3. **Login**: Use the `cargo login` command to log in to crates.io with your account credentials.
4. **Publish**: Use the `cargo publish` command to publish your crate to crates.io. This will upload your crate to the registry and make it available for others to use.

Here's an example of publishing a crate:

```sh
cargo login
cargo publish
```

After publishing your crate, others can use it by adding it to their `Cargo.toml` file:

```toml
[dependencies]
your_crate_name = "0.1.0"
```

This will download and include your crate in their project.
