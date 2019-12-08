
const N: u64 = 100_000_000;

fn main() {
    let r = (0..N)
            .map(|x| x + 1)
            .fold(0, |sum, i| sum+i);
    
    println!("{}", r);

    // Compile in both debug and release modes,
    // and watch the fun!

    // Iterators/closure and most other abstractions
    // in Rust compile to very tight machine code! 
    // You can use them without worry on the tiniest
    // micrcocontroller!
}


