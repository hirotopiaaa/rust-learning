## Tutorial 25 - Fearless Concurrency

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

A channel is a way to send messages between threads. In Rust, `use std::sync::mpsc`, mpsc stands for multiple producer, single consumer. This module provides a way to create channels for communication between threads. The `Sender` and `Receiver` types allow you to send messages between threads in a thread-safe way. Only one receiver can receive messages from a channel, but multiple senders can send messages.

A channel will close when all senders or all receivers are dropped (i.e., go out of scope or are explicitly dropped).

#### Sharing

Share states refers to the ability to have multiple threads access the same data. Rust provides synchronization primitives to ensure that shared data is accessed safely. These primitives include:

- **Mutex**: A mutual exclusion primitive that allows only one thread to access the data at a time.
- **RwLock**: A reader-writer lock that allows multiple readers or one writer to access the data at a time.
- **Arc**: An atomic reference-counted type that allows shared ownership between threads.

One-way data flow is a common pattern in Rust for concurrent programming, where data from a thread is passed to another other thread using message passing rather than shared mutable state. The receiving thread takes ownership of the data, ensuring that only one thread can modify it at a time. Whereas, in shared mutable state concurrency models, multiple threads can access and modify the same data concurrently.

`Mutax` is a mutual exclusion primitive that allows only one thread to access the data at a time. It provides a lock that must be acquired before accessing the data and released after the access is complete. This ensures that only one thread can access and modify the data at a given time. A lock is a data structure that keeps track of whether the data is currently being accessed by a thread. If the data is locked, other threads must wait until the lock is released before they can access the data. Once the thread is done with the data, it can unlock it, allowing other threads to access it. Rust ensures that locks are released correctly, preventing deadlocks thanks to its ownership and borrowing rules.

Mutax uses interior mutability, which allows you to mutate data even when it's immutably borrowed. This is useful when you need to modify data that's shared between threads. However, Mutax enforces borrowing rules at runtime rather than compile time, which can lead to panics if the rules are violated.

Atomic is a concurrency primitive that provides atomic operations, meaning that they are guaranteed to be executed without interruption. This is important in concurrent programming, where multiple threads may try to access and modify the same data at the same time. Atomic operations ensure that the data is updated correctly and consistently, even in the presence of concurrent access.

`Arc` is an atomic reference-counted type that allows shared ownership between threads. It keeps track of the number of references to a value and automatically cleans up the value when the last reference is dropped. This allows multiple threads to share ownership of the same data without worrying about memory safety issues. `Arc` is useful when you need to share data between threads and ensure that it's cleaned up correctly when no longer needed.
