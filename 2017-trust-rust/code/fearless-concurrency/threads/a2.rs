use std::{thread, time};

fn main() {

    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the first spawned thread!", i);
            thread::sleep(time::Duration::from_millis(10));
        }
    });
    
    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the second spawned thread!", i);
            thread::sleep(time::Duration::from_millis(10));
        }
    });

    handle1.join();
    handle2.join();

    println!("main is ending ....");
}

