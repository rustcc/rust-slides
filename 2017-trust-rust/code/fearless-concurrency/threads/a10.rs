use std::thread;
use std::sync::Arc;


fn main() {
    let v1 = Arc::new(vec![10, 20, 30]);
    let v2 = v1.clone();

    let handle1 = thread::spawn(move || {
            println!("{:?}", v1);
    });
    
    let handle2 = thread::spawn(move || {
            println!("{:?}", v2);
    });

    handle1.join();
    handle2.join();
    
}
