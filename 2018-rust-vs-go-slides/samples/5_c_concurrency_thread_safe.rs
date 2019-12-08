use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let (sender, receiver) = channel();
    let gopher_counter = 0;
    
    // Wrap data in a mutex lock in an atomic ref counter...
    let atomic_ref_counter_with_mutex = Arc::new(Mutex::new(gopher_counter));

    for _ in 0..1000 {
        // cloning ARC bumps counter up and gives a thread safe passable ref
        let atomic_ref_counter_with_mutex = atomic_ref_counter_with_mutex.clone();
        let sender = sender.clone();
        thread::spawn(move || {
            // safely mutate shared data after locking
            let mut gopher_counter = atomic_ref_counter_with_mutex.lock().unwrap();
            *gopher_counter = *gopher_counter + 1;
            
            sender.send(true).unwrap();
        }); // locks are dropped/unlocked at end of ownership scope
    }

    for _ in 0..1000 {
        receiver.recv().unwrap();
    }
    
    let gopher_counter = atomic_ref_counter_with_mutex.lock().unwrap();
    println!("{:?}", *gopher_counter);
}
