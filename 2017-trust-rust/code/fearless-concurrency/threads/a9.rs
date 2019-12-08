use std::thread;
use std::sync::Arc;


fn main() {
    let v1 = Arc::new(vec![10, 20, 30]);
    
    println!("refcount = {}", Arc::strong_count(&v1));

    {
        let v2 = v1.clone();
    
        println!("refcount = {}", Arc::strong_count(&v1));
    }

    println!("refcount = {}", Arc::strong_count(&v1));
    
}
