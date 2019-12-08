
fn main() {
    // a tuple
    let x = (1, 2, 3);
    
    let y = x;

    println!("{:?}", x);

    // tuple has "copy semantics", not "move semantics".
}
