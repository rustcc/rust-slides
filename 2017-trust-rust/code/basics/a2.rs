
fn sqr(x: u32) -> u32 {
    println!("called sqr...");
    x*x
}

fn main() {
    let x = sqr(10);
    println!("{}", x);
}
    
