use std::sync::{Arc, Mutex};
use std::thread;

pub fn execute() {
    println!("Threads in Rust");
    thread_example();

    println!("\n");
    println!("Memory management in Rust");
    memory_examole();
}

fn thread_example() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello from thread {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn memory_examole() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
