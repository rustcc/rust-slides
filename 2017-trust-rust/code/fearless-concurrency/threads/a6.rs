use std::thread;

fn fun() {

    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
            println!("vector v = {:?}", v);
    });

    handle.join();

    println!("v = {:?}", v);
}

fn main() {
    fun();
}
