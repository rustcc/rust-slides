use std::thread;
use std::sync::Arc;


fn main() {
    let v1 = Arc::new(vec![1,2,3]);
    let v2 = v1.clone();

    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
