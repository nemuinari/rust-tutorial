// message passing
use std::sync::mpsc;
use std::thread;

pub fn execute() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("Received: {}", data);
    });

    let _ = tx.send("Hello from the main thread!");
    let _ = handle.join();
}
