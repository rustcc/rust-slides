
fn main() {
    let mut v = vec![];
    let m:Option<i32> = v.pop(); 
    match m {
        Some(x) => println!("{}", x),
        None    => println!("popped from empty list")
    }  
}
