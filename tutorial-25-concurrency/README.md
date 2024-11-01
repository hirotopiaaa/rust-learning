## Tutorial 25 - Concurrency

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
    * [Concurrency](#concurrency)
    * [Parallelism](#parallelism)
* [Notes](#notes)
    * [Creating Threads](#creating-threads)
    * [Message](#message)
    * [Sharing](#sharing)

<!-- vim-markdown-toc -->

### The Book

Chapter 16: Fearless Concurrency

### Summary

- Concurrency is the ability to run multiple parts of a program at the same time.
- Rust provides powerful concurrency primitives that make it easy to write safe and efficient concurrent code.
- The `std::thread` module allows you to create threads and run code concurrently.
- The `std::sync` module provides synchronization primitives like `Mutex`, `Arc`, and `RwLock` to share data between threads safely.
- Rust's ownership system ensures that concurrent code is free

Concurrency and parallelism are two concepts often used in the context of multitasking and performance optimization in programming, including in Rust. Here's a brief explanation of each:

#### Concurrency

- **Definition**: Concurrency is about dealing with multiple tasks at the same time. It doesn't necessarily mean that these tasks are running simultaneously; rather, it means that the tasks are making progress over time.
- **Example**: In Rust, concurrency can be achieved using asynchronous programming with `async`/`await` syntax, where tasks are interleaved and managed by an executor.
- **Use Case**: Useful when you have tasks that are I/O-bound or when you need to manage multiple tasks that can be paused and resumed.

#### Parallelism

- **Definition**: Parallelism is about performing multiple tasks simultaneously. This usually involves multiple processors or cores working on different tasks at the same time.
- **Example**: In Rust, parallelism can be achieved using threads with the `std::thread` module or using parallel iterators from the `rayon` crate.
- **Use Case**: Useful for CPU-bound tasks where you can split the work into independent chunks that can be processed in parallel.

### Notes

#### Creating Threads

Rust only includes one-to-one threading, which means that each thread corresponds to one operating system thread. This is different from languages like Go, which use a many-to-many threading model.

#### Message

#### Sharing
