use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // multiple producer, single receiver
    let (sender, receiver) = channel();
    let mut gopher_counter = 0;

    // start 1000 OS threads, increment same counter
    for i in 0..1000 {
        thread::spawn(|| {
            gopher_counter += 1;
            sender.send(true).unwrap();
        });
    }
    
    // receive 1000 msgs to wait for threads to finish
    for _i in 0..1000 {
        receiver.recv().unwrap();
    }

    println!("{:?}", gopher_counter); // ???
}
