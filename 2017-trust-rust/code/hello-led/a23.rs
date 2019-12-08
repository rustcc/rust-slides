
// Difference between debug and release build
// rustc -O a23.rs
// Emit asm: rustc -O --emit=asm a23.rs

// Mention play.rust-lang.org and
// rust.godbolt.org

const N: u64 = 1000_000_000;
fn main() {
    let r = (0..N)
            .map(|x| x + 1)
            .fold(0, |sum, i| sum+i);

    println!("{}", r);
}


