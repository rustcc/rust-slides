
fn main() {
    let s = "hello";
    
    for b in s.as_bytes() {
        println!("{}", b);
    }

    println!("------------");

    for c in s.chars() {
        println!("{}", c);
    }
}
