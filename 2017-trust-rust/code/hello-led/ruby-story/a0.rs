
// Rust version of "all-blank" function

fn main() {
    let s = "             ";

    let r = s.chars().all(|c| c.is_whitespace());

    println!("{}", r);

}
