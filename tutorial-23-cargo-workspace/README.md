## Tutorial 23 - Cargo Workspace

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [What is a Cargo Workspace?](#what-is-a-cargo-workspace)
    * [Benefits of Using a Cargo Workspace](#benefits-of-using-a-cargo-workspace)
    * [Setting Up a Cargo Workspace](#setting-up-a-cargo-workspace)
    * [Example Structure](#example-structure)
    * [Managing Dependencies](#managing-dependencies)
    * [Running Commands in a Workspace](#running-commands-in-a-workspace)
    * [Inter-Crate Dependencies](#inter-crate-dependencies)

<!-- vim-markdown-toc -->

### The Book

Chapter 14: Cargo and Crates.io

### Summary

Cargo `workspace` in Rust is a feature that allows you to manage multiple related packages (also known as crates) under a single workspace. It’s particularly useful when working on large projects with multiple components that need to share code, configuration, or dependencies.

Cargo workspaces are powerful for managing multi-crate projects, sharing code, and maintaining consistent dependencies. They streamline development by providing a unified view and control over multiple related crates.

### Notes

#### What is a Cargo Workspace?

A Cargo workspace is a directory containing multiple Rust crates, sharing a common Cargo.toml file. The primary Cargo.toml is at the root of the workspace, and it manages all the packages (sub-crates) within the workspace. Each sub-crate has its own directory and can have its own Cargo.toml, but the workspace-level Cargo.toml defines common dependencies and settings.

#### Benefits of Using a Cargo Workspace

- Code Sharing: Easily share code between crates without publishing them separately.
- Dependency Management: Manage dependencies centrally at the workspace level, reducing duplication.
- Unified Build: Build, test, and run all crates in the workspace with a single command.
- Consistency: Ensures that all crates are using compatible versions of dependencies.

#### Setting Up a Cargo Workspace

To create a workspace, follow these steps:

1. Create a new directory for your workspace (e.g., my_workspace).
2. Inside this directory, create a Cargo.toml file (the workspace root Cargo.toml):

```toml
[workspace]
members = [
    "crate1",
    "crate2",
]
```

The `[workspace]` section defines the workspace members (the sub-crates).

3. Add sub-crates:

- For each sub-crate, create a new directory (e.g., crate1, crate2).
- Inside each directory, run cargo init to set up individual crates.
- Ensure that the workspace root Cargo.toml lists each sub-crate’s directory name as a member.

#### Example Structure

```console
my_workspace/
│
├── Cargo.toml   (Workspace-level configuration)
├── crate1/
│   └── Cargo.toml
│   └── src/
│       └── lib.rs
├── crate2/
│   └── Cargo.toml
│   └── src/
│       └── lib.rs
```

#### Managing Dependencies

Common dependencies can be added to the workspace root Cargo.toml. Each sub-crate will automatically inherit these dependencies:

```toml
[workspace]
members = ["crate1", "crate2"]

[dependencies]
serde = "1.0"
```

#### Running Commands in a Workspace

- To build all crates: `cargo build`
- To run tests for all crates: `cargo test`
- You can also target a specific crate with `-p`, like `cargo build -p crate1`.

#### Inter-Crate Dependencies

Sub-crates within a workspace can depend on each other without needing to be published to crates.io. For example, if crate2 depends on crate1, you can specify it in crate2's Cargo.toml:

```toml
[dependencies]
crate1 = { path = "../crate1" }
```
