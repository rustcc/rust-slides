use std::sync::Mutex;
use std::thread;

fn main() {

    let m = Mutex::new(vec![0]);

    let handle = thread::spawn(move || {
        let mut v = m.lock().unwrap();
        v[0] += 1;
    });
 
    handle.join();   
    println!("{:?}", m);
}
