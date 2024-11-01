use std::thread;

fn main() {
    // --- Creating threads ---

    // Example 1
    // spawned thread
    let handle: thread::JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // main thread
    // when the main thread ends, the spawned thread will also end
    // regardless of whether it has finished or not
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    // wait for the spawned thread to finish
    // block the main thread until the spawned thread finishes
    handle.join().unwrap();

    // Example 2
    let v = vec![1, 2, 3];

    // enforce the closure to take ownership of the values it uses (v), use the move keyword
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // --- Message ---

    // --- Sharing ---
}
