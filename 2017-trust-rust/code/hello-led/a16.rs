
use std::iter;

fn main() {
    let v1 = vec![10, 20, 30, 40, 50];
    
    let v2 = vec![60, 70, 80, 90, 100];

    // iter::once(x) creates a one-element iterator
    let v3:Vec<&i32> = v1.iter()
                       .chain(iter::once(&v2[0]))
                       .collect();

    println!("{:?}", v3);

}
