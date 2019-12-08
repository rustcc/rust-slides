use std::sync::Mutex;


fn main() {

    let m = Mutex::new(vec![1, 2, 3]);

    {
        let mut v = m.lock().unwrap();
    
        v[0] = 10;
    }


    println!("{:?}", m);
}
