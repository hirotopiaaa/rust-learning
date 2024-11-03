use std::sync::{mpsc, Arc, Mutex};
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

    // Example 1
    let (tx, rx) = mpsc::channel();
    // clone the original sender
    let tx2 = tx.clone();

    thread::spawn(move || {
        // send single value
        // let val = String::from("hi");
        // send() takes ownership of val
        // tx.send(val).unwrap();
        // val is moved to the spawned thread
        // println!("val is {}", val); // error: value borrowed here after move

        // send multiple values
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // send() takes ownership of val
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // second spawned thread
    thread::spawn(move || {
        // send single value
        // let val = String::from("hi");
        // send() takes ownership of val
        // tx.send(val).unwrap();
        // val is moved to the spawned thread
        // println!("val is {}", val); // error: value borrowed here after move

        // send multiple values
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // send() takes ownership of val
            tx2.send(val).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // the try_recv() method does not block the main thread; it is useful to do other tasks while waiting (non-blocking)
    // the recv() method blocks the main thread until a value is sent down the channel
    // it returns a Result<T, E>

    // receive single value
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // receive multiple values
    for received in rx {
        println!("Got: {}", received);
    }

    // --- Sharing ---
    // Example 1
    // m is a mutex that holds an i32 value
    let m: Mutex<i32> = Mutex::new(5);

    {
        // lock the mutex
        // num is a smart pointer of type MutexGuard<i32>
        let mut num = m.lock().unwrap();
        // dereference num to get the value
        *num = 6;
        // release the lock when num goes out of scope (automatically)
    }

    println!("m = {:?}", m);

    // Example 2
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock the mutex
            let mut num = counter.lock().unwrap();
            // dereference num to get the value
            *num += 1;
        });

        handles.push(handle);
    }

    // join all the threads to make sure they finish before the main thread
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
