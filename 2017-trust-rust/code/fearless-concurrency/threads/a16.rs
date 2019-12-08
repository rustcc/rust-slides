use std::sync::{Mutex, Arc};
use std::thread;

fn main() {

    let m1 = Arc::new(Mutex::new(vec![0]));
    let m2 = m1.clone();

    let handle = thread::spawn(move || {
        let mut v = m1.lock().unwrap();
        v[0] += 1;
    });
 
    handle.join();   
    println!("{:?}", m2);
}
