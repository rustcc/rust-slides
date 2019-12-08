use std::thread;

fn fun() {

    let v1 = vec![1, 2, 3, 4];
    let v2 = v1.clone();

    let handle1 = thread::spawn(move || {
            println!("vector v = {:?}", v1);
    });

    let handle2 = thread::spawn(move || {
            println!("vector v = {:?}", v2);
    });

    handle1.join();
    handle2.join();

}

fn main() {
    fun();
}
