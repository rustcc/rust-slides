
fn main() {
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3...5 => println!("two to five"),
        6 => println!("three"),
        _ => println!("not one,two,three"),
    }
}
