use std::sync::{Mutex, Arc};
use std::thread;

fn main() {

    let m1 = Arc::new(Mutex::new(vec![0]));
    let m2 = m1.clone();
    let m3 = m1.clone();

    let handle1 = thread::spawn(move || {
        for i in 0..1000_000 {
            let mut v = m2.lock().unwrap();
            v[0] += 1;
        }
    });
    
    let handle2 = thread::spawn(move || {
        for i in 0..1000_000 {
            let mut v = m3.lock().unwrap();
            v[0] += 1;
        }
    });
 
    handle1.join();   
    handle2.join();   

    println!("{:?}", m1);
}
