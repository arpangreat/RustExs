// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Complete it
    println!("Result: {}", *counter.lock().unwrap());
}
