
fn main() {
    let mut v = vec![10, 20];

    let a = v.pop();

    match a {
        Some(x) => println!("popped: {}", x),
        None => println!("stack empty"),
    }
    
}
