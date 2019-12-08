
// Compile: rustc -O add.rs
const N: u64 = 1000000000;
fn main() {
    let r = (0..N).fold(0, |sum, i| sum+i);
    println!("{}", r);
}


