
use std::io;

fn main() {
    let mut s = String::new();
    
    let r = io::stdin().read_line(&mut s);

    println!("{:?}", r);
    println!("{}", s);

}
