use std::thread;
use std::sync::Arc;


fn main() {
    let mut v1 = Arc::new(vec![10, 20, 30]);
    
    {
        let p = Arc::get_mut(&mut v1).unwrap();

        p[0] = 100;
    }

    println!("{:?}", v1);   
}
