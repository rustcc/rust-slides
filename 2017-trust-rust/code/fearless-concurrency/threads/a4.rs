use std::thread;

fn fun() {

    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(|| {
            println!("vector v = {:?}", v);
    });

    handle.join();

}

fn main() {
    fun();
}
