
fn main() {
    let x = 5;
    match x {
        1 => println!("one"),
        2...5 => println!("two to five"),
        6 => println!("three"),
        _ => println!("not one,two,three"),
    }
}
