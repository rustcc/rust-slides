
fn main() {
    let mut v = vec![1,2,3];
    let m = v.pop(); 
    match m {
        Some(x) => println!("{}", x),
        None    => println!("popped from empty list")
    }  
}
